# Gate Results

Status: PASS. Evidence mode: Ran.

## Acceptance Gates

| Gate | Result | Evidence |
|---|---:|---|
| Archive branch before removal | PASS | Branch `abandoned/hybrid-implicit-stepping` exists at `b1d5fd4410b700012d857ef4056000163e6aa6a0`; recorded in `artifacts/branch-provenance.md`. |
| Pre-strip active-plain identity baseline | PASS | `artifacts/plain-identity-baseline.md/json`; release binary SHA256 `d8aca1a31674a1527c8a0ee4535c329a0077229f622b5a149a339d5126af37bd`. |
| Post-strip active-plain identity | PASS | `artifacts/plain-identity-after.md/json`; release binary SHA256 `11cb3d49f74c1b00966d9fd41b2dba6077313f6dc9919f56ded526155182c43a`. |
| HBP/pass-parquet pre/post equality | PASS | `h2637`, `mn_corn_h4`, `n_idaho_forest_h1`, and `wa_cascades_forest_h1` all `hbp_identical=true` and `pass_parquet_identical=true`. |
| `SC-OFEROUTE-002` deleted/withdrawn | PASS | `test ! -f docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`; registry row status `withdrawn`. |
| Env-var removal posture | PASS | `OPENWEPP_LANED_ACTIVE_IMPLICIT` fails closed at startup; focused test passed. |

## Commands

| Command | Result |
|---|---:|
| `git diff --check` | PASS |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo deny check` | PASS (`advisories ok, bans ok, licenses ok, sources ok`) |
| `cargo nextest run --workspace --profile full` | PASS (`1414` run, `1414` passed, `3` skipped, `4` slow) |
| `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --profile quick` | PASS (`67` run, `67` passed, `249` skipped) |
| `cargo nextest run --test laned_shadow_h2637 --profile quick` | PASS (`4` run, `4` passed, `2` skipped) |
| `cargo nextest run -p openwepp-hillslope-orchestrator material_terminal_bin_deficit_fails_closed_on_public_path --profile quick` | PASS (`1` run, `1` passed) |
| `cargo nextest run --test hphys0279_sc_unit_compliance_lint_contract --profile quick` | PASS (`9` run, `9` passed) |
| `cargo nextest run --test sim_contract_status_taxonomy --test sim_contract_closure_checks --test auth04_release_gate_authority_stack_contract --profile quick` | PASS (`16` run, `16` passed) |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract --profile quick` | PASS (`2` run, `2` passed) |
| `markdown-doc lint --path ...` scoped to touched docs | PASS (`20` files, `0` errors, `0` warnings) |
| Line-count governance scan | PASS: no 3000+ non-exempt `.rs` file; touched 2000+ builder file is a pre-existing WARN and this package did not increase it. |

## Test Count Reconciliation

The prior no-harm selector package reported `1442/1442`. The final
post-strip workspace full gate is `1414/1414`, a net drop of `28`.

Disposition:

- Hybrid implicit stepper, selector, Case-4 hybrid ladder, cooldown,
  deficit-carry composition, bare-skin direct-equilibrium, and hybrid
  direct-runtime selector tests were retired with the abandoned subsystem.
- Retired test names are listed in `artifacts/strip-inventory.md`.
- One retained explicit-path regression was added:
  `material_terminal_bin_deficit_fails_closed_on_public_path`, preserving
  the live `NegativeOutletBin` fail-closed invariant after the hybrid
  composition tests were retired.

## Reference Sweep

Ran targeted source/docs reference sweeps for:

`hybrid_implicit`, `implicit_recession`, `solver_steps_implicit`,
`implicit_equilibrium_map_evaluations`, `implicit_branch_evaluations`,
`run_with_options_deficit_carry`, `is_bare_skin_only`,
`ImplicitSolveNonConvergence`, `bare_skin_equilibrium`,
`OPENWEPP_LANED_ACTIVE_IMPLICIT`, and `SC-OFEROUTE-002`.

Result: PASS. Live-code hits are limited to the ADR-0037 env rejection and
its test. Other hits are historical ADR/contract revision entries,
registry withdrawal pointers, and package evidence.
