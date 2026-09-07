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

## Focused re-review at a5bfbcd6e

Static: exact `6ab4e5fb6..a5bfbcd6ebcecc4c153d542102d21ca898a9d783`
correction diff, scope amendment, context bindings and newly added identity
membership. Ran: complete focused Rust command and three identity/context tests.
Same independent session; requested high, effective UNOBSERVED.

**A-02 MEDIUM — new experiment membership omits actual execution inputs.**
Location: this package's `artifacts/identity-membership.json:2-14`, compared with
`tools/agents/context_report.py:11-17` and
`tools/agents/test_agent_tools.py:248-278`. The new experiment list omits 27
unpinned after-phase context selections, including `AGENTS.md`, current
`SC-SNOWFREEZE-001.md`, the science registry and the source expansion file.
The reporter reads their current worktree bytes; these are actual inputs to
the measured administrative execution. Some are listed only as evidence/claim
inputs and others are absent from all three membership sets. The Rust and
Python governance checks similarly consume documentation/map/schema inputs.
An unchanged experiment digest therefore cannot prove unchanged experiment
inputs. Section 10 explicitly permits overlapping membership and defines
experiment membership by actual execution dependencies, not file extension.
**Disposition:** correction requested. Include live execution dependencies
(and named command/environment/build bindings as applicable), or explicitly
bound an appropriately narrower claim with dependency proof. Hash comparison
must not support reuse beyond the declared complete dependency boundary.

**A-01 resolved and independently verified.** The committed scope amendment
names exactly the administrative test before its correction, preserves frozen
acceptance, and invokes the owner's administrative-test exception. Independent
Git comparisons confirm the Rust diff is only `22` to `27` plus whitespace;
the impact map is identical between review cuts and still has 27 WAT5 rows.
The four shared-path checks remain intact. The file has 357 lines; no
line-count threshold or duplicated Rust implementation is introduced.

Ran from `/workdir/openWEPP`, exit 0:
`nix develop --offline --command cargo nextest run --offline --test
adr0017_comparator_distrust_ratification_contract --test
advisory_linter_authority_contract`: **11 passed, 0 skipped**, run ID
`1a88979f-e0b0-4e31-a544-2658739ba247`. This independently executes the
assertions after the previously failing count. The existing unused-field
warning remains unrelated to the structural correction; this is not a Clippy
claim. The parent's retained 11/11 log was also inspected.

Ran the same focused Python IdentityTests command as the first review:
**3 passed**, exit 0, including the new distinct-revision accounting test.
The reporter now keys unique coverage by path and content digest, so different
revision bytes cannot silently alias. Static comparison with the frozen
Stage-3 kickoff confirms added historical prerequisites and Core edges in
both before/after selections. The authority/impact matrix and common routing
rules did not change, so prior semantic review remains applicable. Full
measurement/security assurance remains B's assigned scope.

Corrected-cut verdict: **A-01 PASS; overall bounded approval pending A-02**.
The initial FAIL remains historical. No scientific failure is waived, and no
science experiment or heavy execution was performed. Require a focused
manifest correction check and the still-mandatory dual independent verification.

## Final focused re-review at 772eade86

Static: `a5bfbcd6e..a97ccc207` administrative-input/roadmap corrections and
`a97ccc207..772eade8632b0dcec17278733b4a7200a7cb66db` final membership repair.
Ran: independent committed membership/hash comparisons and two focused
governance tests. Same session; requested high, effective UNOBSERVED.

**A-02 resolved and independently verified.** All 27 previously omitted live
context selections now appear in experiment membership. Direct test
documentation/schema inputs and an explicit command/environment/Python binding
were added. Frozen context selections retain named Git revisions and report
content hashes. The new execution record truthfully labels its observation
time rather than manufacturing historical telemetry, and its Python binary
digest matches the currently observed executable.

At a97ccc207 I found one newly introduced omission: `docs/ROADMAP.md` was
selected by the owned-Markdown check but absent from identity membership.
The executor accepted that bounded follow-up; 772eade86 adds it to both
experiment and evidence/claim membership and regenerates identity.json.
Independent committed-byte checks now find zero missing owned Markdown inputs
and zero member-hash mismatches across all three identities.

The explicit administrative-input claim is appropriately bounded:
required-reading-map.md and execution-inputs.json state these file bindings
are not complete Rust executable/build custody and never independently prove
reuse. Accountable dependency/impact review remains mandatory. This resolves
the finding without requiring or claiming a new science experiment or full
Rust build-custody capture.

Roadmap changes are limited to date/current-entrypoint routing and the paused
successor. They preserve historical FAIL/HOLD links, acknowledge later existing
authorization, and expressly grant neither experiment resumption nor production
promotion. The scope amendment is explicit; frozen acceptance is unchanged.

Ran from `/workdir/openWEPP`, exit 0:
`PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m unittest discover -s tools/agents
-p 'test_agent_tools.py' -k GovernanceTests -v`: **2 passed**. No Rust source,
tool implementation or test implementation changed since a5bfbcd6e, so the
independent 11/11 A-01 Rust result above is retained; no redundant Rust rerun
or new science execution is claimed.

Verdict: **PASS for Reviewer A's assigned scope at 772eade86**. A-01 and A-02
are fixed and independently rechecked; no remaining authority/correctness
blocker found. Initial FAIL records remain historical. Package completion
still requires B's disposition, dual independent verification, and final
publication/terminal-diff reconciliation. Updated review/publication bytes
must be rebound without presenting this signoff as their prior digest.
