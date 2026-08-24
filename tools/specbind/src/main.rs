use std::{
    io::{self, Write as _},
    path::Path,
    process::ExitCode,
};

use clap::Parser as _;
use specbind::args::{
    AdapterCommand, AdoptionCommand, ArtifactCommand, CheckCommand, Cli, Command, DirectCommand,
    GateCommand, MigrateCommand, MilestoneCommand, ProtocolCommand, ReleaseCommand, ReviewCommand,
    RuleCommand, SchemaCommand, SpecCommand, SpecCompletionCommand, SteeringCommand, TasksCommand,
    TemplateCommand,
};
use specbind::cli::CommandOutput;

fn run_gate(start: &Path, gate: specbind::approval::Gate, command: GateCommand) -> CommandOutput {
    match command {
        GateCommand::Approve {
            spec,
            approval_mode,
            delegation_workflow,
            requirement_ids,
        } => specbind::cli::spec_gate_approve(
            start,
            &spec,
            gate,
            &approval_mode,
            delegation_workflow.as_deref(),
            requirement_ids.as_deref(),
        ),
        GateCommand::Invalidate { spec } => specbind::cli::spec_gate_invalidate(start, &spec, gate),
    }
}

fn run_artifact(start: &Path, command: ArtifactCommand) -> CommandOutput {
    match command {
        ArtifactCommand::List { spec } => specbind::cli::artifact_list(start, &spec),
        ArtifactCommand::Read {
            spec,
            selector,
            purpose,
        } => specbind::cli::artifact_read(start, &spec, &selector, purpose.as_deref()),
    }
}

fn run_install(
    start: &Path,
    dry_run: bool,
    agents: &[String],
    language: Option<&str>,
    spec_dir: Option<String>,
    project_instructions: bool,
) -> CommandOutput {
    let inputs = specbind::install::InstallInputs {
        agents: agents
            .iter()
            .filter_map(|value| specbind::install::Agent::parse(value))
            .collect(),
        language: match language {
            Some("en") => Some(specbind::config::ProjectLanguage::En),
            Some("ja") => Some(specbind::config::ProjectLanguage::Ja),
            _ => None,
        },
        spec_dir,
        project_instructions: project_instructions.then_some(true),
    };
    if dry_run {
        specbind::cli::install_dry_run(start, &inputs)
    } else {
        specbind::cli::install_apply(start, &inputs)
    }
}

fn run_remove_agent(start: &Path, agent: &str, apply: bool) -> CommandOutput {
    let agent = specbind::install::Agent::parse(agent)
        .expect("clap restricts remove-agent to supported agents");
    if apply {
        specbind::cli::remove_agent_apply(start, agent)
    } else {
        specbind::cli::remove_agent_plan(start, agent)
    }
}

fn run_uninstall(start: &Path, knowledge: &str, apply: bool) -> CommandOutput {
    let knowledge = match knowledge {
        "retain" => specbind::removal::KnowledgePolicy::Retain,
        "remove" => specbind::removal::KnowledgePolicy::Remove,
        _ => unreachable!("clap restricts uninstall knowledge policy"),
    };
    if apply {
        specbind::cli::uninstall_apply(start, knowledge)
    } else {
        specbind::cli::uninstall_plan(start, knowledge)
    }
}

fn run_check(start: &Path, command: CheckCommand) -> CommandOutput {
    match command {
        CheckCommand::Traceability { spec } => specbind::cli::check_traceability(start, &spec),
        CheckCommand::Contracts => specbind::cli::check_contracts(start),
    }
}

fn run_template(start: &Path, command: TemplateCommand) -> CommandOutput {
    match command {
        TemplateCommand::List { scope } if scope == "steering" => {
            specbind::cli::template_list_steering(start)
        }
        TemplateCommand::List { scope } if scope == "milestone" => {
            specbind::cli::template_list_milestone(start)
        }
        TemplateCommand::List { scope: _ } => specbind::cli::template_list_spec(start),
        TemplateCommand::Read { scope, selector } if scope == "steering" => {
            specbind::cli::template_read_steering(start, &selector)
        }
        TemplateCommand::Read { scope, selector } if scope == "milestone" => {
            specbind::cli::template_read_milestone(start, &selector)
        }
        TemplateCommand::Read { scope: _, selector } => {
            specbind::cli::template_read_spec(start, &selector)
        }
        TemplateCommand::Resolve {
            scope: _,
            spec,
            selector,
        } => specbind::cli::template_resolve_spec(start, &spec, &selector),
    }
}

fn run_tasks(start: &Path, command: TasksCommand) -> CommandOutput {
    match command {
        TasksCommand::List { spec } => specbind::cli::tasks_list(start, &spec),
        TasksCommand::Show { spec, task_id } => specbind::cli::tasks_show(start, &spec, &task_id),
        TasksCommand::Complete { spec, task_id } => {
            specbind::cli::tasks_complete(start, &spec, &task_id)
        }
        TasksCommand::Block {
            spec,
            task_id,
            reason,
        } => specbind::cli::tasks_block(start, &spec, &task_id, &reason),
        TasksCommand::Reopen { spec, task_id } => {
            specbind::cli::tasks_reopen(start, &spec, &task_id)
        }
    }
}

fn run_schema(command: SchemaCommand) -> CommandOutput {
    match command {
        SchemaCommand::List => specbind::cli::schema_list(),
        SchemaCommand::Read { selector } => specbind::cli::schema_read(&selector),
    }
}

fn run_adapter(start: &Path, command: AdapterCommand) -> CommandOutput {
    match command {
        AdapterCommand::List => specbind::cli::adapter_list(start),
        AdapterCommand::Read { selector } => specbind::cli::adapter_read(start, &selector),
    }
}

fn run_rule(start: &Path, command: RuleCommand) -> CommandOutput {
    match command {
        RuleCommand::List => specbind::cli::rule_list(start),
        RuleCommand::Read { selector, purpose } => {
            specbind::cli::rule_read(start, &selector, purpose.as_deref())
        }
    }
}

fn run_steering(start: &Path, command: SteeringCommand) -> CommandOutput {
    match command {
        SteeringCommand::List => specbind::cli::steering_list(start),
        SteeringCommand::Read { selector, purpose } => {
            specbind::cli::steering_read(start, &selector, purpose.as_deref())
        }
    }
}

fn run_adoption(start: &Path, command: &AdoptionCommand) -> CommandOutput {
    match command {
        AdoptionCommand::Preflight => specbind::cli::adoption_preflight(start),
    }
}

fn run_spec(start: &Path, command: SpecCommand) -> CommandOutput {
    match command {
        SpecCommand::List => specbind::cli::spec_list(start),
        SpecCommand::Status { spec } => specbind::cli::spec_status(start, &spec),
        SpecCommand::Completion { command } => run_spec_completion(start, command),
        SpecCommand::Requirements { command } => {
            run_gate(start, specbind::approval::Gate::Requirements, command)
        }
        SpecCommand::Design { command } => {
            run_gate(start, specbind::approval::Gate::Design, command)
        }
        SpecCommand::Tasks { command } => run_gate(start, specbind::approval::Gate::Tasks, command),
    }
}

fn run_spec_completion(start: &Path, command: SpecCompletionCommand) -> CommandOutput {
    match command {
        SpecCompletionCommand::Preflight { spec } => {
            specbind::cli::spec_completion_preflight(start, &spec)
        }
        SpecCompletionCommand::Accept { spec, evidence } => {
            specbind::cli::spec_completion_accept(start, &spec, &evidence)
        }
        SpecCompletionCommand::Invalidate { spec } => {
            specbind::cli::spec_completion_invalidate(start, &spec)
        }
    }
}

fn run_milestone(start: &Path, command: MilestoneCommand) -> CommandOutput {
    match command {
        MilestoneCommand::Status => specbind::cli::milestone_status(start),
        MilestoneCommand::Scope { include_body } => {
            specbind::cli::milestone_scope(start, include_body)
        }
        MilestoneCommand::BindRelease { version, rebind } => {
            specbind::cli::milestone_bind_release(start, &version, rebind)
        }
        MilestoneCommand::Direct { command } => run_direct(start, command),
        MilestoneCommand::Review {
            command: ReviewCommand::Status,
        } => specbind::cli::milestone_review_status(start),
        MilestoneCommand::Review {
            command: ReviewCommand::Accept { candidate },
        } => specbind::cli::milestone_review_accept(start, &candidate),
        MilestoneCommand::Create { scope } => specbind::cli::milestone_create(start, &scope),
        MilestoneCommand::UpdateScope { scope } => {
            specbind::cli::milestone_update_scope(start, &scope)
        }
        MilestoneCommand::Rebaseline { revision } => {
            specbind::cli::milestone_rebaseline(start, &revision)
        }
    }
}

fn run_direct(start: &Path, command: DirectCommand) -> CommandOutput {
    match command {
        DirectCommand::Preflight { direct } => {
            specbind::cli::direct_completion_preflight(start, &direct)
        }
        DirectCommand::Complete {
            direct,
            implementation_revision,
        } => specbind::cli::direct_completion_complete(start, &direct, &implementation_revision),
    }
}

fn run_release(start: &Path, command: ReleaseCommand) -> CommandOutput {
    match command {
        ReleaseCommand::Preflight => specbind::cli::release_preflight(start),
        ReleaseCommand::Finalize { log_entries } => {
            specbind::cli::release_finalize(start, log_entries.as_deref())
        }
    }
}

fn run_migrate(start: &Path, command: &MigrateCommand) -> CommandOutput {
    match command {
        MigrateCommand::CcSdd {
            apply,
            accept_resolution,
        } => specbind::cli::migrate_cc_sdd(start, *apply, accept_resolution.as_deref()),
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let start = match std::env::current_dir() {
        Ok(start) => start,
        Err(error) => {
            eprintln!("ERROR CURRENT_DIRECTORY_FAILED: {error}");
            return ExitCode::FAILURE;
        }
    };
    let output = match cli.command {
        Command::Artifact { command } => run_artifact(&start, command),
        Command::Install {
            dry_run,
            agents,
            language,
            spec_dir,
            project_instructions,
        } => run_install(
            &start,
            dry_run,
            &agents,
            language.as_deref(),
            spec_dir,
            project_instructions,
        ),
        Command::RemoveAgent { agent, apply } => run_remove_agent(&start, &agent, apply),
        Command::Uninstall { knowledge, apply } => run_uninstall(&start, &knowledge, apply),
        Command::Protocol { command } => match command {
            ProtocolCommand::List => specbind::cli::protocol_list(),
            ProtocolCommand::Read { selector } => specbind::cli::protocol_read(&selector),
        },
        Command::Check { command } => run_check(&start, command),
        Command::Template { command } => run_template(&start, command),
        Command::Tasks { command } => run_tasks(&start, command),
        Command::Schema { command } => run_schema(command),
        Command::Adapter { command } => run_adapter(&start, command),
        Command::Rule { command } => run_rule(&start, command),
        Command::Steering { command } => run_steering(&start, command),
        Command::Adoption { command } => run_adoption(&start, &command),
        Command::Spec { command } => run_spec(&start, command),
        Command::Milestone { command } => run_milestone(&start, command),
        Command::Release { command } => run_release(&start, command),
        Command::Migrate { command } => run_migrate(&start, &command),
    };
    if let Err(error) = io::stdout().write_all(&output.stdout) {
        eprintln!("ERROR STDOUT_WRITE_FAILED: {error}");
        return ExitCode::FAILURE;
    }
    if let Err(error) = io::stderr().write_all(&output.stderr) {
        eprintln!("ERROR STDERR_WRITE_FAILED: {error}");
        return ExitCode::FAILURE;
    }
    if output.success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
