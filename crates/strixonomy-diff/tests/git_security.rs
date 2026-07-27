use strixonomy_core::ensure_extract_path_within;
use strixonomy_diff::parse_git_range;

#[test]
fn reject_path_escape_in_extract() {
    let dir = tempfile::tempdir().unwrap();
    let err = ensure_extract_path_within(dir.path(), "../outside.ttl").expect_err("escape");
    assert!(
        err.contains("..") || err.contains("escapes") || err.contains("outside"),
        "unexpected: {err}"
    );
    let err2 =
        ensure_extract_path_within(dir.path(), "nested/../../outside.ttl").expect_err("escape");
    assert!(
        err2.contains("..") || err2.contains("escapes") || err2.contains("outside"),
        "unexpected: {err2}"
    );
}

#[test]
fn parse_two_dot_range() {
    let (left, right) = parse_git_range("main..feature").unwrap();
    assert_eq!(left, "main");
    assert_eq!(right, "feature");
}

#[test]
fn parse_triple_dot_range_encodes_merge_base_request() {
    let (left, right) = parse_git_range("main...feature").unwrap();
    // Behavioral contract: left encodes both tips; right marks merge-base mode for
    // diff_git_refs (see triple_dot_diff_compares_merge_base_to_feature).
    assert!(left.contains("..."), "left must retain triple-dot tips: {left}");
    assert!(left.starts_with("main") && left.ends_with("feature"));
    assert_eq!(right, "TRIPLE_DOT", "merge-base mode marker required by diff_git_refs");
}
