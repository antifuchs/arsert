use version_sync::*;

#[test]
fn subcrate_versions() {
    assert_contains_regex!("arsert_impl/Cargo.toml", r#"^version = "{version}""#);
    assert_contains_regex!("arsert_failure/Cargo.toml", r#"^version = "{version}""#);

    assert_contains_regex!(
        "arsert_impl/Cargo.toml",
        r#"^version = "{version}" # dep_replace$"#
    );
    assert_contains_regex!("Cargo.toml", r#"^version = "{version}" # dep_replace$"#);
}
