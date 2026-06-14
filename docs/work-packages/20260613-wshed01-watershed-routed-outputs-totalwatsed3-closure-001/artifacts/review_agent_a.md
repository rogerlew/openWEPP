# Review Agent A

Status: T-B2-REDO2 post-review complete

Evidence mode: Static + Ran

## Findings

No blocking W-A findings.

Observations:

1. The W-A current-behavior gate is supported by command evidence: the CLI
   fails at `CLIWAT-E-010`/`IMP-E-004` before output writing.
2. The no-pond classification is evidence-backed: legacy skips impoundment
   initialization/output when `npond=0`, while openWEPP rejects `jpond=0`
   before structural reconciliation.
3. The scope artifact correctly warns that file emission is insufficient for
   W-C because `writers.rs` can default unmapped water-balance fields to zero.

## Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | accepted | W-A gates met; package remains active for W-B. |

## W-D Review

Evidence mode: Static + Ran

Blocking finding:

1. W-D still lacks independent daily PASS `runvol` lineage. The producer
   fills `runvol` from WAT `Q`, so the wepppy audit's runoff consistency check
   compares two values from the same source. The real closure gate remains
   failed with `closure_reconstructed_with_storage_total_mm=2950.498418`.

Non-blocking findings addressed during W-D:

1. The writer test initially covered only part of the exact volume surface.
   It now asserts all short exact hydrology fields that publish as `m^3`.
2. The outlet-only `latqcc` test initially used too narrow a fixture. It now
   exercises multiple `wepp_id` groups with unequal areas and non-outlet OFE
   lateral flow.

Residual risk:

- Nullable selector columns in source WAT shards are still rejected rather than
  normalized. Current openWEPP WAT publication emits non-null selector columns,
  so this is not a W-D acceptance blocker.

## W-D Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| 1 | Missing independent PASS `runvol` lineage | accepted / blocking | W-D status is `executed-hold`; T-B now owns HBP/PASS daily runoff lineage through the dedicated CLI. |
| 2 | Writer test incomplete | fixed | Expanded writer assertions cover all exact volume fields and depth aliases. |
| 3 | Outlet-only lateral fixture too narrow | fixed | Aggregator test now proves outlet-only `latqcc` over multiple contributors. |
| 4 | Nullable selector handling | deferred | Not reached by current producer output; keep as follow-on hardening risk. |

## T-A Review

Evidence mode: Static

Findings:

No blocking T-A findings.

Review observations:

1. `totalwatsed3-cli-scope.md` correctly treats wepppy as semantic reference,
   not a code dependency.
2. The scope removes the channel-loss/storage tangent from totalwatsed3 and
   keeps `WATERSHED-CHANWB-ROUTED-OUTPUT` as a decoupled follow-on.
3. The scope explicitly rejects the W-D tautology: PASS `runvol` is the
   `Runoff` operand, while WAT `Q` remains diagnostic.
4. The current openWEPP gap is localized before implementation: HBP event
   volume slots are zero/not exposed, so T-B must add a real PASS lineage
   surface before claiming closure.

## T-A Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | accepted | T-A gates are design/scope gates and are met. |

## T-B Review

Evidence mode: Static + Ran

Findings:

No blocking T-B findings.

Review observations:

1. The dedicated CLI exists and owns totalwatsed3 production, so the
   watershed CLI path is no longer the active aggregation surface.
2. `Runoff` is derived from PASS `runvol`, while WAT `Q` remains diagnostic;
   the focused fixture proves those can differ.
3. The MOFE `latqcc` collapse test exercises outlet-only behavior and avoids
   cross-OFE double counting.
4. The real arboreal-dendrite audit residual is nonzero
   (`57.409871 mm`), so T-B does not fall into the exact-zero tautology trap.

Residual risk:

- `writers.rs` remains above the 2000-line warning threshold. T-B kept the
  main aggregation logic in a dedicated runner module, but any future writer
  work should consider a split before more growth.
- The remaining closure residual is real T-C work, not a T-B producer-blocking
  defect.

## T-B Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | accepted | T-B producer, lineage, focused tests, real run, and audit-read gates are met. |

## T-B2 Review

Evidence mode: Static + Ran

Findings:

No blocking T-B2 findings.

Review observations:

1. Native PASS parquet publication is openWEPP-owned and opt-in through
   `outputs.pass_parquet`; the required HBP `pass` output remains unchanged.
2. The MOFE `runvol` source is the terminal outlet transfer output, not WAT
   `Q` and not a per-OFE sum.
3. The real-run anchor comparison proves T-B2 did not perturb existing HBP/WAT
   publication.
4. T-B2 does not claim totalwatsed3 conservation closure; T-C remains the
   closure increment.

Residual risk:

- `02_output_and_climate_helpers.rs` is below but near the 2000-line warning
  threshold. T-C should avoid unrelated growth there.

## T-B2 Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | superseded | This local review was later invalidated by the T-B2 runvol area defect and the T-B2-REDO crossed-pairing defect. T-B2-REDO2 is the current accepted review record. |

## T-B2-REDO Review

Evidence mode: Static + Ran

Findings:

No blocking T-B2-REDO findings.

Review observations:

1. T-B2's earlier `QOFE * publication area` formula is explicitly rejected;
   the corrected producer uses the published `Q * Area / 1000` volume dual.
2. The focused fixture separates `Q`, `QOFE`, and areas, so it would fail both
   the old T-B2 formula and the first attempted redo formula.
3. The real arboreal-dendrite HBP/WAT anchor comparison remains unchanged
   (`anchor_mismatches=0`), so the REDO did not perturb existing WAT/HBP
   output surfaces.
4. The water-year precipitation bound is independent of the deleted
   self-consistency check and passes for all `252` hillslope-water-years.

Residual risk:

- The corrected native totalwatsed3 audit still reports
  `closure_reconstructed_with_storage_total_mm=6948.564523`. That is a T-C
  conservation blocker, not a T-B2-REDO publication blocker.
- `02_output_and_climate_helpers.rs` remains below but close to the 2000-line
  warning threshold.

## T-B2-REDO Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | accepted | T-B2-REDO corrected the runvol area defect, preserved HBP/WAT anchors, passed focused and full Rust gates, and records the remaining residual for T-C. |

## T-B2-REDO2 Sidecar Review

Evidence mode: Static + Ran

Findings:

1. `hillslope_pass.runvol` unit metadata still described the old hillslope
   publication-area pairing.
2. The focused REDO2 regression rejected `Q * outlet Area`, but it did not
   distinguish outlet WAT row area from the publication-area argument.

Disposition:

- Fixed
  `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs` so
  `hillslope_pass.runvol` names outlet `QOFE` and outlet WAT row area.
- Tightened
  `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs` so the
  focused fixture uses distinct outlet WAT row area (`200 m2`) and
  publication-area argument (`300 m2`), and rejects
  `QOFE * publication Area`.

## T-B2-REDO2 Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| 1 | `runvol` metadata drift | fixed | Metadata now matches the REDO2 producer formula. |
| 2 | Fixture did not guard publication-area aliasing | fixed | The regression now rejects both `Q * outlet Area` and `QOFE * publication Area`. |
