# Review Disposition

Status: `complete / all findings dispositioned`

Evidence mode: `Static + retained Ran artifact audit`

| Review | Finding | Disposition | Resolution |
| --- | --- | --- | --- |
| A/B | All 24 failures were misclassified as conductivity-path failures | accepted | Corrected every current package/roadmap/catalog claim to 22 conductivity-wrapper and two thickness-reconciliation failures |
| A/B | Mass lineage named routed melt instead of snowpack SWE loss | accepted | Added `snowpack_swe_loss_m` as the mass-audit operand and narrowed `routed_melt_m` to runoff timing |
| A/B | Full surface-component reconstruction was not proved | accepted / HOLD | Cold-content closure is accurately described; missing retained shortwave is a current-scope closure blocker |
| A/B | Unevaluable LS criteria passed vacuously | accepted | Added tri-state results; interaction, robust comparisons, protected-group comparison, and compensation audit are `NOT_ASSESSED` |
| A/B | Package completion was premature | accepted | Status is `executed / hold / nonpromotion`; closure artifacts and terminal gates are being completed without calling the package complete |
| A/B | One-round execution lacked a durable sentinel | accepted | Added `execution-attempt.json`, default rerun refusal, and an explicit retained-output `--analysis-only` path |
| A/B | Provenance claims exceeded retained evidence | accepted | Added command, observation, protocol, attempt, executed-binary, and analysis-binary records; retained the missing dirty-source binding as an explicit limitation |
| A/B | Figure sidecars misstated failure counts/classes | accepted | Corrected two L failures and the 22/2 failure-class language |
| B | Cumulative plot suppressed smaller fluxes | accepted | Split snowpack loss from sublimation/refreeze into separate aligned panels |
| B | Calibration matrix and complete status conflicted | accepted | Package HOLD now matches the current-scope blocked evidence |
| Verification A | Producer-carried latent/mass residual was overstated as independent reconstruction | accepted / HOLD | Narrowed all claims and added missing per-step latent/mass operands as a second current-scope evidence blocker |
| Verification B | Machine report mislabeled all-completed as all-executed | accepted | Split execution metadata into `all_cells_attempted=true` and `all_cells_completed=false`; failure-aware figure branch uses completion |
| Verification B | Header-only effects CSV silently omitted unavailable factorial results | accepted | Emit all 96 lane-response rows with explicit status, retained cell values, and blank unavailable contrasts |

No finding authorizes a rerun, coefficient change, process change, or
promotion. Both terminal rereviews accept `executed / HOLD / nonpromotion`.
Reviewer A's final provenance wording correction was accepted: the attempt
ledger is now described as a retrospective operator attestation and future
sentinel, not contemporaneous proof of no retry.
