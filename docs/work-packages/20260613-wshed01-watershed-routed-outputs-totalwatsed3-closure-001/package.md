# WSHED01 — Watershed Routed Outputs / totalwatsed3 Closure

Status: scaffolded

Package type: staged implementation/closure package (FDHP01/MOFE01 execution
shape: characterize-then-staged-increments, conservation as acceptance)

## Objective

Produce **watershed-level routed outputs** from the closed MOFE01 hillslope
per-OFE pass shards, and close the **end-to-end totalwatsed3 water-balance
audit** on that routed output — the acceptance surface deferred since
WBVAL06/6a. ROADMAP queue item 1.

The architecture (per ADR/contracts): `openwepp-cli-watershed`
(`crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`) orchestrates the
hillslope HBP shards, routes over the channel network, and writes watershed
parquet via the wepppy/wepppyo3 interchange schema. totalwatsed3 (wepppy
Python; its Interception plumbing landed in WBVAL06/6a) is the **acceptance
audit** that consumes the watershed output and closes
`P − (Runoff + Lateral + ET + Percolation + Interception) − ΔStorage`.

## Substrate

`/wc1/runs/ar/arboreal-dendrite/wepp` — the MOFE01 substrate, now with closed
hillslope per-OFE routing. The 36-run hillslope HBP shards are the watershed
input; `chan.inp` is the channel network; `pw0` is the watershed-representative
profile (15-OFE) used by the watershed routing, not a hillslope run.

## Known immediate blocker (M-H finding, to confirm in W-A)

`openwepp-cli-watershed` fails closed before output writing on the
arboreal-dendrite **no-impoundment** state: `pw0.imp` declares `jpond=0`
(datver `99.1`, second line `0`), and the impoundment parser
(`crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`,
`IMP-E-004` DomainError) is wrapped by the CLI as `CLIWAT-E-010`
(`openwepp-cli-watershed.rs:251`). A watershed with zero impoundments is a
normal, valid state — so the lead hypothesis is a **parser defect** (reject
`jpond=0` instead of accepting an empty impoundment set), to be confirmed
against the parser code + legacy behavior in W-A. This is the *first* blocker,
not necessarily the only one between here and totalwatsed3 closure.

## Cross-repo note

This rung spans **openWEPP** (watershed CLI produces routed output) and
**wepppy** (totalwatsed3 audits it). The openWEPP side is the implementation;
the totalwatsed3 audit is the acceptance surface and may run in the wepppy
`.venv`. Keep the openWEPP package authoritative for the watershed-output
implementation; record the totalwatsed3 audit as cross-repo validation
evidence. Do not author wepppy changes from this package without explicit
scope — if totalwatsed3 needs a wepppy change to consume openWEPP watershed
output, name it as a cross-repo follow-on.

## Comparator posture

Per ADR-0017 and the MOFE01 calibration: legacy is a flag, not a target.
Acceptance is **conservation closure** — the totalwatsed3 identity closing at
the established noise/expected floor on routed output, and the watershed-level
water balance conserving against the (already closed) hillslope inputs.
totalwatsed3 magnitude vs legacy is the flag; the ±10–25% per-OFE magnitude
divergence (`MOFE-MAGPARITY01`) carries forward as expected divergence.

## Execution shape (staged, characterize-first)

Per the proven template (`mofe-staged-increment-plan.md`, agent memory
`staged-increment-port-template`): the staged plan
(`artifacts/watershed-staged-increment-plan.md`, authored at first dispatch)
governs increments. The first increment is **characterization** (W-A): read
the watershed CLI + impoundment parser + channel-routing + watershed-output
seams and what totalwatsed3 expects, confirm the `jpond=0` blocker's nature,
and produce the routing scope before any production edit (read the lines —
the Dh lesson). Implementation increments follow behind conservation hard
stops; refuted hypotheses get contract pins; identity checks must use
independent operands (the M-E4-REDO/M-I lesson — no 0==0 or self-built
closure).

## Included scope (refined by W-A)

- W-A characterization: watershed CLI current behavior on arboreal-dendrite;
  the `jpond=0` impoundment finding (parser defect vs invalid input);
  channel-routing + watershed-output state; totalwatsed3 input expectations;
  scope artifact.
- Impoundment no-pond handling (if a parser defect): accept `jpond=0` as an
  empty, valid impoundment set with a typed contract, not a silent default.
- Watershed routing over the hillslope HBP shards + channel network to a
  watershed-level routed output (parquet via the interchange schema).
- totalwatsed3 end-to-end audit on the routed output; closure at the
  established floor.

## Excluded scope / protected boundaries

- No hillslope per-OFE physics changes — MOFE01 closure is settled; the
  hillslope HBP shards are inputs, not to be re-opened.
- No comparator-match tuning (ADR-0017).
- No wepppy production changes without an explicit cross-repo scope/follow-on.
- Sediment/erosion routing at the watershed level is out of scope (the
  `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF` follow-on territory) unless W-A
  shows the water-output seam owns it inseparably.

## Acceptance / exit criteria

- `openwepp-cli-watershed` runs end-to-end on arboreal-dendrite (no-impoundment
  state handled with a typed contract), producing watershed routed output.
- The totalwatsed3 identity closes on that output at the established floor;
  watershed water balance conserves against the closed hillslope inputs.
- Contract-derived red/green tests; conservation identities on independent
  operands; truthful evidence labels.
- On closure: ROADMAP item 1 removed, README execution-log updated, handoff
  names the next mechanism and any cross-repo / sediment follow-ons.

## Subagent Requirement

Subagent requirement: REQUIRED, not optional. This package explicitly
authorizes subagent spawning/delegation to `comparator_suite_runner`
(gpt-5.3-codex-spark) for all heavy batch/closure/comparator runs
(`cargo test --workspace`, clippy/deny loops, watershed cohort runs,
totalwatsed3 audit batches). Do NOT run heavy batch work on the parent model
unless the subagent is unavailable; record command-level evidence if so
(`docs/standards/prompt-wording-guidance.md` §4a). Review/verification
subagents authorized for the dual review/verification artifacts.

## Security-Impact Gate

The watershed CLI parses watershed-structure input files (`chan.inp`,
`*.imp`, channel/impoundment definitions) and invokes hillslope-shard
consumption. Input validation at these file boundaries must be typed and
bounds-checked (the impoundment-parser fix must validate, not silently
default). Subprocess/argument construction for any hillslope orchestration
uses explicit arg arrays, no shell interpolation. No network egress. If
execution discovers a new parser/subprocess surface beyond the watershed
structure files, stop and record the scope change.

## Dependencies

- MOFE01 (`20260612-mofe01-inter-ofe-routing-closure-001/`) — the closed
  hillslope per-OFE routing producing the HBP shards.
- `AGENTS.md`, `docs/codex_exec_plans.md`, `docs/defect_closure_execplans.md`,
  ADR-0011/0017/0018, `docs/standards/kernel-work-package-preparation.md`,
  `docs/standards/prompt-wording-guidance.md` (§4a).
- Contracts: watershed/channel-routing + impoundment contracts (W-A to
  identify), `SC-WATBAL-001`, interchange-schema contracts.
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`,
  `crates/openwepp-watershed-orchestrator/`, `crates/openwepp-watershed-output/`,
  `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`.
- wepppy totalwatsed3: `wepppy/wepp/interchange/totalwatsed3.py`,
  `tools/totalwatsed3_daily_closure_audit.py` (WBVAL06/6a lineage).
- Substrate `/wc1/runs/ar/arboreal-dendrite/wepp/` (HBP shards + chan.inp + pw0).

## Autonomy

Each dispatched increment executes end-to-end per the staged plan without
asking for direction on intermediate steps; hard stops are the conservation
gates, the protected boundaries, and the cross-repo boundary (no wepppy
production edits without scope). Operator decisions (cross-repo changes,
boundary declarations) route back per the staged plan.
