# ASSURE-05 Realization And Path Freeze

Status: FROZEN FOR EVIDENCE EXECUTION

Evidence class: Static

Frozen Git realization: `01ed70550a4e371e99afe35c4bdd4d9b667e812c`

The worktree differed from the frozen commit only by ASSURE-05 documentation
scaffold files when this record was created. No Rust, test, fixture, assurance
source, or `usersum` file was modified. Evidence runs therefore execute the
frozen program and fixture bytes while package documentation remains uncommitted.

## Authority And Fixture Identities

| Object | SHA-256 |
| --- | --- |
| `SC-GWBASEFLOW-001.md` | `97ee00e87df4a87221aa34fc1f44c77176f43922bcfac96c69d4b6de8e230d60` |
| H2637 fixture file-hash stream, sorted by path | `25a180cc40808f021c3418103423256666f469b4894a0c575b3aaab9f21ce78c` |
| `gwcoeff.txt` | `ec23463aad0cd1e3000d68b6af5a1a85a327d4952a239c92efeba060007eb7e8` |
| `p2637.run.toml` | `fc1761adb0a2b486ca749a6e9f4c20e095eda3503a219e65a34eee5a692a28d0` |

H2637 declares initial storage `0 mm`, baseflow coefficient `0.04 d^-1`,
deep-seepage coefficient `0 d^-1`, and contributing-area threshold `1 ha`.
The runfile names the production HBP, loss JSON, and pass-Parquet outputs.

## Declared Groundwater Path Set

`git diff --name-status de520f1ff867ca5c65b1f82dfe32a19c213ae18c..01ed7055 -- <paths>`
returned no paths. The previously integrated groundwater implementation and
consumer surfaces are therefore byte-current at ASSURE-05 intake. This is a
currency check, not a fresh reproduction; execution remains required.

| Path | SHA-256 | Role |
| --- | --- | --- |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs` | `92674b52f7edd4ec67acd07a9069be9de4cfec6512f7c47e19d03f228746cb14` | Daily recurrence and guards |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | `fe79ddd4320eccb840fa6cc3a3cf80e1084478eba8ab8648a6651903fd0d4c95` | Direct-runtime state/orchestration |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs` | `f695b655fddddb02a0a934aa67bda277ea4cca55e785f0595c48df98e3785c76` | Daily publication frame |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | `45a1ee2e7aa7c641e9ee56edcc2eec46f2de071cd29bc3eeb314ac01a470b7d1` | Recurrence/domain tests |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | `be24dc346a6cc2ad52f8848a07551acdbaf51df7ebd334ef8d73b9fcd087d78e` | HBP serialization |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | `fe877946329fea20f4fe458a50762dac7cc612568ac7105c83c826dd7dc1431c` | Runner consumer tests |
| `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` | `8782ff042a27fd3806862c214cedeb6234e281933f2a124b944f92460546ba3e` | Direct output publication |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | `05594e839317f0164dc7ccb54a977dd5c83240821573fdbdb5a3af5d9d198628` | Run manifest and terminal operands |
| `crates/openwepp-input-contract/src/parsers/hbp/types.rs` | `7994b6c210cbbffc9b077d8899ba3ef3847a56f5c9d90aaa7fd9457c17020196` | Typed HBP groundwater fields |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` | `f9cd5d62c276e2cbe53365a22fc05816728fd77e41f0c34f6ec4de1fb7b12abf` | Watershed consumer and `cbase` separation |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | `eb236cf2149c9c42bbf1c58f47245b6508f5c0c9b01b28ed226858e03256e425` | Threshold/authority/consumer checks |
| `tests/integration/laned_shadow_h2637.rs` | `46fdc5a8fb68905df63c36389c38c52ebea5fbede858d3b74e7d44282bbc21b4` | Real H2637 execution and closure |

## Change Control

Any change to a declared path, fixture file, contract, analysis procedure, or
result-bearing source invalidates this freeze. Rerun the affected evidence and
independent reconstruction before manuscript review. A later report-source-only
commit may be the publication release identity, but the assessed implementation
identity remains explicit and the release-transfer record must compare both.

