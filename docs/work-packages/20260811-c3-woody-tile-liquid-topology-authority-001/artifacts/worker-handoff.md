# Worker Handoff

Status: `RELEASED / existing implementation package may resume`

Evidence mode: `Static`

After Stage-A terminal completion, resume the existing implementation package against
V2 definition digest
`38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`.
Implement one exact occupancy state lane, same-tile complete E04 routing,
occupancy-local coupled solves, typed occupancy water transactions, shared
weighted C/N, explicit migration, independent energy/BGC reconstruction, and
all-owner atomic rollback. Preserve V1 as nonexecuting historical evidence and
retain the temporary guard until the complete V2 path passes.

The authority, reviews, oracle, focused gates, heavy gates, and both terminal
verifiers pass. Freeze the Stage-A terminal commit and V2 digest before the
first Stage-B production edit.
