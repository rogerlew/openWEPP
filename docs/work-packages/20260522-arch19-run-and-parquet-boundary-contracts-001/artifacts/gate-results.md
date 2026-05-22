# Gate Results

Static: artifact completeness and source-reference validation only.
Ran: docs-only validation commands.
Status: `complete`.

## Executed Commands

1. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/run-boundary-contract-authority.md`
- result: pass

2. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/parquet-boundary-contract-authority.md`
- result: pass

3. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/wepppyo3-parquet-schema-reference-inventory.md`
- result: pass

4. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/run-parquet-cross-file-closure-map.md`
- result: pass

5. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/arch19-follow-on-acceptance-criteria.md`
- result: pass

6. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/worker-handoff.md`
- result: pass

7. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/owned-file-manifest.md`
- result: pass

8. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/gate-results.md`
- result: pass

9. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/arch19_disposition.md`
- result: pass

10. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/review_agent_a.md`
- result: pass

11. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/review_agent_b.md`
- result: pass

12. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/verification_agent_a.md`
- result: pass

13. `test -f docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/verification_agent_b.md`
- result: pass

14. `rg -n "RUN-HOLD|PRQ-HOLD|XMAP-HOLD" docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts`
- result: pass (hold items explicitly surfaced)

15. `rg -n "finding_id|A-001|B-001" docs/work-packages/20260522-arch19-run-and-parquet-boundary-contracts-001/artifacts/arch19_disposition.md`
- result: pass (disposition rows present for all review findings)

## Non-Executed Gates

Rust gates were not run because ARCH19 execution is docs-only and no code files
were modified:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
