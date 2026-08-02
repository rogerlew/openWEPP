# Gate Results

Status: `PASS`.

Evidence mode: `[Ran]`.

## Focused implementation and contract gates

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS. |
| warnings-denied Clippy for orchestrator and runner, all targets | PASS. |
| orchestrator `snow_density` selection | PASS, 12/12. |
| EB-04V integration contract | PASS, 2/2. |
| HPHYS0296 trace-schema contract | PASS, 3/3. |
| climate-class integration regression | PASS, 5/5. |
| `bash tools/release/check_unit_registry.sh` | PASS, 21/21 registry tests plus workspace check. |
| `cargo deny check` | PASS: advisories, bans, licenses, and sources; pre-existing unused `MIT-0` allowance warning only. |
| stale contract revision scan | PASS: zero `contract_version: 119` bindings remain in `tests/integration/`. |

## Exact release cohort and consumer proof

- Release build: PASS; binary
  `fb670d086937a7785a2549339832f71b96fc98f3c8992ec8d24961123b33826f`.
- Frozen population: PASS, nine lanes and 36/36 B/L/S/LS cells.
- Independent ledger closure: PASS; maximum `3.411e-13 kg m^-3` against
  `1e-9 kg m^-3`.
- Emitted/reconstructed difference: PASS; maximum `5.686e-14 kg m^-3`.
- Anti-tautology: PASS; omitted overburden reaches `22.233 kg m^-3` and
  100,824 fresh-density rows differ from final density.
- Behavior neutrality: PASS; all 36 WAT tables and every pre-existing JSONL
  field are value-identical to EB-04R across 574,196 rows.
- Frozen observation operator: PASS; all nine B-cell paired counts are exact
  and maximum KGE-component difference is `4.441e-16`.
- Figures: PASS; five SVGs parse, were visually inspected without material
  overlap/clipping, and have same-stem Markdown sidecars.

The exact model execution passed 36/36, then its first analysis invocation
failed before emitting a result because the predecessor rubric container was
indexed one level too high. The corrected analysis-only invocation consumed
the unchanged hash-bound receipt/run tree and passed. This is retained
failure/recovery chronology, not a model rerun.

## Broad profiles

- First quick attempt: invalid infrastructure evidence. At `372.928 s`, an
  external SIGINT cancelled three still-running assurance tests after 157 had
  passed; no assertion failed and 1,979 were not run.
- Complete pre-fix quick: 2,138 passed, one failed, 36 skipped in
  `2258.153 s`. The sole failure was the already-stale
  `SC-SNOWENERGY-001` revision-5 assertion in the EB-03 contract test; no
  runtime/science assertion failed. The marker was advanced to canonical
  revision 6. Focused confirmation then passed 11/11, run
  `e522961b-1a0c-49f5-a4df-fbe0b2f031ff`.
- Frost: PASS; run `79301982-08a9-46bb-be69-bc567d633b99`, 338/338 passed,
  1,891 skipped, `525.965 s` test time (`527.52 s` wall).
- Terminal quick: PASS; run `678ea8c3-8fb6-4b1b-baa7-c61995335996`,
  2,139/2,139 passed, 36 skipped, `2236.029 s` test time
  (`2237.23 s` wall).
- Critical full: PASS; run `034af940-49b5-4aab-9627-9de1ee690c19`,
  2,188/2,188 passed, 29 skipped, `2210.445 s` test time
  (`2211.94 s` wall).

## Assurance, documentation, and lifecycle

- Typed source adoption: PASS; latest transaction
  `a703b98e9d1a71bca8911e46ff2703abef64089470d65a2c3bb03fc5d4bea582`,
  generation `e94491a9710cc3a802fb69736dfc4d13d9bdb49564230f3f18043f19c69a7f04`.
- Full-catalog review rendering: PASS; 92 files applied and independently
  checked current.
- Assurance `validate --all`: PASS; three DRAFT sources and zero public
  reports.
- Markdown, exact diff, prompt archive, and final lifecycle disposition: PASS.
- Dual terminal verification: PASS/PASS against the exact closed tree;
  `verification-agent-a.md` and `verification-agent-b.md` report no unresolved
  implementation, science, evidence, or lifecycle finding.

Authority-suite anti-evasion guards were not selected because no authority
suite posture, fixture, cohort binding, or required-case association changed.
