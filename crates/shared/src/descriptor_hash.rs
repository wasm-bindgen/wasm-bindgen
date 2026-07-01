//! Content-addressed hashing for the `__wasm_bindgen_descriptors`
//! custom section (see [`crate::DESCRIPTORS_SECTION_NAME`]).
//!
//! Every schema node in the section is identified by a 128-bit content
//! hash: `id = H(tag, words, child_ids)`. Because the hash folds in the
//! ids of a node's children, it is a Merkle-style identifier — two
//! structurally-identical types produce the same id in **every** crate,
//! with no global registry or linker coordination. That makes cross-node
//! references (a wrapper pointing at its inner type) and cross-fragment
//! references (a descriptor pointing at its root) resolvable purely by
//! id, and makes duplicate emissions (the same leaf type mentioned by
//! thousands of shims, or by several independently-compiled crates)
//! collapse for free when the consumer unions everything into a single
//! `id -> node` map.
//!
//! The hash deliberately excludes a node's closure invoke reference: two
//! closures with the same signature share an id (they are the same type),
//! and the invoke shim is resolved separately by name.
//!
//! # Why FNV-1a
//!
//! The producer computes ids in a `const fn` on the wasm-bindgen MSRV
//! (1.77), where no hashing crate is available and `for`/iterator
//! combinators are not yet `const`. FNV-1a is a handful of `while`-loop
//! XOR/multiply steps over bytes, so it evaluates cleanly at const time
//! while giving good dispersion. 128 bits keeps the birthday-bound
//! collision probability negligible even for very large programs, which
//! matters because a collision would silently conflate two ABI types.

/// FNV-1a 128-bit offset basis.
pub const FNV_OFFSET_128: u128 = 0x6c62272e07bb014262b821756295c58d;

/// FNV-1a 128-bit prime.
pub const FNV_PRIME_128: u128 = 0x0000000001000000000000000000013B;

/// Start a new content hash.
#[inline]
pub const fn init() -> u128 {
    FNV_OFFSET_128
}

/// Fold one byte into the running hash.
#[inline]
pub const fn update_u8(mut h: u128, b: u8) -> u128 {
    h ^= b as u128;
    h.wrapping_mul(FNV_PRIME_128)
}

/// Fold a little-endian `u32` (four bytes) into the running hash.
pub const fn update_u32(mut h: u128, w: u32) -> u128 {
    let bytes = w.to_le_bytes();
    let mut i = 0;
    while i < bytes.len() {
        h = update_u8(h, bytes[i]);
        i += 1;
    }
    h
}

/// Fold a little-endian `u128` (sixteen bytes) into the running hash.
/// Used to mix in a child node's id.
pub const fn update_u128(mut h: u128, v: u128) -> u128 {
    let bytes = v.to_le_bytes();
    let mut i = 0;
    while i < bytes.len() {
        h = update_u8(h, bytes[i]);
        i += 1;
    }
    h
}

/// Fold a byte slice (length-prefixed) into the running hash.
pub const fn update_bytes(mut h: u128, bytes: &[u8]) -> u128 {
    h = update_u32(h, bytes.len() as u32);
    let mut i = 0;
    while i < bytes.len() {
        h = update_u8(h, bytes[i]);
        i += 1;
    }
    h
}

/// Compute a node id from its structural content.
///
/// `tag`/`words`/`child_ids` are hashed length-prefixed so that, for
/// example, `[a]` with no children and `[]` with one child `a` can never
/// collide. The consumer stores ids as opaque 128-bit keys and never has
/// to recompute this, but the algorithm is shared so a future validation
/// pass (or a differently-implemented producer) stays in agreement.
pub const fn node_id(tag: u32, words: &[u32], child_ids: &[u128]) -> u128 {
    let mut h = init();
    h = update_u32(h, tag);
    h = update_u32(h, words.len() as u32);
    let mut i = 0;
    while i < words.len() {
        h = update_u32(h, words[i]);
        i += 1;
    }
    h = update_u32(h, child_ids.len() as u32);
    let mut j = 0;
    while j < child_ids.len() {
        h = update_u128(h, child_ids[j]);
        j += 1;
    }
    h
}
