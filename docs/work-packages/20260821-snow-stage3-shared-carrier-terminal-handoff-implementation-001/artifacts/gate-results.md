# Gate Results

Status: `EXECUTED HOLD / TYPED OPT-IN AND IDENTITY-RESTART GATES PASS; AUTHORITY AND PRODUCTION-PATH GATES OPEN`

The Child 2C authority gates are historical prerequisite evidence. Current
evidence:

- `Ran: .venv/bin/python .../reference_model.py` — 17 authority cases and 3
  restart/rollback cases completed with expected outcomes.
- `Ran: git diff --check` — pass.
- `Ran inside nix develop: cargo fmt --all -- --check` — pass.
- `Ran inside nix develop: cargo check --workspace` — pass.
- `Ran inside nix develop: cargo nextest ... --test
  snow_stage3_shared_carrier_terminal_handoff_implementation` — latest 6/6
  pass.
- `Ran inside nix develop: cargo nextest ... --lib -E
  'test(child2c_scheduler_commits_the_concrete_v11_lse_bgc_soil_owner_candidate)'`
  — 1/1 pass.
- `Ran inside nix develop: cargo nextest ... --lib -E
  'test(v11_full_support_runs_actual_v10_stack_and_finalizes_once) | test(child2c_scheduler_commits_the_concrete_v11_lse_bgc_soil_owner_candidate)'`
  — 2/2 pass.
- `Ran inside nix develop: cargo clippy --workspace --all-targets -- -D warnings`
  — follow-on source warnings cleaned; unrelated existing test lints remain.
- `Ran inside nix develop: cargo deny check` — advisories, bans, licenses, and
  sources passed; existing unmatched `MIT-0` allowance warning remains.
- `Ran inside nix develop: cargo nextest ... --profile frost` — final 390/391
  pass; the unchanged `SC-SNOWENERGY-001.md` v13 marker guard failed outside
  this write set.
- Combined shared-carrier/terminal-receiver authority selection — 17/18 pass;
  one unchanged science-registry lifecycle-string guard failed.

The typed opt-in owner endpoint and its identity/restart hardening are proven,
and dual reviews/verifications are complete. Ordinary-runner integration,
terminal-liquid receiver custody, durable publication closure, and the two
unchanged authority-document guard failures remain release `HOLD` conditions.
