# CLI04 Output Architecture Authority And Guard Map

Status: completed (Phase A)
Evidence mode: Static

## Static

| Authority requirement | Canonical authority surface | Guard behavior | Error / hold surface |
| --- | --- | --- | --- |
| Shared output crate boundary uses CLI04 target `crates/openwepp-output/` with transition-only predecessor `crates/openwepp-hillslope-output/`. | `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md` (`RUNNER-HILL-INV-009`) + `docs/contracts/openwepp-runner-contract.md` | Production implementation outside authorized transition posture is non-compliant. | `hard-fail + package hold` |
| Required hillslope outputs `outputs.pass` (`.hbp`) and `outputs.loss` (`.json`) must exist post-run. | `docs/contracts/openwepp-hillslope-runfile-contract.md`, runner spec invariants | Missing required output blocks acceptance. | `CLIHILL-E-013`, `OPEN_RUNNER-E-018` |
| Optional output path/extension contract is typed and fail-closed. | output contract boundary + runner spec | Invalid extension or missing required path fails contract validation. | `OHOUT-E-001`, `OHOUT-E-002` mapped to `CLIHILL-E-010` |
| `outputs.wat` must preserve field metadata keys `units` and `description`. | runner spec (`RUNNER-HILL-INV-010`), runfile contract, contracts README | Missing metadata keys are typed output-contract failures; no silent default metadata fill. | `hard-fail` (typed output-contract error surface) |
| `outputs.wat` schema metadata must include version keys `dataset_version`, `dataset_version_major`, `dataset_version_minor`, `schema_version`. | runner spec (`RUNNER-HILL-INV-010`), runfile contract, WEPPpy `schema_with_version` authority | Missing version metadata keys block parity acceptance. | `hard-fail` (typed output-contract error surface) |
| WB13 canonical WAT schema authority is preserved with optional producer-authoritative `InterceptionStorage` extension from post-`wepp_260430` lineage. | runner spec (`RUNNER-HILL-INV-011`) + package provenance exception | Unauthorized schema drift or silent substitution is non-compliant. | `hard-fail + review hold` |
| New CLI04 parquet implementation work uses `arrow-rs` stack (`parquet`, `arrow-array`, `arrow-schema`) and does not adopt `arrow2`. | runner spec (`RUNNER-HILL-INV-012`), contracts README, package constraints | Dependency-policy violation blocks package promotion. | `hard-fail + package hold` |
| Placeholder/bootstrap output substitution for required/declared output surfaces is prohibited. | runner spec (`RUNNER-HILL-INV-001`), runner contract | Missing simulation-driven output contract semantics block acceptance. | `hard-fail + release/comparator hold` |

## Notes
- Baseline default remains `/workdir/wepp-forest_260430_baseline` @
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- CLI04 WAT authority exception applies only to output-schema parity semantics;
  it does not authorize unrelated physics changes.

## Ran
- Not run (Phase A artifact is authority/guard mapping only).
