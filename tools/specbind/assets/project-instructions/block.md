## SpecBind

This project uses SpecBind for spec-driven development. The `specbind` CLI owns
the specification lifecycle: it validates artifacts, records approvals, and is
the only supported writer of machine state.

- Hyphenated names such as `sb-status` identify installed Skills, not
  shell commands. Select them through the agent platform; do not translate a
  Skill name into a `specbind ...` command. CLI syntax comes from the selected
  Skill.
- When the user explicitly asks to update the SpecBind binary, change its
  mise-selected version, or refresh project assets as part of that update,
  select `sb-configure` directly before ordinary change-request routing. Its
  update procedure proves installation ownership and preserves the separate
  binary-selection and project-asset checkpoints.
- Work through those installed `sb-*` Skills. Use `sb-discovery` to
  turn a request into scope or to establish new Specs from an explicitly
  selected existing implementation, and `sb-status` to see where work
  stands. Existing code and tests are evidence rather than intended
  specification; the adoption route requires committed Steering.
- Use `sb-drive` when the user asks to drive, continue, or advance the
  active milestone as far as safely possible. It may cross planning,
  implementation, and validation while preserving each owning Skill. A request
  to implement one named Roadmap item still uses `sb-implement`.
- Use `sb-plan` as the default planning entry point when the user asks to
  plan active work. It coordinates Requirements through Tasks approval and is
  also the single entry point when the user explicitly asks to work on only
  Requirements, Design, or Tasks for one named Spec. If an ordinary planning
  request names neither one Spec nor all Specs, select `sb-plan` and let
  it ask for scope; do not infer a single-phase request from the currently
  actionable phase.
- Use `sb-verify-completion` when the user asks whether an explicit claim is
  true and the answer must change nothing, including a claim that names a Spec.
  This route takes precedence whenever the request could also be read as
  lifecycle validation but does not explicitly authorize recording completion.
  Use `sb-validate-implementation` only when every Task for a named Spec is
  complete and the user asks to validate it for lifecycle completion, recording
  completion evidence on `GO`. Words such as done, complete, or ready do not by
  themselves authorize that mutation.
- Use `sb-validate-design` for an independent, read-only judgment of a Spec's
  Design. Use `sb-contract-review` for the milestone-wide Contract Review after
  participating Designs are approved.
- Use `sb-gap-analysis` when the user explicitly asks to compare planned work
  with the existing repository. Use `sb-release` only for an explicit request
  to release and finalize the active milestone.
- When the user asks to review one implemented Task, use
  `sb-review-task`; the review must judge the actual diff without fixing
  it or recording Task state.
- When the user asks why a Task failed or cannot be implemented, use
  `sb-debug` directly. A diagnosis-only request does not start
  implementation, and its final response must preserve the exact diagnosis
  block rather than summarize a nested result.
- Use `sb-steering` when the request creates or updates durable,
  project-wide guidance, including conventions for testing, APIs, security, or
  deployment. This route does not require a Spec or observable behavior change.
- For a change request, run `specbind milestone status` before choosing between
  discovery, steering, ordinary work, or implementation. A request matching a
  pending Spec-backed or Direct item is tracked delivery work and routes to
  `sb-implement`; that match takes precedence even when the requested
  output also looks like durable project-wide guidance. Otherwise a request
  enters the flow when it changes a Spec's artifacts or observable behavior,
  including a validation rule, limit, or rejected case; modifies a path the Spec
  owns; or adds a durable responsibility. For every concrete project-relative
  path supplied by the request, run `specbind contract owners <path>`; any
  returned owner establishes the owned-path condition, while `Owners: none`
  does not waive the other entry conditions. When the classification is
  genuinely unclear, enter the flow. Anything else is ordinary work: say in one
  line that it needs no Spec, and do it.
- An explicit request to establish Specs from an existing implementation is
  not a change request for that routing check. Select `sb-discovery` directly
  and let its reverse procedure run `specbind adoption preflight` before any
  ordinary `specbind milestone status` or `specbind spec list` read.
- Never hand-edit `spec.yaml`, the active roadmap, or the execution state in
  `tasks.yaml`. Those are CLI-owned, and a hand edit produces state no command
  validated. The task plan itself is authored, by the skill that owns it.
- Run `specbind --help` if the command is unfamiliar or appears unavailable.
