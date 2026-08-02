# Independent Science And Code Review A

Evidence class: **Static + Ran**.

## Findings

### A-01 — High (open, package-closure blocking): final broad regression and verification are pending

Paths: `artifacts/gate-results.md:26-28` and
`package.md:141-170`.

The exact post-fix release cohort, analysis, and retained-output comparator pass,
but the package still truthfully marks quick, frost, full, dual review
disposition, and dual terminal verification as pending. Earlier quick attempts
were interrupted after source-invalidating review corrections and are not
admissible evidence. The independent suite runner must complete a fresh bundle
against the final source before acceptance criterion 9 can pass.

Disposition: **OPEN / HOLD PACKAGE CLOSURE**. This is not an implementation or
science-contract rejection; it is a mandatory current-scope validation gate.

### A-02 — High (closed by terminal addendum): command-level validation provenance was incomplete

Paths: `artifacts/gate-results.md:18-24`, `artifacts/logs/`, and
`docs/standards/testing-and-gate-strategy.md:292-305`.

At initial review, the gate table summarized focused, Clippy, formatting,
dependency, assurance, unit-registry, and documentation passes, but there was
not yet one terminal record binding each reused or final command to its exact
argv, working directory, source/dirty identity, relevant inputs, duration, exit
status, log, and supported requirement. The retained `direct-*` logs predated
the final snowbench source correction and therefore could not themselves
establish exact-terminal source identity. Reuse remained possible for
demonstrably unaffected requirements, provided that exclusion and the final
source-sensitive reruns were recorded explicitly.

Disposition: **CLOSED** by the terminal provenance addendum below. The governed
record now binds the clean final cohort (`273.07 s`), comparator (`254.85 s`),
all other claimed final commands, and the intentionally interrupted predecessor
attempts without relabeling those attempts as passes.

### A-03 — High (corrected): release evidence repeatedly postdated the implementation it claimed to validate

Paths: `artifacts/invalidated-pre-terminal-helper-extraction/README.md`,
`artifacts/invalidated-pre-clippy-helper-extraction.md`,
`artifacts/invalidated-pre-snowbench-phase-fix.md`, and
`artifacts/execution-receipt.json`.

Clippy first required extraction of the oversized phase-validation helper, and
later Review A required a snowbench compatibility correction. Each edit made a
prior binary/cohort nonterminal. The package now preserves those generations as
invalidated evidence and reran from release binary
`b50dd71cb00f24806193b98d73fc5444e836efac84ad5a4e0465d1e67c81fec9`.
The final 16-cell receipt is
`6f6bfe361c5b0aa155de1cfba61306e6d20fd570e68f67521eed12a3154dfbf7`;
all ten receipt-bound source hashes match the current files, all 16 provenance
hashes match, every return code is zero, and every cell carries the exact
seven-variable sanitized environment.

Disposition: **ACCEPTED / CORRECTED**. Final result SHA-256 is
`a44c3561cfea5bec64cc7514c4e3701d95111d8368ab96d8f0cb4784fcae6816`.

### A-04 — Medium (corrected): snowbench phase metadata rejected valid non-100 kg/m3 new-snow density

Paths: `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs:637-656`
and `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs:1140-1166`.

The first adapter correction derived active precipitation and phase fractions
from PySnobal snow-water mass. The authoritative orchestrator accumulation
operand remains physical snowfall depth times the legacy `0.1` SWE ratio. Those
quantities agree only when new-snow density is exactly `100 kg m^-3`; otherwise
the new diagnostic guard changed a formerly executable snowbench request into a
typed phase-closure failure. The retained replay fixture used exactly 100 and
did not expose the regression.

The adapter now derives its diagnostic total and fractions from the same
runtime operand without changing modeled rain, snowfall depth, or state
arithmetic. A new `200 kg m^-3` regression reconstructs both components and
executes the replay. Review A independently ran that test: `1/1` passed.

Disposition: **ACCEPTED / CORRECTED**.

### A-05 — Medium (corrected): seasonal peak ratios were attributed to the observed peak date

Paths: `package.md:213-217`, `package.md:250-262`,
`artifacts/scientific-synthesis.md:19-23`, and
`artifacts/scientific-disposition.md:9-19`.

The first synthesis described the `0.39-0.62` modeled-to-observed seasonal
maximum ratios as storage retained at the observed SWE-peak date. Those are
different operators. Baseline modeled SWE on the observed peak dates has lane
medians of approximately `0.21-0.46`. The package, synthesis, and disposition
now separate the two claims and retain the realized-input/endogenous-retention
causal boundary.

Disposition: **ACCEPTED / CORRECTED**.

## Correctness assessment

The current Rust implementation has no open numerical, science-contract,
serialization, typed-error, or substantial-duplication finding.

- `simimpl29_hourly_melt_inches` returns the four terms once, and
  `compute_simimpl29_melt_hour` preserves the original explicit
  `amelt + bmelt + cmelt + dmelt` arithmetic order before the existing unit
  conversion and pack cap. Component conversions are diagnostic only.
- The cap adjustment is `applied - uncapped`; the typed producer guard and the
  independent real-consumer analysis reconstruct uncapped, applied-plus-cap,
  and daily applied-to-raw stages.
- SIMIMPL28 carries an authoritative pre-partition active-precipitation operand
  through `DirectWinterHourlyForcing`, `DirectSnowHourlyForcing`, the typed
  partition result, and the v3 JSONL consumer. Dry and active fraction/amount
  violations fail through the existing `StateSymbolOutOfRange` taxonomy.
- The v3 formatter publishes 24 distinct hourly records, keeps physical snow
  depth separate from SWE, and marks modeled wind redistribution as zero
  without inferring physical site redistribution.
- The final analyzer reproduces all five frozen B operators exactly
  (`-35`, `-46.5`, `-31`, `-37`, and `-44.5` days) across the four-lane,
  16-cell execution and retains B/L/S/LS cell evidence.
- Maximum closure residuals are `2.017e-17 m` for uncapped components,
  `2.027e-17 m` for applied components plus cap, `7.980e-17 m` for
  accumulation, `1.214e-17 m` for phase amounts, `8.882e-16 m` for trace/WAT,
  and `2.998e-15 m` for pre-observed-peak mass, all inside `1e-12 m`.
- Final behavior report
  `b896b53ecb3787dd85fe46732a7154b22788c942af3f6c8093a96113859e7d0e`
  compares 245,456 rows, 736,368 WAT numeric values, and 72,093,744 values over
  all 111 prior-v2 trace fields. Both maximum differences are exactly zero.

## Checks run by Review A

Against the final source, Review A ran:

- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner
  --all-targets -- -D warnings`: pass;
- `cargo fmt --all -- --check`: pass;
- `git diff --check`: pass;
- EB-04W contract target: `2/2` pass;
- phase semantics and CoE component/cap unit filters: `2/2` pass;
- real trace formatter filter: `1/1` pass;
- retained per-day canopy snowbench target: `3/3` pass; and
- noncanonical new-snow-density regression: `1/1` pass.

Review A also independently checked the final receipt/result/comparator hashes,
source identity, population cardinality, return codes, environment policy,
closure values, and operator values. This review did not run or substitute for
the pending quick/frost/full bundle.

## Residual risk and missing tests

- Physical precipitation representativeness, gauge catch, phase truth, wind
  redistribution, liquid-retention causality, and pre-peak loss timing remain
  nonidentifiable from this diagnostic corpus. The correct posture remains
  `DIAGNOSTIC_COMPLETE / CALIBRATION_HOLD / NO_PROMOTION` once mechanical gates
  close.
- The fixed legacy `0.1` snow-depth-to-SWE factor remains repeated at the daily
  accumulator, phase guard, diagnostic projection, and snowbench adapter. This
  is not a substantial duplicated algorithm, and independent closure makes
  divergence fail closed, but a named canonical Rust constant would reduce
  maintenance risk.
- The snowbench adapter has no externally-partitioned phase-model identity and
  therefore retains `LegacyRst` in its unused diagnostic record. Its report does
  not serialize the EB-04W ledger, so this is a future-consumer risk rather than
  a current contract claim.
- Three touched Rust files remain in the documented 2,000-2,999-line warning
  band. No touched Rust file reaches the mandatory 3,000-line split threshold.

## Verdict

**APPROVE IMPLEMENTATION / HOLD PACKAGE CLOSURE.**

No science or Rust blocker remains in the reviewed implementation, and A-02 is
closed by the terminal provenance audit below. Closure may advance only after
A-01 passes on the exact final source, Review B disposes its earlier findings
against the final receipt, and both terminal verification artifacts pass.

## Validation-provenance audit addendum

Evidence class: **Static + Ran**. Audit date: `2026-08-01`.

Review A audited `artifacts/validation-execution-provenance.md` and its durable
logs against the sealed source. The following bindings reproduce exactly:

- HEAD `045cac9475738b0306a89a934702c479803f0935` and tracked binary-diff
  SHA-256 `aaa2d4fede9aba5e1c4eafbb27b596e997e9737ad5613fa6ff54f41b68315f8a`;
- the listed `Cargo.toml`, `Cargo.lock`, contract, focused-test, snowbench,
  release-binary, execution-receipt, analysis-result, and behavior-result
  SHA-256 identities;
- every final `fmt`, `diff --check`, focused/formatter/melt/phase/noncanonical
  test, Clippy, unit-registry, dependency, assurance, and package-Markdown exit,
  elapsed time, and reported test/file count; and
- final cohort exit `0` in `273.07 s` and final comparator exit `0` in
  `254.85 s`, bound respectively to receipt `6f6bfe36…`, result `a44c3561…`,
  and behavior report `b896b53e…`.

The initially missing external-Markdown receipts were then added and audited.
The provenance ledger now records exact commands for `docs/ROADMAP.md`,
`docs/planning/snow-surface-energy-balance-roadmap.md`, and
`docs/work-packages/README.md`; their durable log/time receipts report exit `0`,
one file validated, zero errors, zero warnings, and elapsed times of `0.00 s`,
`0.00 s`, and `0.01 s`, respectively.

**A-02 disposition: `PASS / CLOSED`.** The command-level validation provenance
now satisfies the governed evidence fields for every claimed completed gate.
A-01 remains separately pending and is unaffected by this addendum.
