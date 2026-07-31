# Exact-Diff Reconciliation

Status: `complete`

Evidence mode: `Static`

The terminal tracked diff outside this package is limited to:

- `00c_day_input_builder_impl.rs`: 39 inserted and 10 deleted lines. It
  serializes nine already-computed Stage 3 diagnostic fields into the existing
  opt-in snow trace and extracts that formatting into one local helper. No
  process equation, selector, guard, public WAT schema, or default changed.
- `snow_surface_eb03_contract.rs`: 23 inserted and one deleted line. It adds
  the contract-derived trace-field assertion and updates one stale roadmap
  phrase assertion to the already-authoritative wording.
- `docs/ROADMAP.md`: one inserted and one deleted line.
- `docs/planning/snow-surface-energy-balance-roadmap.md`: two inserted and two
  deleted lines.
- `docs/work-packages/README.md`: 19 inserted lines.

This package tree contains the preregistration, runner, retained results,
figures and sidecars, evidence, reviews, verification, and disposition. The
package-local runner is evidence tooling; its default result-bearing path is
now guarded by the immutable attempt ledger, while `--analysis-only` consumes
retained outputs. Package-local `.gitattributes` exempts generated CSV dialect
and Matplotlib SVG path whitespace from Git's patch whitespace heuristic; it
does not transform the retained bytes.

There is no change to canonical science-contract math, fixtures, observations,
forcing, coefficients, runfile schema, public output schema, security
authority, or dependencies. `Cargo.toml` and `Cargo.lock` are unchanged.
