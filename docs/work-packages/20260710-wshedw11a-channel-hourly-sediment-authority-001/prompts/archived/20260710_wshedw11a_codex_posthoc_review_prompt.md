# WSHED-W11A Codex Post-Hoc Review Prompt

This is local repository engineering work in `/workdir/openWEPP`, starting
from `main` at the commit that completed
`20260710-wshedw11a-channel-hourly-sediment-authority-001`. Read-mostly
review; write access limited to a new review artifact in this package's
`artifacts/` directory plus, if findings are accepted by the operator, a
follow-on fix commit on `main`.

## Context

WSHED-W11A was executed end-to-end by Claude Code under operator direction
(2026-07-10), including the `SC-ROUTE-001` v50 → v51 amendment (channel-
interval sediment sequencing authority, `INV-ROUTE-015..020`), dual
subagent review (21 findings dispositioned), and dual subagent
verification (both PASS-WITH-NOTES). Per program convention, a
Claude-executed contract cycle receives Codex post-hoc review before the
authority is treated as settled for W11 implementation kickoff.

## Your review scope

1. **The v51 amendment itself** (`git show` the completing commit for
   `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`):
   scientific-authority alignment, invariant soundness and falsifiability,
   guard/tolerance completeness, BEI conservation, and internal consistency
   of the lane-activation logic (`INV-ROUTE-015` biconditional +
   `INV-ROUTE-005(a)` dependency-authority definition + the W11A addendum
   Activation section).
2. **The review/verification record quality**: read
   `artifacts/review_agent_a.md`, `review_agent_b.md`,
   `review-disposition.md`, `verification_agent_a.md`,
   `verification_agent_b.md`. Adjudicate whether the dual reviews were
   genuinely independent and adequately critical, whether the A-8 rejection
   holds, and whether any finding was under-fixed.
3. **The authority basis**: spot-check the load-bearing citations against
   the vendored sources (`references/50201000/chap13.pdf`,
   `references/vendorable/creams/312-ch3.md` + scan,
   `references/vendorable/kineros/703.md`,
   `references/vendorable/HEC_RAS_1D_Sediment_Transport_UserManual_20260710.pdf`)
   and the pinned baseline (`/workdir/wepp-forest_260430_baseline`,
   `dac3c950`), especially the INV-ROUTE-018 lineage-realization
   adjudication (linear rate / `1.0176` exponential / fitted `f(x_b)` /
   `timpot`-`timex` partition vs `dcap.for` and the migrated Rust lane
   `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`).
4. **W11 implementability**: walk `artifacts/w11-handoff.md` and the ten
   contract-derived vectors as if scoping the W11 implementation package —
   flag any remaining executor science choice.

## Output

Write `artifacts/codex_posthoc_review.md` with an evidence header
(`Static`/`Ran` labels), severity-ranked findings with file/line
references, and a final recommendation: `RATIFIED` /
`RATIFIED-WITH-AMENDMENTS` (list them) / `REOPEN` (name the blocking
defect). Do not edit the contract in the same pass as the review; propose
amendments as findings for operator disposition.

Autonomy: execute the review end-to-end without asking for another prompt.
Subagent use is authorized for bounded citation spot-checks.
