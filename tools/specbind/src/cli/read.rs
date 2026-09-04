//! CLI execution facade for read and installation commands.

use super::CommandOutput;
use crate::config;

mod artifact_commands;
mod catalog_commands;
mod embedded_catalog_commands;
mod install_commands;
mod project_commands;
mod removal_commands;
mod template_commands;

pub use artifact_commands::{
    artifact_list, artifact_read, check_contracts, check_traceability, contract_consumers,
    contract_dependencies, contract_graph, contract_owners,
};
pub use catalog_commands::{
    adapter_list, adapter_read, adapter_read_for_consume, rule_list, rule_read, steering_check,
    steering_list, steering_read,
};
pub use embedded_catalog_commands::{protocol_list, protocol_read, schema_list, schema_read};
pub use install_commands::{install_apply, install_dry_run};
pub use project_commands::{adoption_preflight, configuration_show, milestone_scope, spec_list};
pub use removal_commands::{
    remove_agent_apply, remove_agent_plan, uninstall_apply, uninstall_plan,
};
pub use template_commands::{
    template_list_milestone, template_list_spec, template_list_steering, template_read_milestone,
    template_read_spec, template_read_steering, template_resolve_spec,
};

fn present(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn project_relative_spec_root(paths: &config::ProjectPaths) -> Result<String, CommandOutput> {
    paths
        .specbind_root
        .strip_prefix(&paths.project_root)
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .map_err(|_| {
            CommandOutput::failure(
                "SPEC_ROOT_INVALID",
                "configured specDir must remain below the resolved project root",
                vec![],
            )
        })
}
