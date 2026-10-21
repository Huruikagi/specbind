# 0231: Preserve sequential readiness through retained completion

Status: Accepted

## Decision

Decisions 0019 and 0146 define immediate ordering edges. Task readiness and
completion guards evaluate their full ordering closure: every preceding
executable Task, including earlier groups and siblings, must be completed.
`depends_on` remains additive. The effective-prerequisite projection lists all
these Tasks in plan order. A pending or blocked Task cannot be bypassed because
an intervening Task has a retained completed record.

Existing completed records remain evidence for their own unchanged obligations.
Revision and reopening do not cascade resets through proven work. A later
pending Task becomes actionable after all preceding work is complete; list,
show, milestone next-task projection and `tasks complete` share this rule.
This changes no persisted schema, plan fingerprint or approval evidence.

## Verification

Read-model and CLI regressions cover insertion/reopening before retained
completion, pending and blocked predecessors, groups, refusal without writes,
and resumed readiness. This resolves the mechanical FT-0074 scheduling gap.
