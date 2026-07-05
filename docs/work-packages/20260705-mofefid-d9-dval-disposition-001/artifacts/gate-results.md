# Gate Results

Status: passed
Evidence mode: Static + Ran

## Focused D-val / Taxonomy

| Gate | Runner | Result | Evidence |
|---|---|---|---|
| Case 1 comparator | subagent `comparator_suite_runner` / Lagrange | PASS | `artifacts/dval-0-20260705-153308.log` |
| Case 2 comparator | subagent `comparator_suite_runner` / Lagrange | PASS | `artifacts/dval-0-20260705-153312.log` |
| Case 3 comparator | subagent `comparator_suite_runner` / Lagrange | PASS | `artifacts/dval-0-20260705-153313.log` |
| Case 2 `Ks=10` sensitivity | subagent `comparator_suite_runner` / Lagrange | PASS | `artifacts/dval-10-20260705-153314.log` |
| Focused Rust D-val test | subagent `comparator_suite_runner` / Lagrange | PASS | `artifacts/nextest-dval-case2-underprediction.log` |
| Zone taxonomy harness | parent | PASS | `artifacts/zone-taxonomy-20260705-1545.json` |

## Required Closure Gates

| Gate | Runner | Result | Evidence |
|---|---|---|---|
| `git diff --check` | parent + review subagents | PASS | no output |
| Markdown lint / doc checks for touched docs | parent | PASS | `markdown-doc lint --path ... --format plain` -> `27 files validated, 0 errors, 0 warnings` |
| Contract BEI check for D9-changed surface | parent | PASS | `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> `PASS-DEFERRED` globally because 3 pre-existing Lane D rows remain `science-review-follow-on`; D9 changed only narrative/evidence text for the existing `OFEROUTE-ACTIVATION-VALIDATION` row and did not add, remove, hide, or reclassify binding IDs. Strict BEI consolidation is not a current D9 exit criterion because D9 claims no production binding consolidation or activation. |
| Contract unit compliance | parent | PASS | `.venv/bin/python tools/release/check_sc_unit_compliance.py --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` -> no findings |
| Focused Python compile for taxonomy harness | parent | PASS | `.venv/bin/python -m py_compile tools/dval/zone_taxonomy.py` |
| Focused taxonomy rerun after review fix | parent | PASS | `.venv/bin/python tools/dval/zone_taxonomy.py --fig9 .../Figure_9.xlsx` with `Psi*` assertion |
| Focused Rust D-val filter | parent | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator dval` -> `6 passed, 273 skipped` |
| `cargo fmt --check` | parent | PASS | no output |
| `cargo clippy --workspace --all-targets -- -D warnings` | parent | PASS | finished dev profile in `12.81s`; no warnings/errors |
| `cargo nextest run --workspace --profile full` | subagent `comparator_suite_runner` / Bohr | PASS | `artifacts/cargo-nextest-full.log`; elapsed `589.508s`; summary `1363 tests run: 1363 passed (2 slow), 1 skipped` |
| `cargo deny check` | parent | PASS | `advisories ok, bans ok, licenses ok, sources ok` |
| Source-level anti-evasion guards | parent | PASS | Trigger false: D9 did not touch external-authority suite posture, cohort fixtures, or required-case bindings; no anti-evasion gate required by package condition |

## BEI Current-Scope Legitimacy

Static:

- Package gate wording is "Contract/profile/BEI checks for changed
  `SC-OFEROUTE-001` surfaces." D9 changed validation evidence/disposition text
  for `INV-OFEROUTE-011`, the activation-validation BEI note, and the
  `GAP-OFEROUTE-005` handoff. It did not claim full BEI consolidation,
  production binding readiness, endpoint activation, or consumer cutover.
- `tools/check_sc_binding_exposure.py` non-strict returned `PASS-DEFERRED`
  because existing Lane D BEI rows remain intentionally routed to
  `science-review-follow-on`. Those rows predate D9 and represent unpromoted
  solver/cascade/activation bindings that D9 is not authorized to consolidate.
- Treating strict BEI consolidation as a D9 closure requirement would force
  D9 to reclassify production-binding posture outside its objective and
  protected boundaries. The current-scope gate is therefore satisfied by the
  non-strict BEI lint plus this artifact-level legitimacy audit.
