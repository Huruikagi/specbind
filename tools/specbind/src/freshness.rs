//! Gate-local artifact freshness with prerequisite cascading.

use std::collections::BTreeMap;

use crate::{
    domain::{SemanticIssue, spec::Spec},
    fingerprint::Fingerprint,
    schema::spec::v1 as wire,
};

/// Current fingerprints resolved from authoritative project artifact discovery.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CurrentGateInputs {
    pub requirements: Option<Fingerprint>,
    pub design: Option<BTreeMap<String, Fingerprint>>,
    pub task_plan: Option<Fingerprint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessStatus {
    NotReached,
    Fresh,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateFreshness {
    pub status: FreshnessStatus,
    pub issues: Vec<SemanticIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactFreshnessReport {
    pub requirements: GateFreshness,
    pub design: GateFreshness,
    pub tasks: GateFreshness,
}

/// Compares current gate-owned input projections with persisted approval evidence.
#[must_use]
pub fn evaluate(spec: &Spec, current: &CurrentGateInputs) -> ArtifactFreshnessReport {
    let active = spec.as_wire().active_change.0.as_ref();
    let evidence = active.and_then(|active| active.gate_evidence.as_ref());

    let requirements = evaluate_requirements(evidence, current.requirements);
    let design = evaluate_design(evidence, current.design.as_ref(), &requirements);
    let tasks = evaluate_tasks(evidence, current.task_plan, &design);

    ArtifactFreshnessReport {
        requirements,
        design,
        tasks,
    }
}

fn evaluate_requirements(
    evidence: Option<&wire::GateEvidence>,
    current: Option<Fingerprint>,
) -> GateFreshness {
    let Some(expected) = evidence
        .and_then(|value| value.requirements.as_ref())
        .map(requirements_fingerprint)
    else {
        return not_reached();
    };

    let issues = match current {
        None => vec![freshness_issue(
            "FRESHNESS_REQUIREMENTS_MISSING",
            "/requirements",
            "current requirements artifact is missing",
        )],
        Some(current) if !current.matches_wire(expected) => vec![freshness_issue(
            "FRESHNESS_REQUIREMENTS_CHANGED",
            "/requirements",
            "current requirements fingerprint differs from approved evidence",
        )],
        Some(_) => vec![],
    };
    reached(issues)
}

fn evaluate_design(
    evidence: Option<&wire::GateEvidence>,
    current: Option<&BTreeMap<String, Fingerprint>>,
    prerequisite: &GateFreshness,
) -> GateFreshness {
    let Some(expected) = evidence.and_then(|value| value.design.as_ref()) else {
        return not_reached();
    };
    let expected = design_fingerprints(expected);
    let mut issues = prerequisite_issue("design", prerequisite);

    match current {
        None => issues.push(freshness_issue(
            "FRESHNESS_DESIGN_INPUTS_MISSING",
            "/design",
            "current contract and design artifact set is missing",
        )),
        Some(current) => {
            for key in expected.keys().filter(|key| !current.contains_key(*key)) {
                issues.push(freshness_issue(
                    "FRESHNESS_DESIGN_INPUT_MISSING",
                    format!("/design/{key}"),
                    format!("current design input {key} is missing"),
                ));
            }
            for key in current.keys().filter(|key| !expected.contains_key(*key)) {
                issues.push(freshness_issue(
                    "FRESHNESS_DESIGN_INPUT_ADDED",
                    format!("/design/{key}"),
                    format!("current design input {key} was not approved"),
                ));
            }
            for (key, fingerprint) in current {
                if let Some(expected) = expected.get(key)
                    && !fingerprint.matches_wire(expected)
                {
                    issues.push(freshness_issue(
                        "FRESHNESS_DESIGN_INPUT_CHANGED",
                        format!("/design/{key}"),
                        format!("current design input {key} differs from approved evidence"),
                    ));
                }
            }
        }
    }
    reached(issues)
}

fn evaluate_tasks(
    evidence: Option<&wire::GateEvidence>,
    current: Option<Fingerprint>,
    prerequisite: &GateFreshness,
) -> GateFreshness {
    let Some(expected) = evidence
        .and_then(|value| value.tasks.as_ref())
        .map(tasks_fingerprint)
    else {
        return not_reached();
    };
    let mut issues = prerequisite_issue("tasks", prerequisite);
    match current {
        None => issues.push(freshness_issue(
            "FRESHNESS_TASK_PLAN_MISSING",
            "/tasks.yaml#plan",
            "current task plan is missing",
        )),
        Some(current) if !current.matches_wire(expected) => issues.push(freshness_issue(
            "FRESHNESS_TASK_PLAN_CHANGED",
            "/tasks.yaml#plan",
            "current task plan differs from approved evidence",
        )),
        Some(_) => {}
    }
    reached(issues)
}

fn requirements_fingerprint(evidence: &wire::RequirementsGateEvidence) -> &wire::Fingerprint {
    match evidence {
        wire::RequirementsGateEvidence::Explicit(value) => &value.input_revisions.requirements,
        wire::RequirementsGateEvidence::Delegated(value) => &value.input_revisions.requirements,
    }
}

fn design_fingerprints(
    evidence: &wire::DesignGateEvidence,
) -> &BTreeMap<String, wire::Fingerprint> {
    match evidence {
        wire::DesignGateEvidence::Explicit(value) => &value.input_revisions.0,
        wire::DesignGateEvidence::Delegated(value) => &value.input_revisions.0,
    }
}

fn tasks_fingerprint(evidence: &wire::TasksGateEvidence) -> &wire::Fingerprint {
    match evidence {
        wire::TasksGateEvidence::Explicit(value) => &value.input_revisions.plan,
        wire::TasksGateEvidence::Delegated(value) => &value.input_revisions.plan,
    }
}

fn prerequisite_issue(gate: &str, prerequisite: &GateFreshness) -> Vec<SemanticIssue> {
    if prerequisite.status == FreshnessStatus::Fresh {
        vec![]
    } else {
        vec![freshness_issue(
            "FRESHNESS_PREREQUISITE_STALE",
            format!("/{gate}"),
            format!("{gate} freshness requires its prerequisite gate to be fresh"),
        )]
    }
}

fn not_reached() -> GateFreshness {
    GateFreshness {
        status: FreshnessStatus::NotReached,
        issues: vec![],
    }
}

fn reached(mut issues: Vec<SemanticIssue>) -> GateFreshness {
    issues.sort();
    issues.dedup();
    GateFreshness {
        status: if issues.is_empty() {
            FreshnessStatus::Fresh
        } else {
            FreshnessStatus::Stale
        },
        issues,
    }
}

fn freshness_issue(
    code: &'static str,
    path: impl Into<String>,
    message: impl Into<String>,
) -> SemanticIssue {
    SemanticIssue {
        code,
        path: path.into(),
        message: message.into(),
    }
}
