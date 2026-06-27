# Kernel Profile Compliance Checklist

Status: complete
Evidence mode: Static/Ran

| Requirement | Status | Evidence |
|---|---|---|
| Contract-first amendment before production code | PASS | `SC-SNOWFREEZE-001` v92 and pre-implementation contract gate. |
| No provisional production physics outside contract | PASS | Harder-Pomeroy opt-in authorized by `INV-SNOWFREEZE-065`; default remains legacy. |
| Typed domain failures rather than silent masking | PASS | selector and humidity-domain errors fail closed; supersaturation normalization is explicit exact-saturation behavior. |
| Active-hour precipitation conservation | PASS | implementation reconstructs `hrrain + hrsnow / 10`; focused tests exercise both paths. |
| Direct consumer evidence, not producer-only evidence | PASS | `snowdensity1035b_direct_snow_consumer_receives_phase_selector`. |
| Default rollback preserved | PASS | absent/empty selector returns `LegacyRst`; frost path pinned to `LegacyRst`. |
| No public output-schema expansion | PASS | no WAT/HBP/PASS schema edits; added runtime-boundary aliases only. |
| No fixture edits or site calibration | PASS | Jennings files read as external fixtures; no file edits or coefficient tuning. |
| Required gates | PASS | fmt, clippy, workspace tests, deny, anti-evasion guard, auth11. |
