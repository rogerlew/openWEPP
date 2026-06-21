# R6G Verification Agent A

Status: complete.

Evidence mode: Independent verification of the terminal hold, focused tests,
and package evidence.

| Check | Evidence reviewed | Result | Notes |
|---|---|---|---|
| HBP identity | Focused runner R6G cutover candidate and CLI fail-closed contract | PASS | Current-fixture HBP byte identity remains green before the WAT hold. |
| First WAT row | Focused R6G WAT reduction evidence | PASS | First direct and compatibility WAT rows match for ET/storage after direct producer binding and residual-inclusive storage projection. |
| Residual WAT blocker | R6G reduced mismatch and marker-reservation test | PASS-HOLD | Remaining mismatch is exactly day-2 `Es`, `Total-Soil`, and `SoilWaterTotal`; marker is reserved for that mechanism. |
| No compatibility authority | Static proof and code review | PASS-HOLD | Direct WAT artifacts are built from retained direct publication rows; compatibility rows are comparators only. Full symbol allowlisting remains follow-up. |
| Required gates | Final local gate table | PASS | `fmt`, `check`, `clippy`, focused tests, workspace tests, dependency policy, and diff check passed. |

## Verdict

Verified `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`. The package is
valid as a held reduction, not as complete publication cutover.
