use crate::schema::spec::v1::{self as wire, WorkflowState};

use super::diagnostics::{SemanticIssues, issue};

/// A `spec.yaml` v1 document whose artifact-local lifecycle invariants hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spec {
    wire: wire::SpecDocument,
}

impl Spec {
    #[must_use]
    pub fn as_wire(&self) -> &wire::SpecDocument {
        &self.wire
    }

    #[must_use]
    pub fn into_wire(self) -> wire::SpecDocument {
        self.wire
    }
}

impl TryFrom<wire::SpecDocument> for Spec {
    type Error = SemanticIssues;

    fn try_from(wire: wire::SpecDocument) -> Result<Self, Self::Error> {
        let issues = validate(&wire);
        if issues.is_empty() {
            Ok(Self { wire })
        } else {
            Err(SemanticIssues::from_unsorted(issues))
        }
    }
}

fn validate(document: &wire::SpecDocument) -> Vec<super::SemanticIssue> {
    let Some(active) = &document.active_change.0 else {
        return Vec::new();
    };

    let mut issues = Vec::new();
    let expected_gates = expected_gate_names(active.state);
    let actual_gates = gate_names(active.gate_evidence.as_ref());
    if actual_gates != expected_gates {
        issues.push(issue(
            "SPEC_STATE_GATE_EVIDENCE",
            "/active_change/gate_evidence",
            format!(
                "state {} requires exactly [{}], found [{}]",
                state_name(active.state),
                expected_gates.join(", "),
                actual_gates.join(", ")
            ),
        ));
    }

    match active.state {
        WorkflowState::Requirements => {
            if active.requirement_ids.0.is_some() {
                issues.push(issue(
                    "SPEC_REQUIREMENT_IDS_PREMATURE",
                    "/active_change/requirement_ids",
                    "requirement_ids must be null in requirements state",
                ));
            }
        }
        WorkflowState::Design
        | WorkflowState::Tasks
        | WorkflowState::AdoptionReady
        | WorkflowState::Implementation
        | WorkflowState::ReleaseReady => match &active.requirement_ids.0 {
            None => issues.push(issue(
                "SPEC_REQUIREMENT_IDS_MISSING",
                "/active_change/requirement_ids",
                "requirement_ids must be present after requirements approval",
            )),
            Some(ids) => {
                validate_requirement_ids(&ids.0, &mut issues);
                if let Some(approved) = approved_requirement_ids(active.gate_evidence.as_ref())
                    && ids.0.as_slice() != approved.as_slice()
                {
                    issues.push(issue(
                        "SPEC_REQUIREMENT_IDS_MISMATCH",
                        "/active_change/requirement_ids",
                        "requirement_ids must exactly match requirements gate approval evidence",
                    ));
                }
            }
        },
    }

    issues
}

fn expected_gate_names(state: WorkflowState) -> Vec<&'static str> {
    match state {
        WorkflowState::Requirements => vec![],
        WorkflowState::Design => vec!["requirements"],
        WorkflowState::Tasks | WorkflowState::AdoptionReady => vec!["requirements", "design"],
        WorkflowState::Implementation => vec!["requirements", "design", "tasks"],
        WorkflowState::ReleaseReady => vec!["requirements", "design", "tasks", "completion"],
    }
}

fn gate_names(evidence: Option<&wire::GateEvidence>) -> Vec<&'static str> {
    let Some(evidence) = evidence else {
        return vec![];
    };
    [
        ("requirements", evidence.requirements.is_some()),
        ("design", evidence.design.is_some()),
        ("tasks", evidence.tasks.is_some()),
        ("completion", evidence.completion.is_some()),
    ]
    .into_iter()
    .filter_map(|(name, present)| present.then_some(name))
    .collect()
}

fn approved_requirement_ids(evidence: Option<&wire::GateEvidence>) -> Option<&Vec<String>> {
    match evidence?.requirements.as_ref()? {
        wire::RequirementsGateEvidence::Explicit(value) => Some(&value.approved_requirement_ids.0),
        wire::RequirementsGateEvidence::Delegated(value) => Some(&value.approved_requirement_ids.0),
    }
}

fn validate_requirement_ids(ids: &[String], issues: &mut Vec<super::SemanticIssue>) {
    let parsed = ids
        .iter()
        .map(|id| super::parse_requirement_id(id))
        .collect::<Option<Vec<_>>>();
    if parsed.is_none() {
        issues.push(issue(
            "SPEC_REQUIREMENT_ID_FORMAT",
            "/active_change/requirement_ids",
            "Requirement IDs must use positive numeric N.M form without leading zeroes",
        ));
        return;
    }
    if !parsed.is_some_and(|values| values.windows(2).all(|pair| pair[0] < pair[1])) {
        issues.push(issue(
            "SPEC_REQUIREMENT_ID_ORDER",
            "/active_change/requirement_ids",
            "Requirement IDs must be in deterministic numeric order",
        ));
    }
}

fn state_name(state: WorkflowState) -> &'static str {
    match state {
        WorkflowState::Requirements => "requirements",
        WorkflowState::Design => "design",
        WorkflowState::Tasks => "tasks",
        WorkflowState::AdoptionReady => "adoption_ready",
        WorkflowState::Implementation => "implementation",
        WorkflowState::ReleaseReady => "release_ready",
    }
}
