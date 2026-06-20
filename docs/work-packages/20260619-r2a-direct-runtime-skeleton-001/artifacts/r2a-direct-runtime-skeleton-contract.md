# R2A Direct Runtime Skeleton Contract

Status: complete.
Evidence mode: Static + Ran.

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

## Implemented Contract

Static:

- R2A direct execution is skeleton-only. It constructs typed direct frames,
  builds a no-op/shadow executor, and exposes audit counters for direct
  skeleton construction and execution.
- R2A direct execution does not produce publication outputs and does not claim
  endpoint improvement or identity readiness.
- Compatibility execution remains the only default execution mode.
- Opt-in direct skeleton selection may run before compatibility execution as a
  shadow/no-op audit, but the direct skeleton itself has no compatibility
  scheduler or kernel request call sites.

Ran:

- Default-disabled runner test proves zero direct skeleton construction.
- Explicit opt-in runner test proves exactly one direct skeleton execution.
- Direct-runtime source scan proves no forbidden compatibility tokens in the
  direct runtime module.
