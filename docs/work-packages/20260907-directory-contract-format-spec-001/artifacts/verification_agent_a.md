# Independent verification A

Static: independently inspected the corrected format, parent schema/profile,
authoring procedure, science-contract guide, reading template, provenance rules,
base-to-cut integration diff, frozen acceptance, reviews and finding disposition.
Ran: own identity/whitespace checks, strict legacy LSE lint and the three existing
Python regression functions. No directory-format implementation was executed.

Role/session: `/root/format_verify_a`, independent specification/routes verifier.
Configured/requested effort: medium; effective runtime settings: UNOBSERVED.
Reviewed identity: `f68411f8f1193e5eb7c4c43fd34152b4399157e3`, against
`d8249849d6e015070818be7caf6f8caa75485098`; corrected substantive source:
`e5971d16da993b8e11a7b3b53d28e8e99a415cac`.
Write scope: this artifact only. Unrelated `$pkg/` and `tmp/` remain untouched.

## Independent checks

Read root/common/role-verification instructions, applicable science/standards
guidance, testing strategy sections 6.2 and 7-10 and 18, local gate selection,
assurance template, package/handoff/reading-map/change-map/gates/line-count and
final-disposition records. Conclusions below come from the primary documents
and actual diffs, not the executor's summary or the other verifier.

| Requirement | Result and primary evidence |
| --- | --- |
| Whole-set authority and rule consistency | PASS. Format Authority and adoption, Layout and membership, and Schema coverage preserve the complete normative set and all subordinate content. Schema/procedure/profile changes explicitly condition the representation change on adoption. Existing contract-first, typed failure, unit, calibration and external-suite duties remain in their owning documents. |
| Safe reading routes | PASS. Entry interface and Mechanism chapters require entry/cross-cutting constraints, concrete applicability, recursive section dependencies and finite cycle unions. Unknown applicability expands reading. Solver review reaches affected evaluators; closure verification reaches physical operands. Role labels do not exempt obligations. The guide and template retain current whole-contract and frozen kickoff requirements. |
| Adoption and preservation | PASS. Adoption gates require clause-level bidirectional provenance, independently adjudicated supersession, checker implementation/testing, source-coupled consumer reconciliation, dual assurance and fresh-agent reading exercises before selective reading. Unidentified binding clauses, retained qualifiers, historical FAIL/HOLD and activation restrictions cannot disappear through relocation. |
| B-01 accepted grammar correction | PASS, CLOSED. Actual correction selects one first-cell anchor plus matching backtick ID, exact quoted anchor syntax and spacing, required nonempty fields, fenced-example exclusion and rejected heading/standalone forms. The invariant example retains all six parent-schema columns. Alias exceptions are explicit and cannot satisfy definition existence. Registry coverage is bidirectional; semantic sufficiency remains review work. |
| B-02 accepted coverage correction | PASS, CLOSED. Compared the two source lists directly: Artifact Required Section Order has 18 items and Profile Required Section Schema has 14. The format enumerates exactly their two qualified key ranges, including correct invariant examples 08 and 07. Unknown/duplicate keys fail; upstream list changes require reconciliation. Metadata and subordinate calibration, algorithm, typed failure, enforcement and compliance duties remain mandatory. |
| Frozen acceptance and non-deferral | PASS for legitimacy. Package acceptance is byte-identical from scaffold 216fb0614 through the frozen cut. Actual migration, checker implementation and measured savings were excluded before substantive work; future adoption gates are not retroactively deferred specification acceptance. Both accepted fixes have actual source changes and separate focused re-reviews. Dual verification and terminal reconciliation remain current obligations; the frozen disposition correctly says NOT COMPLETE. |
| Validation impact and truthful claims | PASS within specification scope. Protected crates/tests/tools/canonical SC paths have no base-to-cut changes. This is substantive prospective governance, with no changed effective production invariant or suite admission. Focused existing tooling/documentation checks are proportionate; no Rust line-count requirement applies. LSE lint supports legacy BEI only. The recorded missing pytest runner is disclosed; direct execution runs the unchanged three assertion functions, whose only fixture input is tmp_path. No scientific equivalence, migration, directory-checker conformance or savings claim is certified. |

The freeze after e5971d16d changes only six package assurance/continuation records;
the corrected specification and integration source are unchanged. Original review
B findings remain visible alongside their focused closure reviews. There are no
rejected or deferred findings requiring rationale verification.

## Own executed evidence

All commands used `/workdir/openWEPP`. Each check below exited 0:

- `git diff --check d8249849d f68411f8f`.
- `git diff --exit-code f68411f8f -- docs/specifications docs/prompt_templates/required-reading-map-template.md docs/work-packages/20260907-directory-contract-format-spec-001/package.md`: no differences; inspected sources match the frozen cut.
- `git diff --exit-code 216fb0614 f68411f8f -- docs/work-packages/20260907-directory-contract-format-spec-001/package.md`: no acceptance drift.
- `git diff --exit-code d8249849d f68411f8f -- crates tests tools docs/specifications/science-contracts/contracts`: protected executable/test/contract sources unchanged.
- `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`: PASS, 16 rows.
- Direct Python regression execution below: all three named tests PASS.

```sh
.venv/bin/python - <<'PY'
import importlib.util
import tempfile
from pathlib import Path
path = Path('tests/python/test_check_sc_binding_exposure.py')
spec = importlib.util.spec_from_file_location('binding_tests', path)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
for name in sorted(n for n in vars(module) if n.startswith('test_')):
    with tempfile.TemporaryDirectory(prefix='openwepp-verify-a-') as tmp:
        getattr(module, name)(Path(tmp))
    print('PASS', name)
PY
```

Findings: none remaining in this verifier's assigned scope. B-01 and B-02 are
independently verified closed; no new correction requested.

Uncertainty: Rust consumer and Markdown scanner results were inspected in
gate-results.md, not independently rerun by A. No directory parser or migrated
contract exists to execute the future conformance/adoption cases. Actual usability,
scientific preservation after relocation, reading exposure and savings require
the separately authorized pilot. Effective settings and workflow-total telemetry
remain UNOBSERVED. This artifact does not substitute for verifier B or the final
bounded publication-diff reconciliation.

Verdict: PASS for the corrected specification, authority/routes consistency,
accepted-fix closure and specification-only claim boundaries at the named cut.
