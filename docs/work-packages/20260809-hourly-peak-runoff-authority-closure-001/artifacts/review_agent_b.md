# Independent Hydrology/Science Re-Review B

Status: `executed`

Evidence class: `Static: exact commit 949349e7055c5d19277eeb708401c4614a52cd77 contract, implementation, tests, package evidence, and HBP/pass consumers`

Verdict: `HOLD`

Reviewer independence: Reviewer A's output was not read before this verdict.

## Severity-Ranked Findings

### `SCI-B2-001` — CRITICAL — proportional frost-retention timing is proxy physics

The amended contract now authorizes a daily
`frost_retained_local_liquid_m` debit to be allocated proportionally over all
positive WB14 runoff bins solely because no finer producer clock exists
(`docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:297`). The
implementation applies exactly that proportional reduction
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1432-1474`),
and the focused test ratifies a 60/40 proportional split
(`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:204-225`).

Absence of finer timing authority does not establish that frost retention acts
proportionally across runoff hours. This allocation preserves the set of
nonzero hours but can change every hourly depth and the maximum-hour magnitude;
it is therefore a temporal process assumption, not merely arithmetic
bookkeeping. The cited anchors in `INV-WATBAL-103` establish hourly liquid/carry
custody and runoff partitioning, but no cited source establishes the
proportional frost debit. Calling the result authoritative hourly runoff turns
missing timing into a production peak through a heuristic, contrary to the
package's no-proxy/no-synthetic-timing boundary.

Proposed disposition: `accepted / closure-blocking`. Produce the frost-retained
debit at hourly resolution in the owning frost/WB14 interaction and subtract it
in the modeled hour, or hold the hourly peak claim on days with a material
daily-only frost debit. If a proportional allocation is retained only for a
diagnostic, label it non-authoritative and prohibit it from erosion, HBP/pass,
or public peak claims. A science-contract assertion cannot by itself supply the
missing physical timing authority.

### `SCI-B2-002` — MEDIUM — `TOL-WATBAL-009` is numerically bounded but its stated provenance is incomplete

`TOL-WATBAL-009` permits an aggregate difference of
`24 * 1e-9 m * max(1, scale)` and applies the residual to the largest runoff bin
(`SC-WATBAL-001.md:873`; `runoff.rs:1451-1495`). The bound is small for ordinary
hillslope depths, material differences hard-fail, and the tests distinguish a
`1e-9 m` accepted discrepancy from a `1e-3 m` rejection
(`direct_runtime_dc01.rs:228-253`). That is a plausible floating-ledger
reconciliation posture.

However, the contract states that "the WB14 interval solve permits `1e-9 m`"
without a specific authority anchor or an implementation-derived error bound,
and multiplying by all 24 bins is conservative even when only one interval
contributes. Applying the residual to the largest bin intentionally changes the
peak operand. This is acceptable only as explicitly bounded numerical
reconciliation, not physical normalization, and it must remain independently
observable.

Proposed disposition: `accepted`. Add the exact source/derivation for the
per-interval `1e-9 m` allowance and publish the signed reconciliation residual
and adjusted bin in provenance. Add boundary tests at just below/above the full
24-bin tolerance and a vector where the correction could change a near-tied
peak hour. This finding becomes closure-blocking if the residual is not exposed
or the tolerance cannot be grounded.

## Prior Blocker Rechecks

### Synthetic runon timing — `PASS`

Positive surface or lateral runon with no hourly source now returns a typed
`MissingDirectUpstream`; the uniform branches are gone
(`runoff.rs:651-733`). Runon is added to WB14 hourly supply and the focused test
now rejects missing positive shape
(`direct_runtime_dc01.rs:27-48`). The downstream peak guard therefore no longer
receives a manufactured uniform hydrograph.

### Melt double counting — `PASS`

Routed melt is now an hourly additional liquid supply admitted to WB14 once.
The post-partition peak assembler consumes only WB14 excess plus WB19
saturation return; routed melt is no longer appended as a runoff limb
(`runoff.rs:1498-1553`;
`artifacts/operand-lineage.md:5-19`). Tests demonstrate both a melt-supply
hydrograph that becomes runoff and a melt supply that fully infiltrates without
creating runoff (`direct_runtime_dc01.rs:50-111`). This closes the prior
source-overlap defect.

### Routed multi-OFE outlet HBP peak — `PASS`

The real multi-OFE integration test reads the outlet pass row, independently
checks `sum(V_h) = runvol`, reconstructs HBP peak as
`max(V_h) / 3600 s`, and proves pass/HBP peak equality
(`tests/integration/erosion_multi_ofe_p102_chain.rs:83-110`). This is adequate
for the routed hillslope-exit hourly-mean claim. It does not establish an
instantaneous, channel-routed, or watershed-outlet peak, and the package
continues to exclude those broader claims.

## Additional No-Finding Results

- Equal-volume concentrated/spread ordering, saturation-only timing, positive
  runoff/missing-timing failure, rectangular-equivalent duration semantics,
  and exact-zero dry behavior remain correctly tested and contract-bound.
- Internal peak units remain `m s^-1`; public pass/HBP units are `m^3 s^-1`;
  publication adjusts to the run-volume depth basis and applies positive area
  exactly once.
- HBP and pass consumers use the same outlet event basis and correctly describe
  maximum hourly mean flow, not an instantaneous peak.
- Normalized weights are now derived only after the post-partition hourly depth
  ledger closes; they no longer manufacture timing from the daily runoff
  scalar.

## Verdict Rationale

The corrective work closes the three prior blockers and materially improves
source custody. `HOLD` remains necessary because a material frost-retention
debit with no hourly producer is allocated proportionally by assumption and
then allowed to control erosion and public peak claims. That is precisely the
kind of missing-timing proxy the package set out to remove. Resolve
`SCI-B2-001`, and ground/expose the bounded arithmetic adjustment in
`SCI-B2-002`, before claiming hourly peak authority complete.
