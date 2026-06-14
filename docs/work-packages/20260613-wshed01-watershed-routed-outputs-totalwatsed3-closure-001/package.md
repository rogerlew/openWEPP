# WSHED01 — Watershed Routed Outputs / totalwatsed3 Closure

Status: COMPLETE 2026-06-14 — T-C closed; totalwatsed3 native closure resolved
the WBVAL06/6a deferral (closes ex-day-1 −0.41 mm/2191 d, independent operands).
Follow-ons: MOFE-FARPOINT01 (next), WATERSHED-CHANWB-ROUTED-OUTPUT,
MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF.

Package type: staged implementation/closure package (FDHP01/MOFE01 execution
shape: characterize-then-staged-increments, conservation as acceptance)

## Objective

Produce the closed **totalwatsed3 water-balance audit** from the closed MOFE01
hillslope per-OFE pass/WAT shards. W-A through W-D also cleared and classified
watershed-CLI routed-output seams, but the operator-directed architecture
pivot makes totalwatsed3 a hillslope-only dedicated CLI, not a channel-routed
watershed output. This closes the acceptance surface deferred since WBVAL06/6a.
ROADMAP queue item 1.

Architecture (revised 2026-06-14 — see decision below): totalwatsed3 is an
**openWEPP-native CLI** (`openwepp-cli-totalwatsed3`) that consumes the
hillslope interchange outputs (`H.pass`/`H.wat`/`H.soil`/`H.element`),
area-weighted, hillslope-only, and closes
`P − (Runoff + Lateral + ET + Percolation + Interception) − ΔStorage` with
independent operands (Runoff from PASS `runvol`). It does NOT depend on the
watershed channel routing, and does NOT share the wepppyo3 `wepp_interchange`
crate (which stays wepp-legacy-only).

## Architecture decision (operator-directed 2026-06-14): totalwatsed3 is its own openWEPP-native CLI

totalwatsed3 is a **hillslope-only** water-balance aggregation (confirmed
against the authoritative producer
`/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py`:
area-weighted over hillslopes from `H.pass`/`H.wat`/`H.soil`/`H.element`;
MOFE-aware per-OFE collapse — `Runoff` from PASS `runvol`, latqcc
outlet-OFE-only, QOFE summed; **no channel routing, no channel loss/storage**).

**Decision:** build totalwatsed3 as its **own openWEPP-native CLI**
(`openwepp-cli-totalwatsed3`), NOT bolted into `openwepp-cli-watershed` (W-C)
and **NOT** by sharing the wepppyo3 `wepp_interchange` crate. Rationale
(operator): wepppyo3 `wepp_interchange` stays **wepp-legacy-only**; openWEPP
owns its full output surface end-to-end, with no obligation to carry the
legacy interchange converters or their constraints. This is a directional
output-surface boundary (candidate ADR).

Consequences:
- The totalwatsed3 closure (the WBVAL06/6a deferral — the real prize) is
  **decoupled from watershed channel routing**: it reads the MOFE01 hillslope
  interchange outputs directly and does not need impoundments or channel
  routing to close. (This is why the W-B impoundment + W-C channel blockers
  were never on the totalwatsed3 path — they were watershed-CLI concerns.)
- The W-A/W-B/W-C **watershed-CLI channel fixes remain valid landed work**
  (no-impoundment parse, zero-sediment channel acceptance) for the separate
  watershed-routed-output (`chanwb`/`chnwb`) deliverable, now a **decoupled
  follow-on** (`WATERSHED-CHANWB-ROUTED-OUTPUT`), lower priority than the
  totalwatsed3 closure.
- W-C's `build_watershed_daily_rows_from_wat` inside the watershed CLI and
  W-D's via-watershed-CLI totalwatsed3 are **superseded** by the dedicated
  CLI (T-arc below). The keepable unit/field fixes from W-D (m³ exact fields,
  depth aliases mm, latqcc outlet-only) carry into the new CLI.

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

Current status after T-B2-REDO2: the no-impoundment parser blocker and the
subsequent WS10 channel guard blocker are both cleared. W-D corrected keepable
totalwatsed3 publication defects, but the independent closure audit still
reports a `2950.498418 mm` whole-run residual. The operator-directed
architecture pivot supersedes the W-D-REDO watershed-CLI route: T-A scoped
`openwepp-cli-totalwatsed3` as a dedicated hillslope-only openWEPP-native CLI.
T-B implemented that dedicated CLI and reduced the real arboreal-dendrite
audit residual to `57.409871 mm` (`0.345805%` of precipitation). T-B2 added
openWEPP-owned per-hillslope runoff-delivery PASS parquet from outlet routed
MOFE runoff, but review found the first `runvol` formula over-scaled MOFE
runoff by using `QOFE` with a publication-area denominator. T-B2-REDO then
under-scaled runoff by crossing `Q` with the outlet area. T-B2-REDO2 corrected
native PASS `runvol` to `QOFE * outlet Area`, proved HBP/WAT anchors remain
byte-identical, and produced native totalwatsed3 from corrected
`H*.pass.parquet`/`H*.wat.parquet`. The wepppy audit now reports
`closure_reconstructed_with_storage_total_mm=30.544142`, with day 1 accounting
for `30.9533178099056 mm` and ex-day-1 basic-storage residual
`-0.409175395336963 mm` over `2191` days. Package closure remains active for
T-C final disposition.

## Cross-repo note

This rung spans **openWEPP** (dedicated native totalwatsed3 producer) and
**wepppy** (semantic reference and audit harness). The openWEPP side is the
implementation; the totalwatsed3 audit is the acceptance surface and may run
in the wepppy `.venv`. Keep the openWEPP package authoritative for the native
producer; record the wepppy audit as cross-repo validation evidence. Do not
author wepppy changes from this package without explicit scope. If the audit
needs a consumer adjustment for a valid openWEPP-native schema, name it as a
cross-repo follow-on.

## Comparator posture

Per ADR-0017 and the MOFE01 calibration: legacy is a flag, not a target.
Acceptance is **conservation closure** — the totalwatsed3 identity closing at
the established noise/expected floor on hillslope-only aggregated output with
independent PASS `runvol` runoff and WAT storage/flux operands.
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
  scope artifact. Executed 2026-06-13 in
  `artifacts/characterization-watershed-cli-current.md`,
  `artifacts/impoundment-no-pond-finding.md`, and
  `artifacts/watershed-routing-scope.md`.
- W-B impoundment no-pond handling: executed 2026-06-14. `jpond=0` is accepted
  as an empty, valid impoundment set only when watershed structure declares
  zero impoundments. The arboreal-dendrite CLI now proceeds past
  `CLIWAT-E-010` and reaches the next channel routing hard stop,
  `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`.
- W-C watershed routing/output publication: executed 2026-06-14. Zero-sediment
  HBP contributors and `nchnum=0` output-disabled channel state are accepted
  under `SC-ROUTE-001` version `45`. The CLI emits all `14` watershed parquet
  outputs and WAT-backed multi-row `totalwatsed3.parquet` on arboreal-dendrite.
- W-D totalwatsed3 audit and publication repairs: executed 2026-06-14.
  Volume/depth metadata, outlet-only MOFE `latqcc`, profile fields, and
  interception now publish into `totalwatsed3`; profile violations are zero.
  The conservation gate is still held on missing independent daily PASS
  `runvol` lineage.
- T-A dedicated totalwatsed3 CLI design and scope: executed 2026-06-14.
  `artifacts/totalwatsed3-cli-scope.md` pins the hillslope-only input
  contract, PASS `runvol` independent operand, MOFE per-OFE collapse,
  openWEPP-native schema, red tests, and T-B/T-C breakdown.
- T-B dedicated totalwatsed3 CLI implementation: executed 2026-06-14.
  `openwepp-cli-totalwatsed3` now reads hillslope interchange PASS/WAT
  parquets plus optional soil/element parquets, publishes a 2192-row
  `totalwatsed3.parquet` for arboreal-dendrite, uses PASS `runvol` for
  `Runoff`, leaves WAT `Q` diagnostic, and preserves MOFE outlet-only
  `latqcc`.
- T-B2 openWEPP-native runoff-delivery output: executed 2026-06-14; reviewed
  defective on 2026-06-14 for `runvol` area/normalization.
  `openwepp-cli-hill` can now emit optional `outputs.pass_parquet` runoff
  delivery files. MOFE rows publish `runvol` from outlet
  `current_transfer_output.qofe` over hillslope publication area, while HBP
  and WAT anchors remain byte-identical. `openwepp-cli-totalwatsed3` consumes
  sorted per-hillslope `H*.pass.parquet`/`H*.wat.parquet` files when combined
  files are absent.
- T-B2-REDO runoff-delivery area correction: executed 2026-06-14, then
  superseded by T-B2-REDO2. REDO rejected `QOFE * publication area` but crossed
  operands to `Q * outlet Area`, which under-scaled native PASS `runvol`.
- T-B2-REDO2 crossed-pairing correction: executed 2026-06-14. MOFE PASS
  `runvol` now uses `QOFE * outlet Area`, not the under-scaled
  `Q * outlet Area`. Corrected arboreal-dendrite PASS output matches WAT
  outlet `QOFE * Area / 1000` over `78912` rows with max diff `0.0 m3`;
  totalwatsed3 closure drops to the expected day-1 storage-init residual
  (`30.544142 mm` whole run, `-0.409175395336963 mm` excluding day 1).
- T-C final disposition at the established floor.

## Excluded scope / protected boundaries

- No hillslope per-OFE physics changes — MOFE01 closure is settled; the
  hillslope HBP shards are inputs, not to be re-opened.
- No comparator-match tuning (ADR-0017).
- No wepppy production changes without an explicit cross-repo scope/follow-on.
- Sediment/erosion routing at the watershed level is out of scope (the
  `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF` follow-on territory) unless W-A
  shows the water-output seam owns it inseparably.

## Acceptance / exit criteria

- `openwepp-cli-totalwatsed3` runs end-to-end on arboreal-dendrite hillslope
  interchange inputs.
- The totalwatsed3 identity closes on that output at the established floor
  using independent operands: PASS `runvol` for `Runoff`, WAT flux/storage
  terms for the remaining water-balance fields, and no channel terms.
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
- wepppy totalwatsed3: `wepppy/wepppy/wepp/interchange/totalwatsed3.py`,
  `tools/totalwatsed3_daily_closure_audit.py` (WBVAL06/6a lineage).
- Substrate `/wc1/runs/ar/arboreal-dendrite/wepp/` (HBP shards + chan.inp + pw0).

## Autonomy

Each dispatched increment executes end-to-end per the staged plan without
asking for direction on intermediate steps; hard stops are the conservation
gates, the protected boundaries, and the cross-repo boundary (no wepppy
production edits without scope). Operator decisions (cross-repo changes,
boundary declarations) route back per the staged plan.
