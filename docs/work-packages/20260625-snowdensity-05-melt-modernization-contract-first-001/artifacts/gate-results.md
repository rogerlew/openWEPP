# Gate Results

Status: complete.
Evidence mode: Ran.

Scoped 05A gates:

| Command | Result | Notes |
|---|---|---|
| `cargo test --test snowdensity05a_melt_contract_guard` before amendment | failed as expected | Missing v76 contract, signed-`bmelt` language, and package closeout. |
| `cargo fmt --check` | pass | Ran after `cargo fmt` corrected new-test wrapping. |
| `cargo test --test snowdensity05a_melt_contract_guard` | pass | 3 passed. |
| `cargo test --test snowdensity02_contract_adr_guard` | pass | 3 passed after header-version marker update. |
| `cargo clippy --test snowdensity05a_melt_contract_guard -- -D warnings` | pass | Focused clippy for the new integration target. |
| `wctl doc-lint --path docs/work-packages/README.md` | pass | 1 file, 0 errors, 0 warnings. |
| `wctl doc-lint --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | pass | Tool reported 0 files validated, 0 errors, 0 warnings for this contract path. |
| `git diff --check` | pass | No whitespace errors. |

Not run:

- `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
  and `cargo deny check` were not run because SNOWDENSITY-05A changed contract,
  docs, artifacts, Cargo test registration, and a guard test only. No
  production runtime code or dependencies changed.
