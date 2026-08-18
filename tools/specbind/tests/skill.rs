use clap::CommandFactory as _;
use specbind::{args::Cli, install::Agent, protocol, rule, skill};

const ACCEPTED_SKILLS: [&str; 10] = [
    "specbind-contract-review",
    "specbind-debug",
    "specbind-design",
    "specbind-discovery",
    "specbind-implement",
    "specbind-requirements",
    "specbind-review-task",
    "specbind-status",
    "specbind-tasks",
    "specbind-validate-implementation",
];

#[test]
fn embeds_the_accepted_skill_set_with_valid_metadata() {
    let names = skill::all()
        .iter()
        .map(|entry| entry.name)
        .collect::<Vec<_>>();
    assert_eq!(names, ACCEPTED_SKILLS);

    for entry in skill::all() {
        let metadata = entry.metadata().expect("parseable Front Matter");
        assert_eq!(metadata.name, entry.name, "name must match the directory");
        assert!(!metadata.description.trim().is_empty());
        let body = entry.body().expect("body");
        assert!(
            body.trim_start().starts_with("# "),
            "{}: body must open with a title heading",
            entry.name
        );
    }
}

#[test]
fn renders_only_the_accepted_front_matter_per_agent() {
    for entry in skill::all() {
        let body = entry.body().expect("body");
        for agent in [Agent::ClaudeCode, Agent::Codex] {
            let rendered = entry.render(agent).expect("rendered skill");
            assert!(
                rendered.ends_with(body),
                "{}: {agent:?} must keep the body unchanged",
                entry.name
            );
            let frontmatter = rendered
                .strip_prefix("---\n")
                .and_then(|rest| rest.split_once("\n---\n"))
                .expect("rendered Front Matter")
                .0;
            assert!(frontmatter.contains("name: "));
            assert!(frontmatter.contains("description: "));
            // A permission grant or invocation restriction is never inferred
            // from skill content.
            for forbidden in ["allowed-tools", "disable-model-invocation"] {
                assert!(
                    !rendered.contains(forbidden),
                    "{}: {agent:?} must not emit {forbidden}",
                    entry.name
                );
            }
            if agent == Agent::Codex {
                assert!(
                    !frontmatter.contains("argument-hint"),
                    "{}: Codex Front Matter is name and description only",
                    entry.name
                );
            }
        }
    }
}

#[test]
fn installs_each_skill_to_the_accepted_target() {
    for entry in skill::all() {
        assert_eq!(
            entry.target(Agent::ClaudeCode),
            format!(".claude/skills/{}/SKILL.md", entry.name)
        );
        assert_eq!(
            entry.target(Agent::Codex),
            format!(".agents/skills/{}/SKILL.md", entry.name)
        );
    }
}

/// Every documented invocation must reference a real command route and only
/// options that route accepts. The command graph is walked, never executed.
#[test]
fn every_documented_invocation_resolves_against_the_command_graph() {
    let root = Cli::command();
    let mut checked = 0;
    for entry in skill::all() {
        for invocation in invocations(entry.body().expect("body")) {
            resolve(&root, entry.name, &invocation);
            checked += 1;
        }
    }
    assert!(
        checked >= 3,
        "the status skill documents several invocations; found {checked}"
    );
}

#[test]
fn every_named_protocol_selector_and_rule_path_exists() {
    for entry in skill::all() {
        let body = entry.body().expect("body");
        for selector in tokens_after(body, "specbind protocol read ") {
            assert!(
                protocol::read(&selector).is_some(),
                "{}: unknown protocol selector {selector}",
                entry.name
            );
        }
        for path in rule_paths(body) {
            let file = path.rsplit('/').next().unwrap_or_default();
            assert!(
                rule::defaults()
                    .iter()
                    .any(|default| default.file_name == file),
                "{}: {path} is not an installed default rule",
                entry.name
            );
        }
    }
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
        assert!(
            command
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

fn rule_paths(body: &str) -> Vec<String> {
    body.split_whitespace()
        .map(|token| token.trim_matches(['`', '.', ',', ')', '(']))
        .filter(|token| token.contains("settings/rules/"))
        .map(ToOwned::to_owned)
        .collect()
}
