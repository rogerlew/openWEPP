# Terminal diagnostic correlation V3 candidate manifest

Status: `REVIEWED HOLD / SOURCE EDITS PROHIBITED`

Base: `8b2a7fe1789fb06386110fb5d6e3bc5fd2f7d962`

| Surface | SHA-256 |
|---|---|
| `terminal-rejected-trial-diagnostic-correlation-authority-v3.md` | `5f9c0d66c13e3b11b921822c114877d1a61f233ae9e20c1164c21c9440c544e4` |
| `terminal-diagnostic-correlation-v3-adapter-schema-manifest.md` | `5209dd6b80c54563755d92ced9ce367df4708053ba9e4a28c4b23882064075db` |
| preserved V2 authority | `f4a7ff15127fdfd5068f16126f440a57a25026b44a5c610f175dfab30417cc5c` |

Ran from the exact dirty authority-document candidate tree:

- `nix develop --command cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- `nix develop --command cargo nextest run --test snow_stage3_terminal_batch_temporal_v20_contract --test snow_stage3_terminal_batch_temporal_v21_contract`:
  PASS, run `6d7d06c9-8447-4141-9375-c67e29de08a3`, 5/5 tests.

Static: `git diff --name-only -- '*.rs' Cargo.toml Cargo.lock` was empty.
No source, Cargo registration, production contract candidate or V2 authority
changed. Two independent reviews must verify both V3 hashes above. Either HOLD
stops before an exact-file implementation intent or source edit.

Review result: numerical/evidence/cardinality `HOLD`; Rust/custody/API/
compilation-boundary `HOLD`. The two-GO gate failed. See
`terminal-diagnostic-correlation-v3-review-disposition.md`.
