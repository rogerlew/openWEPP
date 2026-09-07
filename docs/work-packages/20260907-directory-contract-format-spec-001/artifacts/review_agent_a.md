# Independent review A

Static: inspected the base-to-cut diff, complete directory-format specification,
changed schema/procedure/profile/guide/template integrations, parent schema and
provenance rules, existing BEI checker/tests, and package primary evidence.
Ran: own identity/diff checks and strict legacy LSE BEI command below.

Role/session: independent correctness reviewer `/root/format_review_a`.
Configured/requested effort: high requested by assignment and role guidance;
effective runtime settings: UNOBSERVED without session metadata.
Reviewed identity: `6419ce26b9b97be1a8131e27623d8b66477ffd2a`, against
`d8249849d6e015070818be7caf6f8caa75485098`.
Assigned scope: authority preservation, precedence, unique definitions/BEI,
conditional adoption, self-waiver, implementability and frozen acceptance.
This first review did not consult reviewer B's findings.

## Findings

None in the assigned correctness scope. No correction or finding deferral is
requested by this review.

## Independent assessment

- Authority and precedence: directory-format lines 8-27 preserve the entire
  contract's authority and separate representation from scientific amendments.
  Schema/profile integrations explicitly replace physical order only after
  adoption; all logical content remains required. The procedure's existing
  contract-first, unit, comparator and constitutive-suite duties survive.
- Conditional reading: lines 103-149 require applicable dependencies, receiver
  boundaries and closure operands; unknown tasks and discovered missing edges
  expand the reading map. Frozen historical kickoffs remain binding. Role labels
  cannot waive physics, and selective routes are disabled until adoption gate 7.
- Unique definitions and BEI: lines 162-180 distinguish actual marked definitions
  from registry rows, mentions and aliases. Lines 197-224 separate compatibility
  aliases from definitions, forbid alias chains and require registry/definition
  coverage in both directions. Retaining qualified OBL IDs is grounded in the
  existing checker's regular expression and Python regression assertions.
- Preservation and supersession: lines 184-195 inventory binding clauses without
  IDs as well as identified obligations, retain restrictions, and require exact
  superseding authority. The inspected LSE purpose explicitly preserves earlier
  conservation/failure/owner restrictions despite limited version-3 supersession;
  the proposed migration gates accommodate this problem without adjudicating its
  science here. Uncertain supersession and strict PASS-DEFERRED block adoption.
- Implementability: the specification supplies entry fields, exact membership,
  inventories, definition/alias registries, dependencies, coverage, verdicts and
  positive/negative checker cases. Structural checking is expressly bounded;
  preservation and dependency completeness remain independent semantic review
  duties. Existing legacy checker PASS is explicitly insufficient for adoption.
- Acceptance legitimacy/non-deferral: the package contract is byte-identical
  between scaffold `216fb0614` and the reviewed cut. Actual migration/checker
  implementation and measured savings were excluded before substantive edits;
  their future adoption gates are not deferred current acceptance. Independent
  reviews, finding disposition, verification and final reconciliation remain
  current obligations, and the package truthfully remains executing.
- Impact classification: this is substantive prospective governance, not an
  editorial change. No current contract, runtime, test, checker or authority-suite
  bytes changed; no active reading exemption or production selector changed.
  Focused existing tooling/consumer checks are proportionate to that bounded
  claim. No Rust edits: line-count obligations are not applicable.

## Own commands and evidence limits

Working directory: `/workdir/openWEPP`.

- `git rev-parse HEAD`: exit 0, reviewed commit above.
- `git diff --check d8249849d 6419ce26b`: exit 0.
- `git diff --exit-code 6419ce26b -- docs/specifications docs/prompt_templates/required-reading-map-template.md docs/work-packages/20260907-directory-contract-format-spec-001/package.md`:
  exit 0; inspected substantive sources match the reviewed cut.
- `git diff 216fb0614 6419ce26b -- docs/work-packages/20260907-directory-contract-format-spec-001/package.md`:
  exit 0, no changes to frozen package acceptance.
- `git diff --name-only d8249849d 6419ce26b -- crates tests tools docs/specifications/science-contracts/contracts`:
  exit 0, no changed paths in those protected surfaces.
- `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`:
  exit 0, PASS, 16 binding exposure rows. This proves legacy lint behavior only.

Python regressions, Rust consumer assertion and link scanner were NOT RUN by this
reviewer; inspected their recorded evidence and relevant Python source. The
reported missing pytest runner is not labeled PASS. Its recorded direct invocation
executes the three existing assertion functions with fresh temporary directories;
this supports the frozen requirement to run the existing checker regressions.
No checker implementation, LSE migration, scientific equivalence, fresh-agent
usability result or byte saving is certified by this review.

Disposition: no findings to resolve from reviewer A at this cut. Subsequent
substantive amendments require impact assessment and affected focused re-review.

Verdict: GO for the specification design in the assigned scope. This is not
package closure or authorization to adopt directory-v1; remaining package review
and independent verification obligations must still complete.

## Focused corrected-cut re-review

Static: independently inspected the actual correction diff and finding
disposition, then compared both coverage-key ranges with their primary schema
and profile lists. This append preserves the original first review above.
Ran: own corrected-cut identity/diff checks below.
Reviewed identity: `e5971d16da993b8e11a7b3b53d28e8e99a415cac` against
`6419ce26b9b97be1a8131e27623d8b66477ffd2a`.
Role/requested effort unchanged; effective runtime settings UNOBSERVED.

- B-01 correction: the single anchor-in-first-cell encoding now provides a
  concrete Markdown-safe declaration boundary with explicit required fields,
  nonempty cells and rejected alternatives. Examples cannot become definitions,
  and entry compatibility aliases retain their explicit exception. This narrows
  syntax without changing scientific statement, guard, failure or test duties.
- B-02 correction: the 18 artifact keys and 14 profile keys match the two named
  source lists. The text expressly retains subordinate algorithm, calibration,
  typed-failure, compliance and metadata duties. Non-kernel applicability follows
  the profile's existing scope; it does not authorize kernel packages to mark
  applicable obligations absent. Upstream list changes require reconciliation,
  preventing positional keys from silently changing meaning.
- Invalidation/no-waiver: these are substantive specification fixes, and this
  focused inspection supplies affected assurance rather than treating their
  hashes as proof. Frozen package acceptance and protected source/checker/test/SC
  paths are unchanged. The new exact syntax remains prospective under the
  existing adoption gates; no legacy contract becomes newly nonconformant.
  Unchanged legacy executable evidence can be retained for its bounded claim.

Own commands, all from `/workdir/openWEPP`: `git rev-parse HEAD` confirmed the
corrected identity; `git diff --check 6419ce26b e5971d16d` passed;
`git diff --exit-code e5971d16d -- docs/specifications/science-contract-directory-format.md`
passed; `git diff --name-only 6419ce26b e5971d16d -- crates tests tools docs/specifications/science-contracts/contracts docs/work-packages/20260907-directory-contract-format-spec-001/package.md`
returned no paths. All exited 0. No implementation tests rerun by this reviewer;
the correction changes only the specified future format behavior.

Disposition: no new authority/correctness finding. The actual B-01/B-02 fixes
preserve the obligations inspected by reviewer A. B's focused finding-closure
review and subsequent dual independent verification remain separate requirements.
Verdict: GO for the corrected specification in the assigned correctness scope.
