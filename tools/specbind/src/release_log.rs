//! Strict release-summary input and canonical per-Spec OKF log updates.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
};

use pulldown_cmark::{Event, Parser, Tag};
use serde::Deserialize;
use time::{Date, Month};

use crate::config::ProjectLanguage;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedLogEntries {
    summaries: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogUpdate {
    Updated(String),
    Unchanged,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDocument {
    log_entries: Vec<RawEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEntry {
    spec: String,
    summary: String,
}

struct LogDocument {
    title: String,
    sections: Vec<LogSection>,
}

struct LogSection {
    date: Date,
    entries: Vec<String>,
}

impl ValidatedLogEntries {
    #[must_use]
    pub fn summary(&self, spec: &str) -> Option<&str> {
        self.summaries.get(spec).map(String::as_str)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.summaries.is_empty()
    }
}

/// Validates the strict transient JSON document against the exact participating Spec set.
///
/// # Errors
///
/// Returns JSON shape, summary safety, duplicate, missing, or extra Spec diagnostics.
pub fn validate_input(
    input: Option<&str>,
    expected_specs: &[String],
) -> Result<ValidatedLogEntries, Vec<LogIssue>> {
    let Some(input) = input else {
        if expected_specs.is_empty() {
            return Ok(ValidatedLogEntries {
                summaries: BTreeMap::new(),
            });
        }
        return Err(vec![issue(
            "LOG_ENTRIES_REQUIRED",
            None,
            "Spec-backed release finalization requires --log-entries",
        )]);
    };
    let raw = serde_json::from_str::<RawDocument>(input).map_err(|error| {
        vec![issue(
            "LOG_INPUT_INVALID",
            None,
            format!("log-entry JSON is invalid: {error}"),
        )]
    })?;
    let mut summaries = BTreeMap::new();
    let mut issues = Vec::new();
    for entry in raw.log_entries {
        let summary = entry.summary.trim().to_owned();
        if summary.is_empty() || summary.contains(['\r', '\n']) {
            issues.push(issue(
                "LOG_INPUT_INVALID",
                Some(entry.spec.clone()),
                "summary must be non-empty and single-line after trimming",
            ));
        }
        if summaries.insert(entry.spec.clone(), summary).is_some() {
            issues.push(issue(
                "LOG_ENTRY_SET_MISMATCH",
                Some(entry.spec),
                "participating Spec summaries must be unique",
            ));
        }
    }
    let expected = expected_specs.iter().cloned().collect::<BTreeSet<_>>();
    let actual = summaries.keys().cloned().collect::<BTreeSet<_>>();
    if expected != actual {
        let missing = expected.difference(&actual).cloned().collect::<Vec<_>>();
        let extra = actual.difference(&expected).cloned().collect::<Vec<_>>();
        issues.push(issue(
            "LOG_ENTRY_SET_MISMATCH",
            None,
            format!(
                "log-entry Spec set must exactly match Roadmap participants (missing: {}; extra: {})",
                display_set(&missing),
                display_set(&extra)
            ),
        ));
    }
    issues.sort();
    issues.dedup();
    if issues.is_empty() {
        Ok(ValidatedLogEntries { summaries })
    } else {
        Err(issues)
    }
}

/// Renders or recognizes one canonical milestone entry in an OKF `log.md`.
///
/// # Errors
///
/// Returns existing-log profile, generated-Markdown, or milestone-conflict diagnostics.
#[allow(
    clippy::too_many_arguments,
    reason = "the canonical log entry fields remain explicit at this narrow rendering boundary"
)]
pub fn update_log(
    existing: &str,
    language: ProjectLanguage,
    date: Date,
    version: &str,
    milestone_id: &str,
    roadmap_path: &str,
    summary: &str,
    relative_path: &str,
) -> Result<LogUpdate, Vec<LogIssue>> {
    let desired = canonical_entry(language, version, milestone_id, roadmap_path, summary);
    validate_generated_entry(&desired, language, version, milestone_id, roadmap_path).map_err(
        |message| {
            vec![issue(
                "LOG_INPUT_INVALID",
                Some(relative_path.to_owned()),
                message,
            )]
        },
    )?;
    let mut document = parse_log(existing, relative_path)?;
    let marker = format!("`{milestone_id}`");
    let matches = document
        .sections
        .iter()
        .flat_map(|section| &section.entries)
        .filter(|entry| entry.contains(&marker))
        .collect::<Vec<_>>();
    if matches.len() == 1 && matches[0].as_str() == desired {
        return Ok(LogUpdate::Unchanged);
    }
    if !matches.is_empty() {
        return Err(vec![issue(
            "LOG_ENTRY_CONFLICT",
            Some(relative_path.to_owned()),
            "existing milestone entry differs from the requested canonical release entry",
        )]);
    }
    if let Some(section) = document
        .sections
        .iter_mut()
        .find(|section| section.date == date)
    {
        section.entries.insert(0, desired);
    } else {
        let index = document
            .sections
            .iter()
            .position(|section| section.date < date)
            .unwrap_or(document.sections.len());
        document.sections.insert(
            index,
            LogSection {
                date,
                entries: vec![desired],
            },
        );
    }
    Ok(LogUpdate::Updated(render_log(&document)))
}

/// Verifies that a finalized log contains exactly the requested canonical milestone entry.
///
/// # Errors
///
/// Returns profile, generated-Markdown, missing-entry, or conflict diagnostics.
pub fn verify_entry(
    existing: &str,
    language: ProjectLanguage,
    version: &str,
    milestone_id: &str,
    roadmap_path: &str,
    summary: &str,
    relative_path: &str,
) -> Result<(), Vec<LogIssue>> {
    let desired = canonical_entry(language, version, milestone_id, roadmap_path, summary);
    validate_generated_entry(&desired, language, version, milestone_id, roadmap_path).map_err(
        |message| {
            vec![issue(
                "LOG_INPUT_INVALID",
                Some(relative_path.to_owned()),
                message,
            )]
        },
    )?;
    let document = parse_log(existing, relative_path)?;
    let marker = format!("`{milestone_id}`");
    let matches = document
        .sections
        .iter()
        .flat_map(|section| &section.entries)
        .filter(|entry| entry.contains(&marker))
        .collect::<Vec<_>>();
    if matches.len() == 1 && matches[0].as_str() == desired {
        Ok(())
    } else if matches.is_empty() {
        Err(vec![issue(
            "LOG_ENTRY_MISSING",
            Some(relative_path.to_owned()),
            "finalized Spec log is missing the requested milestone entry",
        )])
    } else {
        Err(vec![issue(
            "LOG_ENTRY_CONFLICT",
            Some(relative_path.to_owned()),
            "finalized Spec log contains a conflicting milestone entry",
        )])
    }
}

#[must_use]
pub fn empty_log(language: ProjectLanguage) -> String {
    format!("# {}\n", title(language))
}

/// Validates an existing OKF `log.md` profile without changing it.
///
/// # Errors
///
/// Returns title, date-order, or flat-entry profile diagnostics.
pub fn validate_existing(input: &str, relative_path: &str) -> Result<(), Vec<LogIssue>> {
    parse_log(input, relative_path).map(|_| ())
}

fn parse_log(input: &str, relative_path: &str) -> Result<LogDocument, Vec<LogIssue>> {
    let normalized = input.replace("\r\n", "\n");
    let lines = normalized.lines().collect::<Vec<_>>();
    let Some(first) = lines.first() else {
        return Err(invalid_log(
            relative_path,
            "log.md must contain one document title",
        ));
    };
    if !first.starts_with("# ") || first.starts_with("## ") || first.trim() == "#" {
        return Err(invalid_log(
            relative_path,
            "log.md must start with one non-empty H1 title",
        ));
    }
    let mut sections = Vec::<LogSection>::new();
    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue;
        }
        if let Some(value) = line.strip_prefix("## ") {
            let date = parse_date(value).ok_or_else(|| {
                invalid_log(
                    relative_path,
                    "log.md H2 headings must be ISO YYYY-MM-DD dates",
                )
            })?;
            if sections.last().is_some_and(|section| section.date <= date) {
                return Err(invalid_log(
                    relative_path,
                    "log.md date headings must be unique and newest first",
                ));
            }
            sections.push(LogSection {
                date,
                entries: Vec::new(),
            });
        } else if line.starts_with('#') {
            return Err(invalid_log(
                relative_path,
                "log.md permits only one H1 title and H2 dates",
            ));
        } else if line.starts_with("* ") {
            let Some(section) = sections.last_mut() else {
                return Err(invalid_log(
                    relative_path,
                    "log entries require a preceding date heading",
                ));
            };
            section.entries.push((*line).to_owned());
        } else {
            return Err(invalid_log(
                relative_path,
                "log.md date sections must contain only flat single-line unordered-list entries",
            ));
        }
    }
    if sections.iter().any(|section| section.entries.is_empty()) {
        return Err(invalid_log(
            relative_path,
            "every log.md date heading requires an entry",
        ));
    }
    Ok(LogDocument {
        title: (*first).to_owned(),
        sections,
    })
}

fn render_log(document: &LogDocument) -> String {
    let mut output = format!("{}\n", document.title);
    for section in &document.sections {
        write!(output, "\n## {}\n\n", section.date).expect("writing to a String cannot fail");
        for entry in &section.entries {
            output.push_str(entry);
            output.push('\n');
        }
    }
    output
}

fn validate_generated_entry(
    input: &str,
    language: ProjectLanguage,
    version: &str,
    milestone_id: &str,
    roadmap_path: &str,
) -> Result<(), String> {
    let mut unordered_lists = 0;
    let mut items = 0;
    let mut paragraphs = 0;
    let mut matching_links = 0;
    let mut matching_codes = 0;
    let mut strong_text = String::new();
    let mut in_strong = false;
    for event in Parser::new(input) {
        match event {
            Event::Start(Tag::List(None)) => unordered_lists += 1,
            Event::Start(Tag::List(Some(_))) => {
                return Err("generated entry must be unordered".to_owned());
            }
            Event::Start(Tag::Item) => items += 1,
            Event::Start(Tag::Paragraph) => paragraphs += 1,
            Event::Start(Tag::Strong) => in_strong = true,
            Event::End(pulldown_cmark::TagEnd::Strong) => in_strong = false,
            Event::Start(Tag::Link { dest_url, .. }) if dest_url.as_ref() == roadmap_path => {
                matching_links += 1;
            }
            Event::Code(value) if value.as_ref() == milestone_id => matching_codes += 1,
            Event::Text(value) if in_strong => strong_text.push_str(&value),
            _ => {}
        }
    }
    let expected_label = format!("{} {version}", release_label(language));
    if unordered_lists == 1
        && items == 1
        && paragraphs <= 1
        && matching_links == 1
        && matching_codes == 1
        && strong_text == expected_label
    {
        Ok(())
    } else {
        Err(format!(
            "summary corrupts the canonical single-item release Markdown structure (lists: {unordered_lists}, items: {items}, paragraphs: {paragraphs}, roadmap links: {matching_links}, milestone codes: {matching_codes}, label: {strong_text})"
        ))
    }
}

fn canonical_entry(
    language: ProjectLanguage,
    version: &str,
    milestone_id: &str,
    roadmap_path: &str,
    summary: &str,
) -> String {
    match language {
        ProjectLanguage::En => format!(
            "* **Release {version}** — {summary} ([roadmap]({roadmap_path}), milestone `{milestone_id}`)"
        ),
        ProjectLanguage::Ja => format!(
            "* **リリース {version}** — {summary} ([ロードマップ]({roadmap_path}), マイルストーン `{milestone_id}`)"
        ),
    }
}

fn title(language: ProjectLanguage) -> &'static str {
    match language {
        ProjectLanguage::En => "Spec Update Log",
        ProjectLanguage::Ja => "スペック更新ログ",
    }
}

fn release_label(language: ProjectLanguage) -> &'static str {
    match language {
        ProjectLanguage::En => "Release",
        ProjectLanguage::Ja => "リリース",
    }
}

fn parse_date(value: &str) -> Option<Date> {
    let components = value.split('-').collect::<Vec<_>>();
    let [year, month, day] = components.as_slice() else {
        return None;
    };
    if year.len() != 4 || month.len() != 2 || day.len() != 2 {
        return None;
    }
    Date::from_calendar_date(
        year.parse().ok()?,
        Month::try_from(month.parse::<u8>().ok()?).ok()?,
        day.parse().ok()?,
    )
    .ok()
}

fn display_set(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_owned()
    } else {
        values.join(", ")
    }
}

fn invalid_log(path: &str, message: impl Into<String>) -> Vec<LogIssue> {
    vec![issue("LOG_PROFILE_INVALID", Some(path.to_owned()), message)]
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> LogIssue {
    LogIssue {
        code,
        path,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MILESTONE: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

    #[test]
    fn validates_exact_summary_sets_and_single_line_content() {
        let expected = vec!["cart".to_owned(), "checkout".to_owned()];
        let input = r#"{"log_entries":[{"spec":"checkout","summary":"  Added checkout.  "},{"spec":"cart","summary":"Kept the cart."}]}"#;
        let entries = validate_input(Some(input), &expected).expect("valid input");
        assert_eq!(entries.summary("checkout"), Some("Added checkout."));

        let error = validate_input(
            Some(r#"{"log_entries":[{"spec":"checkout","summary":"bad\nline"}]}"#),
            &expected,
        )
        .expect_err("invalid summary and missing Spec");
        assert!(error.iter().any(|issue| issue.code == "LOG_INPUT_INVALID"));
        assert!(
            error
                .iter()
                .any(|issue| issue.code == "LOG_ENTRY_SET_MISMATCH")
        );
        let profile = validate_existing(
            "# Log\n\n## not-a-date\n\n* Entry.\n",
            "specs/checkout/log.md",
        )
        .expect_err("invalid date heading");
        assert_eq!(profile[0].code, "LOG_PROFILE_INVALID");
    }

    #[test]
    fn inserts_newest_first_and_recognizes_an_identical_entry() {
        let date = Date::from_calendar_date(2026, Month::August, 16).expect("date");
        let existing = "# Existing title\n\n## 2026-07-20\n\n* Older entry.\n";
        let LogUpdate::Updated(updated) = update_log(
            existing,
            ProjectLanguage::En,
            date,
            "v1.4.0",
            MILESTONE,
            "../../releases/v1.4.0-roadmap.md",
            "Added authenticated checkout.",
            "specs/checkout/log.md",
        )
        .expect("update log") else {
            panic!("new milestone should update the log");
        };
        assert!(updated.starts_with("# Existing title\n\n## 2026-08-16"));
        assert!(updated.contains("## 2026-07-20"));
        assert_eq!(
            update_log(
                &updated,
                ProjectLanguage::En,
                date,
                "v1.4.0",
                MILESTONE,
                "../../releases/v1.4.0-roadmap.md",
                "Added authenticated checkout.",
                "specs/checkout/log.md",
            )
            .expect("idempotent update"),
            LogUpdate::Unchanged
        );
    }

    #[test]
    fn localizes_new_log_prose_and_rejects_conflicting_milestones() {
        let date = Date::from_calendar_date(2026, Month::August, 16).expect("date");
        let LogUpdate::Updated(updated) = update_log(
            &empty_log(ProjectLanguage::Ja),
            ProjectLanguage::Ja,
            date,
            "v1.4.0",
            MILESTONE,
            "../../releases/v1.4.0-roadmap.md",
            "認証済みチェックアウトを追加した。",
            "specs/checkout/log.md",
        )
        .expect("Japanese update") else {
            panic!("new milestone should update the log");
        };
        assert!(updated.starts_with("# スペック更新ログ"));
        assert!(updated.contains("**リリース v1.4.0**"));
        assert!(updated.contains("[ロードマップ]"));

        let error = update_log(
            &updated,
            ProjectLanguage::Ja,
            date,
            "v1.4.0",
            MILESTONE,
            "../../releases/v1.4.0-roadmap.md",
            "異なる要約。",
            "specs/checkout/log.md",
        )
        .expect_err("same milestone with different summary must conflict");
        assert_eq!(error[0].code, "LOG_ENTRY_CONFLICT");
    }
}
