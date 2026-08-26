# Gate results

Status: PASS / qualified against the historical-eleven baseline.

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

## Exact-workspace qualification

Ran first committed candidate `aadc4109e`, clean and equal to `origin/main`:
full-profile Nextest ran 3,337 tests; 3,325 passed, 12 failed, and 34 were
configured skips. Eleven failure names/signatures matched the retained
historical census exactly. The sole delta was
`all_owner_failure_rolls_back_and_index_records_lifecycle`, whose stale v28
registry-note assertion was introduced by this package's canonical v29
amendment. This result was nonqualifying and the assertion was reconciled
without changing receiver authority or behavior.

Ran terminal implementation candidate
`8f4a9b84cdaa953562b02e9aca98be248289ea14`, clean and equal to
`origin/main` before and after execution:

`nix develop --command cargo nextest run --workspace --profile full --no-fail-fast`

Result: 3,337 run; 3,326 passed (3 slow); exactly 11 failed; 34 configured
skips. The failure-name and normalized-signature sets are identical to all
eleven entries in the immediately preceding workspace-baseline census. The V9
test passes on the ordinary `.4` host without overlay. Nonzero exit 100 is due
only to the deliberately visible historical eleven and introduces no waiver,
skip, expected-failure annotation, or reclassification. Complete external log:
`/tmp/openwepp-batch-logs/nextest-full-workspace-8f4a9b84c-20260826T145140.log`.

The lifecycle correction plus vegetation integration rerun passed 37/37,
Nextest `1449ff8e-9124-4d1d-a24b-d5c02afe25bd`.
