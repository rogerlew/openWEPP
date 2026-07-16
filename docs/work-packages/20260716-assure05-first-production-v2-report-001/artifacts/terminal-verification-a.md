# ASSURE-05 terminal verification A

Date: 2026-07-16 UTC
Role: terminal internal verifier A
Evidence class: **Static + Ran (lightweight identity and reproduction checks only)**

## Terminal verdict

**PASS — TERMINAL INTERNAL VERIFICATION A.**

No blocking internal-verification finding remains against the final report source,
technical closure evidence, or disposition. This verdict does not supply human
scientific approval, independent external peer review, publication authority, or
release authority. The package correctly remains `HOLD-HUMAN-APPROVAL`, and the
report correctly remains `DRAFT`.

## Scope and execution boundary

This verification inspected the final report source and disposition, the internal
review and remediation records, the retained Attempt 1/2/3 heavy-gate evidence,
the fresh adjudicated-CRAP evidence, the durable H2637 reproduction objects, and
the protected public surfaces.

The verifier did not rerun formatting, clippy, full nextest, cargo-deny, staging,
or adjudicated-CRAP gates. Heavy-gate conclusions below are static conclusions
from retained logs and structured artifacts. The verifier ran only the existing
assurance binary for validation and planning, hash and inventory checks, and the
two documented report-reproduction procedures.

## Verification results

| Obligation | Evidence | Result |
|---|---|---|
| Exact report-source identity | A current `openwepp-assurance validate --report linear-groundwater-reservoir-recurrence` run returned per-report source root `84a8467ff818411a34c89bf825fc2e9280a7c37c50db9b38636fc831546f4d01`. The current descriptor SHA-256 is `47d1e0be95512c865172802eda34db7c3ec60112192abcaf2ad78c6808997a90`, matching the catalog and final disposition. | PASS |
| Bounded scientific claims after DS-01 through DS-05 | The manuscript, report descriptor, supplement, claim records, and renewed domain-science review distinguish binary64-versus-decimal recurrence residual from the Rust assertion allowance; restrict transfer evidence to separately exercised writer/parser and consumer interfaces; use the corrected Priest River calibration/evaluation split; and state corrected units. Empirical predictive skill and transferability claims remain excluded. | PASS |
| Durable and reproducible H2637 evidence | The retained `manifest.json`, `H2637.hbp`, and `H2637.pass.parquet` have SHA-256 values `756e324e5b4f055ea45c33b0d5f679ab2fc9f4b958e853dc0b70f17aeb592208`, `378a8c1d80a22c9452fb256cf9a95eab09035f3a6cd387c6d626ab26c426c453`, and `915f3b99c2ff20e3e0632b4e90a6ceb1cb8e7fee58f0d3e29b41de10c540f550`. A fresh invocation of the documented H2637 reproduction procedure authenticated the retained inputs and reproduced the normalized result with semantic JSON equality. The analytical reproduction procedure also returned semantic equality. | PASS |
| Explicit CLI-adapter evidence gap | The abstract, methods, results, discussion, limitations, conclusion, descriptor, claim records, and supplement explicitly state that no nonzero H2637 payload was demonstrated through the complete subprocess-to-CLI adapter chain. The gap is preserved as follow-up work rather than obscured by a broader transfer claim. | PASS |
| Internal review disposition | The domain-science review resolves DS-01 through DS-05 and concludes `INTERNAL-AGENT-REVIEW-CLEAR-FOR-REQUIRED-HUMAN-REVIEW`. The reproduction/publication review resolves F1 through F3. `finding-disposition.md` accurately records claim narrowing for DS-02 and leaves the institutional/human boundary open. | PASS |
| Draft lifecycle and human hold | `report.yaml` remains `DRAFT`, has no assigned human report lead or scientific approver, claims no external peer review, has no authorized review entry, approval lock, publication root, release identifier, or public path, and records `unassigned_blocks_review`. The principals registry contains only the agent draft author. | PASS |
| Protected public zero-state | `usersum/assurance` contains only `README.md`; current validation reports zero public reports. The protected public hashes match the frozen values recorded in the final disposition, including `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` for the README/catalog template, `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` for `assurance/catalog.yaml`, and `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` for the generated usersum manifest. | PASS |
| Final-disposition, package, and roadmap consistency | `package.md`, `final-disposition.md`, the work-package index, `docs/ROADMAP.md`, and the scientific-assurance implementation roadmap consistently state technical closure, `DRAFT`, `HOLD-HUMAN-APPROVAL`, unchanged public surfaces, and the requirement for named human approvals before advancement. ASSURE-06 remains blocked behind that boundary. | PASS |

## Retained heavy-gate and CRAP evidence

Static inspection confirmed that the execution history preserves failures rather
than treating retries as waivers:

- Attempt 1 stopped at strict clippy with three `clippy::float_cmp` findings in
  the assurance contract test; downstream gates were not represented as run.
- Attempt 2 passed formatting and strict clippy, then stopped after 22 failures
  in the publication contract suite caused by stale fixture assumptions;
  downstream gates were not represented as run.
- Attempt 3 restarted the complete required sequence. The retained nextest log
  identifies run `f7960089-7439-420e-aa3b-293c7fa5d773` and records 2,049
  passed, three skipped, and four slow tests. The retained evidence records PASS
  for formatting, strict clippy, full-workspace nextest, cargo-deny, assurance
  validation, independent staging, Markdown validation, staging comparison, and
  diff checks.

The fresh adjudicated-CRAP artifact reports `PASS`, `closure_eligible: true`,
9,262 production entries, two raw scores above 30, two current adjudications,
zero actionable entries, and zero touched production files. The source manifests
before, after, and final are byte-identical with SHA-256
`5f0446b67c84ecc1606a8adc6527adf75734ab82dba0df7ee62265635f593fcd`.
Current workspace hashes for all 228 production sources and 438 measurement
inputs matched the retained final manifest. The two over-threshold entries are
therefore governed by the established CQR adjudications, not silently waived or
reclassified for this package.

## Findings

### TV-A-01 — Low — stale archival summary wording

`semantic-differences.md` was not fully refreshed after the final remediation and
staging expansion. It states that there are 11 research objects, while the final
staging evidence contains 15, and it retains the phrase “production-transfer
conclusion” where the accepted report posture is the narrower, separate-interface
verification with continuous CLI-adapter traversal still open.

Disposition: non-blocking documentation follow-up. The stale wording is outside
the exact claim-bearing report source root and does not alter the accurate
package, final disposition, roadmap, internal-review disposition, lifecycle
state, or protected public zero-state. It should be corrected before long-term
package archival; this verifier did not edit it because the terminal-verification
artifact was the only authorized write.

No medium-, high-, or critical-severity findings were identified.

## Terminal boundary

Terminal internal-verifier A disposition is **PASS**. The governing package
disposition remains **HOLD-HUMAN-APPROVAL** until the named human roles review the
exact source root above and complete the repository's approval and publication
controls.

## TV-A-01 remediation verification

Date: 2026-07-16 UTC
Evidence class: **Static + Ran (lightweight documentation and identity checks)**

TV-A-01 is **RESOLVED**. Static inspection confirmed that
`semantic-differences.md` now:

- limits the positive transfer conclusion to separate writer/parser and
  watershed-consumer interface evidence;
- explicitly leaves continuous nonzero CLI-adapter traversal open; and
- records all 15 final public-safe research objects.

`finding-disposition.md` accepts TV-A-01, records the same correction, and marks
it resolved. The original finding above is retained as historical review evidence
rather than silently removed.

The renewed package-tree documentation check validated 23 Markdown files with
zero lint errors, warnings, or validation errors. A renewed assurance validation
returned `PASS`, zero public reports, lifecycle `DRAFT`, and unchanged per-report
source root
`84a8467ff818411a34c89bf825fc2e9280a7c37c50db9b38636fc831546f4d01`.
The protected public inventory remains only `usersum/assurance/README.md`, and all
four protected-surface hashes remain equal to the frozen values recorded above.

Renewed terminal internal-verifier A disposition is **PASS** with no open
internal-verification finding. The governing package disposition remains
**HOLD-HUMAN-APPROVAL**; this remediation check grants no human scientific
approval, external peer-review status, publication authority, or release
authority.

## American-English normalization renewal

Date: 2026-07-16 UTC
Evidence class: **Static + Ran (proportional identity, reproduction, staging,
and affected-test checks)**

**PASS — TERMINAL INTERNAL VERIFIER A RENEWED ON THE NORMALIZED SOURCE.**

### Scientific-meaning review

A direct comparison of the prior terminal staged report with the normalized
staged report found only British-to-American unit-spelling replacements in the
rendered manuscript and supplement, followed by the expected dependent digest
changes. Static inspection of the source descriptor confirmed the same lexical
normalization in the unit definitions and accessible table text. The
corresponding source-contract mutation fixture now uses the normalized literal.

No numerical value, symbol (`m`, `m2`, or `m3`), dimensional assignment,
equation, recurrence order, allowance, result object, evidence classification,
claim boundary, uncertainty statement, or open CLI-adapter limitation changed.
The normalization is therefore editorial and scientifically meaning-preserving.

### Renewed identity and reproduction

- Named assurance validation passed with lifecycle `DRAFT`, zero public
  reports, and per-report source root
  `08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6`.
- The descriptor SHA-256 is
  `64fbfa6756a86bc98a9656235e1c0df5cf06a414806d7291c7ee37fea69cf5d8`
  and matches the catalog binding.
- The normalized manuscript and supplement SHA-256 values are
  `cd23e31bf0e4c9ce121b18e5da8d072f16cb70c0a5cbf22911043c83091e6c90`
  and `47d1e86a990f636d2f9534fb3153f7c0bb470c0100dd9d0753cb4e20784ecd24`.
  The agent packet binds those outputs and hashes to
  `ef191305e56f817d90056091b3ecef7a3d15e4a4ad6ef52987f2529378acfa1f`.
- Two renewed narrative-seeded staging roots were byte-identical. Both build
  manifests hash to
  `072c260e71b835f8f2b5005dd0fe3e489171f82d444191407f9b4ba705af45f2`,
  and both stage the same agent packet.
- Fresh analytical and H2637 procedure invocations reproduced their retained
  results with semantic JSON equality. The H2637 procedure continued to
  authenticate the raw manifest, HBP, and Parquet objects before
  reconstruction.

### Affected-test renewal and preserved failure evidence

The recorded normalization sweep passed 59/59 affected assurance tests in run
`9fb5644b…`, and the normalized source-contract target separately passed 12/12
in run `24330e4b-475f-429a-95e0-f927bdc1ab59`.

Verifier A's additional broad proportional rerun exposed a preexisting timing
weakness in
`source_drift_after_install_rolls_back_prior_selected_bytes`: the mutator could
miss the very brief installed-backup interval, so the intended source mutation
was not injected. The initial run `7c7f5666-c597-464a-9726-81234f46ec0b` and
isolated confirmation runs `95f885c8-8cb9-4e0a-8a7a-4a6bc3e686ac` and
`6a1a44bf-4c60-4de2-a61f-eca82933404b` preserve that failure evidence. This was
a test-harness observability failure, not a scientific, report-source, or
production assembly failure.

The test fixture was strengthened without production-code changes by using an
8 MiB implementation-path payload and a 60-second observation deadline. On the
final bytes, verifier A ran the complete assembly-contract target: all 9 tests
passed in run `28ebb607-16fd-4a91-9178-b3ca61cbe985`. The final report source
root and descriptor digest above remained unchanged after this test-only
repair. No actionable CRAP scope was introduced because no production Rust
file changed.

### Authority and public boundary

The report, review record, and publication record remain `DRAFT`; human report
lead and scientific approver remain unassigned; no external peer review,
approval lock, release transfer, publication path, export, or vendoring is
claimed. The protected public inventory remains only
`usersum/assurance/README.md`, and all four frozen public/template/catalog/
export hashes remain unchanged.

Renewed terminal internal-verifier A disposition is **PASS** on exact report
root `08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6`.
The governing package and publication disposition remains
**HOLD-HUMAN-APPROVAL**.
