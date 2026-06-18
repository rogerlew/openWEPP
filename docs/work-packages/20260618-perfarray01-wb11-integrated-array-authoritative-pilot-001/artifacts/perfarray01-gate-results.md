# PERFARRAY01 Gate Results

Evidence class: Ran + Static + Not run.

Status: executed-no-go 2026-06-18.

## Gate Summary

Stage A landed and passed focused and workspace Rust gates. Stage B did not
advance to H2637 timing or bit-identity execution because static inspection
showed the existing WB11 request and scheduler seams cannot satisfy the
package's two structural proofs without a narrower authority split first.

This is a package-level NO-GO for the integrated WB11 array pilot as scoped,
not a failed Rust gate.

## Ran

- `cargo fmt --check -p openwepp-kernel-contract`: pass.
- `cargo check -p openwepp-kernel-contract`: pass.
- `cargo test -p openwepp-kernel-contract`: pass, 27 tests passed.
- `cargo clippy -p openwepp-kernel-contract --all-targets -- -D warnings`:
  pass after fixing local clippy findings.
- `cargo fmt --check`: pass.
- `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260618-perfarray01-wb11-integrated-array-authoritative-pilot-001 --format json`:
  pass, 20 files scanned, 0 errors, 0 warnings.
- `git diff --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass, including workspace unit tests,
  integration tests, binary tests, and doc tests.
- `cargo deny check`: pass, `advisories ok, bans ok, licenses ok, sources ok`.

## Static

- Default production execution remains unwired from `ArrayHotState`; the new
  module is exported from `openwepp-kernel-contract`, but scheduler and kernel
  request construction continue to use the existing logical surfaces.
- Stage A tests cover logical round-trip export, accept parity with
  `evaluate_kernel_writeback`, reject message-class and subject parity, and
  apply/export identity against logical writeback apply.
- Stage B was blocked by current request and scheduler authority:
  `HillslopeKernelRequest` still carries logical `BTreeMap` state and flux
  surfaces for normal kernel reads; `execute_with_kernel_indexed` still
  validates consumer boundaries against logical writeback state, applies
  accepted writeback to logical maps, then mirrors into the indexed surface.

## Not Run

- H2637 no-UI array-pilot timing: not run. No valid Stage B path existed that
  satisfied both "no per-day full `BTreeMap` export" and "no normal-path
  logical + array dual-write".
- Stage B HBP / loss / wat / plot / pass-parquet bit-identity: not run for the
  same reason.
- Stage B perf-backed structural proof: not run as a timing claim. The package
  disposition relies on static structural evidence that the current seam would
  necessarily time an invalid pilot path.

## Disposition

PERFARRAY01 closes as NO-GO for Stage B as scoped. ADR-0023 is not ready for
ratification from this package. The follow-on should first split WB11 request
and accessor authority so a real array-authoritative pilot can run without
per-day logical export or normal-path dual-write.
