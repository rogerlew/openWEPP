# Independent Hydrology And Ownership Re-Review

Evidence class: `Static + Ran`

Reviewer role: independent hydrology/ownership science reviewer

Disposition: `GO`

## Prior Finding Reassessment

| Finding | Re-review disposition | Evidence |
| --- | --- | --- |
| `HYD-REV-001` | `CLOSED` | The adapter calls the production-owned `direct_runtime::real_water_owner::authorize_direct_layer_withdrawals()` endpoint over seeded `DirectDayFrame` layers. That endpoint owns source availability and delegates only equal-status arithmetic to the shared kernel primitive. Native R4N and the shadow candidate both call `apply_direct_finalized_layer_liquid_debit()`; the earlier adapter-local authorization/debit algorithms are gone. |
| `HYD-REV-002` | `CLOSED` | `RealHydrologyShadowAdapter::try_from_day_start()` calls the same `DirectRunFrame::seed_day_frame()` used by the executor and extracts water/layer facts from those freshly seeded frames before any hydrology span. |
| `HYD-REV-003` | `CLOSED` | The public bridge rejects routed multi-OFE use and validates adapter interval against `configuration.dt_s`, transaction against `beginning.last_transaction_id + 1`, and forcing cardinality/order/beginning liquid/frozen state against the selected production source. Root accessibility is derived from the joined forcing rather than supplied independently. Tests poison interval, transaction and beginning water. |
| `HYD-REV-004` | `CLOSED` | Extraction accepts only exactly unfrozen or exactly fully frozen layers. Partial frost returns a typed operand error and is not converted into whole-layer exclusion or liquid supply. |
| `HYD-REV-005` | `CLOSED` | Reason selection distinguishes one positive storage-limited request (`LiquidStorageLimit`) from multiple positive eligible requests sharing an oversubscribed source (`CompetingDemand`). The vegetation-side reason validator enforces the same distinction. |
| `HYD-REV-006` | `CLOSED` | The canonical bytes are now truthfully bounded to transaction/owner/interval, scheduler identity, lane topology/area, water/transfer operands, ordered layer IDs and all twelve layer fields. Complete production preservation is separately proven by whole-frame structural equality. `production-byte-invariance.md`, `production-state-and-owner-map.md`, and `extraction-parity-evidence.md` use this bounded language. |
| `HYD-REV-007` | `CLOSED WITH DECLARED BOUNDARY` | The public executable bridge explicitly rejects multi-OFE input. Lower-level owner tests prove distinct `(OFE, layer)` supplies and candidate debits across lanes. No routed consumer or activation claim is made; coordinated routing remains a named Child-4 obligation. |

## Final Finding Resolution

### `HYD-REREV-001` — `CLOSED`

`artifacts/production-state-and-owner-map.md` now separates transfer/runon
state projected into bounded snapshot bytes from frost/winter and unrelated
production carry retained in the full cloned frame and protected by whole-frame
structural equality. Exact serializer inspection confirms the bounded bytes
contain the stated transaction, owner, interval, scheduler, topology, water,
transfer, layer-identity and twelve-field layer projection and do not claim the
unserialized winter column. The evidence set is internally consistent.

## Science And Ownership Result

Static: exact scheduler snapshot, theta ownership, full-frost exclusion,
OFE/layer/basis conversion, same-source competition, `D/A/F`, finalized-only
debit, production isolation, and explicit single-OFE scope conform to the
bounded Child-2 contract. No diagnostic proportional owner endpoint is used by
the adapter; the shared proportional primitive is encapsulated by the
production-owned direct hydrology endpoint. The shadow remains unreachable
from runner/direct-runtime dispatch.

Static: the candidate conversion is one local depth-to-mass conversion
(`theta_m * 1000 kg m^-3`) on the OFE-ground basis; OFE area is not applied a
second time. The lower-level multi-lane proof keeps source identities distinct
without claiming routed execution.

## Ran Evidence

- `cargo nextest run --test vegetation_real_hydrology_shadow_contract --profile quick` — `PASS`, 3/3.
- `cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(vegetation_real_hydrology_shadow)' --profile quick` — `PASS`, 11/11.
- `cargo nextest run -p openwepp-vegetation -E 'test(water_phase)' --profile quick` — `PASS`, 4/4.
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-vegetation --all-targets -- -D warnings` — `PASS`.
- `cargo fmt --all -- --check` — `PASS`.
- `git diff --check` — `PASS` after final review-artifact update.

## Final Disposition

`GO`. `HYD-REV-001..007` and `HYD-REREV-001` are closed. This reviewer has no
remaining hydrology, ownership, production-invariance or bounded-scope
objection to Child-2 completion.
