# Verification Agent A

Status: `PASS`.

Evidence mode: `[Static] + [Ran]`.

## Findings

No unresolved correctness, science-contract, real-consumer, numerical,
serialization, evidence-custody, or lifecycle finding remains.

During verification, the package-local Python bytecode cache reappeared and
the review-disposition status still described terminal gates as pending. Both
were reported rather than silently ignored. The cache was removed, and the
review disposition now explicitly distinguishes the reviewers' pre-terminal
holds from the subsequently passing terminal gates. A final check found no
`__pycache__` directory or `.pyc` file in the package.

## Contract, Implementation, And Real Consumer

The exact current diff remains consistent with `SC-SNOWFREEZE-001` revision
120. `INV-SNOWFREEZE-087`, `OBL-SNOWFREEZE-P-061`, and
`TOL-SNOWFREEZE-012` authorize a behavior-neutral, diagnostic-only density
ledger; require direct pre-mixing fresh-snow density, explicit process and cap
terms, typed non-finite and closure failures, and the real consumer; and bind
daily additive closure to `1e-9 kg m^-3` without authorizing state
canonicalization, calibration, or promotion.

Static reinspection confirmed that:

- `SnowDensityProcessDiagnostics` is returned by the selected density runtime,
  propagated through `SnowDensityRuntimeOutcome` and
  `DirectSnowLiquidPartition`, amended with the separate downstream Stage-3
  term, and serialized by the real runner path;
- the runner emits the `openwepp-r7h-direct-production-snow-trace-v2` schema
  and every governed `density_process_*` field;
- non-finite operands and increments fail closed, and closure beyond the
  governed tolerance reaches the typed `DiagnosticClosureViolation` path;
- wet and dry mutation arithmetic remains centralized rather than duplicated
  between state mutation and diagnostic attribution; and
- legacy and snow-free paths emit the explicit neutral/inapplicable record.

Fresh focused execution against the current tree passed:

- `cargo fmt --all -- --check`;
- orchestrator `snow_density`: 12/12;
- EB-04V integration contract and real-consumer binding: 2/2;
- HPHYS0296 trace-schema authority: 3/3;
- climate-class density regression: 5/5; and
- `git diff --check`.

## Exact Cohort And Numerical Evidence

The retained release binary still hashes to
`fb670d086937a7785a2549339832f71b96fc98f3c8992ec8d24961123b33826f`.
It matches the binary identity in execution receipt
`f2cc806de485cdbc00bc4c5b9e0e778ccb62fd6e1582d511fe5ea2b47f7fb1be`.
The receipt binds population
`9b6f7de4ab034228b4ce7e1b765f6f625200ad1bf1b52917486def3b6857abaa`,
nine lanes, 36 B/L/S/LS cells, and the `DIAGNOSTIC_ONLY` evidence role. All
36 retained executions returned zero.

The canonical result object hashes to
`5862917021f0e365f1ee30bfede54df4207a6040d88abfbf5e1692fd719a5ef8`
and binds analyzer
`e8c608af74b4cb9747ff16011484782edc612e6e7da97c604449c5daef21b3e9`.
It records:

- maximum independent density-ledger closure
  `3.410605131648481e-13 kg m^-3`;
- maximum emitted-versus-reconstructed difference `5.686e-14 kg m^-3`;
- omitted-overburden residual reaching `22.232508654103942 kg m^-3`, which
  rejects a plausible omitted-process alias;
- 100,824 rows in which direct fresh-snow density differs from final density;
- exact retained paired counts for all nine B cells; and
- maximum retained KGE-component difference
  `4.440892098500626e-16`.

The independently retained behavior-neutrality audit covers all 36 WAT tables
and 574,196 common JSONL rows. After excluding only the authorized schema
identifier and new diagnostic fields, every preexisting value equals EB-04R.
This supports the bounded observability claim without treating producer-only
self-consistency as acceptance evidence.

## Governing Regression Evidence

The broad-profile logs are intact and bind successful terminal runs from the
same production and test-source state:

| Profile | Run ID | Result | Retained log SHA-256 |
| --- | --- | --- | --- |
| Quick | `678ea8c3-8fb6-4b1b-baa7-c61995335996` | 2,139/2,139 passed; 36 skipped | `f45a5bd89cbecd65294d99b551bea78662f905ecfaa116ed2d8d110a40c7196e` |
| Frost | `79301982-08a9-46bb-be69-bc567d633b99` | 338/338 passed; 1,891 skipped | `3f283c13cc34f26794d5b124a2abe870abafd844a861a704e1e6067b5035b15c` |
| Full | `034af940-49b5-4aab-9627-9de1ee690c19` | 2,188/2,188 passed; 29 skipped | `f8543075cf6d5780f99610f1f42432c4b009f50ee4f7a538defbed6d66aa7fc6` |

The earlier interrupted quick attempt and the stale snow-energy revision
failure remain visible as invalid/nonterminal chronology. The exact terminal
quick rerun passed after the metadata-only revision correction; no assertion
failure was converted into a pass by deletion or threshold relaxation.

## Assurance, Documentation, And Lifecycle

Fresh assurance checks passed:

- `validate --all`: three admitted reports, all `DRAFT`, and zero public
  reports;
- `verify-generation --base-ref 15763d...`: generation
  `e94491a9710cc3a802fb69736dfc4d13d9bdb49564230f3f18043f19c69a7f04`
  with 35 anchored transitions;
- `check --all`: zero public reports and current zero-public outputs; and
- the independent full-catalog renderer check: 92 files current.

The terminal source-adoption receipt is
`a703b98e9d1a71bca8911e46ff2703abef64089470d65a2c3bb03fc5d4bea582`.
The snow review lock remains `DRAFT`; its approval, realization, and release-
transfer roots remain null. The rendered snow research object and build
manifest bind the revised canonical contract, while the approved public report
tree remains unchanged.

Markdown lint passed with zero errors and warnings for all 44 package Markdown
files and separately for `docs/ROADMAP.md`, the snow-surface campaign roadmap,
and the work-package catalog. The package prompt is archived as
`prompts/archived/execute.md`; `prompts/active/` contains only its README. The
package, root roadmap, campaign roadmap, catalog, final disposition, readiness
matrix, and worker handoff consistently make EB-04V diagnostic-complete with
efficacy held, preserve EB-04S nonpromotion/default-off, and identify EB-04W as
next.

The terminal diff reconciles to the declared write set. Root `Cargo.toml` adds
only the package-owned integration test. `Cargo.lock`, dependency declarations,
observation fixtures, authority-suite cohort/required-case bindings,
historical packages, public WAT/PASS schemas, and the public assurance catalog
are unchanged. The three generated assurance transactions and refreshed
review-only outputs are within the declared assurance adoption/render scope.

## Residual Risk And Missing Tests

No missing test blocks the package's behavior-neutral diagnostic claim. This
verification reused the retained quick, frost, and full-workspace runs rather
than rerunning several hours of exact, hash-bound evidence; it freshly reran
the focused implementation/consumer tests and independently checked assurance,
rendering, documentation, diff hygiene, prompt custody, and protected
surfaces.

The empirical result remains deliberately non-promotional. Existing
observations are consumed diagnostic evidence, the process terms remain
equifinal, and neither one coefficient nor an authority-backed corrective
process is identified. The 1,990-line density module has only ten lines of
headroom, and the repeated literal contract-revision bindings remain bounded
maintenance debt; the named maintenance follow-ups are appropriate and do not
invalidate this package.

## Approval

Verification A approves EB-04V as
`DIAGNOSTIC_COMPLETE / EFFICACY_HOLD / NO_PROMOTION`. No blocker remains for
this verification lane. EB-04W may proceed under its prospectively sealed
diagnostic scope; EB-04V does not authorize density fitting, physics amendment,
selector/default change, or promotion.
