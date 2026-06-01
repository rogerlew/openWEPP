# AUTH11 Worker Handoff

Status: completed  
Evidence mode: Static

## Immediate next actions
1. For any suite posture edit or cohort-case edit, run:
   - `bash tools/release/check_authority_suite_antievasion.sh`
   - `cargo test --test auth11_required_suite_obligation_guards_contract`
2. Keep `required-suite-obligations.json` authoritative for case-binding and
   threshold-guard constraints.
3. Maintain closure package linkage while direct-theta suite remains
   non-blocking:
   - `20260531-auth12-fc-rocky-soil-closure-and-promotion-001`.
4. Execute AUTH12 before re-promoting direct-theta suite posture to
   `required`/`hard-fail`.
