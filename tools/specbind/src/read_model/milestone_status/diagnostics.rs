use super::{
    BTreeMap, BTreeSet, Dependency, DiscoveryIssue, GitState, GraphIssueSeverity, ItemFacts,
    ItemKind, MilestoneDiagnostic, MilestoneStatusFailure, Path, RoadmapDocument, WorkflowState,
    artifacts, contract_graph, design_approved, fs, has_specs, repository, spec_predicate,
};

pub(super) fn diagnose_required_contract_graph(
    specbind_root: &Path,
    facts: &[ItemFacts],
    diagnostics: &mut BTreeSet<MilestoneDiagnostic>,
) {
    if !has_specs(facts) || !spec_predicate(facts, design_approved) {
        return;
    }
    let graph = contract_graph::resolve(specbind_root);
    diagnostics.extend(graph.project_issues.iter().map(from_discovery));
    diagnostics.extend(
        graph
            .inventories
            .values()
            .flat_map(|inventory| inventory.issues.iter())
            .map(from_discovery),
    );
    diagnostics.extend(
        graph
            .report
            .issues
            .iter()
            .filter(|issue| issue.severity == GraphIssueSeverity::Error)
            .map(|issue| MilestoneDiagnostic {
                code: issue.code,
                path: issue.source.clone(),
                message: issue.message.clone(),
            }),
    );
}

pub(super) fn design_dependencies_ready(item: &ItemFacts, facts: &[ItemFacts]) -> bool {
    item.dependencies
        .iter()
        .filter(|dependency| dependency.starts_with("spec:"))
        .all(|dependency| {
            facts
                .iter()
                .find(|item| &item.id == dependency)
                .is_some_and(|item| {
                    matches!(&item.kind, ItemKind::Spec { model, .. } if model.as_deref().is_some_and(design_approved))
                })
        })
}

pub(super) fn dependencies_ready(item: &ItemFacts, completion: &BTreeMap<String, bool>) -> bool {
    item.dependencies
        .iter()
        .all(|dependency| completion.get(dependency).copied().unwrap_or(false))
}

pub(super) fn dependency_key(dependency: &Dependency) -> String {
    match dependency {
        Dependency::Spec(value) => format!("spec:{}", value.spec),
        Dependency::Direct(value) => format!("direct:{}", value.direct),
    }
}

pub(super) fn diagnose_unscoped_active_specs(
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    diagnostics: &mut BTreeSet<MilestoneDiagnostic>,
) {
    let scoped = roadmap.spec_ids().into_iter().collect::<BTreeSet<_>>();
    let Ok(entries) = fs::read_dir(specbind_root.join("specs")) else {
        return;
    };
    for entry in entries.flatten() {
        if !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        let Some(spec) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if scoped.contains(&spec) {
            continue;
        }
        let resolution = artifacts::resolve_spec(specbind_root, &spec);
        if resolution
            .wire
            .as_ref()
            .is_some_and(|wire| wire.active_change.0.is_some())
        {
            diagnostics.insert(MilestoneDiagnostic {
                code: "MILESTONE_UNSCOPED_ACTIVE_SPEC",
                path: Some(format!("specs/{spec}/spec.yaml")),
                message: format!("active spec {spec} is absent from Roadmap scope"),
            });
        }
    }
}

pub(super) fn git_state(project_root: &Path) -> GitState {
    let revision = repository::output(project_root, &["rev-parse", "HEAD"]);
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    );
    match (revision, status) {
        (Ok(revision), Ok(status)) => GitState {
            revision: Some(revision.trim().to_owned()),
            clean: status.is_empty(),
            diagnostic: None,
        },
        (Err(error), _) | (_, Err(error)) => git_failure(error.to_string()),
    }
}

fn git_failure(message: String) -> GitState {
    GitState {
        revision: None,
        clean: false,
        diagnostic: Some(MilestoneDiagnostic {
            code: "MILESTONE_GIT_STATE_FAILED",
            path: None,
            message,
        }),
    }
}

pub(super) fn state_rank(state: Option<WorkflowState>) -> u8 {
    match state {
        None => 0,
        Some(WorkflowState::Requirements) => 1,
        Some(WorkflowState::Design) => 2,
        Some(WorkflowState::AdoptionReady | WorkflowState::Tasks) => 3,
        Some(WorkflowState::Implementation) => 4,
        Some(WorkflowState::ReleaseReady) => 5,
    }
}

pub(super) fn from_discovery(issue: &DiscoveryIssue) -> MilestoneDiagnostic {
    MilestoneDiagnostic {
        code: issue.code,
        path: issue.path.as_ref().map(|path| path.as_str().to_owned()),
        message: issue.message.clone(),
    }
}

pub(super) fn failure(code: &'static str, message: impl Into<String>) -> MilestoneStatusFailure {
    MilestoneStatusFailure {
        diagnostics: vec![MilestoneDiagnostic {
            code,
            path: Some("steering/roadmap.md".to_owned()),
            message: message.into(),
        }],
    }
}
