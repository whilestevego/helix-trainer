//! Property-based tests for the `.hxt` parser and diff logic.

use helixir::hxt;
use proptest::prelude::*;

/// Build a minimal well-formed `.hxt` document from `practice` and `expected`
/// bodies. The real files have surrounding decoration; the parser only cares
/// about the markers, so a minimal document is sufficient for parsing tests.
fn build_hxt(practice: &str, expected: &str) -> String {
    format!(
        "\
────────────────────────── PRACTICE ──────────────────────────────

{practice}

────────────────────────── EXPECTED ──────────────────────────────

{expected}

──────────────────────────────────────────────────────────────────
"
    )
}

/// Strings that are safe to embed as section bodies: no newlines, no `─`
/// characters (which could be confused with separator/marker lines).
fn body_line() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9 _.,!?]{0,40}".prop_map(|s| s.to_string())
}

fn body_multiline() -> impl Strategy<Value = String> {
    proptest::collection::vec(body_line(), 1..8).prop_map(|v| v.join("\n"))
}

proptest! {
    /// Identical practice and expected bodies always verify as passing.
    #[test]
    fn practice_eq_expected_passes(body in body_multiline()) {
        let content = build_hxt(&body, &body);
        let result = hxt::verify_content(&content);
        prop_assert!(result.passed, "expected pass for body:\n{}", body);
        prop_assert!(result.diff.is_empty());
    }

    /// Distinct non-empty bodies always verify as failing and produce a
    /// non-empty diff.
    #[test]
    fn distinct_bodies_fail(
        a in body_multiline(),
        b in body_multiline(),
    ) {
        prop_assume!(a.trim_end() != b.trim_end());
        prop_assume!(!a.is_empty() && !b.is_empty());
        let content = build_hxt(&a, &b);
        let result = hxt::verify_content(&content);
        prop_assert!(!result.passed);
        prop_assert!(!result.diff.is_empty());
    }

    /// A well-formed document with valid markers always yields sections.
    #[test]
    fn well_formed_extracts_some(
        a in body_multiline(),
        b in body_multiline(),
    ) {
        let content = build_hxt(&a, &b);
        prop_assert!(hxt::extract_sections(&content).is_some());
    }

    /// `compute_diff` is length-symmetric: diff(a,b).len() == diff(b,a).len().
    #[test]
    fn diff_is_length_symmetric(
        a in body_multiline(),
        b in body_multiline(),
    ) {
        let ab = hxt::compute_diff(&a, &b).len();
        let ba = hxt::compute_diff(&b, &a).len();
        prop_assert_eq!(ab, ba);
    }

    /// Appending trailing spaces to every practice line does not flip the
    /// passed/failed verdict (trim_end semantics).
    #[test]
    fn trailing_whitespace_invariant(
        body in body_multiline(),
        pad in 1usize..8,
    ) {
        let padded: String = body
            .split('\n')
            .map(|l| format!("{}{}", l, " ".repeat(pad)))
            .collect::<Vec<_>>()
            .join("\n");
        let base = hxt::verify_content(&build_hxt(&body, &body)).passed;
        let padded_verdict = hxt::verify_content(&build_hxt(&padded, &body)).passed;
        prop_assert_eq!(base, padded_verdict);
    }

    /// Arbitrary unicode inputs must not panic the parser or diff.
    #[test]
    fn unicode_does_not_panic(s in ".{0,200}") {
        let _ = hxt::extract_sections(&s);
        let _ = hxt::compute_diff(&s, &s);
        let _ = hxt::verify_content(&s);
    }

    /// Missing PRACTICE marker ⇒ extract_sections returns None.
    #[test]
    fn missing_practice_marker_fails(body in body_multiline()) {
        let content = format!(
            "\
────────────────────────── EXPECTED ──────────────────────────────

{body}

──────────────────────────────────────────────────────────────────
"
        );
        prop_assert!(hxt::extract_sections(&content).is_none());
    }
}
