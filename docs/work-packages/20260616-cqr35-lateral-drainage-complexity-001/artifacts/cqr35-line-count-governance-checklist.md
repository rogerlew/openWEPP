# CQR35 Line Count Governance Checklist

Status: complete-with-warnings.

Ran: before line count for the target file:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs` | 2527 | WARN: above 2000, below 3000 hard ceiling |

Static: no `.rs` file was edited by CQR35.

Static: no touched non-exempt `.rs` file is at or above `3000` lines.

Warning: the target file remains above the older `2000` line caution threshold.
CQR35 did not edit it because live metrics already closed the CRAP target and
no behavior-preserving decomposition was needed.
