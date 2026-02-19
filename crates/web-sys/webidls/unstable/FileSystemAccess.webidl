enum FileSystemPermissionMode {
  "read",
  "readwrite"
};

dictionary FileSystemPermissionDescriptor : PermissionDescriptor {
  required FileSystemHandle handle;
  FileSystemPermissionMode mode = "read";
};

dictionary FileSystemHandlePermissionDescriptor {
  FileSystemPermissionMode mode = "read";
};

[Exposed=(Window,Worker), SecureContext, Serializable]
partial interface FileSystemHandle {
  Promise<PermissionState> queryPermission(optional FileSystemHandlePermissionDescriptor descriptor = {});
  Promise<PermissionState> requestPermission(optional FileSystemHandlePermissionDescriptor descriptor = {});
};

enum WellKnownDirectory {
  "desktop",
  "documents",
  "downloads",
  "music",
  "pictures",
  "videos",
};

typedef (WellKnownDirectory or FileSystemHandle) StartInDirectory;

dictionary FilePickerAcceptType {
    USVString description = "";
    record<USVString, (USVString or sequence<USVString>)> accept;
};

dictionary FilePickerOptions {
    sequence<FilePickerAcceptType> types;
    boolean excludeAcceptAllOption = false;
    DOMString id;
    StartInDirectory startIn;
};

dictionary OpenFilePickerOptions : FilePickerOptions {
    boolean multiple = false;
};

dictionary SaveFilePickerOptions : FilePickerOptions {
    USVString? suggestedName;
};

dictionary DirectoryPickerOptions {
    DOMString id;
    StartInDirectory startIn;
    FileSystemPermissionMode mode = "read";
};

[SecureContext]
partial interface Window {
    [Throws]
    Promise<sequence<FileSystemFileHandle>> showOpenFilePicker(optional OpenFilePickerOptions options = {});
    [Throws]
    Promise<FileSystemFileHandle> showSaveFilePicker(optional SaveFilePickerOptions options = {});
    [Throws]
    Promise<FileSystemDirectoryHandle> showDirectoryPicker(optional DirectoryPickerOptions options = {});
};

partial interface DataTransferItem {
    Promise<FileSystemHandle?> getAsFileSystemHandle();
};

// Non-standard extension for createSyncAccessHandle with options
// https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createSyncAccessHandle#mode
enum FileSystemSyncAccessHandleMode {
    "read-only",
    "readwrite",
    "readwrite-unsafe"
};

dictionary FileSystemSyncAccessHandleOptions {
    FileSystemSyncAccessHandleMode mode = "readwrite";
};

[Exposed=DedicatedWorker, SecureContext]
partial interface FileSystemFileHandle {
    Promise<FileSystemSyncAccessHandle> createSyncAccessHandle();
    Promise<FileSystemSyncAccessHandle> createSyncAccessHandle(FileSystemSyncAccessHandleOptions options);
};
