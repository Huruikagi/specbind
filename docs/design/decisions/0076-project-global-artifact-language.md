# 0076: Use one project-global artifact language

Status: Accepted

## Context

The inherited cc-sdd model stores language per Spec. SpecBind v1 supports only English and Japanese, and mixed-language Specs would multiply templates, CLI-generated Markdown variants, review behavior, and migration cases without a current product need.

## Decision

- `specbind install --lang en|ja` stores one project language in `.specbind.json` as `language`.
- `spec.yaml` contains no language field. Decision 0044's `language` member is superseded by this decision.
- Changing the configured project language after managed artifacts exist is unsupported in v1. A future explicit translation migration may add that capability.
- Agent skill instructions, embedded product protocols, and shared authoring rules remain one English-authored product set, following cc-sdd. Skills read `.specbind.json.language` and author user-facing project artifacts and reports in that language.
- Scaffolds a project fills with its own content are localized instead, one per supported language: the [Decision 0059](./0059-okf-artifact-templates.md) artifact templates and the [Decision 0101](./0101-project-adapter-directory-and-git-workflow.md) adapter scaffolds. The dividing line is voice. A protocol or rule states the product's own judgment, so it speaks once in the product's language; a scaffold is an empty vessel for project content, so it opens in the language that content will be written in. Machine identity such as an OKF `type` literal stays English in every language.
- Natural-language headings and prose may be localized. Machine syntax remains fixed English, including YAML and JSON keys, enum values, IDs, fingerprints, exact OKF `type` values, the Design `_Requirements: ..._` marker, and canonical Contract structural headings.
- EARS describes logical requirement patterns rather than machine syntax. Trigger and obligation phrases are fully localized with the surrounding requirement instead of mixing fixed English phrases into Japanese prose.
- CLI terminal results, diagnostics, help, headings, and stable codes remain English-only.
- CLI-authored text inserted into managed Markdown is localized to the project language. This includes a newly created `log.md` title and the visible release-entry wrapper. Existing document titles are preserved.
- The CLI does not attempt natural-language detection. Skills are responsible for writing in the configured language; `check` validates only deterministic language codes and machine syntax.
- User-authored release summaries are required by skill guidance to use the project language, but the CLI validates only non-emptiness, single-line safety, and Markdown structure.

## Consequences

- V1 maintains one skill and template source set while producing coherent English or Japanese project artifacts.
- Machine parsers do not branch on localized tokens except for explicit self-describing mappings such as Requirements heading labels.
- Manual edits that make artifact prose inconsistent with `.specbind.json.language` are review concerns, not unreliable CLI language errors.
