# External DAG Transaction Contract

Status: `FROZEN FOR SCAFFOLD REVIEW`

Evidence class: `Static`

## Canonical Inputs

The adapter binds the committed bytes and SHA-256 values of CAL-04B's
`executor-command-plan.csv` and `observed-command-contract.csv`. It independently
reconstructs exact order, command IDs, argument arrays, environment, working
directory, prerequisites, cost classes, source paths, and declared outputs.
Unknown columns, duplicate IDs/orders, shell command strings, unresolved
variables, undeclared prerequisites, or incomplete output coverage are
`INVALID`.

The versioned JSON plan expands every CSV row. It contains no free-form command
string: `argv` is an array and the executable is direct. Expected cardinality is
18 rows total and these exact transaction inventories:

- `calibration-v1` LIGHT: `prepare`, `build_executor`;
- `calibration-v1` HEAVY, in order:
  `build_production_runner`, `native_proof`, `synthetic_gsi`,
  `hubbard_producer`, `hubbard_primary_reconstruct`,
  `hubbard_verify_reconstruct`, `retain_trace`, `readiness`,
  `summarize_pre_freeze`;
- inter-transaction custody: `freeze`, `freeze_verify_a`,
  `freeze_verify_b`; these execute outside either trusted HEAVY process under
  their existing independent authorities and write only to the custody root;
- `holdout-v1` LIGHT: `freeze_barrier`, which consumes the exact freeze digest,
  two distinct independently produced verifier receipts, and the passing
  `calibration-v1` receipt;
- `holdout-v1` HEAVY, in order: `holdout`,
  `summarize_post_holdout`, `terminal_validate`.

The original order labels, exact argv, environments, prerequisites, source
paths, Harvard-access classifications, and cost classes remain embedded. The
transaction projection may strengthen a later QUICK node to the enclosing
HEAVY stage but cannot weaken a HEAVY row.

## Output Remapping

Execution never writes the source checkout.

- An original `/home/workdir/cal04b-objects/<suffix>` output maps to
  `<attempt-root>/objects/<suffix>`.
- An original repository-relative or absolute-in-repository output maps to
  `<attempt-root>/publication/<repository-relative-path>`.
- Logs, receipts, ledger, audit, checkpoints, and process temporary files have
  disjoint fixed roots below `<attempt-root>`.
- Inputs are never remapped to output/cache paths. A dependent node consumes
  the exact predecessor-manifest path and digest, not a rediscovered filename.
- Every fixed file and every declared directory output is expanded after the
  node into a sorted exhaustive regular-file manifest. Undeclared bytes,
  missing paths, hardlinks, symlinks, special files, path escapes, root
  replacement, or mutation during hashing invalidate the transaction.

The adapter rewrites only plan-declared output operands and their exact
dependent-input references. Any argument change outside that mechanically
derived mapping is `INVALID`. Primary and verification reconstruction roots
remain distinct.

## Durable HEAVY Sequence

For each transition the trusted Rust process:

1. executes LIGHT and seals its canonical receipt;
2. independently reconstructs and persists the canonical READY audit;
3. appends an audit-, plan-, claims-, attempt-, and root-bound HEAVY `STARTED`
   before audit validation, resume admission, executable checks, or subprocess
   preparation;
4. requires ledger state equal to the audited head plus that exact STARTED;
5. validates audit/context and executes HEAVY in exact DAG order; and
6. appends exactly one typed terminal record for the STARTED record.

Every HEAVY receipt embeds the unchanged audit and binds canonical receipt ID,
plan/execution key, roots, source before/after identity, exact prerequisite
receipt IDs/digests/output manifests, argv, toolchains, environment, features,
platform, runner, workflow/job/attempt, results, retry state, and exhaustive
external outputs.

## Publication Transaction

Authenticated execution and repository publication are separate claims.
Execution receipts remain valid because they bind external bytes and an
unchanged source checkout.

Publication accepts only a passing terminal transaction receipt and derives an
exact source/destination manifest. It validates both roots with no-follow
descriptor-relative traversal, requires a clean expected destination baseline,
stages regular files on the destination filesystem, fsyncs files/directories,
atomically renames only after the complete staged manifest verifies, and
refuses collisions or destination drift. Any failure before commit leaves
destination bytes unchanged; an interruption is recoverable by a typed staging
record, never partial acceptance.

After publication, a new commit captures the imported result bytes. The
terminal planner reconstructs the exact base-to-new-head diff and independently
verifies every imported byte against the producing transaction and publication
receipts. Publication is not retroactively added to the earlier READY audit.

## Harvard Custody And Irreversibility

Freeze and verifier activity uses an external custody root. The second LIGHT
stage rejects missing, duplicate, same-actor, executor-produced, forged,
replayed, stale, or wrong-digest verifier receipts and requires barrier order.
No audit, BLOCKED, INVALID, verifier, or pre-spawn path may open Harvard.

After HEAVY STARTED and successful audit/context validation, `holdout` creates
the exclusive opening token immediately before the first Harvard file open.
Successful creation durably records `OPENED_ONCE`. Token preexistence or a race
rejects without a Harvard read. Any later failure is terminal and nonretryable;
the failure receipt and opening record remain preserved. Holdout results,
opening record, summaries, and terminal validation output remain external until
publication.

