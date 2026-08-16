use std::fs;

use specbind::release;

#[test]
fn validates_opaque_portable_release_versions() {
    for valid in [
        "v1.4.0",
        "1.4.0-rc.1",
        "1.4.0+build.7",
        "2026-08-15",
        "release_42",
    ] {
        assert!(release::valid_version(valid), "expected valid: {valid}");
    }
    for invalid in ["", ".1", "bad/version", "bad version", "v1:2", "日本語"] {
        assert!(
            !release::valid_version(invalid),
            "expected invalid: {invalid}"
        );
    }
    assert!(!release::valid_version(&"v".repeat(65)));
}

#[test]
fn resolves_targets_and_rejects_case_insensitive_collisions() {
    let root = tempfile::tempdir().expect("temporary SpecBind root");
    let targets = release::resolve_available_archive_targets(root.path(), "v1.4.0")
        .expect("absent releases directory is available");
    assert_eq!(targets.roadmap, "releases/v1.4.0-roadmap.md");
    assert_eq!(
        targets.cross_spec_review,
        "releases/v1.4.0-cross-spec-review.md"
    );

    fs::create_dir(root.path().join("releases")).expect("create releases");
    fs::write(root.path().join("releases/V1.4.0-ROADMAP.MD"), "occupied")
        .expect("write colliding archive");
    let error = release::resolve_available_archive_targets(root.path(), "v1.4.0")
        .expect_err("ASCII case-insensitive collision must fail");
    assert_eq!(error.issues[0].code, "RELEASE_ARCHIVE_COLLISION");
}

#[test]
fn rejects_an_invalid_archive_root() {
    let root = tempfile::tempdir().expect("temporary SpecBind root");
    fs::write(root.path().join("releases"), "not a directory").expect("write invalid root");
    let error = release::resolve_available_archive_targets(root.path(), "v1")
        .expect_err("regular file archive root must fail");
    assert_eq!(error.issues[0].code, "RELEASE_ARCHIVE_ROOT_INVALID");
}
