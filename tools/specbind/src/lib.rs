mod catalog;
mod documents;
mod foundation;
mod infrastructure;
mod installation;
mod lifecycle;
mod read_model;

pub mod args;
pub mod artifacts;
pub mod cli;
pub mod domain;
pub mod migration;
pub use migration::resolution as migration_resolution;
pub mod schema;

pub use catalog::{adapter, protocol, rule, skill, steering, template};
pub use documents::{contract, design, instruction, requirements, roadmap, traceability};
pub use foundation::{config, fingerprint, yaml};
pub(crate) use infrastructure::{guarded_fs, repository};
pub use installation::{agent_role, install, project_instructions, removal};
pub use lifecycle::{
    approval, completion, cross_spec_review, milestone, release, release_finalize, release_log,
    task_progress,
};
pub use read_model::{
    contract_graph, freshness, milestone_scope, milestone_status, release_readiness, spec_list,
    spec_status, task_read_model,
};
