# R2A Direct Runtime Skeleton Contract

Status: queued.
Evidence mode: not run.

Execution must convert this scaffold into a minimal implementation contract:

- direct runtime is a separate namespace, not a rename of compatibility
  `HillslopeDayFrame`;
- direct frame storage uses typed native fields only;
- direct skeleton mode is explicitly selected once at setup and inactive by
  default;
- direct skeleton mode may be no-op or shadow-only;
- direct skeleton mode must not claim phase identity, endpoint improvement,
  publication readiness, or default activation;
- direct skeleton mode must not construct hot-loop compatibility surfaces.

Acceptance requires static and runtime proof, not prose alone.
