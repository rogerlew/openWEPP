# Disposition

Static: no accepted blockers remain.

Finding disposition:

| Finding | Disposition |
|---|---|
| Baseline `parse_non_runoff_event_payload` CRAP 182 from 0% coverage. | Fixed by direct non-runoff subevent characterization coverage. |
| Focused HBP-only CRAP made `parse_runoff_event_payload` appear cold. | Not a target defect; focused run undercovered workspace runoff tests. Final disposition uses full-workspace LCOV. |
| Fullcov log contains unrelated coverage-lane failures under `--ignore-run-fail`. | Recorded as caveat; final pass/fail gate is dedicated full nextest, not fullcov. |

Target-module final CRAP rows above 30: none.
