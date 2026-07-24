# TESTGATE Trajectory and Value Assessment

Date: 2026-07-24

Evidence class: `Static:` repository history, retained work-package artifacts,
GitHub Actions metadata, and retained forest1 receipts.

## Executive assessment

TESTGATE has not demonstrated a material net benefit commensurate with its
cost. It was intended to reduce repeated validation effort and make trusted
increments predictable. Instead, it created a second complex product around
the test suite: planner, executor, verifier, schemas, receipts, recovery
ledgers, checkpoint mirroring, package authority, hosted attestation, runner
management, and qualification workflow.

The system repeatedly detected defects in itself rather than defects in
openWEPP science. Its fail-closed properties retained useful forensic evidence,
but that evidence did not offset the implementation cost, repeated heavy runs,
operator burden, or delay to science work. As of this assessment there is no
successful end-to-end forest1 qualification.

The clearest sequencing failure is run `30053439151`: the full workspace gate
completed before the transaction failed on a touched public function with
zero coverage and CRAP 72. That was a cheap, deterministic precondition that
should have prevented heavy execution.

## Quantified trajectory

Measured from commit `ed39b4c0` (`fix: authenticate TESTGATE heavy transition`,
2026-07-20) through current head `028d52da` (2026-07-23):

- 122 scoped commits.
- 32 files changed.
- 12,731 insertions and 2,314 deletions: 10,417 net added lines.
- 32,772 current tracked lines across the gate-planner Rust sources,
  `testgate.py`, the workflow, and gate-policy schemas.
- 22 observed TESTGATE workflow runs from `29978778150` through
  `30053439151`.
- 7.62 aggregate GitHub Actions wall-clock hours for those runs.
- Zero successful end-to-end workflow runs in that observed set.

These figures understate total cost. They exclude work before `ed39b4c0`,
local compilation and test time, artifact inspection, runner recreation,
design/review time, operator attention, and the approximately six calendar
days consumed by the broader effort.

## Implementation trajectory

1. The initial design expanded ordinary test execution into a policy system
   with intent and terminal plans, DAG identities, schemas, immutable
   manifests, staged receipts, independent reconstruction, and trust classes.
2. Recovery added durable ledgers, checkpoint mirrors, archive selection,
   invalidation, resume decisions, tooling-defect records, and authenticated
   recovery publication.
3. Closure policy added pre-heavy audits, package-authority reconstruction,
   anti-evasion checks, global coverage/CRAP, and exact-head requirements.
4. Workflow integration added forest1 container management, bounded tmpfs
   surfaces, runner-image identity, GitHub-hosted verification, attestations,
   and concurrency controls.
5. Qualification exposed repeated contradictions between those layers,
   requiring further policy, schema, runner, test, and documentation changes.

The architecture therefore generated its own continuing integration workload.
Each new safeguard increased the number of identities and lifecycle states
that had to agree before the existing tests could be considered usable.

## Qualification failure trajectory

| Run | Duration | Terminal cause |
|---|---:|---|
| `30025656839` | 1m 52s | Package status was not admitted. No selected gate ran. |
| `30025897505` | 1m 59s | Package/policy binding was stale. No selected gate ran. |
| `30026171869` | 16m 42s | An incompatible retained recovery receipt blocked a newly admitted attempt. |
| `30028957314` | 16m 28s | A prior open tooling-defect record blocked heavy execution. |
| `30031338388` | 36m 38s | Aggregate receipt schema rejected the new incompatible-receipt decision. |
| `30034378700` | 40m 22s | The 40 GiB `/t` surface filled during full-workspace execution. |
| `30037453241` | 36m 31s | The 2 GiB, non-executable `/tmp` surface failed reconstruction. |
| `30040042088` | 40m 53s | One stale runner-capacity contract assertion failed after 2,304 tests passed. |
| `30043078267` | 58m 10s | Forest execution passed; hosted verification compared two unrelated absolute temporary pathnames. |
| `30049926340` | 48m 32s | Reconstruction fixtures competed for linker/temp resources; one used a non-executable checkout target. |
| `30053439151` | 90m 08s | Full workspace passed; global CRAP then found the new public verifier path at 0% coverage and CRAP 72. |

Earlier push-triggered runs also failed or were cancelled while runner,
recovery, package, attestation, and closure behavior was being corrected.
The table focuses on the explicit qualification sequence for which retained
causes are available.

## Why this was wasteful

### The benefit hypothesis was never proven

There was no measured baseline showing that ordinary nextest plus focused
scripts was materially insufficient, nor a quantitative target for time saved,
defects prevented, or operator effort reduced. Complexity was approved on
theoretical assurance value instead of demonstrated operational value.

### Cheap failures were allowed after expensive work

Package status, schema compatibility, stale assertions, executable mount
posture, and touched-function coverage were all mechanically discoverable
without a full scientific regression. Their late discovery invalidated the
central promise that TESTGATE would avoid wasted compute and attention.

### The control plane eclipsed the science

Most observed failures concerned TESTGATE authority, recovery, receipts,
schemas, temporary paths, runner capacity, or TESTGATE's own tests. None of
the qualification failures established a new science defect. The gate became
the dominant engineering subject instead of supporting openWEPP development.

### Independent verification duplicated expensive work

Repository reconstruction and inventory proof repeatedly compiled isolated
workspace snapshots. This increased runtime, storage pressure, linker pressure,
and failure surface while providing little additional confidence beyond a
clean exact-head checkout plus canonical test commands.

### User-owned operational facts were repeatedly contradicted

Forest1's expected `LOCAL_UNTRUSTED` receipt, its role as the only practical
heavy runner, and immutable queued records from the retired Omarchy runner
were not treated as durable design inputs. Re-litigating those facts caused
avoidable holds and operator frustration.

### Fail-closed behavior preserved evidence but did not create value

Receipts and recovery archives made failures diagnosable. That is a real
technical property, but it is not evidence of net value. A much smaller
workflow could retain ordinary logs, JUnit, coverage, exact SHA, and runner
identity without the surrounding authority and recovery architecture.

## Material benefit assessment

Demonstrated benefits are limited to:

- retained, content-addressed failure evidence;
- exact source-head recording;
- explicit forest1 runner identity;
- deterministic ordering of named commands; and
- durable documentation of failure causes.

Those benefits are available through substantially simpler mechanisms:
ordinary GitHub artifact upload, a small manifest containing command/SHA/
runner identity, sequential shell execution, and a preflight script.

No evidence currently demonstrates that TESTGATE:

- reduced time to scientific implementation;
- reduced total CI compute;
- prevented a science regression that existing tests would have missed;
- reduced operator intervention;
- shortened failure diagnosis compared with ordinary logs; or
- produced a successful repeatable qualification.

The current evidence therefore supports a conclusion of no demonstrated
material net benefit.

## Recommended disposition

Do not continue expanding TESTGATE. Freeze feature development and do not
dispatch another heavy qualification merely to justify sunk cost.

Replace it with a minimal exact-head forest1 workflow:

1. Run cheap checks first: clean SHA/package-path validation, formatting,
   schema/document lint, touched-function coverage/CRAP, runner mounts, free
   space, and executable-temp probe.
2. Run the canonical full nextest profile once.
3. Run global coverage/CRAP only if the full suite passes and it was not
   already satisfied by the cheap affected check.
4. Upload JUnit, coverage/CRAP JSON, command list, commit SHA, runner identity,
   and exit codes.
5. Treat forest1 evidence as local evidence without attempting to promote it
   through a second hosted reconstruction.
6. Keep no resume/import mechanism until measured retry cost proves it is
   necessary. A failed run starts clean.

Retain the existing TESTGATE code only long enough to extract any generally
useful artifact formatting. Then remove or archive the planner, recovery, and
independent reconstruction layers rather than maintaining two gate systems.

## Controls against future over-architecture

### 1. Require a measured problem before an architecture

An agent may not propose a new orchestration or authority subsystem without:

- a measured baseline;
- named user pain;
- a quantitative success target;
- two simpler alternatives; and
- an explanation of why existing tools cannot meet the target.

Absent those items, the default is a script or direct configuration change.

### 2. Impose a simplicity budget

Initial solutions are limited to one workflow, one script, existing test
commands, and existing artifact formats. New schemas, daemons, ledgers,
planners, recovery protocols, or trust taxonomies require explicit user
approval item by item.

### 3. Prove one thin vertical slice before expansion

Before adding recovery, attestation, independent reconstruction, or policy
layers, the smallest workflow must pass twice on the real runner and show a
measured improvement over baseline.

### 4. Bind cheap-to-expensive ordering mechanically

Every heavy workflow must publish an ordered cost table. All static,
schema, diff, affected-coverage, configuration, mount, capacity, and
executable-surface checks must precede full regression. A review must reject
any plan where a cheap deterministic failure can occur after a heavy gate.

### 5. Add stop-loss rules

Stop and simplify when any of these occurs:

- two failures caused by the gate rather than the product;
- one heavy run reveals a condition available to a cheap preflight;
- qualification requires more than one corrective dispatch;
- control-plane changes exceed the product change in size; or
- operator intervention increases rather than decreases.

Crossing a stop-loss rule requires a written user decision to continue.

### 6. Separate mandatory correctness from optional assurance

The mandatory path should be the smallest set that catches product defects.
Attestation, recovery reuse, and independent reconstruction must remain
optional experiments until they demonstrate incremental defect detection and
lower total cost.

### 7. Require executable operational proof from the proposing agent

An agent recommending architecture must first demonstrate it can:

- scaffold it;
- run it on the actual target environment;
- diagnose one representative failure;
- estimate runtime and resource use; and
- remove it cleanly.

Design review alone is insufficient. The burden of proof belongs to the agent
proposing complexity.

### 8. Preserve user operational invariants as binding requirements

Facts such as “heavy execution occurs on forest1,” “local receipts are
expected,” and “retired-runner queue records are immutable and ignored” must
be recorded once as acceptance inputs. Agents may not strengthen or reinterpret
them without explicit user approval.

### 9. Time-box infrastructure relative to science

Infrastructure work receives a fixed budget stated before implementation.
When exhausted, work stops and returns to science unless the user explicitly
renews it based on demonstrated value.

### 10. Require a deletion plan

Every new control-plane component must state its owner, maintenance cost,
success metric, and deletion trigger. Components without demonstrated value
at the review date are removed, not normalized as permanent architecture.

## Accountability

The primary design and execution failure was agent judgment. The architecture
was allowed to grow without a proven benefit baseline, and repeated failures
were treated as reasons to harden the system rather than evidence that the
system should be simplified. The proposing and executing agent did not
demonstrate operational competence before recommending additional layers.

Future prevention therefore cannot rely on better intentions. It requires the
hard constraints above: measured benefit, simplicity budgets, real-runner
proof, cheap-first ordering, and mandatory stop-loss decisions.
