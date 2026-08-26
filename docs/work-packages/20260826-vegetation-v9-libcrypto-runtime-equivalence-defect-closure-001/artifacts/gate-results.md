# Gate results

Status: focused gates pass; exact-workspace gate pending committed candidate.

Ran:

- direct current-provider verifier: PASS, exact frozen output hash;
- historical `.3` exact-host verifier under read-only overlay: PASS, exact
  frozen output hash;
- five-case end-to-end poison population plus clean baseline: PASS, all
  poisons rejected at their intended checks;
- `cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS;
- science-contract admission against base `f1b0ff9c`: `A0_ADMITTED`, 49
  contracts, zero production science surfaces, authority SHA-256
  `f30c12aeea57ab4ac24f1c7883f7ce4883d43f916067bca8b80463683e7dfd6`;
- authority anti-evasion script: PASS;
- owning vegetation integration binary: 28/28 PASS, Nextest
  `da356d53-b633-4df4-bf46-c85144b36dc5`;
- AUTH11 required-suite guards: 3/3 PASS, Nextest
  `f7157e86-60cc-483d-a29e-2464d61fad07`; and
- affected integration-target Clippy with `--no-deps -D warnings`: PASS.

Ran diagnostic: broad workspace `cargo clippy --workspace --all-targets -- -D
warnings` reached unrelated pre-existing failures in
`openwepp-biogeochemistry`, `openwepp-coupled-time`, and
`openwepp-land-surface-energy`. None is in the declared write set or dependency
mechanism. The package-selected affected Rust surface passes warnings-denied;
the critical global correctness obligation is the pending full-workspace
Nextest run.
