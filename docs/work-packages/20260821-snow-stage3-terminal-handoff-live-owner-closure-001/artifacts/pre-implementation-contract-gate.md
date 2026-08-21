Status: complete
Evidence mode: Static/Ran

Static: The prospective owner-binding table in `package.md` was frozen before
implementation. The package excludes all protected science, selectors,
defaults, CoE ownership, production outputs, deployment, and the prior
package's files.

Static: The stale test guard was corrected from the old SC-SNOWENERGY version
to the released v14/index wording; the released contract body and identity
authority were not edited.

Ran: `bash tools/release/check_authority_suite_antievasion.sh` — PASS.
Ran: `nix develop --command cargo nextest run --test
auth11_required_suite_obligation_guards_contract` — 3 passed.
