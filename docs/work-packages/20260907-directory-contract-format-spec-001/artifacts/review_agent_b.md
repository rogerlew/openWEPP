# Independent review B

Static: format, integration diff, parent schema/profile/provenance rules, package
acceptance, handoff and primary checker/test sources inspected independently.
Ran: strict legacy LSE lint, three existing Python regression functions and
base-to-reviewed-cut whitespace check; all exited 0.

Role/session: `/root/format_review_b`, QA/usability/security reviewer B.
Configured/requested effort: medium per assignment; effective runtime: UNOBSERVED.
Reviewed identity: `6419ce26b` against `d8249849d6e015070818be7caf6f8caa75485098`.
Reviewer A findings were not read before this independent review.
Write scope: this artifact only; no source, configuration or authority edits.

## Checks and scope

Read root/work-package/science-contract/standards guidance, role-review.md,
testing-and-gate-strategy sections 7-10 and 17-18, correctness authority model,
assurance template, package/handoff/reading-map/change-map/gate evidence, complete
format/schema/profile/provenance specification, authoring procedure and changed
integration hunks. Inspected checker implementation and all three Python tests.

The substantive cut is documentation only and retains conditional adoption at
every changed entry point. No existing contract, solver, Rust source, checker,
suite admission or activation bytes change. Focused documentation/tooling
validation is legitimate for this prospective format; immediate full workspace
science regression is not required by an actual changed production invariant.
Rust line-count governance is not applicable. Existing `$pkg/` and `tmp/`
untracked paths were observed and left untouched.

Path containment, symlink rejection, exact inventory membership, no external
fetching, unsupported-format failure and strict deferral behavior are expressly
specified. Routing includes cross-cutting authority, recursive dependencies,
cycle union and expansion on uncertainty. The measurement protocol includes
mandatory dependencies, repeated exposure and later expansion; it claims no
executed savings. Migration and checker conformance execution are explicit future
adoption requirements, not deferred requirements for this specification package.

Own commands, cwd `/workdir/openWEPP`:

- `git diff --check d8249849d 6419ce26b`: exit 0.
- `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`:
  exit 0, PASS, 16 rows; legacy behavior only.
- The exact direct-test Python snippet recorded in gate-results.md, with temporary
  prefix `openwepp-review-b-`: exit 0; all three named regression functions passed
  with a distinct TemporaryDirectory per function. Inspection confirmed these
  tests require only tmp_path and have no pytest-specific hooks/fixtures. This is
  legitimate equivalent assertion execution, not a claim that pytest ran.

The recorded focused Rust consumer result was inspected but not independently
rerun. Directory checker behavior, migration usability and savings were NOT RUN
and are not claimed. The existing checker only collects pre-BEI ID mentions;
its PASS cannot substantiate the new structural rules, as the package discloses.

## Findings

### B-01 — medium — Define a Markdown-safe binding marker grammar

Location: `docs/specifications/science-contract-directory-format.md:162`,
especially lines 165-169.

Evidence: a definition anchor must be "immediately followed by" an invariant
row or heading, but the document does not give the permitted concrete syntax.
Putting a standalone HTML anchor between Markdown table rows interrupts the
table; putting it inside the first cell is a different interpretation of that
requirement. The checker must also distinguish definition headings from aliases,
examples and incidental HTML anchors. The intended unique-definition check is
clear, but two authors/checkers can disagree on valid encoding.

Correction: specify at least one exact, valid table-row form (including anchor
placement) and the heading form if retained; define adjacency/blank-line and
fenced-example handling. Include small accepted and rejected snippets in the
format, without implementing a parser or migrating a contract.

Disposition: proposed acceptance; open pending executor disposition and focused
re-review. This is within frozen format/checker-implementability acceptance.

### B-02 — medium — Disambiguate schema coverage requirement identities

Location: `docs/specifications/science-contract-directory-format.md:153` and
Checker extension contract item 5.

Evidence: coverage must include "each numbered parent-schema and kernel-profile
requirement", but both referenced documents contain several independently
numbered lists. The parent has required sections, draft requirements, calibration
requirements and index rules; the profile has section, algorithm, failure and
compliance lists. A Requirement cell containing `1` or a section title cannot
unambiguously identify the required universe or let a bounded checker detect
missing/duplicate obligations consistently.

Correction: define a stable, explicit coverage key such as source document plus
section anchor plus item number, state which lists the coverage table enumerates
and how subordinate/non-numbered mandatory content remains covered, and specify
unknown/duplicate key handling. A small example suffices; no new authority engine
or implementation is needed.

Disposition: proposed acceptance; open pending executor disposition and focused
re-review. This is within frozen full-schema-coverage acceptance.

## Non-deferral, freshness and verdict

No current acceptance was waived or reclassified as migration work. Independent
review resolution, accepted-fix re-review, corrected-cut verification and terminal
reconciliation are still required before package completion; this review does not
stand in for those roles. No accepted fixes existed at this first review.
Reviewed source matches the named substantive cut; publication artifacts created
afterward require the normal bounded final diff check.

Uncertainty: no directory parser exists to exercise the proposed syntax. Findings
concern specification determinacy, not an observed exploit or production defect.
Runtime effort and actual reading exposure telemetry remain UNOBSERVED.

Verdict: GO-WITH-AMENDMENTS for this specification-only package, conditional on
explicit disposition of B-01/B-02 and focused review of accepted corrections.

## Focused correction re-review

Static: inspected actual `6419ce26b..e5971d16d` format diff and finding-disposition;
original independent findings above remain historical first-review evidence.
Ran: `git diff --check 6419ce26b e5971d16d` in `/workdir/openWEPP`, exit 0;
`git diff --name-only 6419ce26b e5971d16d` confirms bounded specification/package
corrections. Effective runtime effort remains UNOBSERVED.

B-01 CLOSED: the specification now selects one table-row encoding, gives exact
anchor syntax and whitespace, requires same-ID matching and nonempty required
columns, excludes CommonMark fenced examples, rejects heading/standalone forms,
and includes a valid illustrative table plus rejected forms. This resolves the
row-versus-heading ambiguity without a checker implementation or migration.

B-02 CLOSED: exactly 32 source-qualified coverage keys bind the two explicitly
named required-section lists. Unknown and duplicate keys fail; upstream list
changes require reconciliation. Subordinate schema duties and separate metadata
checks remain binding, and non-kernel rows require an applicability rationale.
This supplies a determinate structural universe without reducing semantic review
to heading presence or introducing a new authority engine.

Finding-disposition accurately describes both accepted fixes. No regression found
in path safety, definition uniqueness, inherited schema content, selective-reading
adoption conditions or current acceptance. The changes affect no executable or
fixture input, so unchanged legacy test evidence is retained; it remains legacy
evidence only. No new directory conformance execution or savings is claimed.
Dual independent verification and terminal reconciliation remain required before
closure; neither is waived by this focused review.

Verdict: GO for corrected substantive cut `e5971d16d`, specification-only scope.
Both B findings are closed; no new findings.
