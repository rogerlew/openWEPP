# Verification

Evidence class: Ran.

Commands and outcomes:

| Command | Result |
| --- | --- |
| `.venv/bin/python -m py_compile tools/snowfreeze_observed/paradigm2_stage3_liquid_routing_meltwater_temperature.py` | passed |
| `cargo fmt --check` | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | passed |
| `cargo test --test paradigm2_stage3_liquid_routing_meltwater_temperature` | passed, `6` tests after deferred cold-content cap fix |
| `cargo test --test snowdensity03_physics_bulk_offline_contract` | passed after adding the Stage 3 observed wrapper to the explicit authorized diagnostic surface list |
| Focused Stage 0-3 + adjacent snow-density integration bundle | passed |
| `cargo test -p openwepp-hillslope-orchestrator r7b_constructor_type_size_layout_is_bounded -- --nocapture` | passed |
| `cargo test --workspace` | passed |
| `cargo deny check` | passed: advisories, bans, licenses, sources |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | passed |
| `.venv/bin/python tools/snowfreeze_observed/paradigm2_stage3_liquid_routing_meltwater_temperature.py --hill-binary target/release/openwepp-cli-hill` | passed deferred observed gates: Stage 3 vs Stage 1 rollback had `0` worse robust cells and `0` worse runoff/timing cells; full Stage 3 arm remains `16`/`177` versus current default `15`/`179` |
| H2637 Stage 1 rollback direct endpoint | passed, `69.91 s`, `1150608 KiB`, exit `0` |
| H2637 Stage 3 direct endpoint | passed, `72.59 s`, `1150608 KiB`, exit `0`; within ADR-0025 `<=10x` budget |
| `wctl doc-lint --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/work-packages/README.md --path docs/planning/snow-frost-fidelity-strategy.md --path docs/work-packages/20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001` | passed |
| `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/work-packages/README.md --path docs/planning/snow-frost-fidelity-strategy.md --path docs/work-packages/20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001 --format json` | passed, `13` files, `0` errors, `0` warnings |

The final full workspace test run passed. Earlier failed full-workspace attempts
were resolved before closeout:

- The original Stage 3 implementation failed the direct-runtime type-size guard;
  that failure is recorded in `performance-h2637.md` because it shaped the
  retained boxed diagnostic carry.
- The deferred closeout rerun failed
  `snowdensity03_physics_bulk_offline_contract` because the new observed wrapper
  mentions `physics_bulk_multilayer_density_v1` for Stage 3 isolation. The fix
  adds only that package-bound wrapper to the explicit authorized diagnostic
  surface list; the focused test and final workspace rerun passed.

Deferred gate disposition:

- Real H2637 endpoint timing/RSS is now run and passes the ADR-0025 budget.
- Cross-SNOTEL/cancov observed guardrails are now run. The Stage 3 increment is
  neutral versus Stage 1 rollback, but the full opt-in arm remains worse than
  the current no-env default because it inherits the Stage 1 layered-density
  profile. No activation or promotion is authorized.
- Full in-stream water-temperature routing remains out of scope for this
  package.

No omitted gate is claimed as passed.
