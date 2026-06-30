# Gates

Evidence class: Ran + Static

## Stage 0 Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Release CLI builds | PASS | `cargo build -p openwepp-runner --bin openwepp-cli-hill --release` |
| H2637 current direct run executes | PASS | rc `0`, elapsed `1:09.18`, RSS `1159672 KiB` |
| H2637 minimized-output run executes | PASS | rc `0`, elapsed `1:13.77`, RSS `1159296 KiB` |
| Small fixture direct run executes | PASS | `cli01`, rc `0`, elapsed `0:00.09`, RSS `19584 KiB` |
| Production direct path has zero compatibility invocations | PASS | Manifests report `compatibility_edge_invocations=0` |
| Stage 0 attributes dominant RSS source | PASS | Evidence points to retained whole-run publication/ledger state, not setup-only symbol-map allocation |
| Continue to Stage 1 typed setup | BLOCKED | Stage 1 would not plausibly satisfy the package RSS gate before retained-publication streaming/drop work |

## Full Package Gates

No production code stage was executed after Stage 0 corrected the premise.
Accordingly:

- `cargo fmt --check`: NOT RUN in this package; no Rust code was changed.
- `cargo clippy --workspace --all-targets -- -D warnings`: NOT RUN; no Rust code
  was changed.
- `cargo nextest run --workspace --profile full`: NOT RUN; no Rust code was
  changed.
- `cargo deny check`: NOT RUN; no dependency or Rust code change.
- `bash tools/release/check_authority_suite_antievasion.sh`: NOT RUN; no
  authority suite or required-case binding change.
- `cargo test --test auth11_required_suite_obligation_guards_contract`: NOT RUN;
  no authority suite or required-case binding change.
- Markdown lint: PASS,
  `markdown-doc lint --path docs/work-packages/20260630-typed-direct-setup-symbol-map-elimination-001 --path docs/work-packages/README.md`
  reported `9 files validated, 0 errors, 0 warnings`.
- Markdown validate: PASS,
  `markdown-doc validate --path docs/work-packages/20260630-typed-direct-setup-symbol-map-elimination-001 --path docs/work-packages/README.md`
  reported `9 files validated, 0 errors`.

The held disposition is intentional under the work-package gate non-deferral
rule: the requested Stage 1/2/3 implementation is blocked by a Stage 0 evidence
correction, not silently deferred as complete.
