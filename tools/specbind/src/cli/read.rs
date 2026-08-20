//! CLI execution facade for read and installation commands.

mod artifact_commands;
mod catalog_commands;
mod install_commands;
mod project_commands;

pub use artifact_commands::{artifact_list, artifact_read, check_contracts, check_traceability};
pub use catalog_commands::{
    adapter_list, adapter_read, protocol_list, protocol_read, schema_list, schema_read,
    steering_list, steering_read, template_list_spec, template_list_steering, template_read_spec,
    template_read_steering,
};
pub use install_commands::{install_apply, install_dry_run};
pub use project_commands::{milestone_scope, spec_list};

fn present(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
