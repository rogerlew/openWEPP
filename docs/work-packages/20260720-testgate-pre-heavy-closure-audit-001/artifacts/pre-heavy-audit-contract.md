# Pre-Heavy Audit Contract

Status: proposed contract; Phase A freezes the schema before production edits.

The canonical artifact is `pre-heavy-audit.v1.json`. Its identity is a SHA-256
digest of the canonical payload excluding the derived ID. The payload binds:

- repository, base, head, dirty-tree, base-package admission, current-package,
  and exact changed-path identities;
- current declared write set and intended closure path envelope;
- cheap prerequisite results and artifact digests;
- one stable gate inventory, node DAG, argument arrays, cardinalities, timeouts,
  retry policies, `LIGHT`/`HEAVY` cost classes, and output namespaces;
- environment, toolchain, binary, feature, fixture, policy, configuration,
  runner, and concurrency identities;
- execution, authority, and documentation roots plus evidence-reuse decisions;
- attempt-root allocation and cache-key manifests;
- combined-versus-separate regression/coverage decision and reason code;
- persistent ledger location and prior same-cause attempt/defect links; and
- aggregate `READY`, `BLOCKED`, or `INVALID` status with exhaustive reasons.

The executor consumes the admitted inventory. The verifier independently
enumerates the current expected inventory and compares it to the admitted
inventory; it does not replace or repair the plan. The executor runs `LIGHT`
nodes first and cannot enter the `HEAVY` state without the exact `READY` audit
ID. Recovery imports successful per-node receipts that are current and reusable
in the target attempt under §10.4. Every rejected receipt records the exact
trust, reuse-class, or execution-context reason that requires rerun.

Local ledger entries are canonical, append-only, and hash-chained. A trusted
workflow uploads each attempt and per-node checkpoint before aggregate receipt
creation, publishes their digests to a durable attempt index, and can verify and
re-ingest them after workspace or runner loss. Retention must cover the owning
campaign and release qualification; a disappearing runner directory cannot be
the only copy of claimed audit evidence.

`READY` means every selected node can start using only bound inputs and exact
artifact namespaces. `BLOCKED` means an identified external or prerequisite
condition prevents launch without making the artifact malformed. `INVALID`
means authority, schema, identity, inventory, collision, cache, mutation, or
audit integrity is unsound. Executor launch on either non-ready state is an
error. The verifier recalculates bound inputs and confirms the executor consumed
the exact audit ID.

The human report is rendered from JSON. It is not separately editable evidence.
