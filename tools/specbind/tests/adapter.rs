use specbind::{adapter, config::ProjectLanguage};

const ACCEPTED_SELECTORS: [&str; 3] = ["release", "git", "deferred"];

#[test]
fn accepts_a_closed_selector_set() {
    let selectors = adapter::all()
        .iter()
        .map(|entry| entry.selector)
        .collect::<Vec<_>>();
    assert_eq!(selectors, ACCEPTED_SELECTORS);

    // A file below the adapters root is not an adapter and never becomes one by
    // existing. Only an accepted selector resolves.
    assert!(adapter::find("release").is_some());
    assert!(adapter::find("git").is_some());
    assert!(adapter::find("deferred").is_some());
    for unknown in ["deploy", "release.md", "", "Release"] {
        assert!(
            adapter::find(unknown).is_none(),
            "{unknown} must not resolve"
        );
    }
}

#[test]
fn installs_each_adapter_below_the_accepted_root() {
    assert_eq!(adapter::ADAPTERS_ROOT, "settings/adapters");
    for entry in adapter::all() {
        assert_eq!(
            entry.path(),
            format!("settings/adapters/{}", entry.file_name)
        );
    }
}

#[test]
fn localizes_every_scaffold_while_keeping_the_type_literal_english() {
    for entry in adapter::all() {
        let english = entry.scaffold(ProjectLanguage::En);
        let japanese = entry.scaffold(ProjectLanguage::Ja);
        assert_ne!(
            english, japanese,
            "{}: scaffolds must differ",
            entry.selector
        );
        for scaffold in [english, japanese] {
            // Machine identity stays English in both languages.
            assert!(
                scaffold.starts_with(&format!("---\ntype: {}\n---\n", entry.artifact_type)),
                "{}: {scaffold}",
                entry.selector
            );
            // The scaffold is a vessel the project fills, so it carries the
            // authoring guidance that explains what to put in it.
            assert!(
                scaffold.contains("<!-- specbind:instruction"),
                "{}: scaffold must carry authoring guidance",
                entry.selector
            );
        }
    }
}

#[test]
fn states_that_the_deferred_destination_is_write_only() {
    let deferred = adapter::find("deferred").expect("deferred adapter");
    // A destination an authoring agent may read for work is a scope source, and
    // would reopen from the back door what Decision 0121 closed at the front.
    for scaffold in [
        deferred.scaffold(ProjectLanguage::En),
        deferred.scaffold(ProjectLanguage::Ja),
    ] {
        assert!(
            scaffold.contains("Roadmap")
                && (scaffold.contains("never reads") || scaffold.contains("読むことはなく")),
            "{scaffold}"
        );
    }
}

#[test]
fn states_that_the_git_adapter_grants_no_authority() {
    let git = adapter::find("git").expect("git adapter");
    // Policy is not authority. A scaffold that omitted this would invite a
    // project to believe the file can widen what SpecBind may do.
    for scaffold in [
        git.scaffold(ProjectLanguage::En),
        git.scaffold(ProjectLanguage::Ja),
    ] {
        assert!(
            scaffold.contains("permission") || scaffold.contains("権限"),
            "{scaffold}"
        );
    }
}
