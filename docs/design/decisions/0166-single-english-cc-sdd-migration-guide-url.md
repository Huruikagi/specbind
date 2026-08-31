# 0166: Use one English cc-sdd migration guide URL for every CLI handoff

Status: Accepted

## Context

The cc-sdd migration planner currently chooses a Japanese, English, or
language-neutral guide URL from the detected language of legacy artifacts.
That artifact language is migration input: it determines the language of the
target SpecBind artifacts and can itself be mixed or unresolved. It does not
establish the language in which the person or coding agent consuming CLI
diagnostics prefers to read documentation.

SpecBind CLI diagnostics are English. The migration guide linked by those
diagnostics is also an agent playbook: users commonly pass the complete CLI
output and URL to a coding agent. Maintaining three CLI destinations adds URL
selection behavior, tests, and a language-neutral routing page without changing
the migration safety contract.

SpecBind has not yet made a stable release. The pre-release language-neutral
entry and language-specific CLI routing do not require compatibility aliases.
The Japanese guide remains useful as user-facing documentation discovered from
the Japanese documentation tree.

## Decision

Every `specbind migrate cc-sdd` diagnostic that requires guided work prints the
same canonical English guide URL:

`https://huruikagi.github.io/specbind/guide/en/migrate-from-cc-sdd/`

The CLI does not select documentation from the legacy or target artifact
language. Mixed, unsupported, or unresolved artifact language remains a
semantic migration finding and does not change the guide destination.

The English page is the canonical agent playbook for CLI handoff. It remains
self-contained and aligned with migration finding codes, stop conditions,
resolution acceptance, deterministic validation, and final cutover behavior.

The Japanese migration page remains in the Japanese user guide. It may link to
the English counterpart and may provide a Japanese prompt containing its own
URL, but it is not embedded in the CLI contract. The language-neutral routing
page is removed rather than redirected.

This decision supersedes Decision 0125's language-aware guide selection,
language-neutral fallback, and three stable CLI entry points. It also
supersedes Decision 0142 only where that decision assigns a Japanese canonical
URL to CLI output. Their migration safety and bilingual documentation
boundaries otherwise remain accepted.

## Consequences

- CLI guide routing no longer conflates artifact language with reader language.
- All semantic findings have one deterministic documentation destination.
- The language-neutral page and its navigation layer are unnecessary.
- Japanese users can still discover and use the Japanese migration guide from
  the Japanese documentation tree.
- Changes to the English guide URL, finding semantics, or agent playbook remain
  coordinated product-contract changes.

## Verification

Migration CLI tests cover both mixed-language and Japanese legacy fixtures and
require the same English guide URL. Decision validation and strict MkDocs
validation prove that the removed neutral page has no remaining indexed route.

## Implementation status

Implemented in the Rust migration diagnostic, focused CLI tests, public
navigation, and documentation indexes.
