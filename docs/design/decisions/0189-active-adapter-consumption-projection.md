# 0189: Project only active adapter guidance for consumers

Status: Accepted

## Context

`specbind adapter list` already classifies every accepted project adapter as
`absent`, `scaffold`, or `active`, while `specbind adapter read <selector>`
returns the exact raw project Markdown. Product Skills nevertheless read raw
adapter content and repeatedly inspect the product-owned
`<!-- specbind:adapter-scaffold -->` marker before deciding whether the content
is operational guidance.

That duplicates a deterministic catalog judgment in every consumer and lets an
Agent mistake scaffold prose for project policy. A forward-test debrief observed
exactly that ambiguity: raw `adapter read` output carried no result line saying
that the returned document was an inactive scaffold.

Configuration still needs exact raw Markdown so it can inspect, present, and
replace project-owned content. Runtime consumers need the active policy or an
explicit no-change result, not the scaffold body.

## Decision

`adapter read` gains one consumer projection:

```text
specbind adapter read <selector> --for consume
```

The existing command without `--for` remains byte-for-byte raw Markdown for a
present adapter and preserves `NO_CHANGE ADAPTER_ABSENT` for absence.

The consumer projection resolves and reads the accepted selector in one current
filesystem observation:

- `active` returns the exact raw project Markdown, including an intentionally
  empty body after Front Matter;
- `absent` returns the existing successful
  `NO_CHANGE ADAPTER_ABSENT` result;
- `scaffold` returns the successful
  `NO_CHANGE ADAPTER_SCAFFOLD` result and never exposes scaffold prose as
  operational policy;
- an unknown selector, invalid target, unreadable file, or non-UTF-8 content
  keeps the existing error behavior.

Only the exact purpose `consume` is accepted. Adapters have no managed
`maintain` instruction projection: `sb-configure` uses the raw read when it owns
adapter inspection and editing.

The projection classifies catalog state only. Each consuming Skill still owns
selector-specific absence and scaffold consequences. In particular, Release
routes both results to its one-time bootstrap, while Git, Deferred Findings,
and Validation consumers treat them as no active project guidance according to
their existing contracts. An active free-form body remains Agent-interpreted;
the CLI does not decide applicability, execute code blocks, grant authority, or
judge success.

All product Skill consumption uses `--for consume`. A Skill uses raw
`adapter read` only when it is inspecting or maintaining the adapter itself.

## Consequences

- Product Skills no longer parse or restate the scaffold marker contract.
- A consumer cannot accidentally execute instructional scaffold prose as
  project policy.
- Raw project content remains available for configuration and audit.
- Release's explicit-empty branch remains distinguishable from an inactive
  scaffold because an unmarked empty body is still `active` and is returned.
- Adapter meaning and authority remain outside the Rust CLI.

## Verification

CLI tests cover active, absent, scaffold, raw-scaffold, and invalid-purpose
reads. Skill conformance tests require consumers to use the projection and keep
`sb-configure` adapter maintenance on the raw read. English and Japanese
customization guides document the two read modes.
