//! Suggestion helpers for "did you mean ...?" diagnostics.
//!
//! Used whenever a user-supplied name (struct, class, parent, type, ...)
//! fails to resolve against the set of registered exports. Produces a small,
//! ranked list formatted into a human-readable hint suitable to append to a
//! `bail!`/`anyhow!` error message.

/// Maximum suggestions to surface for a single unresolved name. Three is a
/// commonly-used balance between "enough to be useful" and "not a wall of
/// noise".
const MAX_SUGGESTIONS: usize = 3;

/// Hard upper bound on edit distance for a candidate to be considered a
/// suggestion at all. Catches simple typos (`Pont` → `Point`) without
/// surfacing wildly different names. Scaled down for short inputs by
/// `suggestion_threshold`.
const ABSOLUTE_MAX_DISTANCE: usize = 3;

/// Compute the threshold for a given input length. Shorter inputs need
/// stricter matching (`a` should not match `b`); longer inputs tolerate a
/// little more drift. The `len / 3` formula yields 0 for len ≤ 2, 1 for
/// len ≤ 5, 2 for len ≤ 8, and so on, capped at `ABSOLUTE_MAX_DISTANCE`.
fn suggestion_threshold(input_len: usize) -> usize {
    (input_len / 3).min(ABSOLUTE_MAX_DISTANCE)
}

/// Levenshtein edit distance between two byte slices.
///
/// Hand-rolled to avoid adding a dependency for what's a tiny helper used
/// only on error paths. Runs in O(m*n) time with O(min(m,n)) memory using
/// the two-row optimisation. Inputs are compared as bytes; for ASCII names
/// (which is what wasm-bindgen exports always are after `to_valid_ident`)
/// that matches character-level distance.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for (i, &ca) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let cost = usize::from(ca != cb);
            cur[j + 1] = (prev[j + 1] + 1).min(cur[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

/// Rank `candidates` against `input` by edit distance and return up to
/// `MAX_SUGGESTIONS` entries within threshold. The returned list is sorted
/// by distance ascending (best matches first); ties are broken by candidate
/// order so the input slice's ordering is deterministically preserved.
///
/// Borrows the candidates rather than cloning so callers can pass slices
/// from any source (struct names, qualified names, type names, ...) without
/// extra allocation.
pub fn rank_suggestions<'a>(
    input: &str,
    candidates: impl IntoIterator<Item = &'a str>,
) -> Vec<&'a str> {
    let threshold = suggestion_threshold(input.len());
    let mut ranked: Vec<(usize, &'a str)> = candidates
        .into_iter()
        .filter(|c| !c.is_empty() && *c != input)
        .map(|c| (levenshtein(input, c), c))
        .filter(|(d, _)| *d <= threshold)
        .collect();
    // Stable sort preserves insertion order on ties so the caller controls
    // tie-breaking by ordering its candidate iterator.
    ranked.sort_by_key(|&(d, _)| d);
    ranked.truncate(MAX_SUGGESTIONS);
    ranked.into_iter().map(|(_, c)| c).collect()
}

/// Format a ranked list of suggestions as a multi-line help string suitable
/// to append to an error message. Returns an empty string if there are no
/// suggestions so callers can unconditionally append.
pub fn format_suggestions(suggestions: &[&str]) -> String {
    match suggestions.len() {
        0 => String::new(),
        1 => format!("\nhelp: did you mean `{}`?", suggestions[0]),
        _ => {
            let mut s = String::from("\nhelp: did you mean one of these?");
            for c in suggestions {
                s.push_str(&format!("\n  - {c}"));
            }
            s
        }
    }
}

/// Convenience: rank candidates against an input and format the result in
/// one call. Returns an empty string when no candidate is within edit
/// distance threshold, so callers can append unconditionally.
pub fn suggest<'a>(input: &str, candidates: impl IntoIterator<Item = &'a str>) -> String {
    let ranked = rank_suggestions(input, candidates);
    format_suggestions(&ranked)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_basic() {
        assert_eq!(levenshtein("", ""), 0);
        assert_eq!(levenshtein("a", ""), 1);
        assert_eq!(levenshtein("", "a"), 1);
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("Point", "Pont"), 1);
        assert_eq!(levenshtein("Point", "Pont2"), 2);
    }

    #[test]
    fn threshold_short_inputs_strict() {
        // 2-char input: threshold 0 so nothing fuzzy-matches; protects
        // against `a` suggesting `b` etc.
        let s = suggest("ab", ["ax", "ay", "az"]);
        assert!(s.is_empty(), "got: {s:?}");
    }

    #[test]
    fn threshold_longer_inputs_tolerant() {
        let s = suggest("Pont", ["Point", "Status", "Sphere"]);
        assert!(s.contains("Point"), "got: {s:?}");
        assert!(!s.contains("Status"), "got: {s:?}");
    }

    #[test]
    fn caps_at_max_suggestions() {
        let s = suggest("Foo", ["Foa", "Fob", "Foc", "Fod", "Foe"]);
        let count = s.matches("  - ").count();
        assert!(count <= MAX_SUGGESTIONS, "got: {s:?}");
    }

    #[test]
    fn excludes_exact_match() {
        // Exact match isn't a "suggestion" — caller should be using the
        // exact match directly. This helper is only invoked when lookup
        // already failed.
        let s = suggest("Foo", ["Foo", "Foa"]);
        assert!(!s.contains("`Foo`"), "got: {s:?}");
        assert!(s.contains("Foa"), "got: {s:?}");
    }

    #[test]
    fn empty_when_no_match() {
        let s = suggest("Banana", ["Carburetor", "Pneumatic", "Xylophone"]);
        assert!(s.is_empty(), "got: {s:?}");
    }

    #[test]
    fn single_vs_plural_phrasing() {
        let single = suggest("Pont", ["Point", "Sphere"]);
        assert!(single.contains("did you mean `Point`?"), "got: {single:?}");
        let multi = suggest("Pont", ["Point", "Pent"]);
        assert!(multi.contains("one of these"), "got: {multi:?}");
    }
}
