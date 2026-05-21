# INIMPL17 Wave 2 Gate Evidence

Evidence mode: `Ran` + `Static`

## 1. Intake and Readiness Commands (`Ran`)

- `for i in 11 12 13 14 15 16; do ... check required artifact files in worker worktrees ...; done`
- `ls -1 /home/workdir/openWEPP/.worktrees`
- `git -C /home/workdir/openWEPP/.worktrees/inimplXX-* status --short`

Result:
- Required worker artifact bundles are present for all `INIMPL11..16`.
- All Wave 2 worker worktrees are present.

## 2. Integration Commands (`Ran`)

- Worker stream commits created in each worktree (`INIMPL11..16`) and then cherry-picked in canonical order onto `main`:
  - `git cherry-pick 47c27bc`
  - `git cherry-pick ab650c3`
  - `git cherry-pick 5b9a578` (conflict resolved + `cherry-pick --continue`)
  - `git cherry-pick dcf8784` (conflict resolved + `cherry-pick --continue`)
  - `git cherry-pick 977c3d4`
  - `git cherry-pick 2e63b42`

## 3. Wave 2 Global Gates (`Ran`)

| Gate | Status | Notes |
| --- | --- | --- |
| `cargo fmt --check` | pass | Initial failure on `parsers/mod.rs` ordering; fixed via `rustfmt`, then pass. |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass | No warnings/errors after integration. |
| `cargo test --workspace` | pass | Registered test targets pass. |
| `cargo deny check` | pass | Non-fatal `license-not-encountered` warnings only. |

## 4. Sidecar Acceptance Checks (`Ran`)

Because these six new test targets are not yet declared in root `Cargo.toml`,
they were executed directly via `rustc --test`:

| Surface | Test target file | Result |
| --- | --- | --- |
| `SC-INFILE-PMETPARA-001` | `infile_pmetpara_parser_contract.rs` | pass (13) |
| `SC-INFILE-IRRIGATION-DEPLETION-001` | `infile_irrigation_depletion_parser_contract.rs` | pass (12) |
| `SC-INFILE-IRRIGATION-FIXEDDATE-001` | `infile_irrigation_fixeddate_parser_contract.rs` | pass (14) |
| `SC-INFILE-FROST-001` | `infile_frost_parser_contract.rs` | pass (10) |
| `SC-INFILE-SNOW-001` | `infile_snow_parser_contract.rs` | pass (12) |
| `SC-INFILE-WEPPUI-001` | `infile_weppui_parser_contract.rs` | pass (11) |

Note:
- Frost/snow/weppui test compilation required `CARGO_MANIFEST_DIR=/home/workdir/openWEPP`
  during direct `rustc --test` execution due `env!` usage in test code.

## 5. Verdict

`GO-WITH-AMENDMENTS`

Amendment:
1. Add root `Cargo.toml` integration test registrations for Wave 2 sidecar test
   targets so workspace test gate captures them automatically.
