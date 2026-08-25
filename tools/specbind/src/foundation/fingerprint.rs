//! Foundation values and canonical fingerprint producers for gate-owned inputs.

use std::cmp::Ordering;
use std::fmt;

use sha2::{Digest, Sha256};

use crate::{
    domain::tasks::Tasks,
    roadmap::RoadmapDocument,
    schema::{spec, tasks},
};

/// A lowercase SHA-256 digest rendered with the persisted `sha256:` tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fingerprint([u8; 32]);

impl Fingerprint {
    /// Fingerprints a complete Markdown file after CRLF and bare-CR normalization.
    #[must_use]
    pub fn markdown(bytes: &[u8]) -> Self {
        let mut normalized = Vec::with_capacity(bytes.len());
        let mut index = 0;
        while index < bytes.len() {
            if bytes[index] == b'\r' {
                normalized.push(b'\n');
                index += usize::from(bytes.get(index + 1) == Some(&b'\n'));
            } else {
                normalized.push(bytes[index]);
            }
            index += 1;
        }
        Self::digest(&normalized)
    }

    /// Fingerprints the normalized typed `tasks.yaml#plan` projection using JCS.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema-owned plan cannot be serialized to JCS.
    pub fn task_plan(document: &Tasks) -> Result<Self, serde_json::Error> {
        let mut normalized = document.as_wire().plan.clone();
        normalize_task_plan(&mut normalized);
        serde_json_canonicalizer::to_vec(&normalized).map(|bytes| Self::digest(&bytes))
    }

    /// Fingerprints the normalized `steering/roadmap.md#cross-spec-scope` projection using JCS.
    ///
    /// # Errors
    ///
    /// Returns an error if the typed projection cannot be serialized to JCS.
    pub fn roadmap_cross_spec_scope(document: &RoadmapDocument) -> Result<Self, serde_json::Error> {
        serde_json_canonicalizer::to_vec(&document.cross_spec_scope())
            .map(|bytes| Self::digest(&bytes))
    }

    #[must_use]
    pub fn matches_wire(self, expected: &spec::v1::Fingerprint) -> bool {
        self.to_string() == expected.0
    }

    fn digest(bytes: &[u8]) -> Self {
        Self(Sha256::digest(bytes).into())
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "sha256:{}", hex::encode(self.0))
    }
}

fn normalize_task_plan(plan: &mut tasks::v1::TaskPlan) {
    for item in &mut plan.items {
        match item {
            tasks::v1::PlanItem::Group(group) => {
                for task in &mut group.tasks {
                    normalize_task(task);
                }
            }
            tasks::v1::PlanItem::Task(task) => normalize_task(task),
        }
    }
}

fn normalize_task(task: &mut tasks::v1::ExecutableTask) {
    sort_strings(&mut task.requirement_ids.0);
    sort_optional_strings(task.boundaries.as_mut().map(|values| &mut values.0));
    sort_optional_strings(task.contracts.as_mut().map(|values| &mut values.0));
    sort_task_references(task.depends_on.as_mut());
}

fn sort_optional_strings(values: Option<&mut Vec<String>>) {
    if let Some(values) = values {
        sort_strings(values);
    }
}

fn sort_strings(values: &mut [String]) {
    values.sort_by(|left, right| compare_utf16(left, right));
}

fn sort_task_references(references: Option<&mut tasks::v1::TaskReferenceList>) {
    if let Some(references) = references {
        references
            .0
            .sort_by(|left, right| compare_utf16(&left.0, &right.0));
    }
}

fn compare_utf16(left: &str, right: &str) -> Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}
