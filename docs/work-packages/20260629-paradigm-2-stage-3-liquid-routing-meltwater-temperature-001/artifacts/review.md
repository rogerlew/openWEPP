# Implementation Review

Evidence class: Static + Ran.

Findings:

- No default activation was introduced. Stage 3 is selected only by the
  package-bound environment selector and unsupported values fail closed.
- No public output schema, fixture, density cap, frost solver, or runfile parser
  change was made.
- The CoE melt mass path remains authoritative; Stage 3 consumes the existing
  routed liquid diagnostically.
- Stage 3 consumes a real Stage 1 layer stack and fails closed when selected
  without `physics_bulk_multilayer_density_v1`.
- Per-layer state now carries temperature, liquid water, cold content, and
  refrozen liquid while preserving those fields through Stage 1 layer splitting,
  removal, and merge operations.
- Deferred observed-gate execution found a stale-cold-content domain bug in the
  opt-in path. The retained implementation caps persisted cold content to the
  current layer mass before reconstructing temperature, and a regression test
  covers the failure mode.
- Stage 3 diagnostics are boxed only in direct-runtime carry/state to preserve
  ADR-0025 hot-frame size bounds.
- Real H2637 endpoint timing/RSS passes the ADR-0025 `<=10x` budget.
- The Stage 3 liquid/temperature increment is neutral versus Stage 1 rollback
  on cross-SNOTEL/cancov observed guardrails.

Residual risks:

- The full opt-in arm still depends on Stage 1 layered density and therefore
  inherits Stage 1's snow guardrail failure versus the current no-env default
  (`16`/`177` versus `15`/`179`). That blocks promotion/default activation.
- The full in-stream temperature program remains out of scope. Stage 3 only
  produces the typed meltwater-temperature source diagnostic.
- Event-scale runoff/meltwater-temperature behavior is forcing-bounded by the
  CLIGEN stochastic sub-daily caveat in the Paradigm 2 specification.

Review disposition:

No implementation blocker remains for an opt-in diagnostic candidate. Promotion
or default activation is blocked by the full-arm snow guardrail and by the
package's diagnostic-only discipline.
