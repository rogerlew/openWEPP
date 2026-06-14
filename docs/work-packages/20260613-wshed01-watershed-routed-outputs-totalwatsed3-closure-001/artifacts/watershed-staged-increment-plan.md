# Watershed Staged Increment Plan — Dispatch Artifact

Status: active - W-D executed-hold; W-D-REDO queued
Author: Claude Code, 2026-06-13
Template: FDHP01 `d3-staged-increment-plan.md` / MOFE01
`mofe-staged-increment-plan.md` (proven; agent memory
`staged-increment-port-template`).

## Universal rules (every increment)

- **Conservation is acceptance**: the totalwatsed3 identity + watershed-level
  water balance must close at the established floor on routed output. Identity
  checks use **independent operands** — no 0==0 / self-built / alias closure
  (the M-E4-REDO/M-I lesson, hard-won this series). A residual that is exactly
  0.0 is the tautology smell; genuine closure is nonzero-at-noise.
- **Hillslope inputs are settled**: MOFE01 HBP shards are read-only inputs;
  no per-OFE physics re-opened. Single-OFE/MOFE anchors stay unchanged by any
  watershed-side increment.
- **Comparator posture**: legacy is a flag (ADR-0017). Acceptance is
  conservation, not legacy-match.
- Contract-first per increment; red tests before production edits; commit each
  increment `executed-hold` until acceptance; truthful evidence labels;
  `Static:`/`Ran:`.
- Subagent requirement per package.md §4a (comparator_suite_runner REQUIRED
  for heavy runs).
- Cross-repo boundary: no wepppy production edits without explicit scope; name
  cross-repo needs as follow-ons.
- Read the lines, don't infer from symbol tables (the Dh lesson).

## Increment W-A — characterization + watershed routing scope (no production edits)

Ground everything in measured reality before any code (the M-A lesson):

1. **Watershed CLI current behavior**: run `openwepp-cli-watershed` on
   arboreal-dendrite with the closed MOFE01 hillslope HBP shards. Record where
   it fails/succeeds, what it routes (channel network from `chan.inp`), what
   watershed output it produces (or doesn't), and the full failure chain.
2. **The `jpond=0` impoundment finding**: read
   `watershed_impoundment.rs` — does the parser reject `jpond=0` (no
   impoundments) as `IMP-E-004`, and is that a **parser defect** (zero ponds
   is valid; should yield an empty impoundment set) or genuinely invalid
   input? Confirm against legacy impoundment-file handling. Cite lines.
   Record in `impoundment-no-pond-finding.md`.
3. **Routing + output seams**: map the channel-routing path
   (`openwepp-watershed-orchestrator`), the watershed-output schema
   (`openwepp-watershed-output`, the interchange contract), and what
   **totalwatsed3** (`wepppy/wepp/interchange/totalwatsed3.py`) expects as
   input — the exact columns/schema the audit consumes. Read the lines.
4. **Scope artifact** (`watershed-routing-scope.md`): legacy watershed-routing
   authority map (channel routing, pass-file consumption, watershed WB
   aggregation), openWEPP seam mapping, the totalwatsed3 input contract, the
   conservation identity (watershed-total = Σ hillslope contributions routed,
   closing against external inputs), red-test definitions, and the
   implementation increment breakdown + sizing.

- Gates: no production edits; evidence + scope artifact with current-tree
  file:line citations; the `jpond=0` finding classified (defect vs invalid)
  with evidence; the totalwatsed3 input contract documented.

W-A execution result (2026-06-13):

- Ran current watershed CLI on arboreal-dendrite with closed M-I HBP shards.
  It failed before `chan.inp`, HBP parsing, dispatch, or output writing:
  `CLIWAT-E-010` wrapping `IMP-E-004` on `pw0.imp` line 2, `jpond=0`;
  output file count was `0`.
- Classified `jpond=0` as a parser defect on valid no-impoundment input.
- Mapped the routing, output writer, and totalwatsed3 audit seams.
- W-B is the next implementation increment.

W-B execution result (2026-06-14):

- Amended the watershed runfile contract to pin explicit no-impoundment
  semantics: `inputs.pw0_imp` remains required in schema v1, but supported
  `.imp` files with `jpond=0` are valid typed empty impoundment sets when
  `pw0_str` declares zero impoundments.
- Added red/green parser and watershed CLI tests for explicit `jpond=0`.
- Implemented count reconciliation before the bare `jpond >= 1` guard, so
  `expected_structural_count=Some(0)` admits an empty item set, positive
  structural counts fail as `IMP-E-007`, and bare parses still fail as
  `IMP-E-004`.
- Re-ran the arboreal-dendrite watershed CLI. It now proceeds past
  `CLIWAT-E-010` and reaches the next hard stop:
  `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`, with zero output files.
- W-C is the next implementation increment.

W-C execution result (2026-06-14):

- Classified `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003` as valid routing
  input rejected by over-strict guards: a complete zero-sediment hillslope HBP
  payload with zero particle fractions, plus a hidden `nchnum=0`
  output-disabled channel state.
- Amended `SC-ROUTE-001` to pin zero-sediment contributor semantics and
  `nchnum=0` output-disabled semantics.
- Implemented the WS10 guard corrections and WAT-backed multi-row watershed
  publication.
- Re-ran arboreal-dendrite configured and legacy-discovery CLI paths; both
  exited `0`, emitted all `14` watershed parquet outputs, and produced
  `2192` `totalwatsed3.parquet` rows with non-placeholder WAT fields.
- W-D is the next implementation increment for totalwatsed3 closure.

W-D execution result (2026-06-14):

- Ran the wepppy totalwatsed3 audit against fresh configured and
  legacy-discovery arboreal-dendrite watershed outputs.
- Corrected keepable openWEPP publication defects: exact totalwatsed3
  hydrology columns now emit `m^3` volumes while depth aliases remain mm;
  MOFE `latqcc` aggregates only outlet OFEs; optional WAT profile and
  interception fields now publish into `totalwatsed3`.
- The producer no longer trips profile-cap false violations
  (`profile_violations_days=...:0`) and publishes total interception
  `551.502748 mm`.
- W-D remains held because the independent closure gate still fails:
  `closure_reconstructed_with_storage_total_mm=2950.498418`,
  `17.772166%` of precipitation. The remaining localized blocker is missing
  independent daily PASS `runvol` lineage; the current producer still fills
  `runvol` from WAT `Q`, which makes runoff consistency a self-consistency
  check rather than conservation proof.
- W-D-REDO is the next implementation increment.

## Subsequent increments (refined by W-A; provisional)

- **W-B — impoundment no-pond handling**: executed-hold. Gate met:
  arboreal-dendrite watershed CLI proceeds past the impoundment parse, and
  existing impoundment fixtures are non-regressed.
**W-B execution result (2026-06-14, `ea95d372`) — accepted (Claude):** `jpond=0` accepted only for `expected_structural_count==Some(0)`; the structure-vs-file mismatch is preserved as a distinct typed error `IMP-E-007` (CountMismatch), `IMP-E-004` retained without structure context — guard preserved, not loosened. The absent-`.imp` case got an explicit decision (pw0_imp still required in schema v1, pinned in the runfile contract) — stricter than legacy (which never reads it) but deliberate and pinned. Red-first. The arboreal-dendrite CLI now clears `CLIWAT-E-010`.

**W-C now starts at the next hard stop:** `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003` (a channel-node *DomainViolation* in the WS10 watershed kernel; `kernel/types.rs:154`), 0 output files. W-C's first task is to characterize this exactly as W-A/jpond did: is it (a) valid channel input rejected (guard too strict — confirm legacy runs the same channel state), (b) a genuine domain violation (real bad routed channel state), or (c) missing routing data the channel kernel needs from the hillslope shards? Read the lines + compare to legacy `chnrt`/channel routing; do not assume defect.

- **W-C — watershed routing + output**: executed-hold. Gate met for routed
  publication: the CLI routes the hillslope shards over the channel network,
  emits all `14` interchange parquet outputs, publishes multi-row WAT-backed
  `totalwatsed3.parquet`, and rejects placeholder/default-zero publication.
- **W-D — totalwatsed3 end-to-end audit + closure**: executed-hold. Gate not
  met: totalwatsed3 profile/interception/unit defects were corrected, but the
  independent closure audit still reports `2950.498418 mm` whole-run residual.
- **W-D-REDO — PASS runvol lineage + closure**: expose or reconstruct
  canonical daily PASS runoff volume from HBP/PASS publication authority,
  bind it into `totalwatsed3.runvol`/`Runoff`, and rerun the configured and
  legacy-discovery audits. Acceptance remains nonzero-at-noise independent
  closure; exact WAT `runvol == Q * Area / 1000` self-consistency is not
  sufficient.

(W-A sizing may split W-C/W-D further; each increment behind a conservation
hard stop.)

## Dispatch instructions

Each Codex dispatch: *"Execute increment <W-A|W-B|W-C|W-D|W-D-REDO> of
`docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md`
end-to-end."* Required reading order: this plan; `package.md`;
`watershed-routing-scope.md` (once it exists); the MOFE01 + FDHP01 staged
plans (the failure modes these rules encode — tautology, clone, hollow
closure). An increment that cannot meet its gates backs out with evidence,
localized to its seam.
