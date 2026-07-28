# Frozen Trial Protocol

Evidence class: Static.

Seed `20260728` produced `arm-order.csv`. The first shuffled nine cases are
manual-first and the remaining nine are linter-first. Participant assignment
is round-robin. Each participant completes one discarded warm-up against the
Order-4 retirement package before scored cases.

## Snapshot Reconstruction

For each arm, create a fresh local shared clone. `pre-edit` checks out the
scaffold SHA. `terminal` checks out the terminal SHA. `working-tree` checks out
the scaffold SHA and applies the exact scaffold-to-terminal binary diff without
committing. Confirm the paired repositories have identical tracked/untracked
content hashes before exposure.

Invoke the current linter externally:

```text
.venv/bin/python /home/workdir/openWEPP/tools/validation/workplan_lint.py
  --package <package-path> --mode <mode> --format json
```

The command runs with the trial repository as working directory. Run once for
cold latency and once for warm latency; compare canonical JSON bytes. The
first output is exposed to the linter arm. Command runtime is not planning
time. Missing historical policy inputs or package declarations remain measured
tool behavior and are not patched.

## Arm Exposure

Both arms receive the same task:

> Produce an exact obligation and proposed-command plan for this package at
> this observation mode. Identify scope, governing requirements, affected
> paths, required evidence, prohibited actions, unknowns, and closure blockers.
> Do not execute suggested commands.

The manual arm receives the repository and the canonical manual-route pointer.
The linter arm receives the byte-equivalent repository, the same pointer, and
the linter JSON. Agents may inspect further read-only content in either arm.

Planning begins after exposure and ends when the plan text is complete.
Measurement timestamp calls are excluded; every intervening outer inspection
tool call counts as one interaction. Record monotonic nanoseconds. Do not reuse
text between arms. Counterbalancing is the only learning control.

## Raw Result Schema

Each participant writes one ignored JSON file under
`target/order5-qualification/participants/` with:

- participant and warm-up record;
- case ID, package, mode, order, snapshot hashes, and byte-equivalence result;
- per arm: anonymized arm token, raw plan, start/end/duration nanoseconds,
  interaction count, clarification count, and interruption minutes;
- linter cold/warm milliseconds, output digest, deterministic findings, top
  status, unavailable analyses, and canonical-repeat result; and
- protocol deviations.

The parent validates all 18 rows, replaces `manual`/`linter` with blinded
tokens, and sends only plans plus finding lists to scorers.

Repository write access for participants is read-only. Disposable shared clones
and ignored `target/order5-qualification/**` result files are the only allowed
writes. No suggested command may run.
