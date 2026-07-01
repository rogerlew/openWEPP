# Gate Results

Status: `EXECUTED-COMPLETE`

| Gate | Result | Evidence |
| --- | --- | --- |
| Handoff prompt authored | `PASS` | `prompts/active/kickoff.md` |
| Package scaffold authored | `PASS` | `package.md` |
| Fixture committed under `tests/fixtures/watershed/` | `PASS` | `tests/fixtures/watershed/carnivorous-adobo/`, `tests/fixtures/watershed/carnivorous-adobo/README.md`, `tests/fixtures/watershed/carnivorous-adobo/input-manifest.sha256` |
| Source substrate is 32-hillslope carnivorous-adobo-derived watershed | `PASS` | `artifacts/fixture-provenance.md`; explorer review confirmed `p1..p32` input sets and `pw0.str` topology |
| Required input/runfile inventory recorded | `PASS` | `tests/fixtures/watershed/carnivorous-adobo/README.md`, `artifacts/fixture-provenance.md` |
| Focused fixture-contract test added and run | `PASS` | `cargo nextest run --test infile_watershed_structure_parser_contract carnivorous_adobo_committed_fixture_is_repo_local_32_hillslope_gate`: 1 passed |
| Full touched integration test binary run | `PASS` | `cargo nextest run --test infile_watershed_structure_parser_contract`: 21 passed |
| Focused clippy for touched test file | `PASS` | `cargo clippy --test infile_watershed_structure_parser_contract -- -D warnings` |
| Fixture manifest integrity | `PASS` | `cd tests/fixtures/watershed/carnivorous-adobo && sha256sum --quiet -c input-manifest.sha256` |
| Fixture has no embedded `/wc1` or wepppy dependency | `PASS` | Focused test asserts paths and contents; `rg -n "/wc1\|wepppy" tests/fixtures/watershed/carnivorous-adobo/runs || true` returned no matches |
| Scoped docs lint | `PASS` | Per-file `markdown-doc lint --path` for package docs, `docs/ROADMAP.md`, `docs/work-packages/README.md`, and fixture README: all 0 errors, 0 warnings |
| `git diff --check` | `PASS` | Final run recorded in `artifacts/verification.md` |
| Dual review and verification dispositioned | `PASS` | `artifacts/review-disposition.md`, `artifacts/verification.md` |
| Full Rust workspace gates | `NOT RUN` | Package kickoff says not to run full Rust workspace gates unless production Rust is touched; only test and fixture/docs files changed |
| Final validation recorded | `PASS` | `artifacts/verification.md`, `artifacts/disposition.md` |
