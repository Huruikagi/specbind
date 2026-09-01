## SpecBind

This project uses SpecBind for spec-driven development. The `specbind` CLI owns
the specification lifecycle: it validates artifacts, records approvals, and is
the only supported writer of machine state.

- Hyphenated names such as `sb-status` identify installed Skills, not
  shell commands. Select them through the agent platform; do not translate a
  Skill name into a `specbind ...` command. CLI syntax comes from the selected
  Skill.
- Work through those installed `specbind-*` Skills. Use `sb-discovery` to
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
- When every Task for a named Spec is complete and the user asks whether that
  Spec is done, complete, or ready, use `sb-validate-implementation`.
  Do not answer that question from status or consequence-free claim checking.
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
  owns; or adds a durable responsibility. When the classification is genuinely
  unclear, enter the flow. Anything else is ordinary work: say in one line that
  it needs no Spec, and do it.
- Never hand-edit `spec.yaml`, the active roadmap, or the execution state in
  `tasks.yaml`. Those are CLI-owned, and a hand edit produces state no command
  validated. The task plan itself is authored, by the skill that owns it.
- Run `specbind --help` if the command is unfamiliar or appears unavailable.
