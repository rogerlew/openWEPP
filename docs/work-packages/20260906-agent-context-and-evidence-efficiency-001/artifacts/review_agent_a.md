# Independent review A

Static: primary base-to-cut guidance, role configuration, identity utility/tests,
package acceptance and retained command log. Ran: two focused administrative
tests and independent Git/JSON/section comparisons; no Rust suite or science
experiment run by this reviewer.

Role/session: `/root/review_a`, independent non-forked correctness review.
Configured/requested effort: high; effective runtime effort: UNOBSERVED.
Reviewed cut: `6ab4e5fb67740bcf746e219d6ae83e8ebce9d5d1` against
`033dfe30073bc22aaa30aed877cc301b6745a086`. Later working-tree fixes are excluded.

## Findings

- **A-01 HIGH — selected failing gate prevents frozen acceptance.** Location:
  `artifacts/gate-results.md:11-13`, `package.md:27`, and
  `tests/integration/advisory_linter_authority_contract.rs:185` (repository
  paths for the first two are this package). The retained corrected Nextest
  log `/tmp/openwepp-context-governance-nix-corrected.log` reports 10/11 passed,
  exit 100, with 27 WAT5 rows versus expected 22. I independently compared
  committed JSON: base and cut each contain 163 entries, including 27
  WAT5-only entries; every entry is identical. Only `policy_sha256` changes,
  and its new value correctly hashes the cut's live standard. The entire
  Rust test file is unchanged. Thus the failure is inherited, but the frozen
  closure criterion still explicitly requires all selected checks PASS;
  inheritance does not authorize retrospective exclusion or a reviewer waiver.
  The changed hash assertion executes before the failure; assertions after
  line 185 do not receive execution evidence from that test run.
  **Disposition:** accepted by executor, correction pending at this cut.
  A prospectively authorized, narrowly justified inventory-assertion repair
  may preserve the existing bindings and requirement, followed by the complete
  focused test command and independent focused re-review. Do not delete/filter
  the test, change science entries, or label its failed execution PASS.

## Authority and mutation challenge

The three relocated science sections and four specialized-workflow sections
are exactly equal to their baseline bodies after trimming outer whitespace.
The common and role guides retain recursive mandatory references, conditional
kernel/consumer/conservation routing, canonical SC priority, contract-first
sequencing, independent reconstruction, consumer proof, anti-tautology and
non-deferral. Context targets explicitly cannot omit authority. Active
templates route independent roles without transferring authoring write grants.
Stage-3 handoff preserves pause, failed broad checks, unrun comparisons and
historical attribution limits; its locator grants no experiment authority.

Independent challenges against testing strategy section 10:

| Mutation | Required disposition confirmed |
| --- | --- |
| Equivalent typo/layout | Bounded semantic inspection; publication identity changes; unchanged substantive evidence may survive. |
| Report claim/verdict/number/unit | Changed claim assurance reopens even when only Markdown/publication bytes change. |
| Acceptance threshold/predicate/exception | Re-adjudicate legitimate authority; no retrospective self-waiver. A-01 applies this rule. |
| Result row | Affected claim/reconstruction assurance invalidates; corrected data require verification. |
| Reconstruction code/output | Rerun affected reconstruction and verify claims; a new hash cannot establish correctness. |
| Fixture/config/protocol | Affected experiment and dependent claims invalidate unless an explicit dependency proof excludes the change. |
| Source/build/executable/environment/command | Same experiment invalidation; publication classification cannot preserve dependent experiment claims. |
| Broken/moved required reference | No reuse until resolution and equivalent authority/dependency bindings are verified. |

Accepted finding fixes require actual verification and focused re-review;
unknown impact requires conservative escalation. Hashes classify byte changes,
not meaning or acceptance. A substantive claim in a publication file still
reopens assurance. Final publication binding avoids recursive self-invalidation
without relaxing exact-clean campaign or exact-source release requirements.

## Independent evidence and limits

- Ran, exit 0: `PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m unittest discover
  -s tools/agents -p 'test_agent_tools.py' -k IdentityTests -v`, from
  `/workdir/openWEPP`: 2 passed. Tests confirm explicit membership changes and
  recursive/range exposure accounting; semantic dispositions above are review
  judgments, not claimed automated semantic classification.
- Ran, exit 0: Python read-only `git show`/JSON comparisons established the
  WAT5 counts, unchanged map entries/test, correct live hash and exact seven
  relocated section bodies. An initial section-comparison regex failed;
  corrected comparison completed successfully. No repository files were
  changed by those diagnostics.
- Static: all four shared-path binding lists checked after the failed count
  assertion match the test's current expectations. This does not replace the
  required successful complete test execution.
- Rust line-count/duplication checks: not applicable to this stable cut; its
  base-to-cut diff contains no `.rs` edits. Any later Rust fix needs its own
  scoped review and line-count record.
- Recovery/security execution, complete context measurement, CLI support and
  final package identity reconciliation remain assigned to B/verification.
  Runtime effort and workflow-total consumption are UNOBSERVED. This record
  supplies one review, not dual verification or package completion.

Verdict: **FAIL for closure at the reviewed cut**, due to A-01. No additional
authority/correctness blocker found in the assigned guidance and identity
policy. Re-review the accepted fix and freeze the corrected cut before the
required two independent verifications.
