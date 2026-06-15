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
//! 2. Expose each entry's schema as a `Vec<u32>` ([`Entry::schema_words`])
//!    that feeds directly into [`crate::descriptor::Descriptor::decode_with_symbols`].
//!
//! Symbol (`SYMBOL_REF`) resolution and grammar decoding are *not* done
//! here: they share a single traversal in `crate::descriptor`, so the
//! schema grammar lives in exactly one place and cannot drift. This module
//! only owns the outer section framing.
//!
//! This module is deliberately self-contained: it has no dependency on
//! `walrus` so it can be unit-tested with hand-built byte arrays.
//! Plumbing into the wider cli-support pipeline lives in
//! `crates/cli-support/src/descriptors.rs`.

use anyhow::{anyhow, bail, Result};
use wasm_bindgen_shared::{
    DESCRIPTOR_FORMAT_VERSION, DESCRIPTOR_KIND_CAST, DESCRIPTOR_KIND_REGULAR, DESCRIPTOR_KIND_STATIC,
};

/// One descriptor entry recovered from the section. Only entries with a
/// recognised `format_version` are decoded into this struct; entries with
/// unknown versions are skipped by the parser (after logging) and never
/// reach the rest of cli-support.
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
    /// the legacy interpreter would have produced. Any `SYMBOL_REF` slot
    /// (a length-prefixed UTF-8 symbol name in a `FUNCTION`'s `shim_idx`
    /// position) is resolved by
    /// [`crate::descriptor::Descriptor::decode_with_symbols`]. Always a
    /// multiple of 4 bytes (the parser reads `schema_word_count * 4` bytes).
    pub schema_bytes: Vec<u8>,
}

impl Entry {
    /// The entry's schema stream as little-endian `u32` words, ready for
    /// [`crate::descriptor::Descriptor::decode_with_symbols`]. `schema_bytes`
    /// is always a multiple of 4 bytes by construction.
    pub fn schema_words(&self) -> Vec<u32> {
        self.schema_bytes
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect()
    }
}

/// Statistics about a parse, useful for the caller's log output when
/// some entries had to be skipped due to unrecognised versions.
#[derive(Debug, Default, Clone)]
pub struct ParseStats {
    /// Number of entries successfully decoded.
    pub decoded: usize,
    /// Number of entries skipped because their `format_version` byte was
    /// not [`DESCRIPTOR_FORMAT_VERSION`]. The keys of the map are the
    /// unrecognised version numbers; values are how many entries had
    /// that version. Empty in the common case.
    pub skipped_unknown_version: std::collections::BTreeMap<u8, usize>,
}

impl ParseStats {
    /// Total number of skipped entries across all unknown versions.
    pub fn skipped_total(&self) -> usize {
        self.skipped_unknown_version.values().copied().sum()
    }
}

/// Parse the raw bytes of a `__wasm_bindgen_descriptors` custom section
/// into a list of [`Entry`] records, plus statistics about anything
/// that had to be skipped due to version mismatches.
///
/// Format (also documented on `DESCRIPTORS_SECTION_NAME`):
///
/// ```text
/// section bytes:
///   entry repeated until the section ends:
///     u8        format_version
///     u32 LE    entry_body_byte_len
///     ----- entry body (entry_body_byte_len bytes) -----
///     u8        shim_name_len
///     [u8; n]   shim_name
///     u8        kind
///     u32 LE    schema_word_count
///     [u32 LE]  schema (schema_word_count * 4 bytes)
///     ----- end of entry body -----
/// ```
///
/// The outer `format_version` + `entry_body_byte_len` framing is part
/// of the stable contract and must remain identical across versions so
/// that a CLI which does not recognise the `format_version` can still
/// skip the body byte-accurately and continue parsing later entries.
pub fn parse(section: &[u8]) -> Result<(Vec<Entry>, ParseStats)> {
    let mut r = Reader::new(section);
    let body_end = section.len();

    let mut entries = Vec::new();
    let mut stats = ParseStats::default();
    while r.pos < body_end {
        let format_version = r.u8()?;
        let body_len = r.u32()? as usize;
        let body_start = r.pos;
        let body_finish = body_start
            .checked_add(body_len)
            .ok_or_else(|| anyhow!("descriptor entry body_len overflows section"))?;
        if body_finish > body_end {
            // The most likely cause of a body_len that vastly overshoots
            // the section is a wasm binary produced by a pre-versioning
            // build of wasm-bindgen — the parser misinterprets the
            // legacy shim_name_len byte as a version byte and the next
            // few bytes as a wildly large body_len. Help the user
            // diagnose this rather than producing an opaque size error.
            bail!(
                "descriptor entry declares body_byte_len = {body_len}, which extends \
                 past the section end ({body_end} bytes). This usually means the \
                 wasm was produced by a wasm-bindgen older than this CLI and uses \
                 an incompatible __wasm_bindgen_descriptors layout. Rebuild the \
                 wasm with a matching wasm-bindgen version."
            );
        }

        if format_version != DESCRIPTOR_FORMAT_VERSION {
            // Forward-compat: skip this entry whole. We can do this safely
            // because the framing fields (format_version + body_len) are
            // part of the stable contract.
            *stats
                .skipped_unknown_version
                .entry(format_version)
                .or_insert(0) += 1;
            r.pos = body_finish;
            continue;
        }

        let name_len = r.u8()? as usize;
        if name_len == 0 {
            bail!("descriptor entry has empty shim name");
        }
        let name_bytes = r.bytes(name_len)?;
        let name = std::str::from_utf8(name_bytes)
            .map_err(|e| anyhow!("descriptor entry shim name is not UTF-8: {e}"))?
            .to_owned();
        let kind = r.u8()?;
        if kind != DESCRIPTOR_KIND_REGULAR
            && kind != DESCRIPTOR_KIND_CAST
            && kind != DESCRIPTOR_KIND_STATIC
        {
            bail!("descriptor entry for {name:?} has unknown kind byte {kind}");
        }
        let word_count = r.u32()? as usize;
        let byte_count = word_count
            .checked_mul(4)
            .ok_or_else(|| anyhow!("descriptor schema_word_count overflows"))?;
        let schema_bytes = r.bytes(byte_count)?.to_vec();
        if r.pos != body_finish {
            bail!(
                "descriptor entry {name:?}: declared body length {body_len} \
                 does not match decoded body length {}",
                r.pos - body_start
            );
        }
        entries.push(Entry {
            name,
            kind,
            schema_bytes,
        });
        stats.decoded += 1;
    }
    if r.pos != body_end {
        bail!(
            "descriptor section trailer mismatch: parsed up to {} but body ends at {body_end}",
            r.pos
        );
    }
    Ok((entries, stats))
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
    use crate::descriptor::Descriptor;
    use wasm_bindgen_shared::tys;

    /// Pack the *body* of a single entry (everything after the outer
    /// `format_version` + `body_len` framing fields).
    fn entry_body(name: &str, kind: u8, schema_words: &[u32]) -> Vec<u8> {
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

    /// Wrap a body with the outer framing (version + body_len). Defaults
    /// to the current [`DESCRIPTOR_FORMAT_VERSION`].
    fn frame(body: Vec<u8>) -> Vec<u8> {
        frame_with_version(DESCRIPTOR_FORMAT_VERSION, body)
    }

    fn frame_with_version(version: u8, body: Vec<u8>) -> Vec<u8> {
        let mut out = Vec::with_capacity(1 + 4 + body.len());
        out.push(version);
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        out
    }

    fn section(framed_entries: &[Vec<u8>]) -> Vec<u8> {
        framed_entries.iter().flatten().copied().collect()
    }

    #[test]
    fn parses_empty_section() {
        let bytes = section(&[]);
        let (entries, stats) = parse(&bytes).unwrap();
        assert!(entries.is_empty());
        assert_eq!(stats.decoded, 0);
        assert_eq!(stats.skipped_total(), 0);
    }

    #[test]
    fn parses_single_regular_entry() {
        // FUNCTION: shim_idx=0, nargs=1, arg=I32, ret=I32, inner_ret=I32.
        let words = [tys::FUNCTION, 0, 1, tys::I32, tys::I32, tys::I32];
        let bytes = section(&[frame(entry_body("foo", DESCRIPTOR_KIND_REGULAR, &words))]);
        let (entries, stats) = parse(&bytes).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(stats.decoded, 1);
        assert_eq!(entries[0].name, "foo");
        assert_eq!(entries[0].kind, DESCRIPTOR_KIND_REGULAR);
        // `schema_words` round-trips the bytes, and the stream decodes
        // through the shared grammar in `Descriptor`.
        assert_eq!(entries[0].schema_words(), words);
        let func = Descriptor::decode(&entries[0].schema_words()).unwrap_function();
        assert_eq!(func.shim_idx, 0);
        assert_eq!(func.arguments, vec![Descriptor::I32]);
    }

    #[test]
    fn parses_multiple_entries() {
        let a = frame(entry_body("a", DESCRIPTOR_KIND_REGULAR, &[tys::I32]));
        let b = frame(entry_body(
            "bb",
            DESCRIPTOR_KIND_CAST,
            &[tys::U32, tys::U32],
        ));
        let bytes = section(&[a, b]);
        let (entries, stats) = parse(&bytes).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(stats.decoded, 2);
        assert_eq!(entries[0].name, "a");
        assert_eq!(entries[0].kind, DESCRIPTOR_KIND_REGULAR);
        assert_eq!(entries[1].name, "bb");
        assert_eq!(entries[1].kind, DESCRIPTOR_KIND_CAST);
    }

    #[test]
    fn skips_unknown_version() {
        // A v1 entry followed by a "v99" entry followed by another v1.
        // The middle entry must be skipped cleanly without disturbing
        // the trailing one.
        let body_a = entry_body("a", DESCRIPTOR_KIND_REGULAR, &[tys::I32]);
        let body_b = entry_body("future", DESCRIPTOR_KIND_REGULAR, &[1, 2, 3, 4]);
        let body_c = entry_body("c", DESCRIPTOR_KIND_REGULAR, &[tys::U32]);
        let bytes = section(&[frame(body_a), frame_with_version(99, body_b), frame(body_c)]);
        let (entries, stats) = parse(&bytes).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(stats.decoded, 2);
        assert_eq!(stats.skipped_total(), 1);
        assert_eq!(stats.skipped_unknown_version.get(&99), Some(&1));
        assert_eq!(entries[0].name, "a");
        assert_eq!(entries[1].name, "c");
    }

    #[test]
    fn skipping_unknown_version_does_not_decode_garbage() {
        // The "future" body is deliberately not a valid v1 entry; if the
        // parser were to attempt to decode it anyway it would either
        // panic or yield bogus data.
        let mut garbage = Vec::new();
        garbage.extend_from_slice(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        garbage.extend_from_slice(b"this is not a v1 entry");
        let bytes = section(&[
            frame_with_version(7, garbage),
            frame(entry_body("after", DESCRIPTOR_KIND_REGULAR, &[tys::U32])),
        ]);
        let (entries, stats) = parse(&bytes).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "after");
        assert_eq!(stats.skipped_total(), 1);
        assert_eq!(stats.skipped_unknown_version.get(&7), Some(&1));
    }

    #[test]
    fn unknown_kind_is_rejected() {
        let body = entry_body("x", DESCRIPTOR_KIND_REGULAR, &[]);
        let mut framed = frame(body);
        // Find the kind byte: after [version(1) | body_len(4) | name_len(1) | "x"]
        let kind_pos = 1 + 4 + 1 + 1;
        framed[kind_pos] = 99;
        let bytes = section(&[framed]);
        let err = parse(&bytes).unwrap_err();
        assert!(err.to_string().contains("unknown kind"));
    }

    #[test]
    fn declared_body_length_mismatch_is_rejected() {
        // Pretend the body is longer than it actually is.
        let body = entry_body("x", DESCRIPTOR_KIND_REGULAR, &[tys::I32]);
        let mut framed = vec![DESCRIPTOR_FORMAT_VERSION];
        framed.extend_from_slice(&((body.len() + 4) as u32).to_le_bytes());
        framed.extend_from_slice(&body);
        framed.extend_from_slice(&[0, 0, 0, 0]); // padding bytes that look like a stray word
        let bytes = section(&[framed]);
        let err = parse(&bytes).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("declared body length") || msg.contains("extends past section end"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn truncated_section_is_rejected() {
        let bytes = section(&[frame(entry_body(
            "x",
            DESCRIPTOR_KIND_REGULAR,
            &[tys::I32, tys::I32],
        ))]);
        let err = parse(&bytes[..bytes.len() - 1]).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("unexpected end") || msg.contains("extends past"),
            "unexpected truncation error message: {msg}"
        );
    }
}
