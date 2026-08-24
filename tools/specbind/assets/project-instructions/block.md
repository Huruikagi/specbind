## SpecBind

This project uses SpecBind for spec-driven development. The `specbind` CLI owns
the specification lifecycle: it validates artifacts, records approvals, and is
the only supported writer of machine state.

- Work through the installed `specbind-*` skills. Use `specbind-discovery` to
  turn a request into scope, and `specbind-status` to see where work stands.
- Use `specbind-steering` when the request creates or updates durable,
  project-wide guidance, including conventions for testing, APIs, security, or
  deployment. This route does not require a Spec or observable behavior change.
- Use `specbind-adopt-existing` only when the user explicitly wants to establish
  new Specs from an existing implementation. It requires committed Steering and
  treats code and tests as evidence rather than intended specification.
- A request enters that flow when it changes a Spec's artifacts or observable
  behavior, including a validation rule, limit, or rejected case; modifies a path
  the Spec owns; adds a durable responsibility; or belongs to a delivery the
  project is tracking. When that classification is genuinely unclear, enter the
  flow. Anything else is ordinary work: say in one line that it needs no Spec,
  and do it.
- Never hand-edit `spec.yaml`, the active roadmap, or the execution state in
  `tasks.yaml`. Those are CLI-owned, and a hand edit produces state no command
  validated. The task plan itself is authored, by the skill that owns it.
- Run `specbind --help` if the command is unfamiliar or appears unavailable.
