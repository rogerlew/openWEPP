# PERFIDX04 Line Count Governance

Static:
- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` grew to host generic indexed surface mutation, hot symbol tables, PL dispatch parsing, and request indexed context.
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` grew to carry indexed execution mirrors, apply transfer updates to both logical and indexed surfaces, and build the resolve-once table.
- `state_access.rs` grew because indexed/fallback accessors are centralized there to keep per-phase code small and preserve error semantics.

Static:
- Local clippy allowances were added only where the package intentionally expanded an existing table/invariant function or preserved an owned error-symbol API:
  - `HillslopeKernelRequest::with_transition_context_and_indexed`: argument count mirrors existing request construction fields.
  - `require_shadow_fine_state_domains`: invariant-check function crossing line threshold after request-aware symbol lookup.
  - `build_hillslope_hot_symbol_tables`: table-builder list of resolved roots.
  - `require_dynamic_state_range`: owned symbol remains useful for error ownership at call sites.

Ran:
- `cargo clippy --workspace --all-targets -- -D warnings` passed with these allowances.
