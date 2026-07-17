# Terminal Verification B — Workflow And Operability

Evidence class: Static and Ran

Current disposition: **PASS**. Attempt 1 was a superseded FAIL against the
pre-response source. Attempt 2 verified all three corrections against the exact
source snapshot closed by heavy Run 5. No current-scope workflow or operability
finding remains.

## Attempt 1 — Superseded Pre-Fix Terminal Review

Three current-scope closure findings were reported. The root executor accepted
B-01 and changed the shared source after this review, invalidating the reviewed
source manifest and requiring a fresh heavy run and terminal verification. The
sections below retain that finding record; they are not closure evidence for
the post-response source.

The reviewed source passed the focused and heavy implementation gates,
preserved the protected report science and public surface, and had no pending
transaction recovery. It did not implement or prove the gate-selection
contract it claimed for implementation rebinding, and its final performance
evidence was not the required ten-trial evidence for the terminal binary and
44-test manifest.

## Findings

### B-01 — Blocking: implementation rebinding emits focused report-data authority

`rebind_implementation` passes `governance-focused` to the common successor
builder. That builder installs
`assurance-amendment-governance-v1` and the pinned 44-test focused command in
the receipt. This conflicts with the canonical specification:

- the mechanical-impact matrix classifies schema, builder, implementation, and
  other unknown changes as `scientific-full`;
- a focused receipt is sufficient only for a report-data-only change; and
- assurance implementation or schema changes are explicit escalation
  conditions.

This is not only a label on the migration receipt. Retained implementation-only
rebind receipts `af193215...` and `f49b27fc...` affect only the generated
identity and review locks, carry `governance-focused`, and contain the focused
nextest argv. The focused runner rejects the current `f53b85cd...` receipt only
because that transition also lists schema paths. The runner does not reject the
`rebind-implementation` operation or generated-lock-only paths, so an ordinary
implementation-only rebind can be admitted as complete focused authority even
though implementation closure requires the full package gates.

Ran: invoking the production receipt runner against the unique current receipt
failed before execution with `schema changes require implementation-package
gates`. Static inspection shows that removing schema drift from the same
operation leaves only runner-admissible generated-lock paths while preserving
the incorrect focused impact and gate declarations.

Required disposition: give implementation rebinds a full implementation class
and non-focused gate contract, make the runner reject the operation regardless
of its affected-path accident, migrate the active receipt/generation, and add a
negative contract test.

### B-02 — High: final performance evidence does not meet the terminal trial contract

The ten current and ten scaled trials used release binary
`b0d13a441094fc47abc0e953686ce6f2dea7e724061dabebc08ad61dd30979a0`
and the then-current 39-test profile. The terminal binary is
`dde766f98fee857d1d546c48c64249b1283d30c2dea2587b5fd98318a3fb1c67`,
and the pinned profile now selects 44 tests. `performance-evidence.md` records
only one 38.35-second run after that change. This does not establish p95 or
maximum from the required minimum of ten isolated trials on the final binary
and selected manifest, and the artifact's assertion that measured paths did
not change is not a substitute for the specified measurement.

Ran: an independent terminal execution selected and passed 44 of 44 tests in
43.85 seconds wall time. That supports plausibility of the limit but is one
observation, not the required final p95 evidence for current and scaled
apply-through-evidence workflows.

Required disposition: repeat the specified ten isolated current and scaled
trials with the terminal release binary and 44-test manifest, or revise the
performance contract before accepting implementation evidence.

### B-03 — High: receipt-forgery and non-escalation requirements lack the required contract coverage

The focused profile has one runner test, and it proves only that an off-archive
copy is rejected. No retained test exercises the runner's mismatched gate ID,
mismatched argv, forbidden escalation token, symlinked archive member,
noncanonical/content-name mismatch, duplicate-current-transition, non-current
generation, or schema/implementation-path rejection. The Python implementation
contains explicit fail-closed checks for those cases, but the package requires
negative contracts, and `finding-disposition.md` states that all of these
behaviors were covered. The 44-test manifest therefore does not mechanically
protect the complete receipt-forgery and non-escalation boundary it is intended
to authorize for future routine amendments.

Required disposition: add deterministic, temporary-root tests for the complete
receipt rejection matrix and pin them in `assurance-amendment`.

## Passing Evidence

Ran:

- The current production-source manifest is byte-identical to the final heavy
  run: 234 sources, SHA-256
  `41191e288047bed7f597f775b31371574a84e128f9289c50f28d4c393be5f85f`.
- The final heavy artifact records PASS for formatting, workspace Clippy, all
  2,071 full-profile tests, dependency policy, and fresh adjudicated CRAP with
  two raw, two adjudicated, and zero actionable rows.
- `verify-generation --base-ref
  15763d7f6d5d4125333d9b7583424c714f5f5ea4` passed through 17 canonical
  transitions to generation
  `1b3c92574e68c675010548f339ca9ebf52f0ff039a6f9d1595a9223c8478d3f5`.
- Production `validate --all`, seeded disposable `build --all`, and `check
  --all` passed for both reports. The implementation rebind check is a no-op.
- `cargo nextest run --workspace --profile assurance-amendment` passed 44 of 44
  selected tests, run ID `b03bd13a-449b-4ea9-b271-6349720a5b01`.
- Recovery inspection reports `pending_cleanup: false`; no active assurance
  symlink or held generation was found.
- A frozen-base/current normalized science projection compared equal for both
  reports. Its digests independently reproduce the recorded values:
  `76d2d469...` for groundwater and `b28dee0c...` for snow/frost.
- `git diff --exit-code <frozen-base> -- usersum` passed with zero diff.
  Production validation reports two internal reports and zero public reports;
  groundwater remains `DRAFT`, and snow/frost remains `IN_REVIEW` without an
  approval or transfer root.

Static:

- Authored catalog/report sources no longer store calculated file digests or
  review roots. The remaining `sha256:` strings are declared immutable external
  science-contract identities, not locally propagated file hashes.
- Test mutation helpers delegate identity calculation to the production
  fixture API; no production manual digest-refresh helper remains.
- The confined transaction retains precommit rollback, postcommit receipt,
  held-tree verification, compare-and-swap, external read-set, symlink, and
  explicit typed-recovery checks. The focused tests exercise its fault and
  selected-generation recovery paths.
- The finite implementation-contract adoption set is explicitly enumerated as
  the v2 README and seven schemas, and report, evidence, principal, catalog, and
  event drift remains rejected. Finding B-01 concerns the gate class assigned
  after that finite adoption, not the write-set confinement itself.
- Governance, source/build, lifecycle, assurance README, and local-CI
  documentation consistently reserve the focused runner for report-data-only
  changes and full gates for implementation/schema changes. That consistency
  is what exposes B-01 in production code and receipts.

## Artifact And Ownership Check

The specification, source/build contract, lifecycle contract, operator README,
mechanical-work queue, baseline inventory, migration record, implementation
reviews, finding disposition, focused gates, performance evidence, and heavy
runner artifact are present and have named ownership boundaries. Terminal
verification A and the final disposition were not yet present when this
verifier ran; those are sequential closure artifacts and must be added after
both terminal reviews and finding disposition. The three findings above prevent
this verifier from issuing PASS before that final sequencing step.

## Attempt 2 — Post-Fix Terminal Verification

Evidence class: Static and Ran

Disposition: **PASS**. Heavy Run 5 and independent terminal checks bind the
three corrections to production-source manifest
`7227650f30319b95c279367c384bd8bed2af40840a124b37041fa19270b41784`.

### Prior-Finding Closure

#### B-01 — Closed: implementation rebind cannot authorize the focused runner

Static: changed and no-op `rebind-implementation` receipts are now calculated
as `scientific-full`. A changed receipt names
`assurance-implementation-package-v1`, and `gate_argv` is empty for every
non-focused impact class. The focused runner admits only `metadata-fast`,
`editorial-fast`, or `governance-focused`. The finite v2 README/schema adoption
surface remains explicit, and manuscript, report, evidence, principal, catalog,
and event drift remains rejected.

Ran: the pinned
`implementation_rebind_adopts_only_the_finite_contract_surface` contract passed
and asserts the full impact class, implementation-package gate, empty argv, and
strict report-drift rejection. The production no-op check returned
`scientific-full`, no gate IDs, and no gate argv. The retained current
transition receipt preserves its immutable pre-fix classification, but the
current runner rejects that schema-bearing historical receipt before executing
any gate. It therefore supplies migration history, not focused authority; all
new implementation rebind calculation follows the corrected contract.

#### B-02 — Closed: terminal performance campaign meets the specified trial contract

Static and Ran: the replacement campaign binds every observation to terminal
release binary
`010cf889644f8c921bcac204cf09330e908709e29b905d83e44240184ebd9c66`
and the final 45-test manifest. It records ten current-corpus and twenty scaled
100-report/32-MiB isolated trials. Independent nearest-rank reconstruction from
the disclosed raw observations reproduced every reported percentile and
maximum:

| Corpus | Apply p95 / max (s) | Runner p95 / max (s) | End-to-end p95 / max (s) |
| --- | ---: | ---: | ---: |
| current | 1.574 / 1.574 | 47.517 / 47.517 | 48.766 / 48.766 |
| scaled | 9.515 / 9.661 | 48.691 / 51.016 | 57.551 / 60.197 |

The retained 60.197-second scaled maximum remains visible rather than being
dropped when the campaign expanded to twenty observations. Current and scaled
transaction p95 values meet their five- and ten-second limits; both end-to-end
p95 values are at most 60 seconds, both maxima are below 120 seconds, and both
remain below the 300-second hard regression threshold.

#### B-03 — Closed: complete receipt rejection matrix is pinned and passes

Static: `focused_runner_rejects_forged_receipt_matrix` exercises mismatched gate
ID, mismatched argv, forbidden `full` escalation, schema-path escalation,
non-current generation, duplicate successor, noncanonical serialization, and a
symlinked receipt. The existing off-archive test remains separate. The complete
amendment integration binary is selected by the focused profile, and `cargo
nextest list` enumerated both rejection tests among exactly 45 selected tests.

Ran: `cargo nextest run --workspace --profile assurance-amendment` passed 45 of
45 selected tests in run `dfca99ae-a329-4528-b5b1-5b3e6618c6a2`. No receipt
case reached gate execution when its archive identity, generation, impact,
gate, argv, path, or canonical-byte contract was invalid.

### Terminal Workflow And Protected-Surface Evidence

Ran:

- A fresh production-source snapshot exactly matched heavy Run 5's final
  234-source manifest; both files hash to
  `7227650f30319b95c279367c384bd8bed2af40840a124b37041fa19270b41784`.
- Heavy Run 5 passed formatting, workspace Clippy, full nextest with 2,072
  tests, dependency policy, and fresh adjudicated CRAP with raw 2,
  adjudicated 2, actionable 0. Touched and untouched actionable counts are
  both zero.
- Anchored generation verification passed through 17 canonical transitions to
  generation
  `1b3c92574e68c675010548f339ca9ebf52f0ff039a6f9d1595a9223c8478d3f5`.
- Production validation, a seeded disposable all-report build, and all-report
  check passed for both reports. Typed recovery inspection reports no pending
  cleanup or held generation.
- Frozen-base/current normalized scientific projections remain exactly equal.
  Their independently reproduced digests are `76d2d469...` for groundwater
  and `b28dee0c...` for snow/frost.
- The protected `usersum` diff from frozen base
  `15763d7f6d5d4125333d9b7583424c714f5f5ea4` is zero.

Static:

- Operator and governance documentation consistently reserve the receipt
  runner for report-data-only focused transactions and route schema, builder,
  implementation, unclear, publication, and release work to their full gates.
  The amendment specification now states the implementation-rebind impact,
  stable full-gate ID, and absence of focused argv explicitly.
- Authored sources retain logical and scientific inputs without calculated
  local-file hashes or review roots. Test mutation helpers delegate identity
  work to the production fixture API; no agent or operator manually propagates
  digest consequences.
- Groundwater remains `DRAFT`. Snow/frost remains `IN_REVIEW` with one active
  review-entry event and no approval or transfer root. Validation reports two
  internal reports and zero public reports. No human authority, public report,
  release transfer, export, vendoring, or WEPPcloud action was created.
- The specification, governance contracts, operator documentation, mechanical
  queue, migration and preservation records, dual implementation reviews,
  finding disposition, focused/performance/heavy evidence, and terminal
  verification A are present. Final disposition is correctly sequenced after
  this second terminal verification.

### Final Disposition

PASS. The corrected workflow mechanically owns bounded amendment bookkeeping,
keeps implementation work out of the focused lane, rejects the complete forged
receipt matrix, meets the final-binary current/scaled timing contract without
hiding the observed maximum, preserves confined transaction and recovery
behavior, and leaves scientific values, lifecycle authority, and the public
surface unchanged. No assigned closure blocker remains.
