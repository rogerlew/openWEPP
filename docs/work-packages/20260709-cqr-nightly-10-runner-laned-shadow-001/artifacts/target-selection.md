# Target Selection

Evidence label: Static/Ran.

Status: `SCAFFOLDED`

Selected target:

- Rank: `10` of `10`
- Module: `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- Quality dimension: `CRAP/cyclomatic-complexity`
- Production line count: `706`
- Baseline target LCOV: `251/452` lines (`55.53097345132743%`)
- Baseline function coverage: `23/39`
- Baseline CRAP rows above `30`: `3`
- Baseline max CRAP: `210.0`
- Baseline total excess over `30`: `374`

Nightly measurement inputs:

- Coverage source: `/tmp/openwepp-cqr-nightly.lcov`
- Coverage SHA-256:
  `7dd0b93fcd5e0f217d5b4e6fd0a6871a04976ac0b5c788dbb2a9fdffca37217a`
- CRAP source: `/tmp/openwepp-cqr-nightly-crap.json`
- CRAP SHA-256:
  `636ce39bc06a7172ee9e62ee9946afd2dda25f0cd56a76cfd5cad047d6438289`
- Target selection follows
  `docs/work-packages/cqr-nightly-burndown-execplan.md`.

Rationale:

The module is the tenth eligible nightly CQR module by live CRAP burden. The
target is below the line-count WARN threshold and has three zero-covered
high-CRAP helpers in the Lane D diagnostic-shadow collector. Existing unit,
source-guard, and H2637 integration tests give a starting oracle for
behavior-preserving decomposition.

Selected nightly table:

| Rank | Module | Total excess | Rows >30 | Max CRAP |
|---:|---|---:|---:|---:|
| 1 | `crates/openwepp-kernel-contract/src/lib_mod/core_types/01_typed_symbol_surfaces.rs` | `2915.669` | `3` | `2833.422` |
| 2 | `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs` | `1593.634` | `18` | `306.0` |
| 3 | `crates/openwepp-runner/src/bin/openwepp-snowbench.rs` | `1176.0` | `2` | `930.0` |
| 4 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` | `892.0` | `10` | `306.0` |
| 5 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | `525.880` | `2` | `547.238` |
| 6 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` | `504.950` | `7` | `296.516` |
| 7 | `crates/openwepp-input-contract/src/parsers/management.rs` | `482.183` | `6` | `203.621` |
| 8 | `crates/openwepp-runner/src/errors.rs` | `402.769` | `6` | `192.899` |
| 9 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs` | `386.0` | `4` | `272.0` |
| 10 | `crates/openwepp-runner/src/hillslope/laned_shadow.rs` | `374.0` | `3` | `210.0` |

Exclusion note:

- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` was
  excluded from target selection because CQR nightly selects production modules
  under `crates/`, not test files.

Instruction discovery:

`tools/agents/find-agents --for
crates/openwepp-runner/src/hillslope/laned_shadow.rs
crates/openwepp-runner/src/hillslope/03_tests.rs
crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs
tests/integration/laned_shadow_h2637.rs
docs/work-packages/README.md` reported:

- target module: `AGENTS.md`, `crates/AGENTS.md`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`: `AGENTS.md`,
  `crates/AGENTS.md`
- `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs`:
  `AGENTS.md`, `crates/AGENTS.md`
- `tests/integration/laned_shadow_h2637.rs`: `AGENTS.md`, `tests/AGENTS.md`
- `docs/work-packages/README.md`: `AGENTS.md`,
  `docs/work-packages/AGENTS.md`
