# Implementation and Test Evidence

Status: `EXECUTED HOLD / FOCUSED TYPED-OWNER AND IDENTITY-RESTART TESTS PASS; WORKSPACE AUTHORITY BASELINE FAILURES`

Added: `tests/integration/snow_stage3_shared_carrier_terminal_handoff_implementation.rs`
covers the authority carrier vector, sealed-wind/independent-node/scope poison
cases, support/tolerance event selection, `ERR-CT-021` no-candidate retry,
complete-owner stage/commit, exact terminal liquid, snow-free operand rejection,
canonical restart round-trip/tamper rejection, and the opt-in direct scheduler
publication hook.

Ran: `.venv/bin/python docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/artifacts/reference_model.py`
completed 17 reference cases and 3 restart/rollback cases successfully.

Ran inside `nix develop`:

- `cargo fmt --all -- --check` — pass;
- `cargo check --workspace` — pass;
- `cargo nextest run --workspace --profile quick --test snow_stage3_shared_carrier_terminal_handoff_implementation --no-fail-fast` — baseline 4/4 pass; latest identity/restart-hardening run 6/6 pass;
- latest package run after identity/restart hardening — 6/6 pass;
- `cargo nextest run --workspace --profile quick --lib -E
  'test(child2c_scheduler_commits_the_concrete_v11_lse_bgc_soil_owner_candidate)'
  --no-fail-fast` — 1/1 pass;
- `cargo nextest run --workspace --profile quick --lib -E
  'test(v11_full_support_runs_actual_v10_stack_and_finalizes_once) | test(child2c_scheduler_commits_the_concrete_v11_lse_bgc_soil_owner_candidate)'
  --no-fail-fast` — 2/2 pass;
- `cargo clippy --workspace --all-targets -- -D warnings` — follow-on source
  lints cleaned; workspace remains non-clean on pre-existing unrelated test
  lints;
- `cargo deny check` — pass for advisories, bans, licenses, and sources; the
  existing unmatched `MIT-0` allowance warning remains;
- combined shared-carrier/terminal-receiver authority selection — 17/18 pass;
- `cargo nextest run --workspace --profile frost --no-fail-fast` — final 390/391
  pass; one unchanged contract-document guard failed outside this write set.

The combined authority selection has one unchanged documentation guard failure
for a missing v13 lifecycle phrase in `science-contracts/index.md`. The frost
profile has one unchanged documentation guard failure for missing
`contract_version: 13` in `SC-SNOWENERGY-001.md`.

The typed V11/LSE/BGC/soil-thermal executor is now wired to the new opt-in
owner-aware scheduler method and its endpoint test reaches commit. The normal
hillslope runner remains on the ordinary scheduler path, so this does not close
the production direct-path claim. The package also proves carrier/event
participant joining, nonzero-remainder successor custody, event identity and
tie metadata, contiguous event ordinals, and restart-preserved receipt bodies.
No separate `domain` profile exists in
`.config/nextest.toml`.
