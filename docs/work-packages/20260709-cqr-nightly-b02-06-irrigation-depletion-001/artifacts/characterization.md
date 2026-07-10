# Characterization

Before decomposition, 21 public parser-contract cases were established: all
public error renderings/IDs; strict/compat datver and warning detail; sprinkler
and furrow arity/domain/token/date paths; topology/cross-file/continuation
rules; default options; error sources; and zero-start transition behavior.

Ran against detached scaffold `1c6d7b8c` with the same test-only diff:

```text
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t06-predecomp-target \
  cargo nextest run -p openwepp --test infile_irrigation_depletion_parser_contract \
  --profile quick
```

Exit `0`: `21` passed. The detached worktree was removed after the proof. The
tests consume the public parser and compare complete typed output/error
contracts, not only helper values.
