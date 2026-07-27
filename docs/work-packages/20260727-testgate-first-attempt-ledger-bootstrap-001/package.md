# TESTGATE First-Attempt Ledger Bootstrap

Package ID: `20260727-testgate-first-attempt-ledger-bootstrap-001`

Queue ID: `TESTGATE-LEDGER-BOOTSTRAP-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to complete CAL-04B and resolve
its assurance/TESTGATE blockers through reviewed work-package corrections.

This defect-closure ExecPlan is maintained under
`docs/defect_closure_execplans.md`.

## Objective

Make the canonical `testgate.py --execute` helper securely create and validate
a fresh durable history ledger, then bind the admitted open file identity
across the Python-to-Rust transition boundary, so a first authoritative attempt
can reach LIGHT without a manual placeholder file or pathname race.

## Reproducer

At exact commit `2efbee531361639f0815820bc43c7506ae62eb12`, the
comparator-owned command used fresh absent paths:

- artifact root
  `/home/workdir/gate-auth11-test-provider-canonical-001`;
- ledger `/home/workdir/gate-auth11-test-provider-history.jsonl`.

Intent and terminal planning succeeded, but transition failed in 16 ms with
`GATE-CLI-INPUT: No such file or directory`. No LIGHT, audit, receipt, or HEAVY
artifact exists. `testgate.py` passes the absent ledger as `--resume`; Rust
`validate_transition_outputs` requires `resume` to be an existing regular file.
The helper currently creates/appends the ledger only after transition returns.

An operator-launched second root used the wrong campaign identity and was
terminated during intent planning. It is invalid, unexecuted evidence and is
not a retry authorization.

## Included Scope

- lexical, no-follow path selection without `Path.resolve()` or equivalent
  symlink traversal;
- a no-follow, create-once durable ledger bootstrap before transition;
- preservation and chain validation of an existing regular ledger;
- rejection of final-component or ancestor symlinks, directories, non-regular
  files, malformed chains, and path-identity swaps throughout bootstrap,
  transition, append, and failure finalization;
- file and containing-directory durability before transition;
- a transition-only inherited file-descriptor transport that binds Rust to the
  already-admitted ledger inode while preserving the original lexical path as
  the recorded authority identity;
- an owned Rust ledger handle used for every transition chain read,
  reconciliation read, audit-head read, resume discovery read, and durable
  append without reopening the selected pathname;
- focused Python regression tests for fresh, existing, malformed, symlink,
  collision, and path-swap cases;
- canonical exact-head admission and one comparator-owned execution after dual
  review.

## Excluded Scope

- weakening or bypassing Rust transition input validation;
- changing ledger record schema, predecessor chaining, retry policy, recovery,
  planner selection, executor behavior, or CAL science;
- manual ledger placeholders, manual gate injection, Harvard access,
  deployment, or release.

## Declared Write Set

- `tools/local_ci/testgate.py`
- `tests/python/test_testgate.py`
- `crates/openwepp-gate-planner/src/main.rs`
- `crates/openwepp-gate-planner/src/pre_heavy.rs`
- `crates/openwepp-gate-planner/src/resume.rs`
- `tools/local_ci/README.md`
- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260727-gate-planner-auth11-fixed-inventory-test-provider-001/artifacts/implementation-gates.md`
- `docs/work-packages/20260727-testgate-first-attempt-ledger-bootstrap-001/**`

No other path is writable. This write set must not widen.

## Execution Plan

1. Commit this prospective scaffold and obtain two independent read-only
   scaffold reviews before tooling edits.
2. Add the smallest secure bootstrap helper and focused tests. Operate on the
   raw selected path lexically, validate every existing ancestor without
   following links, create with `O_CREAT|O_EXCL|O_NOFOLLOW`, and retain enough
   identity information for the file and every ancestor to reject substitution
   before every later ledger use.
3. After the initial implementation review, disposition its cross-process
   pathname-race HOLD prospectively: inherit the admitted descriptor, require a
   transition-only `--resume-fd`, safely duplicate `/proc/self/fd/<n>` without
   `unsafe`, compare its device/inode with the no-follow original path, and use
   one owned Rust handle for every transition ledger operation.
4. Run every exact command in `Validation Commands`.
5. Commit one exact corrected implementation state and obtain dual
   implementation re-review with explicit finding disposition.
6. Obtain dual terminal verification of the exact diff and failed-attempt
   retention.
7. Delegate one fresh exact-head canonical transaction to
   `comparator_suite_runner`; require READY audit, valid receipt/ledger, every
   selected LIGHT/HEAVY node PASS, and two receipt verifiers.
8. Close this package and resume AUTH11/external-DAG/CAL execution.

## Acceptance

- A fresh ledger is created as a regular file with no-follow, exclusive-create
  semantics before `--stage transition` is invoked. Its bytes and parent
  directory entry are durably synchronized first.
- The raw operator-selected path is handled lexically without resolving
  symlinks. Every existing ancestor and the final component are checked without
  following links.
- Python passes exactly the guarded ledger descriptor with `pass_fds`; Rust
  requires the matching transition-only `--resume-fd`, duplicates it safely,
  and rejects absent, malformed, closed, directory, or device/inode-mismatched
  descriptors before LIGHT.
- Existing no-follow pathname admission remains mandatory. After admission,
  every Rust transition ledger read and append uses the owned handle; no
  transition ledger operation reopens the operator-selected pathname.
- The audit path hash and recovery-root derivation continue to bind the
  original lexical operator-selected path.
- Existing regular ledgers are not truncated or replaced and pass strict
  predecessor/hash-chain validation before reuse. The selected file, its
  parent, and every ancestor identity are revalidated before transition,
  history append, and failure finalization.
- Final-component symlinks, ancestor symlinks, directories, FIFOs or other
  non-regular files, malformed JSON/hash/predecessor chains, exclusive-create
  collisions, and path substitution fail closed without modifying an outside
  target or competing file.
- Bootstrap failure is typed as a helper failure and does not launch gates.
- `_append_history` remains the sole attempt-record appender; record schema and
  digest chaining do not change.
- Focused tests cover fresh creation and durability, existing-byte
  preservation, final and ancestor symlink rejection, directory and FIFO
  rejection where supported, malformed JSON/hash/predecessor rejection,
  exclusive-create collision preservation, bootstrap failure with transition
  `_invoke` never called, and final/ancestor path swaps before transition,
  append, and failure finalization with outside targets untouched.
- Cross-process tests cover exact descriptor forwarding, missing/malformed/
  closed/non-regular/mismatched descriptor rejection, replacement before Rust
  admission, and final/ancestor replacement after admission. The retained
  inode receives authorized appends while replacement and outside targets
  remain untouched.
- The retained original failure and invalid wrong-campaign root remain
  byte-identical to the immutable evidence-file baselines in
  `artifacts/failed-root-baselines.md` and are never admitted. The original
  unindexed `execution/.work/**` reconstruction cache is disposable and is not
  evidence.
- Full focused and exact-head canonical gates pass; dual review, dual terminal
  verification, and dual receipt verification pass.

## Security-Impact Gate

The change may create only the exact lexically selected ledger file and missing
parent directories. It must not resolve or follow final or ancestor symlinks,
truncate existing bytes, accept malformed history, overwrite an exclusive
create collision, continue after path substitution, relax Rust preflight, or
create/write a ledger outside the operator-selected path. These obligations
apply during bootstrap, verification, transition preparation, append, and
failure finalization. Any such behavior is `FAIL`.

The inherited descriptor is defense in depth, not a validation bypass. Rust
must retain the original no-follow path checks, compare the path and handle
identities before LIGHT, and record the original path identity. Descriptor
transport is valid only for the in-process transition; standalone or
non-transition use is `FAIL`.

## Validation Commands

```text
.venv/bin/python -m unittest tests.python.test_testgate
.venv/bin/python -m py_compile tools/local_ci/testgate.py tests/python/test_testgate.py
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
cargo nextest run -p openwepp-gate-planner
cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings
cargo fmt --all -- --check
markdown-doc lint --path docs/work-packages/20260727-testgate-first-attempt-ledger-bootstrap-001
git diff --check
```

Harvard remains sealed and CAL population remains prohibited.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one bounded implementation worker, two independent
read-only reviewers, two independent terminal/receipt verifiers, and the
`comparator_suite_runner` for one exact-head canonical admitted execution;
expected outputs are bounded helper/tests/evidence changes and retained
plan/audit/receipt/ledger verdicts; write access is limited to the declared
write set.
