# Independent verification A

Static: primary instructions, templates, frozen acceptance, paired context
inputs/outputs, role settings, review dispositions and terminal claims.
Ran: independent context regeneration, five focused Python tests, TOML parsing,
science-section comparison, instruction discovery and diff checks below.
Role/session: `/root/verify_a`, independent non-forked bounded verifier.
Requested effort: medium; effective runtime effort: UNOBSERVED.
Reviewed substantive cut: `772eade86`; publication candidate: `74e291193`;
base: `033dfe30073bc22aaa30aed877cc301b6745a086`.

## Finding

VA-01 MEDIUM — `docs/work-packages/templates/cqr-nightly-package.md:93`:
the explicit Intended Write Set replaces the historical catalog with active.md,
but the immediately following guard still requires conditional catalog edits,
aggregate admission must enumerate the catalog, and role-authoring.md requires
adding/updating README.md. A generated package therefore has contradictory
scope for its required discoverability work. Frozen acceptance C requires
consistent templates/entrypoints. Add README.md alongside active.md; preserve
both obligations. Executor accepted this finding; correction and focused
re-review/verification pending. This is not a scientific or execution waiver.

## Independent evidence

All commands ran from `/workdir/openWEPP`; exit 0 unless stated otherwise.

- `tools/agents/find-agents --for <assigned artifact>` resolved root/common.
  The same command on both declared science expansion paths resolved exactly
  root plus crates/AGENTS.md or tests/AGENTS.md, already counted in selections.
- `PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -` imported
  tools/agents/context_report.py and called report(root, selection[phase],
  baseline_revision for before, otherwise None). Both generated objects equal
  their complete primary JSON reports exactly, across the same 12 assignments.
  Independently inspected frozen Stage-3 package/protocol and checked the
  recursive science_state -> science_protocol -> 12 Core/history references
  in both phases; profile/schema, LSE and source/test instructions are retained.
- Admin bootstrap unique bytes before/after: author 515018/71218,
  implementer 500458/51620, correctness 501816/52487, QA 501405/52078,
  verifier 500458/50898, runner 51294/25053. Science: author
  1649564/1211837, implementer 1630203/1191536, correctness
  1631561/1192403, QA 1631150/1191994, verifier 1630203/1190814,
  runner 229695/189497. These match final-disposition.md. Root/common =
  16150 bytes; reasoning-role expansion = 5183 admin / 170737 science;
  runner expansion = 0. Repeated exposure is separately reported, including
  science author 1756322/1281408. Frozen task-state files are identical
  before/after. Overruns are disclosed; no session/quota savings are inferred.
- `PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m unittest discover -s
  tools/agents -p 'test_agent_tools.py' -k IdentityTests -v`: 3 PASS.
  Tests exercise ranges, repeated and recursive exposure, distinct content at
  one path, and identity mutations without automatic semantic classification.
- Same command with `-k GovernanceTests -v`: 2 PASS, including changed
  Markdown paths/anchors and role/science routing. This does not by itself
  detect the semantic write-set conflict VA-01.
- Independent inline Python extraction compared all three relocated science
  sections to base common guidance: byte-equivalent section bodies. Canonical
  authority, calibration-readiness obligations, direct-consumer proof and
  independent conservation/anti-tautology acceptance are retained. Testing
  strategy preserves A0/A1/A3 and exact-clean campaign/release obligations.
- Inline `tomllib` parsed all five changed configuration files. Direct inspection
  confirms correctness high, QA medium, verifier medium, existing runner low;
  reviewer configuration write grants are removed. role-settings.md honestly
  records rejected local strict-config commands and distinguishes declared
  settings from unavailable execution telemetry/service evidence.
- `git diff 033dfe300..74e291193 --check`: PASS. Inspected correction-to-candidate
  diff: publication preparation and pending statuses, no concealed tool/source
  change. Read original reviewer findings and their corrected-cut re-reviews;
  inspected retained Nextest log ending in 11 passed / 0 skipped. That Rust run
  is primary inspected evidence, not execution by this verifier.

## Legitimacy, limits and verdict

Non-deferral checked explicitly: frozen acceptance still requires every selected
check PASS, accepted-fix re-review and dual corrected-cut verification. Prior
failures remain historical; inherited failure was corrected, not excluded.
Stage-3 current handoff preserves owner pause, historical FAIL/HOLD, unexecuted
measurements and existing prospective authority without authorizing resumption.
Impact matrix treats changed claims/criteria/references as substantive and
requires actual accepted-fix verification; byte equality is not reuse authority.

No network, science experiment, full Rust suite, restoration fixture or effective
runtime measurement was run by A. Recovery/security execution belongs to B's
independent assignment; no substitution or copied verdict is claimed. Context
figures are structural declared initial selections, not exhaustive subsequent
physics investigation or observed parent/child workflow exposure.

Verdict: BLOCKED for A's terminal scope solely on accepted VA-01. All other
assigned checks above PASS within their stated limits. Require correction,
focused independent re-review and A's bounded fix verification, then final
publication-diff confirmation; do not mark the package complete yet.

## Focused VA-01 correction verification

Static: exact `74e291193..4ea8a5518` correction. Ran: the same focused
GovernanceTests command above, 2 PASS, exit 0; correction `git diff --check`,
exit 0. Requested medium; effective runtime remains UNOBSERVED.

VA-01 is resolved: the sole template change adds README.md alongside active.md
in Intended Write Set. Catalog discoverability, aggregate admission and the
conditional catalog guard now agree. The other two changed files record the
accepted finding and refresh identity; source utilities, context inputs and
paired measurements are unchanged. No authority, assertion or acceptance was
removed. Prior BLOCKED above describes the pre-fix candidate only.

Verdict: PASS for A's assigned terminal scope at corrected cut `4ea8a5518`.
No unresolved A finding. Package closure separately requires focused review
confirmation, B's independent verification and final publication collection/
bounded diff reconciliation; this artifact does not preapprove future changes.

## Final bounded publication confirmation

Static: exact `4ea8a5518..56b315f23` publication diff, both primary focused
review records, B's independent verification record and final completion claims.
Ran: independent inline Python report/identity regeneration and committed-member
hash checks; `git diff 4ea8a5518..56b315f23 --check`; all exit 0 from repository
root. Same session, requested medium, effective runtime UNOBSERVED.

The 15 changed paths are package evidence/status plus catalog/active locator.
No tool, test, configuration, template or scientific implementation changed.
Both reviewers independently approved VA-01. The final reports correctly retain
the resolved findings, historical failures, paused Stage-3 boundary and limits
on custody, runtime telemetry and scientific claims. B's recovery summary agrees
with its primary independent record; an independent read of the retained bundle
manifest reproduces the published SHA-256. A does not claim B's fixture execution
as its own run. Invocation facts agree with the assigned role records; effective
runtime remains explicitly UNOBSERVED.

Both paired context reports regenerate exactly. Active locator wording adds
14 bytes only to author bootstrap: admin after 71232, science after 1211851,
science exposure 1281422. Other reported counts, frozen selections, mandatory
dependencies and expansion figures are unchanged. Experiment and evidence_claim
identities regenerate exactly; all three committed identity aggregates and
their 138/36/7 member content hashes/sizes match candidate bytes. This checks
declared membership, not complete Rust build custody or automatic semantic reuse.

Verdict: PASS for the final bounded publication at `56b315f23`; no new finding
or unresolved A obligation. Completion claims are supported by the independent
corrected-cut records. Parent may collect these final confirmations, rebind only
their publication identity and commit that collection without recursive signoff;
any further substantive claim/source change reopens its affected assurance.
