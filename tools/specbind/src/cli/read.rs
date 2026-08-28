//! CLI execution facade for read and installation commands.

mod artifact_commands;
mod catalog_commands;
mod install_commands;
mod project_commands;
mod removal_commands;

pub use artifact_commands::{artifact_list, artifact_read, check_contracts, check_traceability};
pub use catalog_commands::{
    adapter_list, adapter_read, protocol_list, protocol_read, rule_list, rule_read, schema_list,
    schema_read, steering_list, steering_read, template_list_milestone, template_list_spec,
    template_list_steering, template_read_milestone, template_read_spec, template_read_steering,
    template_resolve_spec,
};
pub use install_commands::{install_apply, install_dry_run};
pub use project_commands::{adoption_preflight, configuration_show, milestone_scope, spec_list};
pub use removal_commands::{
    remove_agent_apply, remove_agent_plan, uninstall_apply, uninstall_plan,
};

fn present(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
