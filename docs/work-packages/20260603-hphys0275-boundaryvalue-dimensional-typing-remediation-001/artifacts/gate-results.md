# Gate Results

Status: completed/HOLD
Evidence mode: ran

Static: HPHYS0275 scoped gates pass. Workspace test remains HOLD due known
SIMIMPL18 fixture failures previously recorded in HPHYS0268, HPHYS0269,
HPHYS0270, HPHYS0271, and HPHYS0272 artifacts.

Ran:

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | pass | final-state run after accepted review fixes |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | final-state run after checked-count test helper |
| `cargo test -p openwepp-unit-boundary` | pass | 19 unit tests passed |
| `cargo test -p openwepp-kernel-contract` | pass | 14 unit tests passed |
| `cargo test -p openwepp-hillslope-orchestrator runtime_inputs` | pass | 47 runtime-input tests passed |
| `cargo test --test hphys0275_boundary_value_dimensional_typing_contract` | pass | 4 integration tests passed |
| `cargo test --test sim_contract_boundary_unit_registry` | pass | 10 integration tests passed |
| `tools/release/check_unit_registry.sh` | pass | registry test + focused clippy passed |
| `cargo deny check` | pass | advisories/bans/licenses/sources ok; existing duplicate and unmatched-license warnings remain |
| `markdown-doc lint --path ...` | pass | 26 files validated, 0 errors, 0 warnings |
| `git diff --check` | pass | no whitespace errors after doc cleanup |
| `cargo test --workspace` | fail/HOLD | two known SIMIMPL18 fixture tests fail with `HKERNEL-WB11-ET-E-003` in `pl14s_tier_a_candidate_emission_and_replay_contract` |

## Workspace Test HOLD Details

Ran: `cargo test --workspace` returned 101. Failing tests:

- `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`
- `simimpl18_contract_requires_multi_day_storage_state_mutation`

Both fail during fixture execution with:

- `message_id=HKERNEL-WB11-ET-E-003`
- `last_phase=evapotranspiration`
- `boundary_class=DOMAIN_VIOLATION`

Static: The same failure pattern is documented as known/unrelated in prior
work-package artifacts, including HPHYS0268, HPHYS0269, HPHYS0270, HPHYS0271,
and HPHYS0272.
