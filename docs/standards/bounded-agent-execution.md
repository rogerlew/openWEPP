# Bounded agent execution

This standard implements the owner's checkpoint and external-review workflow.
The [package guide](../work-packages/AGENTS.md) owns acceptance and review counts;
[testing strategy](testing-and-gate-strategy.md) owns validation. Budget boundaries
stop execution, never convert incomplete acceptance into PASS.

## Checkpoint contract

Keep the overall objective and backlog in package.md. Before execution, state the
active checkpoint's source identity, one defect/hypothesis or bounded deliverable,
permitted changes, selected checks, run allowance, and stopping condition there.
Incoming `/tmp/openwepp*.md` documents are owner-adopted instructions; incorporate
their current scope into package.md without creating a parallel live narrative.
An external model's proposed authorization only becomes authorization when the
owner adopts it. Preserve already granted permissions and explicit exclusions.

Default bounds are two unsuccessful correction cycles or 60 minutes of active
wall time, whichever comes first. Active wall time includes thinking, reading,
editing, command orchestration and review work, but excludes recorded intervals
spent solely waiting for a machine, reviewer or owner. Concurrent work counts
once for wall time; delegate usage still counts separately for cost. Record start,
excluded waits and correction count compactly in package.md at handoff. These are
executor-enforced bounds, not a hard runtime timer or token cap. The owner may
explicitly revise them. A correction cycle is a corrective edit followed by
verification that still fails the checkpoint; ordinary compile, collector,
integration and review fixes are included. Do not reset the count by renaming the
failure, changing threads, delegating, or compacting context.

For scientific execution, normally allow one planned authentic result-bearing
run after focused checks. Required reproductions and comparison pairs must be
named prospectively in the checkpoint allowance; "one run" never removes a
required control or acceptance test. A new downstream failure becomes evidence
for the next checkpoint, not automatic scope growth. At the limit, finish only
safe state preservation and return an incomplete disposition with pending checks.
Record ongoing job identity and collection command; do not kill useful long jobs
or keep a reasoning agent polling them. Starting the next checkpoint requires
owner direction or a previously explicit bounded sequence, not the old general
instruction to finish the package. A fresh thread does not renew a spent budget.

## Models and delegation

Repository defaults for new trusted sessions:

| Responsibility | Model / effort |
| --- | --- |
| Primary bounded implementation | gpt-5.6-terra / medium |
| Narrow extraction or simple delegated edit | gpt-5.6-luna / medium |
| Scientific design and substantial static review | Owner's ChatGPT Pro workflow |
| Local independent correctness when external review is unavailable or insufficient | gpt-5.6-sol / high |
| Local independent QA/evidence | gpt-5.6-terra / medium |
| Exceptional local scientific reasoning | gpt-6-astra / high, for a named unresolved issue |
| Declared command batches | Local scripts; existing Spark runner when useful |

The [project config](../../.codex/config.toml) pins these local roles. Model names
are explicit routing choices, not a quality guarantee. Use at most two concurrent
children, one source writer, and no nested spawning. A review phase may use both
children. No mandatory delegation pipeline: the primary executes directly unless
a bounded independent task or required review justifies delegation.

Use minimal explicit task packets with `fork_turns="none"` where supported.
Named roles can override spawn settings; verify actual controls and report an
unavailable requested route without silently using an expensive parent/fallback.
If the runtime cannot enforce a pin, do not claim it did. Reuse reviewers through
their fix verification; use fresh primary context at a new checkpoint with the
short current package brief and targeted source, not full historical threads.

Prefer external Pro analysis before premium local reasoning. A local escalation
must name the question and why repository execution is needed; one scoped answer
and one correction follow-up are the default allowance. Missing required review
or unresolved science remains incomplete when the budget is exhausted.

These files configure future sessions; they do not switch an already running
primary model. Start a fresh trusted repo session and check its model/settings.
UI/CLI overrides and the host's live tool policy take precedence. Configuration
loading is not proof of actual model routing. No API-key or nested CLI workaround
is part of this workflow.

## Evidence and manual exchange

Use one mutable isolated development copy per checkpoint where isolation is
needed. Run focused checks before freezing a result-bearing source/binary. Never
edit a frozen running tree; keep distinct targets for distinct frozen sources.
Preserve failed attempts and exact experiment inputs with existing Git/patch and
evidence tooling, without generating a complete new archive for every typo.

Use [the Pro role](chatgpt-pro-role.md) and
[kickoff](../work-packages/templates/chatgpt-pro-kickoff.md) for manual exchange.
[review_packet.py](../../tools/agents/review_packet.py) exports explicitly selected
committed files and a diff; it neither chooses review coverage nor supplies a
validation verdict. Detached experiments need their actual retained source and
input identities, never a primary-checkout SHA substituted for experiment state.
Keep results compact: command, source/run identity, exit, actual test count,
first relevant failure and full-log path. Zero selected tests is not a pass.

The owner transfers packets and saves returned reviews. Bind each external review
to its reviewed source and independent role in package.md. Neither an author nor
a material design adviser can count as either required independent reviewer.
Keep required reviewers distinct from each other. Retain the original
response, then send only affected fixes back to the same reviewer conversation.
Source drift requires affected review, not automatic full re-review. Do not repeat
accepted static analysis merely because it happened outside Codex.

## Measurement

For the initial pilot, record checkpoint outcome, model/thread IDs when observable,
correction count, handoff time and observed input/cached/output usage for all
participating threads, including failed and review work. Missing fields and
missing threads remain UNOBSERVED; mixed-model cumulative totals cannot be
attributed to the last model. Cached input and reasoning output are subsets,
not additive charges. Deduplicate repeated cumulative records and separate
per-call from aggregate totals. Account-wide quota snapshots are contextual,
not causal package accounting; repriced API tokens are estimates, not subscription
quota. Prefer available telemetry; no new dashboard or mandatory parsing campaign.
The owner's external-Pro quota separation is an operating observation, not an
open-source entitlement claim. A 70% premium-token reduction is a pilot target,
not measured savings or acceptance for a scientific change.
