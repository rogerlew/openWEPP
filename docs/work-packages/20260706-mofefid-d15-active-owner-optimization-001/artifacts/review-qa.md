# Review — QA (governance/QA lane)

Status: **EXECUTED**.

Reviewer: delegated in-session QA/governance review subagent (general-purpose,
this session; charter = `docs/work-packages/AGENTS.md` rules 1-8). Evidence
class: **Static** (full package + contract + working-tree diff + production
source read; read-only commands only; no gate executed by the reviewer).
Findings below are recorded verbatim in substance; disposition in
`review-disposition.md`.

Verdict: **GO-WITH-AMENDMENTS**. "The implementation itself is well-built and
the consumer-path/byte-identity/behavioral evidence is real; nothing found
indicates wrong physics or scope breach", but complete disposition was blocked
on the findings below at review time.

## Findings

### HIGH

- **QA-H1 — Completion claimed in authority documents while required gates
  were unresolved (snapshot).** At review snapshot: `gate-results.md` carried
  two IN-FLIGHT rows (workspace full suite; the ignored H2637 pair with an
  observed first-run FAIL), review/verification/disposition artifacts were
  QUEUED, yet `artifacts/README.md`, `docs/work-packages/README.md`,
  `docs/ROADMAP.md` row M, the campaign D15 row, and `worker-handoff.md`
  already stated EXECUTED/complete — violating Gate Evidence Non-Deferral and
  verb-evidence truthfulness. `worker-handoff.md` also said reviews "were"
  delegated while none existed in the artifact set (no-pre-filled-evidence
  rule).
- **QA-H2 — The rev-27 closure hard-fails are structurally self-closing;
  the conservation-acceptance leg does not meet the package's own bar.**
  (a) The per-lane R4B `closure_residual_m` is identically zero by
  construction (`storage.rs:808-838` re-evaluates its own definition), so
  `lane_net_m3` is guaranteed ≈ 0. (b) The day cascade residual books all
  four operands from the solver's own mass ledger — producer
  self-consistency + handoff telescoping (tautological w.r.t. the
  soil↔router seam), especially after the mesh-basis fix re-based
  `injected_m3` onto `rainfall_excess_m2 × width`. (c) The "assembled
  hillslope-day identity" therefore reduces to (b) + fp noise; **no runtime
  check ties the soil-released volume (`q_runoff × area`) to the router's
  injected mass** — a wrong `basis_factor`, dropped source hours, or
  seam-rate integration error would pass all three tolerances. The check
  class that caught the day-one mesh-basis defect no longer exists in code;
  `consumer-path-proof.md` Consumer 3's liveness narrative describes the
  removed formulation. The package's Conservation/Output Acceptance text
  ("independent reconstruction from produced outputs") is not met on the
  routed conservation surface.

### MEDIUM

- **QA-M1** — The mesh-basis conversion (`area/(slplen×width)`) has NO
  contract authority: rev 27 does not record it, while code comments and
  `active-owner-implementation.md` claim "recorded in the rev-27 rows" —
  the QOFE/Q area-duality class living only in code/artifacts.
- **QA-M2** — "latqcc in closure" is contract/artifact text, not code:
  `latqcc_outlet_m3` never enters an enforced residual (manifest total
  only), and the `operand-lineage.md` day-identity formula's explicit
  `− latqcc_outlet` term would DOUBLE-COUNT (lateral export already rides
  inside `subsurface_loss_m`, `subsurface.rs:617`). Formula, contract status
  text, and implementation disagree.
- **QA-M3** — Active mode consumes the D12 uniform-fallback shape as REAL
  production water on 3 H2637 days, while the unamended D12 branch row still
  binds "uniform fallback … diagnostic residual class … cannot carry
  activation evidence". Rev 27 should authorize (counted) or fail closed.
- **QA-M4** — The erosion water-magnitude follow-on is mis-attributed as a
  "named rev-27" gate in `worker-handoff.md`/ROADMAP — the contract does not
  record it (only the HBP outlet re-pointing and the inter-day carry).
- **QA-M5** — Consumer-path rubric coverage incomplete per named elements
  (runner-handoff / state-object / output-surface / negative-proof rows
  missing on Consumers 2-5, though substance is largely inferable).

### LOW

- **QA-L1** — Line-count governance numbers stale/wrong
  (`kinematic_wave.rs` 1,639 not 1,668; `03_executor.rs` 1,214 not 1,391;
  `laned_active.rs` 777 not ~700) and "all others < 1,300" false:
  `00_builders_and_authority.rs` is 2,732 (pre-existing WARN band, touched
  +~38 lines, no WARN disposition recorded).
- **QA-L2** — Manifest `total_source_m3` accumulates solver-booked injection
  while `operand-lineage.md` describes `Σ depths × A_i` (they agree only via
  the unrecorded basis conversion).
- **QA-L3** — Nits: only run 1 of 3 active-endpoint timing logs retained;
  window-rule wording "(last active source hour) + 6 h" vs implemented
  `(h+1)·3600 + 6 h`; handoff quotes the heavy gate with `--no-fail-fast`
  vs package's `--no-capture`.

## Charter checks (reviewer's table)

Gate non-deferral **FAIL** (H1/H2 at snapshot) · consumer-path
**PASS-WITH-GAPS** (M5) · conservation acceptance **FAIL** (H2/M2/L2) ·
truthfulness **FAIL on named items** (H1/M1/M2/M4; Ran/Static labeling and
the timing-adjudication framing themselves PASS) · contract consistency
**FAIL** (M1/M2/M3) · scope/exclusions **PASS** · follow-on gates
**PASS-WITH-DEFECT** (M4) · line-count **FAIL on accuracy** (L1).

## What the reviewer verified as sound (not just present)

Selector/mutual-exclusion/fail-closed chain; tolerance constants ↔ contract;
full-mesh-hold degeneracy contract-first and in-authority; no
double-feed/partial flip (surface transfer zeroed, INV-009 guard on both
branches, lateral untouched, `sbrunv` preserved); OPT-5..9 bit-identity
reasoning traced diff-by-diff with Ran witnesses; `route_single_ofe`
extraction faithful; timing adjudication honestly framed and
operator-flagged; Ran/Static labeling systematic; HBP-outlet and inter-day
carry follow-ons genuinely recorded.
