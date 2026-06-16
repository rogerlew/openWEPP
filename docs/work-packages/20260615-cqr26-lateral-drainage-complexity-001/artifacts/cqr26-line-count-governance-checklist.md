# CQR26 Line-Count Governance Checklist

Status: complete-with-warnings.

Ran: before line counts:

| Path | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs` | 2527 | Below hard `3000` ceiling; above older caution threshold; not edited |
| `docs/work-packages/README.md` | 637 | Documentation catalog, package registration only |
| `docs/work-packages/cqr-burndown-execplan.md` | 722 | Tracker excluded from package commit until after package push |

Static: no non-exempt production `.rs` file was edited. The target file line
count warning is recorded but does not block package closure because live CRAP
metrics are already below threshold and no kernel code changed.

Ran: suppression census for the target file found one pre-existing suppression:
`#[allow(clippy::wildcard_imports)]` at line `1`.
