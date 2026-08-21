# 0138: Give adapter scaffolds a dedicated exact marker

Status: Accepted

## Context

Decision 0059 defines `specbind:instruction` as template-only guidance that is
removed while materializing an authoritative artifact. Decision 0101 reused the
same token to mean that an adapter was still an inactive scaffold. Those are
different contracts: template instructions are individual authoring directives,
while an adapter scaffold marker classifies the state of the whole document.

The implementation made the overlap broader by checking for the token as a raw
substring. A code example, ordinary sentence, or unrelated HTML comment could
therefore make an otherwise active adapter appear inactive. Decision 0131 then
needed a selector-specific compatibility exception for the deferred adapter.

SpecBind has no external user base whose installed adapters require migration
compatibility during this stabilization phase. Carrying the old interpretation
forward would preserve accidental complexity rather than a product contract.

## Decision

An inactive adapter scaffold contains the exact complete Markdown HTML comment:

```markdown
<!-- specbind:adapter-scaffold -->
```

The adapter catalog recognizes the marker from the Markdown syntax tree. Text in
a code fence, inline code, ordinary prose, a longer marker-like comment, or any
other occurrence is ordinary adapter content and does not affect state.

Adapter state is selector-independent:

- `absent` when the project file does not exist;
- `scaffold` when its body has no content or contains the exact scaffold marker;
- `active` otherwise.

`specbind:instruction` retains only its managed-Markdown meaning under Decisions
0059 and 0139. It has no special meaning in an adapter, and no legacy adapter
interpretation is retained.
Active embedded defaults such as Git and deferred carry no scaffold marker. The
inactive embedded release adapter carries the dedicated marker until the project
replaces the scaffold with its own procedure.

The marker classifies state; it does not grant authority, define adapter
semantics, or become executable syntax. Consuming skills treat a marked adapter
the same as no project-specific guidance according to that selector's existing
absence contract.

## Consequences

- Managed-artifact instruction validation remains independent and can speak
  precisely about `specbind:instruction` as a scoped Markdown directive.
- Adapter state no longer depends on a substring search or a deferred-specific
  exception.
- Existing development fixtures or project-owned copies using the old marker
  must be replaced or edited; the installer does not rewrite project-owned
  settings.
- Future adapter defaults can be active or inactive without overloading template
  syntax.

## Implementation status

Implemented. The Rust adapter catalog recognizes the exact Markdown marker, the
release assets and consuming skills use it, and focused tests prove that
marker-like prose, code fences, longer comments, and `specbind:instruction` do
not change adapter state.
