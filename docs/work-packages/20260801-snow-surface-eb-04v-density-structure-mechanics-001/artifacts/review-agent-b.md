# Independent Science / QA Review B

Evidence class: `Static + independently reconstructed Ran + Reused Ran`.

Verdict: `PASS_CURRENT_IMPLEMENTATION / HOLD_TERMINAL_CLOSURE_GATES`.

## Scope Reviewed

Independently reviewed the EB-04V package contract and write set; revision 120
of `SC-SNOWFREEZE-001`; the density runtime, downstream Stage-3 handoff, typed
partition carrier, and real JSONL consumer; contract and unit tests; the
nine-lane population freeze; all three generations of cohort evidence; the
terminal execution receipt and retained target outputs; the analysis program,
machine-readable results, scientific synthesis, calibration-readiness matrix,
five figures and their same-stem sidecars; behavior-neutral comparisons; line-
count governance; and gates available at review time. Reviewer A's artifact was
not used to form this review.

## Finding Chronology And Disposition

### EB04V-B-001 — First-run multilayer cap attribution was not exact in bulk-density space

Severity: `major science / closure`; disposition: `accepted and fixed`.

The first implementation projected capped local-layer density changes into an
aggregate ledger in a way that could close arithmetically without representing
the actual uncapped bulk-density contribution. That evidence was invalidated
and retained under `artifacts/invalidated-pre-review/`. The terminal
implementation now reconstructs the uncapped bulk response at each layer
mutation, assigns PTM and POC by their same-state raw-tendency shares, and
records realized-minus-uncapped movement as the separate internal-cap term.
The independent two-layer analytical vector and the real-cohort closure both
pass.

### EB04V-B-002 — Inapplicable, non-finite, and failed-closure states were insufficiently guarded

Severity: `major correctness`; disposition: `accepted and fixed`.

The initial ledger could be mutated by downstream Stage 3 after the density
model declared it inapplicable, and closure was observational rather than a
typed runtime failure. The terminal implementation returns a neutral default
ledger for legacy and snow-free states, makes Stage-3 adjustment a no-op for an
inapplicable ledger, validates every dimensional diagnostic field as finite,
and maps closure failure through the typed WB11 guard. Focused tests cover the
neutral states, non-finite driver and Stage-3 inputs, omitted terms, structural
projection, fallback, internal and runtime caps, and downstream adjustment.

### EB04V-B-003 — The pre-terminal analysis changed the frozen density-pairing operator

Severity: `major science / anti-leakage`; disposition: `accepted and fixed`.

The pre-terminal analyzer dropped observed-snow dates on which the model had no
snow. This changed the retained paired counts and reversed the apparent beta
direction in four of five SNOTEL lanes while continuing to label the lanes with
their EB-04U under-beta partition. That cohort was explicitly invalidated and
retained under `artifacts/pre-terminal-finite-guard/`; its result-bearing target
tree is preserved separately.

The terminal analyzer hash-binds the EB-04R result that defines the observation
operator, retains model-no-snow dates at modeled density `0 kg m^-3`, and fails
if any B-cell paired count or KGE component drifts. All nine terminal anchors
pass: paired counts are exact and the maximum absolute KGE-component difference
is `4.441e-16`. The over-/under-beta group labels are again consistent with the
reported B-cell beta values.

### EB04V-B-004 — The terminal execution initially exposed an analyzer-schema defect

Severity: `moderate evidence tooling`; disposition: `accepted and fixed without
model rerun`.

The exact terminal binary completed all 36 cells, but the first analysis pass
treated `rubric_profile` as a list instead of reading its `cells` member. No
scientific result was emitted. The parser was corrected and an analysis-only
pass consumed the unchanged hash-bound receipt and run tree. The terminal
results bind analysis tool `e8c608af...b3e9`, execution receipt
`f2cc806d...b1be`, population freeze `9b6f7de4...abaa`, and release binary
`fb670d08...26f`; all current hashes match.

### EB04V-B-005 — Figure and evidence explanations omitted material context

Severity: `moderate human-review ergonomics`; disposition: `accepted and
fixed`.

The signed-effects sidecar originally omitted the plotted internal cap, and the
association/KGE sidecars did not explain repeated B/L/S/LS observations or the
model-no-snow pairing rule. The generated sidecars now name the internal cap,
identify association rows as applicable modeled-snow rows, state that the four
factorial cells are not independent observation replicates, and explain the
frozen zero-density pairing rule. Visual inspection of all five terminal SVGs
found no clipped labels, grid/marker conflicts, legend overlap, or unreadable
panel layout.

## Independent Checks

### Ran directly by Review B

- `cargo fmt --all -- --check` — passed.
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner
  --all-targets -- -D warnings` — passed.
- `cargo test -p openwepp-hillslope-orchestrator
  cqr_row5_snow_density_tests -- --nocapture` — `11 passed`.
- `cargo test --test
  snow_surface_eb04v_density_process_diagnostics_contract` — `2 passed`.
- `cargo deny check` — exit `0`; advisories, bans, licenses, and sources pass;
  the existing unmatched `MIT-0` allowance emits a non-failing warning.
- `git diff --check` — passed.
- Receipt validation re-hashed all 36 current cell provenances and their bound
  runfile, manifest, stdout, stderr, JSONL, and WAT files — passed for terminal
  binary `fb670d08...26f`.
- Independent Parquet comparison found all 36 terminal WAT tables exactly equal
  to EB-04R across `574,196` rows.
- Independent parsed JSON comparison found all 36 terminal traces equal to
  EB-04R across `574,196` common rows after excluding only the schema identifier
  and new `density_process_*` fields.
- Terminal result identity, all nine retained-pairing anchors, figures, and
  sidecars were inspected after final analysis regeneration.

### Reused Ran evidence independently checked

- The terminal release build produced `target/release/openwepp-cli-hill` at
  SHA-256 `fb670d086937a7785a2549339832f71b96fc98f3c8992ec8d24961123b33826f`.
- All 36 frozen B/L/S/LS cells returned zero and produced the complete retained
  target inventory.
- Independent consumer reconstruction reports maximum absolute additive
  closure `3.411e-13 kg m^-3`, maximum disagreement with the emitted residual
  `5.686e-14 kg m^-3`, and a maximum omitted-overburden residual
  `22.233 kg m^-3`.
- `100,824` fresh-density rows differ materially from final density, rejecting
  the most important same-day alias.

## Science And Maintainability Assessment

The terminal ledger is additive, unit-explicit, behavior-neutral, and carried
through the real production consumer. It distinguishes direct inputs and
fresh-snow density from process increments, cap corrections, structural
projection, fallback invocation, and downstream Stage-3 change. It supports
the bounded conclusion that compaction is active in both retained bias groups,
while fresh mixing, projection, caps, and Stage 3 can oppose it. It does not
identify one coefficient, prove compaction sufficiency, fit consumed
observations, authorize new physics, or reopen EB-04S promotion. The scientific
synthesis and calibration-readiness matrix preserve those limits.

The implementation is readable for the size of the existing density module,
but `09_snow_density.rs` now has `1,990` lines and only ten lines of headroom
below the repository ceiling. A named mechanical follow-up should extract the
large test or diagnostic-helper block at the next semantic edit. Separately,
33 existing integration contracts had to change only to follow the canonical
contract-version increment; consolidating that repeated exact-version binding
would reduce future maintenance churn if governance permits it. Neither item
invalidates EB-04V.

## Remaining Closure Blockers

No unresolved science, Rust correctness, pairing, behavior-neutrality,
retention, or figure blocker remains in the reviewed terminal state. Package
closure is still blocked procedurally because, at review time:

- `artifacts/gate-results.md` and `artifacts/owned-file-manifest.md` remain
  queued;
- the package-required quick, frost, exact-head full-workspace Nextest, unit/
  schema/documentation, and assurance-render freshness results have not yet
  been recorded in terminal gate evidence;
- the exact terminal diff, prompt archive, review disposition, dual terminal
  verification, worker handoff, roadmap/catalog closure state, and final
  disposition remain to be completed; and
- the package-local Python bytecode cache must remain excluded from the commit.

Review B therefore passes the current science and implementation but does not
authorize marking EB-04V complete until those terminal lifecycle requirements
are run, recorded truthfully, and independently verified.
