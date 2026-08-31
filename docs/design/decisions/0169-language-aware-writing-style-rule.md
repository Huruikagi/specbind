# 0169: Offer a language-aware shared writing-style Rule

Status: Accepted

## Context

The configured artifact language tells product Skills which language to use,
but that alone does not make prose natural. Japanese artifacts and user-facing
reports can still accumulate generic English words even when the exact
SpecBind concepts, commands, fields, states, and diagnostics are correctly
preserved.

Putting the same Japanese guidance in every product Skill would duplicate
project-adjustable prose policy across product-managed files. Template
instructions are also too narrow because the guidance applies across Specs,
Steering, reviews, orchestration reports, and release reports.

Decision 0093 deliberately gives cross-artifact authoring preferences to the
closed project-owned Rule catalog. It currently installs one English-authored
default set for both configured languages and has no cross-workflow prose Rule.

## Decision

Add the optional `language-style` selector at
`settings/rules/language-style.md` to the closed shared-Rule catalog.

The Rule owns project-adjustable natural-language preferences only. It may say
when ordinary English words should become natural Japanese, which established
SpecBind concepts remain useful in English, and which exact machine identifiers
must remain unchanged. It cannot change commands, artifact grammar, lifecycle,
approval, mutation authority, or any other product baseline.

Initial installation offers the embedded `language-style.md` default only when
the configured language is `ja`. An English installation still accepts and can
read the selector, but leaves it absent by default. Absence is the ordinary
successful `NO_CHANGE RULE_ABSENT` result and does not weaken the requirement
for each Skill to report in the configured project language.

The file is project-owned. Installation creates it only when missing and never
overwrites it. A later Japanese refresh may offer the newly introduced file as
an uncommitted addition. Changing the configured language does not rewrite or
delete an existing project-owned copy; the configuration workflow reports it
among retained language-sensitive content for explicit review.

Every product Skill reads `language-style --for consume` before authoring an
artifact or user-facing prose. This includes read-only judgments and thin
orchestrators because their reports are part of the product experience. The
Rule applies only to natural-language prose; exact commands, paths, fields,
state values, diagnostics, structured output, and quoted output stay exact.

## Consequences

- Japanese projects receive one central, customizable prose policy instead of
  duplicated Japanese wording guidance in every Skill.
- All product Skills use the same policy for artifacts and reports.
- English projects pay one successful absent-Rule read but receive no Japanese
  policy file by default.
- The accepted Rule catalog grows from six to seven selectors, while only six
  defaults remain language-neutral.
- Decision 0093's one-English-set-for-both-languages statement is superseded
  only for this language-aware default and its consumer set.

## Implementation status

Implemented by the embedded Japanese `language-style.md` Rule, language-aware
installation filtering, the closed Rule catalog and read surfaces, product
Skill reads, configuration reporting, focused tests, paired public guidance,
and a fresh Japanese forward test.
