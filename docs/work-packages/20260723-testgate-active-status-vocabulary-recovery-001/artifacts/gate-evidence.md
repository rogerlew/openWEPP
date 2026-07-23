# Gate Evidence

Ran: exact package-chain validation initially rejected base status `ACTIVE / READY-REPOSITORY-ATTESTATION` with `GATE-PACKAGE-CHAIN-ANCHOR-INACTIVE`.

Ran: correction `c4d2b32a72d0cee1834d1a0c7f7322afd8f84e3b` adds only that exact active state and retains terminal-state rejection. The focused Rust unit case passed 1/1. After rebuilding the planner, exact chain validation from `be7853fe...` to that commit returned READY with chain ID `66bdf8d5eb415951318f51912466e5382144d9db1b33550604d98d7928210f63` and zero unauthorized paths.
