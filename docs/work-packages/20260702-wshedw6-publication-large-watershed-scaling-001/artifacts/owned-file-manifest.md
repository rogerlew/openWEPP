# Owned File Manifest

Status: `passed`

Evidence mode: `Static:` git status plus fixture checksum manifests.

All W6 files below are inside the package intended write set. The unrelated
untracked MOFE artifact is outside W6 scope and is not part of this manifest.

| Path | Class | Disposition |
| --- | --- | --- |
| `Cargo.lock` | build metadata | updated for `openwepp-watershed-output` dependency on `openwepp-watershed-orchestrator` |
| `crates/openwepp-watershed-output/Cargo.toml` | production crate metadata | adds direct typed publication-frame dependency |
| `crates/openwepp-watershed-output/src/writers.rs` | production + unit tests | direct typed writer, nullable operand projection, null regression tests |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | production | publication frame nullable operands, source-area aggregation, pass-backed detachment/deposition publication |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | production CLI | removes public row-seed staging, parses source runfile slope area, calls typed writer |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | test | public CLI source guard requires typed writer and forbids row-seed staging markers |
| `tests/integration/cli03_runner_contract_derived_tests.rs` | test | derived CLI marker updated to typed writer |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | test | typed contribution fixture updated with area |
| `tests/integration/infile_watershed_structure_parser_contract.rs` | test | onshore full fixture gate and carnivorous launch-file inventory gate |
| `tests/fixtures/watershed/carnivorous-adobo/README.md` | fixture docs | documents W6 committed launch files |
| `tests/fixtures/watershed/carnivorous-adobo/input-manifest.sha256` | fixture manifest | updated to `208` entries |
| `tests/fixtures/watershed/carnivorous-adobo/runs/case.run` | fixture input | committed watershed launch file |
| `tests/fixtures/watershed/carnivorous-adobo/runs/p1.source.run` through `p32.source.run` | fixture input | committed hillslope launch files |
| `tests/fixtures/watershed/onshore-xenophobia/**` | fixture input/docs | full `1305`-hillslope committed fixture; file-level inventory is `input-manifest.sha256` with `7847` entries |
| `docs/ROADMAP.md` | documentation | W6 roadmap state and full-fixture wording |
| `docs/work-packages/README.md` | documentation | W6 catalog wording |
| `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/package.md` | package docs | status/scope wording for full fixture execution |
| `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/prompts/active/wshedw6_kickoff_agent_prompt.md` | package prompt | execution guidance for no surrogate physics and full fixture closure |
| `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/*.md` | package artifacts | W6 evidence, reviews, gates, and disposition |
| `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/scaling/*.json` | generated evidence | current openWEPP scaling summaries |
| `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/scaling/w6-scaling-summary.csv` | generated evidence | aggregate scaling matrix |
