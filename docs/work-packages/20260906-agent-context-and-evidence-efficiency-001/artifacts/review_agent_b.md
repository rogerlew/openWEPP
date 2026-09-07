# Independent QA review B

Static: primary files at 6ab4e5fb67740bcf746e219d6ae83e8ebce9d5d1 versus
033dfe30073bc22aaa30aed877cc301b6745a086; acceptance, handoff, context
selections/reports, role configurations/procedures, testing strategy and tools.
Ran: own 19-test suite, report regeneration and disposable custody probes below.
No Rust/heavy/Stage-3/network run.
Role/session: independent Reviewer B, /root/review_b, non-forked assignment.
Configured/requested QA effort: medium; effective runtime: UNOBSERVED.

## Findings

- B1 — HIGH — tools/agents/evidence_bundle.py:63-76, link_safe.
  Lexical traversal ignores other restored symlinks. Own reproduction extended
  CaptureTests' disposable source selection with `alias -> .` and
  `escape -> alias/../fixture/fixture`. Capture, verify and restore succeeded;
  reading restored `source/escape` returned restored `fixture/fixture` bytes,
  outside its source group. Longer chains can traverse outside the destination.
  This violates the documented confinement guarantee and frozen acceptance F.
  Reject unsafe composed targets before restoration, or adopt a documented
  conservative subset. Cover staged and working symlinks independently.
  Disposition: executor explicitly accepted; fix and independent re-review
  pending. Not resolved by this review.

- B2 — MEDIUM — tools/agents/evidence_bundle.py:47-54, destination.
  Path.absolute() preserves `..`; overlap checks compare lexical paths.
  Own disposable test created sibling `other/`, then successfully captured to
  `<base>/other/../original/bundle`, inside the original source checkout.
  This violates the outside-checkout recovery boundary and risks losing the
  recovery copy with its source. Normalize safely or reject noncanonical
  spellings before overlap checks/writes, retaining symlink protection.
  Add a regression for this destination spelling.
  Disposition: executor explicitly accepted; fix and independent re-review
  pending.

- B3 — MEDIUM — artifacts/context-inputs.json, before/after science_state
  dependency and science_sensitive assignments; corresponding reports.
  The selected Stage-3 package at 3c6813d39 calls `prompts/active/kickoff.md`
  its full executable specification and binding sampling/decision protocol
  (package.md:12). Neither selection counts that 25,550-byte file or assigns
  its triggered expansion. Only profile->schema is a recursive edge. The
  reports reproduce exactly but do not establish frozen B's complete mandatory
  reference accounting. Include the bound protocol at the applicable stage;
  audit other explicit mandatory edges and regenerate both reports under the
  same convention. This does not require counting every ordinary hyperlink.
  Disposition: executor explicitly accepted; corrected-cut re-review pending.

## Independent checks and assessment

Ran from /workdir/openWEPP:
`.venv/bin/python -m unittest discover -s tools/agents -p 'test_*.py'`:
19 tests, exit 0. The existing restoration test removes its original repository,
checks distinct index/worktree bytes, modes, symlinks, deletions/rename names
and untracked bytes, then executes the restored fixture against restored
input/result. This supports standalone byte custody, not scientific experiment
qualification, rebuildability or original Git-history reconstruction.

Own Python probes imported CaptureTests/evidence_bundle and added the links or
destination above, using only TemporaryDirectory fixtures with owner cleanup.
An obvious lexical escape `alias/../../outside` was rejected; the composed
escape above passed, exposing the existing simple negatives' coverage gap.

Ran context_report.report for both phases with context-inputs.json and its
baseline revision: output equals committed before/after reports exactly.
Administrative QA bootstrap unique bytes fall from 501,405 to 52,078; science
QA from 1,222,606 to 779,428. This establishes structural reduction for the
declared selections, subject to B3, not session/quota savings. Expansion and
repeated exposure are separate and workflow-total is correctly UNOBSERVED.
Science overruns retain authority. Administrative author is 71,218 bytes,
slightly above 64 KiB; the target is not a quota.

Independently challenged all requested mutation classes against strategy
section 10 and IdentityTests: equivalent typo/layout requires accountable diff
review; broken required references suspend reuse until resolved; claim/verdict
changes reopen claim assurance even inside a report; threshold changes require
legitimate re-adjudication; result rows and reconstruction require independent
checking and affected reruns; fixture and source/executable changes invalidate
affected experiments and dependent claims. Tests establish byte-membership
detection, not semantic classification. The utility appropriately grants no
automatic reuse decision. Accepted fixes need actual verification and focused
re-review. Campaign/release exact-clean requirements remain present.

Configuration diff removes reviewer config-write grants and requests
high/medium/medium/low for correctness/QA/verifier/runner. Tests parse TOML and
inspect values. role-settings.md truthfully separates syntax, failed local
strict-config command, bundled model information and unavailable effective
runtime/service evidence. It does not claim the runner model executed or was
available. No Rust changes exist; Rust line-count obligations are inapplicable
and broad fmt/Clippy/test/deny are not justified by production changes here.
The necessary administrative security negatives remain incomplete per B1/B2.

## Non-blocking follow-ups and uncertainty

The utility is small, stdlib-only and explicitly selects files without running
recorded commands. Context reads deduplicate by path/byte position rather than
content identity: future multi-revision reads of one path within a role need
different accounting. Current selections do not exercise that case.
Owner: package executor if extending measurement use; not a waiver of B3.

Commands/environment are caller assertions; hashes detect corruption, not
hostile replacement. No adversarial atomic snapshot guarantee is claimed.
Independent tests use synthetic fixtures only. Runtime effort and total
conversation exposure remain UNOBSERVED.

Non-deferral explicitly checked: frozen acceptance requires all selected
checks PASS. gate-results.md retains a selected Rust test failure (inherited
WAT5 27 versus 22). Inheritance is useful diagnosis, not PASS or permission to
waive frozen acceptance. Closure remains blocked unless legitimately resolved
without weakening acceptance; no heavy rerun was assigned to this review.
Independent verification and accepted-fix re-reviews are also pending.

Verdict: FAIL for the reviewed stable cut. B1/B2 violate recovery safety and B3
leaves required measurement incomplete. No QA pass or closure approval yet.
Assigned write artifact only: this file.
