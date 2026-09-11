# Bounded execution and external Pro review

Status: IMPLEMENTED; final committed-source export pending. Source base: b0fbfbf7ee769f4cbe6c4937f9f34e184360ccd2.
Owner authorization: 2026-09-11, implement the full quota strategy as a work-package.

## Objective and scope

Make bounded Codex execution and owner-mediated ChatGPT Pro analysis/review the
normal workflow. Preserve the existing `/tmp/openwepp*.md` incoming handoffs.
Implement model pins, small concurrency, checkpoint stopping rules, reusable Pro
role/kickoff guidance, and a small outgoing review-packet utility. Keep one
maintained package record and the existing independent review/science obligations.
No B01 execution, production Rust, scientific authority, runtime validation
selection, dependency, or frozen TESTGATE changes. No plugin installation,
automated ChatGPT messaging, new billing route, or quota dashboard.

## Active checkpoint

Deliver the strategy and validate the actual administrative workflow. Two failed
correction cycles or 60 minutes of agent-active wall time bound this checkpoint;
exclude recorded intervals spent only waiting on machines, reviewers or the owner.
At the boundary retain incomplete work and pending acceptance; do not resume B01.
Budget accounting uses a conservative 2026-09-11T11:20:00Z anchor, earlier than
the package file creation at 11:30:07Z; no waiting intervals excluded.
The initial implementation and its first validation form one attempt; a changed
implementation followed by failed verification consumes a correction cycle.

## Authority and acceptance

Authority: root and write-path AGENTS; common package and role-review guides;
testing-and-gate-strategy sections 7-10 and 18; prompt-wording-guidance.
Governance changes require two complementary independent reviewers. Source code
here is an isolated administrative exporter, not a validator or admission engine.
No production/kernel/test-selection path changes, so Rust workspace regression is
not applicable. Required checks: focused exporter behavioral tests, real CLI packet
generation from immutable repo commits, TOML parsing and installed Codex config
loading, changed Markdown reference integrity, diff whitespace and scope review.

Acceptance:
- Terra medium execution and explicit cheaper delegate pins; two concurrent
  children, one writer, no automatic fan-out or expensive fallback.
- Owner-approved checkpoint bounds apply to correction loops, without waiving
  current scientific acceptance or changing historical FAIL/HOLD outcomes.
- External review identifies source, scope, independence, static versus executed
  evidence and fix verification; missing review remains unmet.
- Pro role and new-thread kickoff produce a usable bounded incoming handoff.
- Exporter produces selected committed source, diff and context with exact
  identities; it does not execute supplied text, publish, infer authority, or
  claim coverage of detached experiments or omitted source.
- Usage measurement records actual coverage and unknowns; no savings claim from
  bytes, elapsed time or account-wide quota alone.

## Current state and evidence

Clean intake at base. Installed CLI: codex-cli 0.153.4. Existing config allows ten
children and leaves correctness/QA/verifier models unpinned. Existing B01 handoff
authorizes package-wide continuation and unlimited ordinary corrections. Existing
lightweight guides already preserve reviewer reuse and one package narrative.
Official configuration references inspected 2026-09-11:
[configuration](https://learn.chatgpt.com/docs/config-file/config-reference) and
[subagents](https://learn.chatgpt.com/docs/agent-configuration/subagents).
Their configuration precedence is guidance, not proof of this session's model.

Implemented 25 scoped files: root/standards/package routing and templates;
explicit model/effort configuration plus narrow worker/explorer/science-adviser
roles; Pro role and new-thread kickoff; selected-source review exporter/tests and
usage instructions. No third-party plugin code was imported. The reviewed
[Astra Advisor](https://github.com/DannyMac180/astra-advisor/tree/c72d3280551f118eba51a5884e3971a0c0058aa6)
and [Astra-Luna Orchestrator](https://github.com/donvito/codex-astra-luna-orchestrator/tree/a513e2fca0cea7d495c7fd95ea8882ac614438b4)
informed explicit routing and compact exchange; permanent premium coordination,
mandatory delegation, fresh reviewer waves and unvalidated billing estimates
were not adopted.

Ran from /workdir/openWEPP on the base plus this scoped diff:
- `.venv/bin/python -m unittest discover -s tools/agents -p 'test_review_packet.py' -v`:
  5 PASS. Tests exercise immutable versus dirty source, omitted/deleted files,
  fences, literal inputs, invalid paths/revisions, binary/link/size rejection,
  output overwrite refusal and failure before output creation.
- `.venv/bin/python -m unittest discover -s tools/agents -p 'test_*.py'`:
  initial 28-test run had one existing navigation failure. Static inspection of
  HEAD confirmed common guidance already lacked author/implementer/runner links.
  Restored those relevant navigation links without changing the test; final
  28 PASS (1.017 s), including after CG-001 correction.
- `codex app-server --strict-config --stdio`: JSON-RPC initialize with
  clientInfo {name: openwepp-config-check, version: 1}, then config/read with
  {cwd: /workdir/openWEPP, includeLayers: true}. Kept stdin open for responses.
  PASS: enabled project layer, Terra/medium primary, Luna/medium delegate defaults,
  cap 2, all seven role paths resolved. A prior request without cwd returned user
  Astra/high defaults; it did not load project settings. No model turn was started.
  Python tomllib parsed every declared config_file and checked explicit model/effort.
- `.venv/bin/python tools/agents/review_packet.py --root /workdir/openWEPP --base b0fbfbf7^ --head b0fbfbf7 --path docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/.gitattributes --context AGENTS.md --output /tmp/openwepp_strategy_packet_smoke_20260911.md`:
  PASS, selected nonempty immutable diff and context exported; source inspection
  only, no B01 execution. This is tool smoke evidence, not a scientific review.
- Changed Markdown local-reference scan: 31 PASS (new catalog links only; no
  historical catalog sweep). `git diff --check`: PASS. Final scope inspection:
  no crates, Cargo or science-contract paths changed.

One review correction (CG-001) and one navigation repair verified successfully;
zero unsuccessful correction cycles after corrective edits. No usage-token totals
or actual model-routing execution observed; quota savings remain UNOBSERVED.
Owner handoff timing and the 70% premium-token reduction target remain for a future
owner-selected pilot, not prerequisites to administrative tool delivery.

Next: freeze scoped implementation and exercise exporter on that committed source.

## Review and corrections

Two complementary native reviewers were explicitly requested without inherited
history: strategy_correctness (Sol/high) and strategy_qa (Terra/medium). These are
requested routes, not a measured token allocation. Neither authored this change.
They returned findings for incorporation and made no source edits.

Correctness/governance reviewer identified CG-001: draft wording barred a designer
from correctness review but did not consistently bar them from required QA review.
Fixed all guidance, kickoff and generated review text: authors/material design
advisers cannot satisfy either independent review; required reviewers are distinct
from each other and the author/adviser. Same reviewer verified the correction.
Final reviewer response, verbatim:

> Static: CG-001 correction verified.
>
> Ran: Scoped `git diff --check` passed. No runtime tests or edits.
>
> **CG-001 — RESOLVED.** The guidance now bars authors and material design advisers from either required independent review, including QA/evidence. Required reviewers must be distinct from each other and independent of the author/adviser. Same-reviewer fix verification remains intact.
>
> Final scoped verdict: **PASS. No remaining blockers.**

QA/tooling reviewer corrected two descriptions in its own initial report (hashes
identify attachment bytes, and literal wildcard filenames are allowed). Corrected
review record, verbatim:

> Identity/scope: independent QA/tooling review of the `.codex` config/roles, review-packet exporter/tests, Pro role/kickoff, README usage and common role navigation; no edits and no Rust/B01 execution.
>
> Static: No blocking finding. `review_packet.py` computes SHA-256 over the exact included attachment bytes and labels each attachment with that digest (lines 110–119); those hashes identify packet contents only and do not authenticate the asserted run, publisher, or workflow. The Git calls are argument-vector calls with `--literal-pathspecs`; `*.md` is only rejected by the current test because no file of that literal name exists—literal wildcard filenames are valid if committed and must not be characterized as categorically rejected. The exporter remains safe against supplied-text execution and accidental overwrite: it emits committed blobs, builds before exclusive output creation, and does not shell-evaluate inputs. The README accurately states hashes' limited meaning and quiescent attachment caveat.
>
> Static: Confirmed the common guide now restores discoverable role procedure navigation at `docs/work-packages/AGENTS.md:20-21`: authoring, implementation, review; runner/verifier only when assigned. This is compatible with the focused reading rule and does not add mandatory verifier work. README packet usage accurately supplies concrete selected paths/context/attachments, committed-source limits, LFS behavior, and same-reviewer correction handling.
>
> Ran: prior direct evidence remains `.venv/bin/python tools/agents/test_review_packet.py` PASS, 5 tests/0.352 s; it is meaningful behavioral coverage as previously described. Supplied evidence from parent (not independently run by me): the full `tools/agents` suite is now 28 PASS; real immutable-commit packet `/tmp/openwepp_strategy_packet_smoke_20260911.md` was generated from `87f4..b0fb`, selecting B01 `.gitattributes` plus `AGENTS.md`, without execution; strict config load with cwd `/workdir/openWEPP` resolved project config/roles and reported Terra/medium, Luna/medium, and cap 2. I therefore do not recast runtime configuration loading as my Ran evidence.
>
> Findings: none. Verdict: PASS for assigned QA/tooling/static scope; suitable for verbatim incorporation. Final package disposition should retain parent’s exact config-load/test/packet commands and result identities as execution evidence.

No second verifier wave. Governance correction was verified by its original
reviewer; unchanged exporter behavior reused QA evidence and passed executor tests.

## Disposition

Implementation and independent reviews pass; final own-commit export pending. B01 remains PAUSED BY OWNER at the pushed stop point. Runtime model
selection for future sessions and measured savings must be distinguished from
static configuration delivery. The proposed cold145 pilot is not authorized by
this administrative package.
