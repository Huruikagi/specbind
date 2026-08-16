use std::fmt;

/// One stable, owned semantic-validation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticIssue {
    pub code: &'static str,
    pub path: String,
    pub message: String,
}

/// All semantic contradictions found in one structurally valid artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticIssues {
    pub issues: Vec<SemanticIssue>,
}

impl SemanticIssues {
    pub(crate) fn from_unsorted(mut issues: Vec<SemanticIssue>) -> Self {
        issues.sort();
        issues.dedup();
        Self { issues }
    }
}

impl fmt::Display for SemanticIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "structured artifact has {} semantic issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for SemanticIssues {}

pub(crate) fn issue(
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
