# Child-2 Comparator and Gate Results

Status: PASS

- `20260814-vegetation-real-hydrology-arbitration-shadow-001` comparator log bundle:
  - `/tmp/openwepp-child2-comparator-20260814-111850`
- All commands were executed exactly once; no retries were required.

## Commands Executed

1. `cargo check -p openwepp-kernel-contract`
   - PASS (0)
   - log: `/tmp/openwepp-child2-comparator-20260814-111850/check-kernel-contract.log`

2. `cargo check -p openwepp-vegetation`
   - PASS (0)
   - log: `/tmp/openwepp-child2-comparator-20260814-111850/check-vegetation.log`

3. `cargo check -p openwepp-biogeochemistry`
   - PASS (0)
   - log: `/tmp/openwepp-child2-comparator-20260814-111850/check-biogeochemistry.log`

4. `cargo check -p openwepp-hillslope-orchestrator`
   - PASS (0)
   - log: `/tmp/openwepp-child2-comparator-20260814-111850/check-hillslope-orchestrator.log`

5. `cargo clippy -p openwepp-kernel-contract --all-targets -- -D warnings`
   - PASS (0)
   - log: `/tmp/openwepp-child2-comparator-20260814-111850/clippy-kernel-contract.log`

6. `cargo clippy -p openwepp-vegetation --all-targets -- -D warnings`
   - PASS (0)
   - log: `/tmp/openwepp-child2-comparator-20260814-111850/clippy-vegetation.log`

7. `cargo clippy -p openwepp-biogeochemistry --all-targets -- -D warnings`
   - PASS (0)
   - log: `/tmp/openwepp-child2-comparator-20260814-111850/clippy-biogeochemistry.log`

8. `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
   - PASS (0)
   - log: `/tmp/openwepp-child2-comparator-20260814-111850/clippy-hillslope-orchestrator.log`

9. `cargo nextest run --test c3_vegetation_implementation_contract --profile quick`
   - PASS (17/17 tests)
   - log: `/tmp/openwepp-child2-comparator-20260814-111850/nextest-c3-contract.log`

10. `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`
    - PASS (26/26 tests)
    - log: `/tmp/openwepp-child2-comparator-20260814-111850/nextest-boundary-authority.log`

11. `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
    - PASS (3/3 tests)
    - log: `/tmp/openwepp-child2-comparator-20260814-111850/nextest-auth11.log`

12. `cargo nextest run --test vegetation_real_hydrology_shadow_contract --profile quick`
    - PASS (3/3 tests)
    - log: `/tmp/openwepp-child2-comparator-20260814-111850/nextest-veg-real-hydrology.log`

13. `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick`
    - PASS (505/505 tests)
    - log: `/tmp/openwepp-child2-comparator-20260814-111850/nextest-hillslope.log`

14. `bash tools/release/check_authority_suite_antievasion.sh`
    - PASS
    - log: `/tmp/openwepp-child2-comparator-20260814-111850/check-authority-antievasion.log`

15. `bash tools/release/check_science_contract_admission.sh --base-ref 0db196012 --worktree`
    - PASS
    - summary: `A0_ADMITTED contracts=45 science_surfaces=8 head=WORKTREE`
    - log: `/tmp/openwepp-child2-comparator-20260814-111850/check-science-admission.log`

16. `cargo fmt --all -- --check`
    - PASS (0)
    - log: `/tmp/openwepp-child2-comparator-20260814-111850/fmt-check.log`

17. `git diff --check`
    - PASS (0)
    - log: `/tmp/openwepp-child2-comparator-20260814-111850/git-diff-check.log`

18. `markdown-doc lint --path docs/work-packages/20260814-vegetation-real-hydrology-arbitration-shadow-001`
    - PASS
    - summary: `20 files validated, 0 errors, 0 warnings`
    - log: `/tmp/openwepp-child2-comparator-20260814-111850/markdown-doc-lint.log`

## Overall

- Result: PASS
- Command failures: 0
- Preserved command logs and exit codes in the comparator bundle.
## Final comparator rerun (timestamped, preserving prior results)

- Timestamp (UTC): 2026-08-14T18:39:38Z
- Head sha: 3f1cf8ee32855a501d7d5b07ac3459d8a3fc8cc3
- Base: 0db196012
- Log bundle: /tmp/openwepp-child2-comparator-20260814-20260814-113603
- Result: PASS (all required commands exit 0, no retries)

### Command outcomes

- `cargo check -p openwepp-kernel-contract` => 0
- `cargo check -p openwepp-vegetation` => 0
- `cargo check -p openwepp-biogeochemistry` => 0
- `cargo check -p openwepp-hillslope-orchestrator` => 0
- `cargo clippy -p openwepp-kernel-contract --all-targets -- -D warnings` => 0
- `cargo clippy -p openwepp-vegetation --all-targets -- -D warnings` => 0
- `cargo clippy -p openwepp-biogeochemistry --all-targets -- -D warnings` => 0
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` => 0
- `cargo nextest run --test c3_vegetation_implementation_contract --profile quick` => 0
- `cargo nextest run --test vegetation_boundary_authority_contract --profile quick` => 0
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` => 0
- `cargo nextest run --test vegetation_real_hydrology_shadow_contract --profile quick` => 0
- `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick` => 0
- `bash tools/release/check_authority_suite_antievasion.sh` => 0
- `bash tools/release/check_science_contract_admission.sh --base-ref 0db196012 --worktree` => 0
- `cargo fmt --all -- --check` => 0
- `git diff --check` => 0
- `markdown-doc lint --path docs/work-packages/20260814-vegetation-real-hydrology-arbitration-shadow-001` => 0

### Focused command outcomes

- `c3_vegetation_implementation_contract`: 17 tests run, 17 passed, 0 skipped
- `vegetation_boundary_authority_contract`: 26 tests run, 26 passed, 0 skipped
- `auth11_required_suite_obligation_guards_contract`: 3 tests run, 3 passed, 0 skipped
- `vegetation_real_hydrology_shadow_contract`: 3 tests run, 3 passed, 0 skipped
- `openwepp-hillslope-orchestrator` profile quick: 507 tests run, 507 passed, 0 skipped (3 slow)
- `check_science_contract_admission`: A0_ADMITTED contracts=45 science_surfaces=9
- authority_sha256=ac829c7b73c92022e269823a2f88c3329efcc4785e4c8cd10caef6dfb455e5af
- `markdown-doc lint`: 22 files, 0 errors, 0 warnings
