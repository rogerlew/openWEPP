# Review agent A

Status: **HOLD**
Evidence mode: Static and Ran as labeled
Review date: 2026-07-11

## Verdict

**HOLD.** The WAT-reader decomposition is mechanically credible and the
cover-first/terminal metric thresholds pass, but current evidence does not yet
close the package's exact output-identity and conservation/publication gates.
The A-H map also overstates several unimplemented bindings, the terminal
coverage-closure artifact is internally stale, and the required full closure
loop remains queued. All findings are in the declared package envelope and
must be corrected rather than deferred.

## Findings

### A-001 — High: independent conservation/publication acceptance is incomplete

Static: `optional_soil_element_outputs_are_independently_area_reconstructed`
is a useful anti-alias test, but it independently asserts only `Area`,
`Precipitation`, `Q`, outlet `latqcc`, all-row `QOFE`, PASS `runvol`, and
`TSMF`/`QRain`/`QSnow`. The existing nominal test additionally checks `Runoff`,
one sediment class, and `sed_del`, but it is not the complete literal oracle
required by `operand-lineage.md` and the package's conservation gate.

No current test independently reconstructs or differentiates:

- `sbrunv`, `RM`, `Dp`, `Ep`, `Es`, `Er`, `Evaporation`, `ET`, `UpStrmQ`,
  `SubRIn`, `Total-Soil Water`, present `SoilWaterTotal`, the profile fields,
  present `Interception`, `InterceptionStorage`, `frozwt`, `Snow-Water`,
  `Tile`, or `Irr`;
- `tdet`, `tdep`, `seddep_2..5`, or the nonzero class-density
  `sed_vol_conc` formula; or
- daily `delta(storage)` and the primary closure residual from independent
  source rows.

This leaves adjacent-column swaps, storage/interception aliases, ET component
swaps, class/density-order defects, and several denominator errors able to pass
the suite. Add the literal multi-day oracle described in `operand-lineage.md`
and assert every authoritative water/sediment/optional output. At least two
days with different total areas are necessary so depth normalization,
mm-to-volume conversion, and storage delta do not share one accidental numeric
divisor. Independently reconstruct the daily and whole-fixture closure and
bind a two-sided magnitude/non-tautology check.

The package must also either rerun the accepted real WSHED01 cohort closure on
the final producer or explicitly bind the prior cohort evidence to current
source/output identities and prove that it remains a valid real-consumer audit.
A one-day self-consistency fixture alone cannot close the repository's
Conservation / Publication Acceptance Rule.

### A-002 — High: post-decomposition exact output identity is explicitly pending

Static: `numeric-equivalence.md` remains
`safety-net oracle PASS; post-decomposition identity pending`. The production
hash changed from
`1b9d8d124bf34a3d5f9189eb901a2ac87ff89d51076a58632c596ec878e47ac9`
to
`c31512f697a5867ae089b599a9131de1247069fa50da03dfaa96248f748530e0`,
but no recorded before/after Parquet schema, row-order, null-map, and exact
value comparison closes the package's required output-identity gate. Passing
the same partial assertions before and after extraction is necessary but not
sufficient for unasserted columns.

Record a post-decomposition identity gate over every output column and row,
including types, nullability/null positions, date order, optional defaults, and
floating values. An exact pre/post fixture snapshot is acceptable; a complete
independent oracle is stronger. Update `numeric-equivalence.md` to PASS only
after that evidence exists.

### A-003 — High: the A-H map is categorical rather than exact and overclaims coverage

Static: `obligation-to-test-map.md` lists categories such as “existing CLI
nominal/per-hillslope cases” instead of exact test-function names. Several PASS
claims are not supported by the current 15 tests:

- B does not bind partial optional coverage or nonempty PASS-only/extra WAT-day
  domain behavior; it binds only empty PASS defaulting.
- C does not test WAT optional mixed-null/all-null behavior across storage,
  profile, `Interception`, `Tile`, and `Irr`; the mixed-null vector is `QRain`
  and the required-soil vector is `TSMF`.
- F's WAT loop omits `Interception`; optional soil coverage does not exercise a
  non-finite `TSMF`; optional element coverage exercises non-finite `QSnow`
  only. “Every real operand” is therefore not established.
- G lacks the full water/sediment and multi-day closure reconstruction in
  A-001.
- H has no multiple-day BTreeMap row-order vector, path-order accumulation
  vector, duplicate optional-lookup collision vector, or explicit
  `DateOfeKey`-without-`sim_day_index` disposition.

Replace every family row with exact function names and canonical clauses. Add
the missing vectors or narrowly and explicitly justify a non-applicable branch;
do not leave category prose standing in for exact bindings.

### A-004 — High: required terminal validation gates are not run

Static: `gate-results.md`, `review-disposition.md`, and `disposition.md` remain
queued/not-run. The package requires focused CLI, exact output identity,
formatting, workspace/all-target Clippy, full-profile nextest, deny, Markdown,
diff, line/security, dual review/disposition, dual verification, and terminal
commit evidence. Current focused results cannot substitute for that closure
loop. Run and record each gate after resolving A-001 through A-003; any failure
blocks PASS.

### A-005 — Medium: coverage closure is stale and contradicts terminal evidence

Static: `coverage-after.md` correctly records terminal 97.233% lines, 92.953%
regions, no eligible CRAP above 30, and the named `for_batch` exception.
However, `coverage-closure.md` still says “terminal measurement pending” and
asserts all source-named non-format helpers meet 75%. Reconcile it with the
terminal source hash, terminal raw hashes, 67/73 function count, and the exact
closed-list exclusion below. A stale safety-net-only closure artifact cannot
serve as terminal coverage disposition.

## `for_batch` 66.667% reviewed-exclusion disposition

**ACCEPTED as a closed-list, non-science infrastructure exclusion**, subject to
A-005 being recorded in the canonical coverage-closure artifact.

Static/Ran basis:

- Raw terminal CRAP identifies exactly one source-named function below the 75%
  floor: `for_batch`, 66.667% coverage, CC 7, CRAP 8.815.
- Valid single/multi-row reading, callback success, callback error propagation,
  row-offset handling, file-open mapping, malformed-Parquet reader
  construction, and stable public `Read`/`Open` error displays are exercised.
- Remaining arms are dependency-origin reader-build/page-read failures that
  cannot be selected deterministically through the public API without coupling
  the test to a particular corrupt-Parquet encoding or adding a test-only
  production seam.
- `for_batch` performs no aggregation math, area selection, normalization,
  conservation calculation, or output mapping. The global science-tier line
  and region thresholds remain above 90%, and its CRAP is well below 30.

This exclusion does not waive error-order or output-identity obligations in
A-001 through A-004.

## Passing evidence

Static:

- The production diff is an in-envelope decomposition into typed identity,
  required, storage, row, and value helpers. Column lookup order remains
  identity → required → storage; per-row validation remains `wepp_id`, OFE,
  area, day key, required soil water, then WAT values. Path/batch/row and
  floating accumulation order are unchanged.
- No formula, unit, public API, schema, accepted alias, optional fallback, or
  writer mapping changed in the production diff.
- Terminal raw evidence matches the live source: 1,019/1,048 lines (97.233%),
  1,596/1,717 regions (92.953%), zero target CRAP rows above 30, and maximum
  CRAP 23.00075 in `read_wat_values`.
- Both Rust files remain below line-count governance thresholds, and the
  security artifact identifies no new authority or dependency surface.

Ran:

- `cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract`:
  run `f35d8165-6aad-4984-87de-4ff5492ad501`, 15/15 passed.
- `cargo fmt --check`: PASS.
- Targeted `git diff --check`: PASS.

## Lift conditions

1. Complete the multi-day all-operand independent reconstruction, sediment
   class/density oracle, storage-delta closure, and real-cohort/magnitude
   evidence.
2. Record complete post-decomposition schema/row/null/value identity.
3. Replace category A-H rows with exact test names and close the missing
   branches.
4. Reconcile terminal coverage closure, explicitly retaining the accepted
   `for_batch` exclusion.
5. Run the complete final gate loop, dual reviews/disposition, and dual
   verification with no deferred current gate.

## Re-review after remediation

Status: **HOLD**
Evidence mode: Static and Ran as labeled
Re-review date: 2026-07-11

### A-001 disposition — CLOSED

Static/Ran: The new
`two_day_water_storage_and_sediment_oracle_rejects_wrong_aliases` test closes
the fixture-side conservation finding. It uses two WAT days with different
areas, asserts exact `DayKey` row order, independently reconstructs the full
water/storage/profile field set, all five sediment class masses, `sbrunv`,
`tdet`, `tdep`, `sed_del`, and the class-density `sed_vol_conc`, and computes
an independent storage delta plus deliberately nonzero primary closure
residual. The separate optional oracle continues to bind matched-WAT-area
TSMF/QRain/QSnow and rejected unmatched rows.

The real-cohort binding is also valid. Ran:

```text
git show 1a4d6cd6:crates/openwepp-runner/src/totalwatsed3.rs | sha256sum
1b9d8d124bf34a3d5f9189eb901a2ac87ff89d51076a58632c596ec878e47ac9
```

Commit `1a4d6cd6901aa422bab68763bf982caa2be2f14b` therefore contains exactly the
pre-refactor producer bound by the cover-first evidence. The accepted WSHED01
cohort's 78,912 source rows, 2,192 output rows, zero maximum runoff-pairing
delta, `-4.0978193283081055e-08 m3` PASS/output runoff-sum delta, and
`-0.409175395336963 mm` ex-initialization residual over 2,191 days provide the
required real magnitude/non-tautology evidence. The current diff changes only
reader structure, and the comprehensive current-source oracle bridges that
content-identical pre-refactor cohort to the final producer.

### A-002 disposition — CLOSED

Static/Ran: `numeric-equivalence.md` now records PASS and binds the 16-test
post-decomposition oracle. The test validates both output rows, date order,
Float64/nullability posture for the totalwatsed3-derived surface, all water,
storage/profile, sediment, and default optional values, and the rejected alias
relationships. The unchanged writer/schema source plus the mechanical
production diff completes the exact output-identity argument; no field
formula, writer mapping, public schema, or accumulation expression changed.

### A-003 disposition — PARTIALLY CLOSED; remaining blocker

The two-day oracle closes the substantive water/sediment and storage-order
gaps, and the A-H table now names its main test functions. However the table
still claims exact completion while retaining category shorthands and two
unbound optional-input risks:

1. Family E says “missing optional CLI inputs” instead of naming
   `totalwatsed3_cli_rejects_missing_explicit_optional_inputs`; Family F says
   “loops plus optional ... vectors” instead of enumerating the exact function
   bindings; Family B's “two-day per-hillslope outlet selection” likewise must
   point explicitly to the oracle function. Replace every shorthand so the map
   is mechanically reviewable.
2. `every_wat_float_family_rejects_nonfinite_and_area_rejects_nonpositive`
   still omits optional WAT `Interception`. The added
   `if column == "Interception"` branch is inside the PASS-column loop, where
   that condition is unreachable, so it does not close the claimed exhaustive
   WAT real-family binding. Add `Interception` to the WAT loop with the required
   optional-column setup and exact E007 assertion.
3. The map states that duplicate WAT `DateOfeKey` overwrite behavior is
   “characterized,” but no test constructs a duplicate optional lookup key.
   Likewise, the optional fixture has all three WAT keys matched plus one
   unmatched row; it does not distinguish the current matched-area denominator
   from total WAT area under partial optional coverage. Add an exact vector for
   the documented ordered overwrite/key-without-`sim_day_index` behavior and a
   partial-match vector whose two possible denominators differ, or remove the
   unsupported characterization claim and hold that current-scope risk.

These are bounded test/artifact corrections, not legitimate follow-up scope.
Family-level PASS is premature until they are green and exactly named.

### A-004 disposition — CONDITIONAL HOLD; full gates still running/not recorded

`gate-results.md` remains `queued` / `not-run` at re-review time. This review
does not infer results from concurrent execution. Final GO requires recorded
PASS for focused CLI, exact output identity, formatting, workspace/all-target
Clippy, full-profile nextest, deny, Markdown, diff, line/security, dual review
disposition, and dual verification. Any failure remains closure-blocking.

### A-005 disposition — CLOSED

`coverage-closure.md` now records both cover-first and terminal evidence,
including live production hash
`c31512f697a5867ae089b599a9131de1247069fa50da03dfaa96248f748530e0`,
1,019/1,048 lines (97.233%), 1,596/1,717 regions (92.953%), maximum CRAP
23.00075, and no eligible row above 30. Its raw hashes agree with the current
terminal artifacts.

The prior `for_batch` exclusion remains **ACCEPTED**: 66.667%, CC 7, CRAP
8.815, with all public/selectable infrastructure behavior covered and only
dependency-origin corrupt-reader arms omitted. It is now explicitly recorded
in the terminal closure artifact and does not waive any science/output gate.

### Fresh re-review execution

Ran:

- `cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract`:
  run `a7f86b36-2f83-4319-85c4-c5ea9e769da0`, 16/16 passed.
- `cargo fmt --check`: PASS.
- Targeted `git diff --check`: PASS.
- Raw terminal metrics independently re-read: 97.233% lines, 92.953% regions,
  67/73 functions, zero CRAP rows above 30, and exact `for_batch` exception as
  recorded.

### Final re-review verdict

**HOLD.** A-001, A-002, and A-005 are closed. A-003 still has exact-binding and
optional-join/non-finite gaps, and A-004 remains conditional on the genuinely
running terminal gates. Final GO is authorized only after A-003 is corrected
and the complete gate/disposition record is available and green.

## Final re-review after A-003/A-004 remediation

Status: **GO — FINAL REVIEW A PASS**
Evidence mode: Static and Ran as labeled
Final re-review date: 2026-07-11

### Final finding disposition

| Finding | Disposition | Final evidence |
| --- | --- | --- |
| `A-001` | closed | Two-day, different-area literal oracle covers all water/storage/profile and sediment/class-density operands; optional soil/element oracle plus real WSHED01 cohort binding closes anti-tautology and magnitude acceptance. |
| `A-002` | closed | Post-decomposition 17-test output oracle, unchanged writer/schema source, and mechanical production diff bind schema/value/order/null/default identity. |
| `A-003` | closed | A-H map now enumerates exact test functions; executable WAT `Interception` non-finite, TSMF/QRain/QSnow non-finite, partial matched-area, and last-duplicate-key vectors pass. |
| `A-004` | closed | Required focused and root closure commands are recorded PASS; final scoped Markdown and diff checks also pass in this review. |
| `A-005` | closed | Terminal coverage/CRAP and the accepted `for_batch` closed-list exclusion remain current and hash-consistent. |

No finding is rejected, deferred, converted to follow-up, or left conditional.

### A-003 final verification

Static/Ran:

- `obligation-to-test-map.md` now binds each A-H family to concrete test
  function names. No category-only placeholder remains.
- `every_wat_float_family_rejects_nonfinite_and_area_rejects_nonpositive`
  includes `Interception` in the WAT list, creates the optional column before
  mutation, and asserts exact E007/row-zero behavior for NaN and both
  infinities. The formerly unreachable PASS-loop branch is gone from the
  binding path.
- `optional_columns_cover_all_null_mixed_null_nonfinite_and_missing_partitions`
  binds non-finite TSMF, QRain, and QSnow plus all-null/mixed-null behavior.
- `optional_join_partial_coverage_uses_last_duplicate_wat_key_area` is an
  executable anti-alias vector. It creates duplicate WAT optional keys with
  areas `400` then `1600`, proves ordered last-write area selection, uses only
  the matched `1600+500` denominator, and rejects both first-write and total
  WAT-area candidates. This closes the documented `DateOfeKey`/partial-match
  risk without authorizing a semantic change.

Ran final focused suite:

```text
cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract
Nextest run ID: beba40ce-bca8-4baf-8704-767d3197de2b
17 tests run: 17 passed, 0 skipped
```

### A-004 final gate verification

Static/Ran: `gate-results.md` records current-worktree PASS for:

- `cargo fmt --check`, exit zero;
- `cargo clippy --workspace --all-targets -- -D warnings`, exit zero;
- `cargo nextest run --workspace --profile full`, run
  `fb1f0fd0-96aa-49b3-b92b-587ee3d446d4`, 1,776/1,776 passed, three skipped,
  four slow, 171 binaries;
- `cargo deny check`, with advisories/bans/licenses/sources all `ok`; and
- `git diff --check`, exit zero.

The referenced logs exist and their independently recomputed SHA-256 values
are:

- format: `7174aedfdac0f29248fa82562e503b2c8ba857a0a8be90108a22db99a5895989`;
- Clippy: `f605f718349fff89caea1f2467b86369beaff5cf0a5dacaf0c030a2112a66589`;
- full nextest:
  `c66e25a8e9b746e02d86367fcb7c085bff5f6a60c601c31aa2b2bed32d8a0c25`;
- deny: `2369aa2c3034bae2d68610029c22b32cd654bf6d0f62a786b531c32c6541aba8`;
- diff: `cc0631fc6fe9c409ded18ec7a2f856aeaed4f5bdf464bbebdafa55c3d18019e7`.

Ran supplemental final-document checks after the last artifact/test updates:

- `markdown-doc lint --path
  docs/work-packages/20260711-cqr-followup-totalwatsed3-001`: 30 files, zero
  errors and zero warnings;
- `cargo fmt --check`: PASS; and
- targeted package/source/test `git diff --check`: PASS.

### Terminal metric and authority recheck

Static/Ran:

- Live production SHA-256 remains
  `c31512f697a5867ae089b599a9131de1247069fa50da03dfaa96248f748530e0`.
- Terminal raw evidence now reports 1,020/1,048 lines (97.328%), 1,597/1,717
  regions (93.011%), zero target CRAP rows above 30, and maximum CRAP 23.0.
- `for_batch` remains the sole accepted floor exception at 66.667%, CC 7,
  CRAP 8.815, with the exact previously reviewed infrastructure rationale.
- `git show 1a4d6cd6:crates/openwepp-runner/src/totalwatsed3.rs` still hashes to
  the exact pre-refactor authority
  `1b9d8d124bf34a3d5f9189eb901a2ac87ff89d51076a58632c596ec878e47ac9`.
- No source formula, area/normalization basis, error priority, accepted alias,
  optional fallback, path/batch/row accumulation order, schema, unit, or output
  mapping changed.

### Final verdict

**GO — FINAL REVIEW A PASS.** All A-001 through A-005 findings are closed with
current direct evidence. The package satisfies Review A's contract/semantic,
conservation/publication, optional-join, exact A-H, cover-first, CRAP,
line/security, and validation criteria with no deferred current gate. Final
package disposition may proceed through the separately required Review B,
finding disposition, and dual verification workflow.
