# Disposition

Status: executed-hold
Evidence mode: Static + Ran

Result:
- HPHYS0287 closes the fail-open canonicalize-and-proceed bug for material invalid projected runtime snow state before inactive snow fallback and same-pass/runoff partition.
- HPHYS0287 now also fails closed on partial projected snow-state vectors when any snow option/control/runtime state is present.
- HPHYS0287 is guard hardening, not valid-run snow-magnitude parity progress.
- HPHYS0287 does not close valid-run H1/H7/H39 snow liquid retention/runoff/infiltration residuals; selected semantic metrics are unchanged from HPHYS0286.

Ran:
- Focused HPHYS0287 test: pass, 7 tests.
- Adjacent HPHYS0284/0285/0286 and clim06 tests: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass with existing warnings.
- Full H1..H39 release semantic suite: runtime `39/39`, semantic reports `39/39`, semantic pass `0/39`; root `/tmp/hphys0287_full_release_after_review_20260604T221027Z`.

Hold rationale:
- Valid-run snow liquid retention/release and runoff/infiltration magnitude remain open.
- `Q`/`RM`/`Snow-Water` metrics have not moved across HPHYS0284 through HPHYS0287; the next package should not route around the magnitude defect again.
- Global SC unit compliance remains open with 219 findings.

Continuation recommendation:
- Scaffold the next package for baseline-authoritative `winter.for` liquid retention/release and `runoff.for` melt/rain partition magnitude, anchored on H1/H7/H39 and preserving this fail-closed snow-state guard.
