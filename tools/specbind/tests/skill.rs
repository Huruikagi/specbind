use clap::CommandFactory as _;
use specbind::{agent_role, args::Cli, install::Agent, protocol, rule, skill};

const ACCEPTED_SKILLS: [&str; 16] = [
    "sb-adopt",
    "sb-contract-review",
    "sb-configure",
    "sb-debug",
    "sb-discovery",
    "sb-drive",
    "sb-gap-analysis",
    "sb-implement",
    "sb-plan",
    "sb-release",
    "sb-review-task",
    "sb-status",
    "sb-steering",
    "sb-validate-design",
    "sb-validate-implementation",
    "sb-verify-completion",
];

#[path = "skill/authoring_contracts.rs"]
mod authoring_contracts;
#[path = "skill/cross_skill_contracts.rs"]
mod cross_skill_contracts;
#[path = "skill/delivery_contracts.rs"]
mod delivery_contracts;
#[path = "skill/planning_contracts.rs"]
mod planning_contracts;

fn skill_documents(entry: skill::Skill) -> Vec<&'static str> {
    std::iter::once(entry.body().expect("body"))
        .chain(entry.resources().iter().map(|resource| resource.content()))
        .collect()
}

fn skill_package_text(name: &str) -> String {
    let entry = skill::find(name).unwrap_or_else(|| panic!("missing skill {name}"));
    skill_documents(entry).join("\n")
}

fn skill_resource_text(name: &str, relative_path: &str) -> &'static str {
    skill::find(name)
        .unwrap_or_else(|| panic!("missing skill {name}"))
        .resources()
        .iter()
        .find(|resource| resource.relative_path == relative_path)
        .unwrap_or_else(|| panic!("missing resource {name}/{relative_path}"))
        .content()
}

/// Extracts each literal invocation: a standalone inline code span or one line
/// of a shell fence, beginning with the exact token `specbind `.
fn invocations(body: &str) -> Vec<Vec<String>> {
    let mut found = Vec::new();
    let mut in_fence = false;
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            if let Some(rest) = trimmed.strip_prefix("specbind ") {
                found.push(words(rest));
            }
            continue;
        }
        let mut rest = trimmed;
        while let Some(open) = rest.find('`') {
            let after = &rest[open + 1..];
            let Some(close) = after.find('`') else { break };
            let span = &after[..close];
            if let Some(arguments) = span.strip_prefix("specbind ") {
                found.push(words(arguments));
            }
            rest = &after[close + 1..];
        }
    }
    found
}

fn words(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>()
}

/// Walks the subcommand path, then checks every long option against that route.
fn resolve(root: &clap::Command, skill: &str, invocation: &[String]) {
    let mut command = root;
    let mut index = 0;
    while index < invocation.len() {
        let token = invocation[index].as_str();
        if token.starts_with('-') || is_metavariable(token) {
            break;
        }
        // A leaf route takes positional values, and a literal one such as the
        // `spec` scope of `template read` is a value, not a missing command.
        if command.get_subcommands().next().is_none() {
            break;
        }
        let Some(next) = command
            .get_subcommands()
            .find(|candidate| candidate.get_name() == token)
        else {
            panic!(
                "{skill}: `specbind {}` has no command {token}",
                invocation.join(" ")
            );
        };
        command = next;
        index += 1;
    }
    for token in &invocation[index..] {
        let Some(long) = token.strip_prefix("--") else {
            continue;
        };
        let name = long.split('=').next().unwrap_or(long);
        let clap_builtin = index == 0 && matches!(name, "help" | "version");
        assert!(
            clap_builtin
                || command
                    .get_arguments()
                    .any(|argument| argument.get_long() == Some(name)),
            "{skill}: `specbind {}` uses unknown option --{name}",
            invocation.join(" ")
        );
    }
}

/// Presentation metavariables are not runtime values.
fn is_metavariable(token: &str) -> bool {
    (token.starts_with('<') && token.ends_with('>'))
        || (token.starts_with('[') && token.ends_with(']'))
}

fn tokens_after(body: &str, prefix: &str) -> Vec<String> {
    body.lines()
        .filter_map(|line| line.trim().split_once(prefix))
        .filter_map(|(_, rest)| rest.split_whitespace().next())
        .map(|token| token.trim_matches('`').to_owned())
        .filter(|token| !is_metavariable(token))
        .collect()
}
