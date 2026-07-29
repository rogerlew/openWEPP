# Finding Disposition

Status: `prospective corrections complete; both re-reviews passed`

Evidence class: `Static`

| Finding | Disposition | Correction |
| --- | --- | --- |
| `PRA-001`, `B-07` | `CORRECTED` | Added `authority-source-ledger.md` with distinct Kloeppel, Keane journal, Keane RMRS-RP-70, Lim main/supplement, baseline, and CLM identities, SHA-256 digests, access date, and exact anchors. Unauthenticated White/Bernier passages determine no production operand. |
| `PRA-002`, `B-01` | `CORRECTED` | Split `measured_daily` from `prescribed_scenario`; interval totals are non-executable without separate temporal authority. Added explicit per-tissue inclusive support, exhaustive daily requirements, and mode-specific omitted-day semantics. |
| `B-02` | `CORRECTED` | Each tissue now independently declares `complete`, `not_represented`, or authority-backed `not_applicable`; only `complete` carries numbers. Support is explicit rather than inferred from event dates. |
| `B-03` | `CORRECTED` | Added typed source/digest, transformation, material, dry-mass, spatial/OFE, temporal/calendar, and fine-wood diameter/bark metadata. Undocumented conversion fails closed. |
| `B-04` | `CORRECTED` | Applicability is bound to typed, authority-backed vegetation/material classes. Evergreen fraction and aggregate structural biomass are explicitly prohibited proxies. |
| `PRA-003`, `B-05` | `CORRECTED` | Added separate surface/interrill/rill pre/post-decay recurrences, parallel per-area semantics, weighted ground state, internal leaf debit/credit, external influx, authorized losses, and real-consumer reconstruction requirements. |
| `PRA-004`, `B-06` | `CORRECTED` | Frozen separate predictive and external-boundary ADR rows. Predictive rows remain `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`; the boundary row is `NOT_IMPLEMENTED / NOT_APPLICABLE / NOT_APPLICABLE` until implemented, with mode recorded outside the triple. Narrowed the package objective and completion claim. |
| `PRA-002` re-review residual, `RB-01` | `CORRECTED` | Added typed original support/resolution/units and raw-byte digest scope. The executable object is an exact UTF-8 CSV with a fully specified byte grammar. Original/identity payloads prohibit transformation metadata and bind identical source/executable files and digests; derived payloads require full derivation identity and distinct executable digest. |
| `RB-02` | `CORRECTED` | Required every complete payload's material/functional class and site/plot/lane/OFE binding to match the authority-backed vegetation and active simulation, with typed fail-closed rejection vectors. |
| `RB-03` | `CORRECTED` | Made the non-ADR implementation mode mutually exclusive and conditional: `PRESCRIBED_BOUNDARY_ONLY` for prescribed scenarios or `MEASURED_DAILY_BOUNDARY` for exhaustive daily observations. |

No finding is accepted as waived. Both original reviews remain immutable
`FAIL / HOLD` evidence. Both independent prospective re-reviews passed the
corrected packet before contract and production implementation began.

## Terminal correction round

Both initial terminal reviews and verifications failed closed. Every finding
was accepted and corrected:

| Findings | Correction |
| --- | --- |
| `TRA-001`, terminal B-1, verification authority findings | Added SHA-256-authenticated classification CSV, exact inline/source class match, needleleaf/woody tissue compatibility, and rejected all derived/interval execution in this identity-only increment. |
| `TRA-002`, terminal B-2, `VA-001..003` | Bound original/executable support exactly, validated authority dates, and required drying duration or constant-mass criterion. |
| `TRA-003`, terminal B-3 | Unrepresented/inapplicable tissue now publishes null; aggregate completeness is explicit and validated. |
| `TRA-004`, terminal B-4 | Expanded contract suite from 6 to 16 tests spanning admitted modes and reviewed rejection families. |
| `TRA-005`, terminal B-5 | Published/reconstructed weighted mass, cover, and depth; added exact typed erosion/frost consumer records and no-readdition source guard. |
| `TRA-006`, terminal B-6, `VA-004..005` | Re-ran Clippy, focused gates, size guard, line counts, and exact-head full profile after all code corrections. |
| Verification B write-set finding | Amended package write set for root Cargo manifest/lock changes. |

The immutable failed terminal sections remain in the assigned artifacts.
Independent terminal re-review and re-verification must assess these
corrections before closure.
