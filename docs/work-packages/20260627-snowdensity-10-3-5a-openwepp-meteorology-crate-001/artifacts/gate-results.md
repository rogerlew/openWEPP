# Gate Results

Status: complete
Evidence mode: Ran/Static

| Gate | Status | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | Ran: workspace format check passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran: workspace clippy passed after fixing the package contract-test formatting lint. |
| `cargo test --workspace` | PASS | Ran: workspace tests and doc-tests passed after updating stale SNOWDENSITY contract-version guard markers to v91. |
| `cargo deny check` | PASS | Ran: advisories, bans, licenses, and sources checks passed. |
| Clean-room provenance complete | PASS | Static: `clean-room-provenance.md` maps implemented equations/constants to source authority and records no CHM/GPL code use. |
| No-production-wiring scan complete | PASS | Ran/Static: no production runtime/output/parser/default files changed, production source scans found no `Harder`, `Pomeroy`, `psychrometric`, or `openwepp-meteorology` references, and cargo metadata reported no production dependency on `openwepp-meteorology`. |
| Review findings disposition complete | PASS | Static: `review_agent_a.md`, `review_agent_b.md`, and `review-disposition.md` completed with accepted/fixed internal findings and no unresolved blocker. |
| Verification complete | PASS | Static/Ran: verification artifacts confirm contract-first order, production isolation, clean-room evidence, and green gates. |

## Gate Notes

- Ran: `cargo test -p openwepp-meteorology` passed all crate-local tests.
- Ran: `cargo test --test snowdensity10_3_5a_meteorology_crate_contract`
  passed the contract marker and no-production-dependency checks.
- Ran: `cargo clippy -p openwepp-meteorology --all-targets -- -D warnings`
  passed before the workspace clippy gate.
- Ran: affected older SNOWDENSITY contract guard tests passed after the v91
  marker update.
