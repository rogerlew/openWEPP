# Kernel-profile and readiness matrix

Evidence mode: `Static + Ran + Expected-red`

Contract set: `SC-LANDSURFACEENERGY-001` version 30, extending existing
`INV-LANDSURFACEENERGY-159` with `OBL-LANDSURFACEENERGY-C-019`.

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

| Profile obligation | Status | Evidence and rationale |
|---|---|---|
| Canonical authority and registry | `PASS` | Contract v30 and the lifecycle index agree on identity, path, lifecycle, and date; tests do not pin index narrative. |
| Existing invariant binding | `PASS` | Extends `INV-159`; no `INV-164` or solver version is created. |
| Source-real object lineage | `PASS` | V8 structural and resident V3/V2 objects are explicitly distinct. Native omission is sourced only from the resident's matching validated revision. |
| Algorithm/state surfaces | `PASS` | Lazy plan joins occur at replaced structural checks; forcing proof follows the existing pre-V8 validation; fallible ingress precedes the resident-revision/native join. |
| Resident mutation/trust boundary | `PASS` | Persistent revision may clone only inseparably with the whole resident. Every successor and restart is fully validated; the borrowed execution proof is non-Clone, single-map, and immediate-use. |
| Branch and error precedence | `PASS` | New joins cannot move before existing guards. Adjacent paired poisons bind full/admitted first-error equality across carrier, forcing, V8, ingress, resident, dynamic vegetation -> surface -> soil/hydrology, native solver/residual, and output-validation boundaries. |
| Guard map | `PASS` | `INV-159` binds validation custody; `INV-161` and `SC-COUPLEDTIME-001#INV-COUPLEDTIME-030` retain role/disposition custody. |
| Units, aliases, constants, tolerances | `NOT_APPLICABLE` | No dimensional, normalization, parameter, constant, or tolerance change. |
| Calibration and identifiability | `NOT_APPLICABLE` | No estimable parameter, observation operator, objective, evidence, or calibration claim. |
| External constitutive suite posture | `NOT_APPLICABLE` | No equation, constitutive family, cohort, or required-case change. |
| Binding Exposure Index | `PASS` | Active `maps-to-existing-INV` row binds `INV-159/C-019` and dual review/verification. |
| Contract-derived assertions | `PASS` | Focused v30 contract assertion passes and detailed authority is read from the contract, not the index note. |
| Executable real-consumer behavior | `EXPECTED_RED` | Production owner/type/audit/oracle APIs are absent. Expected-red tests require authentic `1/52/52`, per-regime role/path parity, ordinary zero native use, paired precedence, and rollback. |
| Review disposition | `PASS` | Procedure-compliant `disposition.md` accepts and answers A-001 through A-006, B-01 through B-04, and B-FINAL-01. |
| Independent re-review/verification | `PASS` | Both independent reviewers confirmed manifest `f6bd360c2711b5ec4fce212a56dd4dc1167567cfeca863fd4c156c1a33a748be`; verification A returned `PASS` and verification B returned `PASS-WITH-NOTES`, authorizing only the bounded production attempt. |

## Calibration-readiness obligations

Every calibration schema item is `NOT_APPLICABLE`: the increment adds no typed
parameter, observation mapping, candidate objective, sensitivity,
identifiability, uncertainty, synthetic recovery, or additional-data claim.
Runtime boundary/error behavior is governed by C-019 instead.

## Obligation-to-test binding

| Canonical obligation | Pre-implementation binding |
|---|---|
| `INV-LANDSURFACEENERGY-159` | Integration assertions bind existing-invariant reuse, separate structural/resident objects, source-ordered custody, trust-boundary replay, and no new invariant. |
| Authentic counts/order | `carrier_parent_static_and_same_map_validation_once_has_authentic_1_52_52_counts`. |
| Full/admitted parity | `carrier_validation_once_is_bitwise_equal_for_every_role_path_and_regime`, including per-regime applicability and ordinary zero-native proof. |
| Poison/error/rollback | `carrier_validation_once_poisons_keep_first_error_no_fallback_and_rollback`, including distinct structural/native surfaces, dynamic state, solver/residual, output, and adjacent paired poisons. |
| Private anti-cache surface | `carrier_validation_once_source_surface_forbids_dynamic_or_wire_caches`, scanning intended and actual seams plus derive/manual-impl restrictions; supplemental only. |
