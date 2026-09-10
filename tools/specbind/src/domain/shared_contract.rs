//! Validated project shared resources; no Spec lifecycle or execution policy.
use super::{SemanticIssues, diagnostics::issue};
use crate::schema::shared_contract::v1::SharedContractDocument;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedContract(SharedContractDocument);

impl SharedContract {
    #[must_use]
    pub fn as_wire(&self) -> &SharedContractDocument {
        &self.0
    }
}

impl TryFrom<SharedContractDocument> for SharedContract {
    type Error = SemanticIssues;
    fn try_from(value: SharedContractDocument) -> Result<Self, Self::Error> {
        let mut issues = Vec::new();
        let mut ids = BTreeSet::new();
        for (index, resource) in value.resources.iter().enumerate() {
            let pointer = format!("/resources/{index}");
            if !ids.insert(&resource.id.0) {
                issues.push(issue(
                    "SHARED_CONTRACT_ID_DUPLICATE",
                    &pointer,
                    "resource IDs must be unique",
                ));
            }
            if resource.description.trim().is_empty()
                || resource.change_policy.trim().is_empty()
                || resource
                    .invariants
                    .iter()
                    .any(|text| text.trim().is_empty())
            {
                issues.push(issue(
                    "SHARED_CONTRACT_TEXT_EMPTY",
                    &pointer,
                    "resource text must not be blank",
                ));
            }
            let mut paths = BTreeSet::new();
            for path in &resource.paths {
                if !super::contract::valid_path(path) {
                    issues.push(issue(
                        "SHARED_CONTRACT_PATH_INVALID",
                        &pointer,
                        format!("invalid resource path: {path}"),
                    ));
                } else if !paths.insert(path.to_ascii_lowercase()) {
                    issues.push(issue(
                        "SHARED_CONTRACT_PATH_DUPLICATE",
                        &pointer,
                        format!("duplicate resource path: {path}"),
                    ));
                }
            }
        }
        if issues.is_empty() {
            Ok(Self(value))
        } else {
            Err(SemanticIssues::from_unsorted(issues))
        }
    }
}
