# Bounded agent execution

This standard governs Astra-led package execution and between-package review.
The [package guide](../work-packages/AGENTS.md) owns acceptance and review counts;
[testing strategy](testing-and-gate-strategy.md) owns validation. Explicit owner budget boundaries
stop execution, never convert incomplete acceptance into PASS.

## Package scope and checkpoints

Keep the overall objective and backlog in package.md. Before execution, state the
package's source identity, bounded deliverables and internal checkpoints,
permitted changes, selected checks, run allowance, and stopping condition there.
Incoming `/tmp/openwepp*.md` documents are owner-adopted instructions; incorporate
their current scope into package.md without creating a parallel live narrative.
An external model's proposed authorization only becomes authorization when the
owner adopts it. Preserve already granted permissions and explicit exclusions.

Astra owns execution through the authorized package outcome. It may sequence
internal checkpoints and correct newly exposed in-scope failures under existing
authority without a fresh owner/adviser prompt for each step. A package objective
does not authorize new scientific methods, relaxed acceptance, successor scope,
or resumption of an explicitly paused experiment.

Use two unsuccessful correction cycles or 60 active minutes as the default
internal reassessment point. The worker returns to Astra, which checks evidence
of progress and records a concrete continue/reassign/escalate decision. Continuing
requires a supported next action within scope; repeating the same failed approach
without new evidence is not progress. Routine build/integration corrections count.
This replaces the generic automatic owner handoff, not an explicit owner limit.

Owner-adopted time, cycle, run and failure limits remain hard stops, including in
older packages. Carry their consumed ledger; only the owner can revise them.
Never reinterpret an explicit checkpoint-only authorization as package-wide.
Active time includes reading, reasoning, edits, orchestration and review; exclude
only recorded pure waits. Count concurrent wall time once and delegate usage
separately. A corrective edit followed by failed verification is an unsuccessful
cycle, including compile failure. Do not reset consumption through renaming,
delegation, a fresh thread or compaction.

For scientific execution, normally allow one planned authentic result-bearing
run after focused checks. Required reproductions and comparison pairs must be
named prospectively in the checkpoint allowance; "one run" never removes a
required control or acceptance test. Preserve the first downstream failure and
classify it against the package envelope: Astra manages authorized in-scope work;
out-of-scope defects or explicit failure stops return to the owner. At a hard limit, finish only
safe state preservation and return an incomplete disposition with pending checks.
Record ongoing job identity and collection command; do not kill useful long jobs
or keep a reasoning agent polling them. Starting a successor package requires
owner adoption or an already authorized sequence.

## Models and delegation

Repository defaults for new trusted sessions:

| Responsibility | Model / effort |
| --- | --- |
| Work-package orchestrator | gpt-6-astra / medium |
| Detailed implementation and focused verification | gpt-5.6-terra / medium |
| Narrow extraction or simple delegated edit | gpt-5.6-luna / medium |
| Between-package science/technical direction | Codex gpt-6-astra / high or owner's ChatGPT Pro workflow |
| Local independent correctness when external review is unavailable or insufficient | gpt-5.6-sol / high |
| Local independent QA/evidence | gpt-5.6-terra / medium |
| In-package scientific escalation | gpt-6-astra / high or ChatGPT Pro, for a named unresolved issue |
| Declared command batches | Local scripts; existing Spark runner when useful |

The [project config](../../.codex/config.toml) pins these local roles. Model names
are explicit routing choices, not a quality guarantee. Use at most two concurrent
children, one source writer, and no nested spawning. A review phase may use both
children. Astra delegates detailed implementation with the `implementer` role;
Luna is for narrow extraction or simple edits. Astra can do small coordinating
edits/checks directly. Do not add agents merely to populate roles.

Use minimal explicit task packets with `fork_turns="none"` where supported.
Named roles can override spawn settings; verify actual controls and report an
unavailable requested route without silently using an expensive parent/fallback.
If the runtime cannot enforce a pin, do not claim it did or repeatedly retry an
unavailable route. Reuse implementers and reviewers through their corrections.
Keep Astra's context centered on package decisions and decisive primary evidence;
fresh package sessions use the current brief, not full historical threads.

Between packages, Codex Astra or ChatGPT Pro reviews the scientific/technical
outcome and recommends next scope, acceptance and risks. The owner adopts the
direction. Within a package, escalate only a named unresolved scientific or
architectural decision; ordinary execution friction stays with Astra. Direction
review is not an additional generic closure gate or independent acceptance review
of the adviser's own design. Missing required review remains incomplete.

These files configure future sessions; they do not switch an already running
primary model. Start a fresh trusted repo session and check its model/settings.
UI/CLI overrides and the host's live tool policy take precedence. Configuration
loading is not proof of actual model routing. No API-key or nested CLI workaround
is part of this workflow.

## Evidence and manual exchange

Keep active isolated source and unique evidence in durable storage outside `/tmp`;
record the actual paths in package.md and use separate disposable build targets.
Use one mutable development copy where isolation is needed. Run focused checks
before freezing a result-bearing source/binary. Never
edit a frozen running tree; keep distinct targets for distinct frozen sources.
Preserve failed attempts and exact experiment inputs with existing Git/patch and
evidence tooling, without generating a complete new archive for every typo.

Astra owns cleanup scope and recoverability. Before deleting unique source,
verify a durable exact copy or successfully reconstruct it from retained base and
patches and compare the recorded source hashes. A patch without its available
base, a source hash alone, or a status note does not prove recoverability.
Delegate cleanup only with explicit resolved targets; classify source, evidence
and generated caches separately. Never infer disposability from a `/tmp` path.
An absent source directory triggers bounded inspection of retained recovery
evidence; it does not by itself prove irrecoverable loss. Preserve explicit
source-loss stop conditions while reporting any evidenced recovery route.

At handoff, preserve source and evidence durably and record whether each is local,
committed or remotely published. Carry granted commit/push permissions; a pause
does not revoke them or grant new ones. When publishing is authorized, include
the necessary base/patch evidence and verify remote availability, including LFS
objects when used, before claiming a recoverable remote handoff. Otherwise retain
the local recovery copy and state the outstanding publication action.

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

Judge efficiency by delivered, verified package outcomes and total effort,
including rework, recovery, delegates, reviews and owner coordination. Lower token
use without delivery is not success. Record material progress and blockers in
package.md; use available usage telemetry only when useful or owner-requested.
Unobserved usage stays UNOBSERVED; quota snapshots and file sizes are not causal
token accounting. Do not create a measurement campaign or a second status record
to certify ordinary completion. This supersedes the token-reduction pilot target.
