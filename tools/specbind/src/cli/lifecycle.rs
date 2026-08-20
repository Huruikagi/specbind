//! CLI execution facade for lifecycle commands.

mod completion_commands;
mod gate_commands;
mod milestone_commands;
mod release_commands;
mod review_commands;
mod status_commands;

pub use completion_commands::{
    direct_completion_complete, direct_completion_preflight, spec_completion_accept,
    spec_completion_invalidate, spec_completion_preflight,
};
pub use gate_commands::{spec_gate_approve, spec_gate_invalidate};
pub use milestone_commands::{
    milestone_bind_release, milestone_create, milestone_rebaseline, milestone_update_scope,
};
pub use release_commands::{release_finalize, release_preflight};
pub use review_commands::{milestone_review_accept, milestone_review_status};
pub use status_commands::{milestone_status, spec_status};
