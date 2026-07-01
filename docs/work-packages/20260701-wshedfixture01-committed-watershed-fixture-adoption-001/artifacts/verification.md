# Verification

Status: `EXECUTED`

## Ran

- `cargo fmt --check`
  - Result: `PASS`.
- `cargo clippy --test infile_watershed_structure_parser_contract -- -D warnings`
  - Result: `PASS`.
- `cargo nextest run --test infile_watershed_structure_parser_contract carnivorous_adobo_committed_fixture_is_repo_local_32_hillslope_gate`
  - Result: `PASS`; 1 test passed.
- `cargo nextest run --test infile_watershed_structure_parser_contract`
  - Result: `PASS`; 21 tests passed.
- `cd tests/fixtures/watershed/carnivorous-adobo && sha256sum --quiet -c input-manifest.sha256`
  - Result: `PASS`.
- `rg -n "/wc1|wepppy" tests/fixtures/watershed/carnivorous-adobo/runs || true`
  - Result: `PASS`; no matches.
- `rg -n "[ \t]$" tests/fixtures/watershed/carnivorous-adobo`
  - Result: `PASS`; no matches.
- `find docs/work-packages/20260701-wshedfixture01-committed-watershed-fixture-adoption-001 -name '*.md' -print0 | xargs -0 -n1 markdown-doc lint --path`
  - Result: `PASS`; each file reported 0 errors and 0 warnings.
- `markdown-doc lint --path docs/ROADMAP.md`
  - Result: `PASS`.
- `markdown-doc lint --path docs/work-packages/README.md`
  - Result: `PASS`.
- `markdown-doc lint --path tests/fixtures/watershed/carnivorous-adobo/README.md`
  - Result: `PASS`.
- `git diff --check`
  - Result: `PASS`.

## Static

- `tests/integration/infile_watershed_structure_parser_contract.rs` line count:
  `623`, below the 2000-line work-package warning threshold.
- New fixture subtree size: about `9.5M`.
- New fixture file count: `177` files total, including README and checksum
  manifest; `175` input/runfile files under `runs/`.
- Full production Rust workspace gates are not required by this package because
  no production Rust files were touched.

## Not Run

- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

Rationale: package kickoff explicitly says not to run full Rust workspace gates
unless production Rust is touched.
