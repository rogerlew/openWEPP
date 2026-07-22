# Review B

Status: `HOLD` at `9923cf5c`; corrections applied for repeat review.

Static: Review B found that a test-only or out-of-tree package beneath `crates/`
could pass preflight, and that retained `affected-package-scope.json` was not
consumed or compared at final adjudication. The correction validates a direct,
non-symlink production target and revalidates the retained scope while binding
its SHA-256 in acquisition provenance.

Ran: the reviewer independently passed five focused regressions, Bash syntax,
real production-package preflight, adapter digest checks, and diff hygiene before
issuing the hold.

Static: Repeat review at `c79bf202` found one residual: target kind was not
checked, so a `[[test]]` target beneath `src/` could be admitted. The correction
now admits only library, binary, or proc-macro target kinds and includes the
exact negative regression.
