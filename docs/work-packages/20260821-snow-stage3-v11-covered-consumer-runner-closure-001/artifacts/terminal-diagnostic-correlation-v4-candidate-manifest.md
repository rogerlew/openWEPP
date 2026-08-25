# Terminal diagnostic correlation V4 candidate manifest

Status: `REVIEWED HOLD / SOURCE EDITS PROHIBITED`

Base: `ba7a9bfb42e49d8f0dd7db8084726a8c11e7f22a`.

| Frozen surface | SHA-256 |
|---|---|
| V4 authority | `1b0b85661fba67e9161fca2bda2e2e5cc8ca7d507f9eeceed6c93af7289aeda1` |
| V4 adapter schema | `0ea6d5449b401860a9d10336b980adfab90102087488f67ba54ec032b61f1edc` |
| generated live-type census | `e1ade576923c0267eb139e8b5e5fcd4930218737c1a98ca032f09e42ecd9b255` |
| generator Cargo.toml | `373dd0ee0aa2ea6b7d7474afb8a4ff47c03a540da55f1cdaa1e8a7f9cf8ea113` |
| generator Cargo.lock | `6891f3a17daf0aaf37f47ad50dec7bc4b693c15b2a7201f8463fbf83ba316b0f` |
| generator source | `07175588c66b4a27ecf7dcb1449050e544fcb577351f95e42d653bc638fa9e58` |
| generated-schema guard | `a8408a5a96c2eee65c5f6983152d591cbbbf5a4f45b5c70ba5392c27d1755357` |

Ran:

- generated census guard: PASS;
- `nix develop --command cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS;
- V20/V21 historical/structural nextest command with `--no-fail-fast`: PASS,
  run `3fabf7e8-9e34-4cfa-b3f2-208e72d7edeb`, 5/5.

No production Rust, workspace Cargo file, V3 artifact or physical behavior was
changed. Both reviews must verify every frozen hash. Either HOLD stops before
source edits; two GO reviews authorize only a subsequent exact-file intent.

Review result: numerical/evidence/cardinality `HOLD`; Rust/schema/custody/
privacy `HOLD`. See `terminal-diagnostic-correlation-v4-review-disposition.md`.
