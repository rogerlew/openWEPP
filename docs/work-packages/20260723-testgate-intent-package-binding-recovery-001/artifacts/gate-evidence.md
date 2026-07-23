# Gate Evidence

Ran: automatic push run `29981856347` at exact head `be7853fecfeaf791e458ade1a02cc6853fbecff2` passed CLI preflight, durable-history restore, and superseded-head rejection, then failed before planning because `testgate.py` received no `--intent-package`. Its authenticated recovery artifact binds an empty `attempts.jsonl`; no TESTGATE node or expensive gate ran.

Ran: the event-bound resolver passed 8/8 unit cases for exact push trailer, explicit dispatch input, missing/duplicate/malformed declarations, push/input inconsistency, and unsupported events. The owning integration target passed 10/10; the gate-policy authority target passed 11/11; Rust formatting, Python compilation, YAML parsing, scoped Markdown lint, and diff checks passed.

Ran: at exact correction head `c4d2b32a72d0cee1834d1a0c7f7322afd8f84e3b`, the head trailer resolves to `docs/work-packages/20260720-testgate-recovery-trust-001/package.md`. Exact package-chain validation from pushed base `be7853fe...` is READY with chain ID `66bdf8d5eb415951318f51912466e5382144d9db1b33550604d98d7928210f63`, 25 changed paths, and zero unauthorized paths.
