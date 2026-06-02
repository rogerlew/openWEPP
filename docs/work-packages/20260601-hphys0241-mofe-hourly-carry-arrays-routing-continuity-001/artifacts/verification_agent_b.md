# HPHYS0241 Verification Agent B

Status: complete
Evidence mode: ran + static

Ran verification:

- `cargo test --test wb11_hydrology_kernel_contract hphys0241` passed.
- `cargo test --test cli03_runner_contract_derived_tests hphys0241` passed.
- `cargo test --test cli03_runner_contract_derived_tests mofe04` passed.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_mofe05` passed.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract` passed.

Static verification:

- HPHYS0241-specific tests cover array-derived carryover, copy-forward,
  malformed upstream payload rejection, manifest metadata publication, and
  watershed contributor metadata intake.
- HPHYS0242 remains required for cadence-dependent positive saturation carry.
