# Verification agent A

Status: PASS
Evidence mode: Static and Ran as labeled

## Objective and write-set verification

Static: the final tree closes the package's cover-first and decomposition
objective. The production diff is confined to named WAT column groups and
row/value helpers; it preserves the original lookup sequence, row loop,
area-lookup insertion, optional fallbacks, and value-read order. The focused
test diff supplies the required characterization and independent reconstruction
without changing production authority.

Static: all FQ-04 changes are inside the declared target source, focused test,
and package-local evidence/prompt write set. No focused fixture file, public
schema, dependency, contract, threshold, serialization surface, or unrelated
production module changed. The current source and focused test identities are:

- production: SHA-256
  `c31512f697a5867ae089b599a9131de1247069fa50da03dfaa96248f748530e0`,
  45,789 bytes, 1,421 lines;
- focused test: SHA-256
  `9b33fdfcaa29d4559205c28e1dfb1f83467395d33b1d411c398ea3837ed0f519`,
  71,405 bytes, 1,809 lines; and
- pre-refactor source at commit `1a4d6cd6`: SHA-256
  `1b9d8d124bf34a3d5f9189eb901a2ac87ff89d51076a58632c596ec878e47ac9`.

The pre-refactor identity is the exact source bound to the accepted WSHED01
T-B2-REDO2 real-consumer cohort. The current production change is therefore a
mechanical decomposition backed by current-source output oracles, not a new
science or output-meaning claim.

## A-H and finding verification

Static and Ran: every required family has an executable current binding:

| Family | Independently verified closure |
| --- | --- |
| A nominal | PASS runoff authority, per-hillslope discovery, exact WAT publication, and the two-day all-field oracle pass. |
| B boundary | Nonpositive/overflow area, empty PASS/zero runoff, outlet OFE selection, and partial optional-area behavior pass. |
| C branch | Column aliases, optional absent/all-null/mixed-null states, partial joins, and ordered last-duplicate WAT-key overwrite pass. |
| D domain reject | PASS nonnegative families, WAT positive area, and exact typed invalid-value rejection pass. |
| E missing/type/null | Required/optional paths, empty/read/write failures, schema types, nulls, row indices, and error priority pass. |
| F non-finite | NaN and both infinities are covered for PASS/WAT families, including WAT `Interception`; TSMF/QRain/QSnow non-finite paths pass. |
| G conservation/publication | Literal two-day water/storage/sediment reconstruction, class-density sediment concentration, nonzero storage-delta residual, and optional matched-area reconstruction pass without producer-helper reuse. |
| H fail closed/order | CLI/error contracts, output row/schema order, no-partial-output paths, and optional collision/unmatched-row semantics pass. |

The deliberately unequal aliases reject WAT Q/QOFE as PASS runoff, all-OFE
lateral flow, wrong storage/interception columns, concentration sums, a common
sediment density, first-duplicate optional area, and total-WAT-area optional
normalization. The A-H map names the exact test functions rather than relying
on category-only assertions.

Static: Review A findings A-001 through A-005 and Review B findings B-001 and
B-002 are all explicitly closed in the final reviews and
`review-disposition.md`. No finding is rejected, deferred, assigned to
follow-up, or left conditional. The sole per-function floor exception,
`for_batch`, was independently accepted by both reviews as bounded
non-science reader infrastructure.

## Raw metric and identity verification

Static: independent JSON inspection and SHA-256 recomputation confirm:

- `lcov-after.info`:
  `7d2ce90592050ac2ee8edddf8f1129202767126c6a436d9817d8555ae4c0a569`;
- `coverage-after.json`:
  `f2fc7e7434e43dc8545daebe0cb45120138a77980d2a6ca367d9181c1c693be1`;
- `crap-after.json`:
  `df3a152353ffb4891858d4ef3f4c403df7a92d3cb7c602249689ca97f4c5a078`;
- lines: 1,020/1,048, 97.328%;
- regions: 1,597/1,717, 93.011%;
- functions: 67/73, 91.781%; and
- zero target CRAP rows above 30, with maximum CRAP 23.0.

The only source-named row below the ordinary 75% region floor is `for_batch`
at 66.667%, cyclomatic complexity 7, and CRAP 8.815. All selectable public
reader and error-propagation behavior is covered; the excluded arms require
dependency-specific corrupt-Parquet behavior or a production test seam and do
not contain aggregation, normalization, conservation, or output mapping.

## Execution and gate verification

Ran on the final source/test identities:

```text
cargo nextest run -p openwepp-runner \
  --test totalwatsed3_cli_contract --no-fail-fast
Nextest run ID: 4c0c630e-05e7-4699-b987-2eef15de99be
17 tests run: 17 passed, 0 skipped
```

Ran: `cargo fmt --check` passed. Ran: targeted source/test/package
`git diff --check` passed. Ran: package Markdown lint validated 30 files with
zero errors and zero warnings, including this finalized verification artifact.

Static: the retained terminal logs postdate the final production and test edits
and record successful current-tree closure. Their independently recomputed
SHA-256 values and results are:

| Gate | Result | Log SHA-256 |
| --- | --- | --- |
| `cargo fmt --check` | PASS, exit 0 | `7174aedfdac0f29248fa82562e503b2c8ba857a0a8be90108a22db99a5895989` |
| workspace all-target Clippy with `-D warnings` | PASS, exit 0 | `f605f718349fff89caea1f2467b86369beaff5cf0a5dacaf0c030a2112a66589` |
| full-profile workspace nextest | PASS, 1,776/1,776, 3 skipped, 4 slow, run `fb1f0fd0-96aa-49b3-b92b-587ee3d446d4` | `c66e25a8e9b746e02d86367fcb7c085bff5f6a60c601c31aa2b2bed32d8a0c25` |
| `cargo deny check` | PASS, all four categories `ok` | `2369aa2c3034bae2d68610029c22b32cd654bf6d0f62a786b531c32c6541aba8` |
| `git diff --check` | PASS, exit 0 | `cc0631fc6fe9c409ded18ec7a2f856aeaed4f5bdf464bbebdafa55c3d18019e7` |

The security artifact is consistent with the diff: there is no new network,
subprocess, dependency, unsafe, filesystem-authority, deserialization-format,
or public-API surface. Line-count governance is below both production warning
and mandatory-refactor thresholds.

## Terminal readiness

**PASS.** FQ-04 satisfies its objective, declared write set, cover-first
science-tier thresholds, exact A-H obligations, conservation/publication
acceptance, CRAP closure, full Rust gate loop, documentation checks, dual
review, and finding disposition. Verification A finds no unresolved technical,
evidence, security, scope, or governance finding.

The package is ready for its terminal status update and completion commit. That
commit is the required next sequencing action; its necessary absence while
verification was being authored is not a HOLD condition.
