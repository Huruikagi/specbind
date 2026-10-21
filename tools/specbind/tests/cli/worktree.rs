use super::*;
use serde_json::{Value, json};
use std::io::Write as _;

fn fixture() -> TempDir {
    let root = project_fixture();
    git(root.path(), &["config", "core.autocrlf", "false"]);
    write(root.path(), "source.txt", "original\n");
    write(root.path(), "unrelated.txt", "unrelated\n");
    write(root.path(), ".gitignore", "ignored/\n");
    commit_all(root.path());
    root
}

fn capture(root: &Path) -> String {
    let index = fs::read(root.join(".git/index")).unwrap();
    let result = specbind_command()
        .current_dir(root)
        .args(["worktree", "capture"])
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    assert_eq!(
        index,
        fs::read(root.join(".git/index")).unwrap(),
        "capture refreshed index"
    );
    String::from_utf8(result).unwrap()
}

fn check(root: &Path, snapshot: &str, changed: &[&str], success: bool) -> Value {
    let index = fs::read(root.join(".git/index")).unwrap();
    let mut command = specbind_command();
    command
        .current_dir(root)
        .args(["worktree", "check", "--snapshot", "-"])
        .write_stdin(snapshot);
    for name in changed {
        command.args(["--changed", name]);
    }
    let result = command
        .assert()
        .code(i32::from(!success))
        .stderr("")
        .get_output()
        .stdout
        .clone();
    assert_eq!(
        index,
        fs::read(root.join(".git/index")).unwrap(),
        "check refreshed index"
    );
    serde_json::from_slice(&result).unwrap()
}

fn check_error(root: &Path, snapshot: &str, changed: &[&str]) {
    let mut command = specbind_command();
    command
        .current_dir(root)
        .args(["worktree", "check", "--snapshot", "-"])
        .write_stdin(snapshot);
    for name in changed {
        command.args(["--changed", name]);
    }
    command
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("WORKTREE_CHECK_FAILED"));
}

#[test]
fn worktree_no_changes_and_declared_edits_leave_index_and_files_untouched() {
    let root = fixture();
    let snapshot = capture(root.path());
    let repeated = capture(root.path());
    assert_eq!(snapshot, repeated);
    let result = check(root.path(), &snapshot, &[], true);
    assert_eq!(result["result"], "WORKTREE_CHECK_MATCHED");
    assert_eq!(result["changes"], json!([]));
    write(root.path(), "source.txt", "changed\n");
    let result = check(root.path(), &snapshot, &["source.txt", "unused.txt"], true);
    assert_eq!(result["declaredUnchangedPaths"], json!(["unused.txt"]));
    assert_eq!(result["changes"][0]["indexChanged"], false);
    assert_eq!(result["changes"][0]["worktreeChanged"], true);
    assert_eq!(
        fs::read_to_string(root.path().join("source.txt")).unwrap(),
        "changed\n"
    );
}

#[test]
fn worktree_detects_same_path_additional_edits_over_staged_and_unstaged_work() {
    let root = fixture();
    write(root.path(), "source.txt", "staged\n");
    git(root.path(), &["add", "source.txt"]);
    write(root.path(), "source.txt", "before\n");
    write(root.path(), "unrelated.txt", "keep me\n");
    let snapshot = capture(root.path());
    write(root.path(), "source.txt", "after!\n"); // same byte length
    let result = check(root.path(), &snapshot, &["source.txt"], true);
    assert_eq!(result["preexistingTouchedPaths"], json!(["source.txt"]));
    assert_eq!(result["changes"].as_array().unwrap().len(), 1);
    assert_eq!(result["changes"][0]["indexChanged"], false);
    assert_eq!(
        fs::read_to_string(root.path().join("unrelated.txt")).unwrap(),
        "keep me\n"
    );
    let result = check(root.path(), &snapshot, &[], false);
    assert_eq!(result["unexpectedPaths"], json!(["source.txt"]));
}

#[test]
fn worktree_detects_stage_only_changes_and_unstaging_without_content_changes() {
    let root = fixture();
    write(root.path(), "source.txt", "new\n");
    let snapshot = capture(root.path());
    git(root.path(), &["add", "source.txt"]);
    let result = check(root.path(), &snapshot, &["source.txt"], true);
    assert_eq!(result["changes"][0]["indexChanged"], true);
    assert_eq!(result["changes"][0]["worktreeChanged"], false);
    let staged = capture(root.path());
    git(root.path(), &["restore", "--staged", "source.txt"]);
    let result = check(root.path(), &staged, &[], false);
    assert_eq!(result["unexpectedPaths"], json!(["source.txt"]));
}

#[test]
fn worktree_rename_is_two_exact_paths_and_directory_claims_do_not_cover_children() {
    let root = fixture();
    let snapshot = capture(root.path());
    fs::rename(
        root.path().join("source.txt"),
        root.path().join("renamed.txt"),
    )
    .unwrap();
    write(root.path(), "reports/result.txt", "leftover");
    let result = check(
        root.path(),
        &snapshot,
        &["source.txt", "renamed.txt", "reports"],
        false,
    );
    assert_eq!(result["unexpectedPaths"], json!(["reports/result.txt"]));
    assert_eq!(result["changes"].as_array().unwrap().len(), 3);
    let result = check(root.path(), &snapshot, &["*"], false);
    assert_eq!(result["unexpectedPaths"].as_array().unwrap().len(), 3);
}

#[test]
fn worktree_untracked_changes_and_deletions_survive_new_ignore_rules() {
    let root = fixture();
    write(root.path(), "scratch.txt", "before");
    write(root.path(), "deleted.txt", "before");
    let snapshot = capture(root.path());
    write(root.path(), ".gitignore", "ignored/\nscratch.txt\n");
    write(root.path(), "scratch.txt", "after!");
    fs::remove_file(root.path().join("deleted.txt")).unwrap();
    write(root.path(), "ignored/new.txt", "normal ignored cache");
    let result = check(root.path(), &snapshot, &[".gitignore"], false);
    assert_eq!(
        result["unexpectedPaths"],
        json!(["deleted.txt", "scratch.txt"])
    );
    assert_eq!(
        result["preexistingTouchedPaths"],
        json!(["deleted.txt", "scratch.txt"])
    );
}

#[test]
fn worktree_head_movement_is_a_mismatch_even_without_file_changes() {
    let root = fixture();
    let snapshot = capture(root.path());
    git(
        root.path(),
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "--allow-empty",
            "-m",
            "head moved",
        ],
    );
    let result = check(root.path(), &snapshot, &[], false);
    assert_eq!(result["headChanged"], true);
    assert_eq!(result["changes"], json!([]));
    assert_eq!(result["result"], "WORKTREE_CHECK_MISMATCH");
}

#[test]
fn worktree_strict_snapshot_and_exact_paths_reject_ambiguous_inputs() {
    let root = fixture();
    let snapshot = capture(root.path());
    for bad in [
        "../escape",
        "/absolute",
        "C:/absolute",
        ".git/config",
        ".git./config",
        ".git /config",
        "a/../b",
        "a\\b",
    ] {
        check_error(root.path(), &snapshot, &[bad]);
    }
    check_error(root.path(), &snapshot, &["source.txt", "source.txt"]);
    check_error(
        root.path(),
        &snapshot.replacen(
            "\"schemaVersion\": 1",
            "\"schemaVersion\": 1, \"schemaVersion\": 1",
            1,
        ),
        &[],
    );
    let original: Value = serde_json::from_str(&snapshot).unwrap();
    for altered in [
        {
            let mut v = original.clone();
            v["schemaVersion"] = json!(2);
            v
        },
        {
            let mut v = original.clone();
            v["extra"] = json!(true);
            v
        },
        {
            let mut v = original.clone();
            v["headRevision"] = json!("bad");
            v
        },
        {
            let mut v = original.clone();
            v["gitDirectory"] = json!("another-git-directory");
            v
        },
        {
            let mut v = original.clone();
            let p = v["paths"][0].clone();
            v["paths"].as_array_mut().unwrap().push(p);
            v
        },
        {
            let mut v = original.clone();
            v["paths"][0]["path"] = json!("../escape");
            v
        },
    ] {
        check_error(root.path(), &altered.to_string(), &[]);
    }
    let other = fixture();
    check_error(other.path(), &snapshot, &[]);
}

#[test]
fn worktree_external_snapshot_file_is_accepted_and_in_tree_file_rejected() {
    let root = fixture();
    let snapshot = capture(root.path());
    let external = tempfile::tempdir().unwrap();
    write(external.path(), "capture.json", &snapshot);
    specbind_command()
        .current_dir(root.path())
        .args(["worktree", "check", "--snapshot"])
        .arg(external.path().join("capture.json"))
        .assert()
        .success();
    write(root.path(), "capture.json", &snapshot);
    specbind_command()
        .current_dir(root.path())
        .args(["worktree", "check", "--snapshot", "capture.json"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("WORKTREE_SNAPSHOT_TARGET_INVALID"));
}

#[test]
fn worktree_refuses_assume_unchanged_skip_worktree_and_intent_to_add() {
    let root = fixture();
    for flag in ["--assume-unchanged", "--skip-worktree"] {
        git(root.path(), &["update-index", flag, "source.txt"]);
        specbind_command()
            .current_dir(root.path())
            .args(["worktree", "capture"])
            .assert()
            .failure()
            .stdout("")
            .stderr(predicate::str::contains("unsupported index flag"));
        git(
            root.path(),
            &["update-index", "--no-assume-unchanged", "source.txt"],
        );
        git(
            root.path(),
            &["update-index", "--no-skip-worktree", "source.txt"],
        );
    }
    write(root.path(), "new.txt", "");
    git(root.path(), &["add", "--intent-to-add", "new.txt"]);
    specbind_command()
        .current_dir(root.path())
        .args(["worktree", "capture"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("intent-to-add"));
}

#[test]
fn worktree_preserves_binary_bytes_and_literal_unicode_space_paths() {
    let root = fixture();
    let name = "src/[item] 日本語.bin";
    write(root.path(), name, "");
    fs::write(root.path().join(name), [0, 255, 10, 13]).unwrap();
    let snapshot = capture(root.path());
    fs::write(root.path().join(name), [0, 254, 10, 13]).unwrap();
    let result = check(root.path(), &snapshot, &[name], true);
    assert_eq!(result["preexistingTouchedPaths"], json!([name]));
    assert_eq!(result["changes"][0]["path"], name);
}

#[test]
fn worktree_reports_file_directory_replacements_without_reading_outside_paths() {
    let root = fixture();
    let snapshot = capture(root.path());
    fs::remove_file(root.path().join("source.txt")).unwrap();
    write(root.path(), "source.txt/child.txt", "child");
    let result = check(
        root.path(),
        &snapshot,
        &["source.txt", "source.txt/child.txt"],
        true,
    );
    assert_eq!(result["changes"].as_array().unwrap().len(), 2);
    assert_eq!(
        result["changes"][0]["after"]["worktree"]["kind"],
        "directory"
    );
}

#[test]
fn worktree_rejects_gitlinks_and_unmerged_index_entries() {
    let root = fixture();
    let head = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    git(
        root.path(),
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("160000,{head},submodule"),
        ],
    );
    specbind_command()
        .current_dir(root.path())
        .args(["worktree", "capture"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("submodules"));
    git(
        root.path(),
        &["update-index", "--force-remove", "submodule"],
    );
    let blob = git_stdout(root.path(), &["rev-parse", "HEAD:source.txt"]);
    let mut command = ProcessCommand::new("git");
    let mut process = command
        .arg("-C")
        .arg(root.path())
        .args(["update-index", "--index-info"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    writeln!(
        process.stdin.take().unwrap(),
        "0 {}\tsource.txt\n100644 {blob} 1\tsource.txt\n100644 {blob} 2\tsource.txt",
        "0".repeat(40)
    )
    .unwrap();
    assert!(process.wait().unwrap().success());
    specbind_command()
        .current_dir(root.path())
        .args(["worktree", "capture"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("conflicts"));
}
