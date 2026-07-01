# Disposition

Status: `EXECUTED-COMPLETE-WSHED-FIXTURE01`

Final disposition: `EXECUTED-COMPLETE-WSHED-FIXTURE01`.

Static:

- WSHED-FIXTURE01 adopted the committed fixture at
  `tests/fixtures/watershed/carnivorous-adobo/`.
- The fixture is derived from `/wc1/runs/ca/carnivorous-adobo/wepp` and contains
  the local input/runfile substrate for a 32-hillslope watershed.
- The fixture records provenance, input inventory, topology summary, intended
  scope, and checksum manifest.
- The focused Rust gate proves the persistent fixture contract reads the
  committed openWEPP fixture path and rejects `/wc1` or wepppy as persistent
  fixture dependencies.
- Boundary: this package closes input/runfile fixture adoption and parser gate
  readiness only. It does not claim current `openwepp-cli-watershed` end-to-end
  execution from this fixture because that surface requires TOML
  `openwepp-watershed-runfile-v1` plus HBP pass bindings.
- `docs/ROADMAP.md` and `docs/work-packages/README.md` were updated to move
  WSHED-FIXTURE01 from queued to executed and leave WSHED-W2 as the next
  watershed runtime queue rung.

Ran:

- `cargo fmt --check`
- `cargo clippy --test infile_watershed_structure_parser_contract -- -D warnings`
- `cargo nextest run --test infile_watershed_structure_parser_contract carnivorous_adobo_committed_fixture_is_repo_local_32_hillslope_gate`
- `cargo nextest run --test infile_watershed_structure_parser_contract`
- `cd tests/fixtures/watershed/carnivorous-adobo && sha256sum --quiet -c input-manifest.sha256`
- `markdown-doc lint --path` for every package Markdown file plus
  `docs/ROADMAP.md`, `docs/work-packages/README.md`, and fixture README
- `git diff --check`

Not run:

- Full Rust workspace gates. Package kickoff explicitly says not to run full
  Rust workspace gates unless production Rust is touched; this package changed
  fixture files, docs, and one focused integration test.

Review disposition:

- `rust_code_reviewer` finding about stale closure artifacts: accepted and
  fixed in `gate-results.md`, `disposition.md`, `review-disposition.md`, and
  `verification.md`.
- `rust_qa_reviewer` finding about focused clippy failure: accepted and fixed by
  splitting the long test into helpers and rerunning focused clippy.
- `rust_qa_reviewer` finding about untracked/new fixture diff hygiene: accepted;
  final verification includes manifest integrity, trailing-whitespace check, and
  `git diff --check` with new files made visible by intent-to-add.
- `rust_qa_reviewer` finding about missing review/verification artifacts:
  accepted and fixed.
