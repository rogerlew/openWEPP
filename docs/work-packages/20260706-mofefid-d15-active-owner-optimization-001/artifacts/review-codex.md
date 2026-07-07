# Review — Code correctness lane

Status: **EXECUTED**.

Reviewer: delegated in-session code-correctness review subagent
(general-purpose, this session; rust_code_reviewer charter). Evidence class:
**Static** (full diff + new modules read; nothing executed by the reviewer —
all Ran figures were checked for mechanism-consistency only). Recorded
verbatim in substance; disposition in `review-disposition.md`. NOTE: this
review ran against the pre-seam-fix tree; the QA-H2 seam repair (breakpoints +
soil-release cross-ledger check) landed after its snapshot and strengthens the
Q3 surface it examined.

Verdict: **GO-WITH-AMENDMENTS** — "no correctness defect in the OPT-5..9
bit-identity mechanisms, the two-phase active loop's cross-lane ordering, the
closure-books algebra, the weights producer/consumer seam, or the DC01-disable
containment"; amendments are test-hygiene and evidence-label items.

## Verified sound (deliberately traced, per charter question)

- **Q1 OPT-5..9 bit-identity**: each mechanism traced including zero paths
  (`(0.0, 0.0)` vs `0.0×h_pow=+0.0` on all three zero branches, incl.
  `0 < h <= DRY_DEPTH_M`), negative-zero impossibility, NaN propagation
  parity, counter-semantics preservation, TV reorder observability (error
  path leaves no surviving state), material-breaks immutability.
- **Q2 two-phase loop**: lateral-transfer edge identical to the default
  loop; `upstream_erosion_downstream_operands` staleness has NO consumer;
  day-input builder reads own-lane state only with identical (day, lane)
  call sequence; intake refresh correctly sequenced; counter/row parity;
  chain topology validated up front and re-validated per edge.
- **Q3 closure books**: per-lane solver identity signs correct (clamp on
  input side); handoff telescoping exact (bin-series integral over tiling
  steps, width scaling cancels); basis conversion correct; `note_term` is
  scale-only; mesh end storage genuine (fresh dry solver per lane-day).
- **Q4 weights seam**: producer/consumer wet-gate is BIT-EXACT parity
  (`runoff_shadow_projection.q_runoff_m` pass-through to
  `peak.q_runoff_m`, same `WB11_ZERO_THRESHOLD`); every branch composes;
  no failure case found.
- **Q5 DC01 containment**: guard observable is surface-only (R4J resolves
  `runon_input_m` exclusively from surface transfer terms); zeroed carries
  leak into no validator; `dc01_distribute_runon_supply` short-circuits
  safely on the lateral-only path; the `_with_ownership(false)` wrapper
  preserves default-path statement order.
- **Q6/Q7**: no unwrap/expect/panicking index on new production paths; all
  fail-closed guards confirmed.

## Findings

- **CR-M1 (Medium)** — `tests/integration/laned_shadow_h2637.rs`: the env
  mutation pattern is sound only under nextest (process-per-test); under
  stock threaded `cargo test` the six env-mutating tests race (glibc
  `setenv`/`getenv` concurrency is UB — can abort with no panic message),
  and the SAFETY comments overstate ("before any runner threads") for the
  mid-test `set_var`.
- **CR-M2 (Medium)** — the active helpers never clear the SIBLING env var
  (and vice versa): an inherited `OPENWEPP_LANED_SHADOW=1` from the
  invoking shell silently turns the "active_off" baseline into a shadow
  run and aborts "active_on" on the mutual-exclusion guard.
- **CR-L1 (Low)** — `total_latqcc_outlet_m3` accumulates on ROUTED days
  only (early return skips zero-source days), while consumer-path-proof
  presents it as *the* bypass term without that qualifier — evidence-scope
  mislabel (no water lost; the lane pipeline still exports lateral flow).
- **CR-L2 (Low)** — day-closure hard-fail details carry no day coordinate
  (a day-600 abort would need instrumented reruns to localize).
- **CR-L3 (Low)** — OPT-5 zero-path equivalence assumes finite `h_pow`
  (`h ≳ 3e205 m` overflow gives old `NaN` vs new `0.0`); physically
  unreachable and correctly scoped in the plan artifact — recorded as the
  claim's input-class boundary.
- **CR-L4 (Low)** — the in-test INV-010 comment implies in-test byte
  identity while the test asserts only non-emptiness (the SHA comparison
  lives in the package logs).
- **INFO** — on routed days zero-source lanes run the full-window solve as
  pass-throughs (correct for the handoff; part of the cost profile);
  artifact claims spot-checked against code all held.

## One-time ignored-test failure assessment

Fixture-collision and nextest env-leak mechanisms are RULED OUT by code
(tag+pid-keyed temp dirs; process-per-test). Plausible causes in order:
(a) inherited shell env (CR-M2 — mechanically real; timing fit imperfect),
(b) a threaded-harness invocation (CR-M1 UB abort, matching the lost
message) if the first run was not under nextest, (c) transient host-level
I/O fault at output finalize (best timing fit; unfalsifiable). Not
candidates: nextest timeout, assertion nondeterminism (single-threaded
deterministic runtime). The package's unreproduced-monitor disposition is
reasonable; fixing CR-M2 and CR-L2 makes any recurrence self-diagnosing.
