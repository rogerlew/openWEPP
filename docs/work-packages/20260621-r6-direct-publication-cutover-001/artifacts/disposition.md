# Disposition

Status: executed-hold.
Evidence mode: Static + Ran.

Final disposition:
`HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER`.

## Evidence

Ran:

- R5E completion evidence was read from
  `docs/work-packages/20260621-r5e-full-ofe-day-endpoint-readiness-001/` after
  pushed commit `d8f6bbea`.
- R6A completion evidence was read from
  `docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001/`.
- `cargo test -p openwepp-runner r6_ -- --nocapture`: PASS.
- `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture`:
  PASS.
- `cargo test -p openwepp-runner r6a_ -- --nocapture`: PASS.
- `cargo test -p openwepp-runner r2a_default_fixture_run_constructs_no_direct_runtime_skeleton -- --nocapture`:
  PASS.
- `cargo test -p openwepp-runner`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260621-r6-direct-publication-cutover-001 --format json`:
  PASS.
- `git diff --check`: PASS.
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- ... --direct-publication-frame-cutover`:
  exit status `1`, fail-closed at HBP identity with
  `R6-DIRECT-PUBLICATION-PARITY`.

Static:

- `docs/architecture/array-native-runtime-specification.md` section `5.2.1`
  is canonical publication ledger authority.
- `HillslopeRuntimeSelection::DirectPublicationFrameCutover` and
  `--direct-publication-frame-cutover` now exist.
- Candidate HBP/WAT/PASS/loss output branches consume
  `DirectPublicationArtifacts` built from `DirectRunPublicationFrame`.
- Candidate output writes are blocked until parity gates pass.
- The production manifest writer remains compatibility-provenance based.

## Outcome

R6 execution progressed past the old frame-absent blocker but cannot close.
The direct publication frame and consumers exist, and the real output boundary
has an opt-in fail-closed candidate. The first identity gate fails because the
current direct publication frame is still skeleton/zero populated for parity-
critical operands. Manifest cutover is also not complete.

No public direct publication output is written by the cutover candidate while
known identity and manifest gates are failing.
