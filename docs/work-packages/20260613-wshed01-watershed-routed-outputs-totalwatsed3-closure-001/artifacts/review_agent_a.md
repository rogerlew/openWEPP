# Review Agent A

Status: T-A local review complete

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
