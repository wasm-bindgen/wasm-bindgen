#![cfg(js_sys_unstable_apis)]
//! # JSPI + OPFS Example
//!
//! Demonstrates using JSPI to call the normally-async
//! [Origin Private File System](https://developer.mozilla.org/en-US/docs/Web/API/File_System_API/Origin_private_file_system)
//! (OPFS) APIs from plain (non-`async`) Rust.
//!
//! Paths may contain `/`-separated components; the library automatically
//! creates intermediate directories as needed (on write paths).
//!
//! All functions work in both `Window` and `DedicatedWorkerGlobalScope`
//! contexts so the same WASM module can be used from both the main page
//! and a Web Worker.
//!
//! ## Exports
//!
//! | function | description |
//! |---|---|
//! | `opfs_write(path, content)` | write UTF-8 text to `path` |
//! | `opfs_read(path) → String` | read UTF-8 text from `path` |
//! | `opfs_has(path) → bool` | `true` if the file exists |
//! | `opfs_delete(path)` | delete the file at `path` |

use js_sys::futures::jspi::block_on_promise;
use wasm_bindgen::prelude::*;
use web_sys::{
    FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemGetDirectoryOptions,
    FileSystemGetFileOptions,
};

// ─── navigator.storage helper ────────────────────────────────────────────────
//
// Uses Reflect so this works in both Window (web_sys::Navigator) and
// DedicatedWorkerGlobalScope (web_sys::WorkerNavigator) without importing
// both types.

fn opfs_root() -> FileSystemDirectoryHandle {
    use wasm_bindgen::JsCast;

    let global = js_sys::global(); // Object<JsValue>
    let navigator: JsValue = js_sys::Reflect::get(&global, &js_sys::JsString::from("navigator"))
        .expect_throw("Reflect.get failed on global")
        .expect_throw("no navigator in global scope");
    let navigator_obj = navigator.unchecked_ref::<js_sys::Object>();
    let storage: web_sys::StorageManager =
        js_sys::Reflect::get(navigator_obj, &js_sys::JsString::from("storage"))
            .expect_throw("Reflect.get failed on navigator")
            .expect_throw("no storage on navigator")
            .dyn_into()
            .expect_throw("expected StorageManager");

    block_on_promise(&storage.get_directory())
        .expect_throw("getDirectory() failed — must run in a secure context")
        .dyn_into()
        .expect_throw("expected FileSystemDirectoryHandle")
}

// ─── Path traversal ──────────────────────────────────────────────────────────
//
// OPFS `getFileHandle` / `removeEntry` only accept a bare name (no slashes).
// For paths like "a/b/c.txt" we must traverse "a" → "b" first.

/// Traverse path components, creating directories that don't exist.
/// Used by write operations.
fn resolve_path_create(path: &str) -> (FileSystemDirectoryHandle, String) {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    // throw_val returns `!` so the None arm satisfies the match type.
    let (file_name, dir_parts) = match parts.split_last() {
        Some(x) => x,
        None => wasm_bindgen::throw_val(JsValue::from_str("opfs path must not be empty")),
    };

    let mut dir = opfs_root();
    for &component in dir_parts {
        let opts = FileSystemGetDirectoryOptions::new();
        opts.set_create(true);
        dir = block_on_promise(&dir.get_directory_handle_with_options(component, &opts))
            .expect_throw("getDirectoryHandle() failed")
            .dyn_into()
            .expect_throw("expected FileSystemDirectoryHandle");
    }

    (dir, (*file_name).to_string())
}

/// Traverse path components without creating anything.
/// Returns `Ok((dir, name))` or `Err(())` if any directory component is missing.
/// Used by read/has/delete operations.
fn try_resolve_path(path: &str) -> Result<(FileSystemDirectoryHandle, String), ()> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let (file_name, dir_parts) = parts.split_last().ok_or(())?;

    let mut dir = opfs_root();
    for &component in dir_parts {
        match block_on_promise(&dir.get_directory_handle(component)) {
            Ok(val) => {
                dir = val
                    .dyn_into()
                    .expect_throw("expected FileSystemDirectoryHandle");
            }
            Err(_) => return Err(()),
        }
    }

    Ok((dir, (*file_name).to_string()))
}

// ─────────────────────────────────────────────────────────────────────────────

/// Write UTF-8 `content` to `path` in the Origin Private File System.
///
/// Creates intermediate directories and the file if they do not exist;
/// overwrites the file if it does.
#[wasm_bindgen(jspi)]
pub fn opfs_write(path: String, content: String) {
    let (dir, name) = resolve_path_create(&path);

    let opts = FileSystemGetFileOptions::new();
    opts.set_create(true);

    let file_handle: FileSystemFileHandle =
        block_on_promise(&dir.get_file_handle_with_options(&name, &opts))
            .expect_throw("getFileHandle() failed")
            .dyn_into()
            .expect_throw("expected FileSystemFileHandle");

    let writable: web_sys::FileSystemWritableFileStream =
        block_on_promise(&file_handle.create_writable())
            .expect_throw("createWritable() failed")
            .dyn_into()
            .expect_throw("expected FileSystemWritableFileStream");

    block_on_promise(
        &writable
            .write_with_str(&content)
            .expect_throw("write() argument error"),
    )
    .expect_throw("write() failed");

    block_on_promise(&web_sys::WritableStream::close(&writable)).expect_throw("close() failed");
}

/// Read and return the UTF-8 text content of `path`.
/// Throws if the path does not exist.
#[wasm_bindgen(jspi)]
pub fn opfs_read(path: String) -> String {
    let (dir, name) =
        try_resolve_path(&path).expect_throw("path not found (directory component missing)");

    let file_handle: FileSystemFileHandle = block_on_promise(&dir.get_file_handle(&name))
        .expect_throw("getFileHandle() failed — file may not exist")
        .dyn_into()
        .expect_throw("expected FileSystemFileHandle");

    let file: web_sys::File = block_on_promise(&file_handle.get_file())
        .expect_throw("getFile() failed")
        .dyn_into()
        .expect_throw("expected File");

    block_on_promise(&web_sys::Blob::text(&file))
        .expect_throw("text() failed")
        .as_string()
        .unwrap_or_default()
}

/// Return `true` if `path` exists in the Origin Private File System.
/// Returns `false` if any path component (directory or file) is missing.
#[wasm_bindgen(jspi)]
pub fn opfs_has(path: String) -> bool {
    let (dir, name) = match try_resolve_path(&path) {
        Ok(x) => x,
        Err(()) => return false,
    };
    block_on_promise(&dir.get_file_handle(&name)).is_ok()
}

/// Delete the file at `path`.  Throws if the path does not exist.
#[wasm_bindgen(jspi)]
pub fn opfs_delete(path: String) {
    let (dir, name) =
        try_resolve_path(&path).expect_throw("path not found (directory component missing)");
    block_on_promise(&dir.remove_entry(&name))
        .expect_throw("removeEntry() failed — file may not exist");
}
