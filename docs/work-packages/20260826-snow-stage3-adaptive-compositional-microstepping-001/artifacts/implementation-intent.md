# Implementation intent

Static: `Critical` production kernel chronology, conservation, restart, shared
owner, runner-consumer, and default-cutover change. Contract-first sequence is
contracts, contract-derived tests, pre-implementation gate, then production.
The intended write set is package-wide and compiler-discovered as declared in
`package.md`. Exact terminal diff controls the final gate selection.
