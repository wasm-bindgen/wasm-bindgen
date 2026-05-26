// The producer side (#[wasm_bindgen] macro) and the wiring into
// `descriptors::execute` land in later commits. Until then this module is
// reachable only from its unit tests, so silence the unused warnings.
#![allow(dead_code)]

//! Parser for the `__wasm_bindgen_descriptors` custom section.
//!
//! This is the consumer side of the new descriptor transport that is
//! replacing the wasm interpreter in `crates/cli-support/src/interpreter/`.
//! See `wasm_bindgen_shared::DESCRIPTORS_SECTION_NAME` for the on-wire
//! format.
//!
//! Responsibilities:
//!
//! 1. Decode the raw section bytes into a list of [`Entry`] records.
//! 2. Resolve any `SYMBOL_REF` slots in each entry's schema stream against
//!    a caller-supplied `HashMap<String, u32>` of symbol-name → resolved
//!    `u32` value (typically a function-table index).
//! 3. Hand back a `&[u32]` stream that is byte-compatible with what the
//!    interpreter previously produced, so the existing
//!    `Descriptor::decode` consumes it unchanged.
//!
//! This module is deliberately self-contained: it has no dependency on
//! `walrus` so it can be unit-tested with hand-built byte arrays.
//! Plumbing into the wider cli-support pipeline lives in
//! `crates/cli-support/src/descriptors.rs`.

use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use wasm_bindgen_shared::{
    tys::SYMBOL_REF, DESCRIPTOR_KIND_CAST, DESCRIPTOR_KIND_REGULAR,
};

/// One descriptor entry recovered from the section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Name of the shim this descriptor describes. For regular entries
    /// this matches the suffix of a former `__wbindgen_describe_<name>`
    /// synthetic export; for cast entries it is the symbol name of the
    /// `breaks_if_inlined::<From, To>` monomorphization.
    pub name: String,
    /// Kind discriminator. See [`wasm_bindgen_shared::DESCRIPTOR_KIND_REGULAR`].
    pub kind: u8,
    /// Schema stream encoded as little-endian `u32`s, exactly the bytes
    /// the legacy interpreter would have produced. `SYMBOL_REF` opcodes
    /// (followed by a length-prefixed UTF-8 symbol name padded to 4 bytes)
    /// remain in this stream and must be resolved by
    /// [`resolve_symbols`] before feeding into `Descriptor::decode`.
    pub schema_bytes: Vec<u8>,
}

/// Parse the raw bytes of a `__wasm_bindgen_descriptors` custom section
/// into a list of [`Entry`] records.
///
/// Format (also documented on `DESCRIPTORS_SECTION_NAME`):
///
/// ```text
/// section bytes:
///   u32 LE  total_len
///   entry repeated until total_len bytes consumed:
///     u8        shim_name_len
///     [u8; n]   shim_name
///     u8        kind
///     u32 LE    schema_word_count
///     [u32 LE]  schema (schema_word_count * 4 bytes)
/// ```
pub fn parse(section: &[u8]) -> Result<Vec<Entry>> {
    let mut r = Reader::new(section);
    let total_len = r.u32()? as usize;
    let body_start = r.pos;
    let body_end = body_start
        .checked_add(total_len)
        .ok_or_else(|| anyhow!("descriptor section total_len overflows"))?;
    if body_end > section.len() {
        bail!(
            "descriptor section total_len ({total_len}) exceeds section size ({})",
            section.len() - body_start
        );
    }

    let mut entries = Vec::new();
    while r.pos < body_end {
        let name_len = r.u8()? as usize;
        if name_len == 0 {
            bail!("descriptor entry has empty shim name");
        }
        let name_bytes = r.bytes(name_len)?;
        let name = std::str::from_utf8(name_bytes)
            .map_err(|e| anyhow!("descriptor entry shim name is not UTF-8: {e}"))?
            .to_owned();
        let kind = r.u8()?;
        if kind != DESCRIPTOR_KIND_REGULAR && kind != DESCRIPTOR_KIND_CAST {
            bail!("descriptor entry for {name:?} has unknown kind byte {kind}");
        }
        let word_count = r.u32()? as usize;
        let byte_count = word_count
            .checked_mul(4)
            .ok_or_else(|| anyhow!("descriptor schema_word_count overflows"))?;
        let schema_bytes = r.bytes(byte_count)?.to_vec();
        entries.push(Entry {
            name,
            kind,
            schema_bytes,
        });
    }
    if r.pos != body_end {
        bail!(
            "descriptor section trailer mismatch: parsed up to {} but body ends at {body_end}",
            r.pos
        );
    }
    Ok(entries)
}

/// Walk a schema byte stream and replace any `SYMBOL_REF` opcode plus its
/// inline name payload with the resolved `u32` value from `resolved`.
///
/// On input the stream is a sequence of little-endian `u32` words. When the
/// word `SYMBOL_REF` is encountered, the following layout is expected:
///
/// ```text
/// SYMBOL_REF  (u32)
/// name_len    (u32)         // length in bytes of the UTF-8 symbol name
/// name_bytes  ([u8; n], padded to 4-byte alignment with zeros)
/// ```
///
/// The output stream replaces those words with a single resolved `u32`,
/// matching the layout the legacy interpreter would have produced (a raw
/// function-table index in place of the original `i32.const N; call
/// $__wbindgen_describe` pair).
pub fn resolve_symbols(
    schema_bytes: &[u8],
    resolved: &HashMap<String, u32>,
) -> Result<Vec<u32>> {
    if !schema_bytes.len().is_multiple_of(4) {
        bail!(
            "schema byte length {} is not a multiple of 4",
            schema_bytes.len()
        );
    }
    let mut out = Vec::with_capacity(schema_bytes.len() / 4);
    let mut r = Reader::new(schema_bytes);
    while r.pos < schema_bytes.len() {
        let word = r.u32()?;
        if word != SYMBOL_REF {
            out.push(word);
            continue;
        }
        let name_len = r.u32()? as usize;
        if name_len == 0 {
            bail!("SYMBOL_REF entry has empty name");
        }
        let name_bytes = r.bytes(name_len)?;
        let name = std::str::from_utf8(name_bytes)
            .map_err(|e| anyhow!("SYMBOL_REF name is not UTF-8: {e}"))?;
        let padding = (4 - (name_len % 4)) % 4;
        let pad = r.bytes(padding)?;
        if pad.iter().any(|&b| b != 0) {
            bail!("SYMBOL_REF padding bytes for {name:?} are not zero");
        }
        let value = resolved.get(name).copied().ok_or_else(|| {
            anyhow!("SYMBOL_REF target {name:?} was not found in the wasm module")
        })?;
        out.push(value);
    }
    Ok(out)
}

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Reader { buf, pos: 0 }
    }
    fn need(&self, n: usize) -> Result<()> {
        if self.pos.saturating_add(n) > self.buf.len() {
            bail!(
                "unexpected end of descriptor section at offset {} (need {n} bytes, have {})",
                self.pos,
                self.buf.len() - self.pos
            );
        }
        Ok(())
    }
    fn u8(&mut self) -> Result<u8> {
        self.need(1)?;
        let b = self.buf[self.pos];
        self.pos += 1;
        Ok(b)
    }
    fn u32(&mut self) -> Result<u32> {
        self.need(4)?;
        let arr = [
            self.buf[self.pos],
            self.buf[self.pos + 1],
            self.buf[self.pos + 2],
            self.buf[self.pos + 3],
        ];
        self.pos += 4;
        Ok(u32::from_le_bytes(arr))
    }
    fn bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        self.need(n)?;
        let out = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_shared::tys;

    /// Pack a single regular entry's body (without the outer total_len).
    fn entry_bytes(name: &str, kind: u8, schema_words: &[u32]) -> Vec<u8> {
        let mut b = Vec::new();
        b.push(name.len() as u8);
        b.extend_from_slice(name.as_bytes());
        b.push(kind);
        b.extend_from_slice(&(schema_words.len() as u32).to_le_bytes());
        for w in schema_words {
            b.extend_from_slice(&w.to_le_bytes());
        }
        b
    }

    fn section(entries: &[Vec<u8>]) -> Vec<u8> {
        let body: Vec<u8> = entries.iter().flatten().copied().collect();
        let mut out = (body.len() as u32).to_le_bytes().to_vec();
        out.extend(body);
        out
    }

    #[test]
    fn parses_empty_section() {
        let bytes = section(&[]);
        let entries = parse(&bytes).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn parses_single_regular_entry() {
        // Equivalent to: a `pub fn foo(x: i32) -> i32` schema.
        //   FUNCTION, shim_idx=0, nargs=1, I32 (arg), I32 (ret)
        let words = [tys::FUNCTION, 0, 1, tys::I32, tys::I32];
        let bytes = section(&[entry_bytes("foo", DESCRIPTOR_KIND_REGULAR, &words)]);
        let entries = parse(&bytes).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "foo");
        assert_eq!(entries[0].kind, DESCRIPTOR_KIND_REGULAR);
        let resolved =
            resolve_symbols(&entries[0].schema_bytes, &HashMap::new()).unwrap();
        assert_eq!(resolved, words);
    }

    #[test]
    fn parses_multiple_entries() {
        let a = entry_bytes("a", DESCRIPTOR_KIND_REGULAR, &[tys::I32]);
        let b = entry_bytes("bb", DESCRIPTOR_KIND_CAST, &[tys::U32, tys::U32]);
        let bytes = section(&[a, b]);
        let entries = parse(&bytes).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "a");
        assert_eq!(entries[0].kind, DESCRIPTOR_KIND_REGULAR);
        assert_eq!(entries[1].name, "bb");
        assert_eq!(entries[1].kind, DESCRIPTOR_KIND_CAST);
    }

    #[test]
    fn resolves_symbol_ref() {
        // Schema: REF, SYMBOL_REF, name_len=4, "shim", I32
        let name = "shim";
        let name_len = name.len() as u32;
        let mut schema = Vec::new();
        schema.extend_from_slice(&tys::REF.to_le_bytes());
        schema.extend_from_slice(&SYMBOL_REF.to_le_bytes());
        schema.extend_from_slice(&name_len.to_le_bytes());
        schema.extend_from_slice(name.as_bytes());
        // Already 4-byte aligned (len 4).
        schema.extend_from_slice(&tys::I32.to_le_bytes());

        let mut map = HashMap::new();
        map.insert("shim".to_string(), 17);
        let out = resolve_symbols(&schema, &map).unwrap();
        assert_eq!(out, [tys::REF, 17, tys::I32]);
    }

    #[test]
    fn resolves_symbol_ref_with_padding() {
        // 5-byte name needs 3 bytes of zero padding.
        let name = "abcde";
        let mut schema = Vec::new();
        schema.extend_from_slice(&SYMBOL_REF.to_le_bytes());
        schema.extend_from_slice(&(name.len() as u32).to_le_bytes());
        schema.extend_from_slice(name.as_bytes());
        schema.extend_from_slice(&[0, 0, 0]); // padding
        let mut map = HashMap::new();
        map.insert("abcde".to_string(), 99);
        let out = resolve_symbols(&schema, &map).unwrap();
        assert_eq!(out, [99]);
    }

    #[test]
    fn unresolved_symbol_is_an_error() {
        let mut schema = Vec::new();
        schema.extend_from_slice(&SYMBOL_REF.to_le_bytes());
        schema.extend_from_slice(&4u32.to_le_bytes());
        schema.extend_from_slice(b"shim");
        let err = resolve_symbols(&schema, &HashMap::new()).unwrap_err();
        assert!(err.to_string().contains("shim"));
    }

    #[test]
    fn unknown_kind_is_rejected() {
        let mut body = entry_bytes("x", DESCRIPTOR_KIND_REGULAR, &[]);
        // Overwrite the kind byte (immediately after name).
        body[1 + "x".len()] = 99;
        let bytes = section(&[body]);
        let err = parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("unknown kind"));
    }

    #[test]
    fn truncated_section_is_rejected() {
        let bytes = section(&[entry_bytes(
            "x",
            DESCRIPTOR_KIND_REGULAR,
            &[tys::I32, tys::I32],
        )]);
        let err = parse(&bytes[..bytes.len() - 1]).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("exceeds section size") || msg.contains("unexpected end"),
            "unexpected truncation error message: {msg}"
        );
    }

    #[test]
    fn truncated_mid_entry_is_rejected() {
        // Build a section whose total_len is honest but whose entry body is
        // cut short mid-schema (5 bytes lopped off the tail), so the inner
        // reader hits "unexpected end" rather than the section-size check.
        let entry = entry_bytes(
            "x",
            DESCRIPTOR_KIND_REGULAR,
            &[tys::I32, tys::I32],
        );
        let truncated_entry = &entry[..entry.len() - 5];
        let mut bytes = (truncated_entry.len() as u32).to_le_bytes().to_vec();
        bytes.extend_from_slice(truncated_entry);
        let err = parse(&bytes).unwrap_err();
        assert!(
            err.to_string().contains("unexpected end"),
            "got: {err}"
        );
    }

    #[test]
    fn rejects_misaligned_schema() {
        let err = resolve_symbols(&[1, 2, 3], &HashMap::new()).unwrap_err();
        assert!(err.to_string().contains("not a multiple of 4"));
    }
}
