# Local CI Gate Selection

This standard governs local agent and maintainer gate selection. It does not
weaken package-specific acceptance gates, science-contract gates, or release
candidate gates.

## Principles

- Match gate cost to change risk.
- Keep full-suite evidence for branch-head closure, merge readiness, release
  readiness, and packages that explicitly require it.
- Do not run the full suite as a reflex after every narrow review-response fix.
- Record timing for expensive local CI runs so future profile decisions are
  empirical.
- Never use a faster profile to claim coverage it did not execute.

## Gate Tiers

| Tier | Use when | Typical commands |
|---|---|---|
| Focused | Editing a narrow crate, module, fixture, contract test, or review finding | `cargo nextest run -p <crate> <filter>` or `cargo nextest run --test <integration_test>` |
| Fast workspace | Routine implementation loop where broad regressions matter but slow fixtures are unrelated | `cargo nextest run --workspace --profile quick` |
| Domain profile | The change touches a known expensive domain | `cargo nextest run --workspace --profile frost` or `cargo nextest run --workspace --profile erosion` |
| Assurance editorial | The assurance normalizer produced the complete diff for pre-review DRAFT prose | `cargo nextest run --workspace --profile assurance-editorial` |
| Assurance amendment | A report-data-only transaction produced a current valid `metadata-fast`, `editorial-fast`, or `governance-focused` receipt | `.venv/bin/python tools/local_ci/run_assurance_amendment.py --receipt <path>` |
| Full branch-head | Merge readiness for Rust implementation/mechanical refactor packages, cutovers, publication changes, or package-required closure | `cargo nextest run --workspace --profile full` |
| Release/manual | Release candidates, observed cohorts, legacy comparators, stability lanes, or manual authority lanes | `tools/release/*` and explicitly named comparator/cohort commands |

## Deferrable Slow Families

The following families are not every-review gates unless the change touches
their owning domain or package acceptance criteria name them:

- Snowbench, PySnobal, SNOTEL, and observed snow/frost cohorts.
- Broad CLI/parquet fixture binaries when runner, HBP/WAT/PASS serialization,
  manifest, or file-format surfaces are not touched.
- Legacy comparator cohorts and release stability lanes.
- External-authority anti-evasion guards except when authority-suite posture,
  required-case bindings, cohorts, or fixtures change.

Domain-specific acceptance fixtures remain required for their domain. For
example, erosion packages should continue to run their p61/p102/DFF-WS3 gates
when those surfaces are in scope.

The `assurance-editorial` profile is proportional only when
`openwepp-assurance normalize --apply` produced the entire change and its
receipt identifies only manuscript/supplement prose plus mechanically rebound
packet, descriptor, and catalog digests. It retains the assurance crate and the
source, planner, assembly, normalization, and report integration boundaries.
Publication integration is excluded because this operation is confined to a
pre-review DRAFT. It is not an impact classifier. Any hand edit, mixed change,
lifecycle or authority transition, schema/builder change, or unclear diff uses
the applicable full package gates.

After `ASSURE-MAINT-01` implements the accepted
[amendment specification](../specifications/assurance-amendment-and-identity-workflow.md),
a current focused amendment receipt supersedes the legacy paragraph above for
the exact transaction it identifies. Before that implementation exists, no such
receipt can be claimed.

For a report-data-only change, a valid `metadata-fast`, `editorial-fast`, or
`governance-focused` receipt plus a passing receipt-runner evidence record is
the complete local gate contract. Do not scaffold a work package, dispatch
coding-agent review or terminal verification, run `quick` or `full`, run CRAP,
repeat staging, or reproduce unchanged science. Escalate only when the
amendment tool refuses the focused class, assurance implementation/schema/
builder code changed, or publication/release work is requested. The one-time
implementation of the amendment machinery remains an implementation package
and must pass its full closure and CRAP gates.

## Timing Diagnostics

Use `tools/local_ci/nextest_timing.py` for expensive nextest runs:

```bash
python tools/local_ci/nextest_timing.py run \
  --label quick \
  --profile quick \
  -- cargo nextest run --workspace --profile quick
```

The tool writes untracked history to `target/local-ci-history/`:

- `nextest-runs.jsonl` append-only machine-local records,
- `latest.json` full structured summary,
- `latest.md` human-readable slow-test table.

When changing nextest scheduling caps, run a sweep with a temporary config and
record the result in the owning work package before editing `.config/nextest.toml`.
For fixture groups that set `threads-required = 2`, a group `max-threads = 4`
means two matching fixture tests may run at once.

## Reporting Rules

- Report `Ran:` only for commands executed in the current session.
- Report `Static:` for timing conclusions derived from existing JUnit/history.
- If a package skips `full`, state why the package scope permits that and name
  the strongest gate actually run.
- If `full` is required by the package, do not replace it with `quick` or a
  domain profile unless the package is explicitly amended before execution.
