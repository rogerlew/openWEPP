# Line-Count Governance

Status: complete.

## Ran

- Ran: `wc -l crates/openwepp-runner/src/hillslope/04_direct_publication.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`.
- Ran: current counts:
  - `924 crates/openwepp-runner/src/hillslope/04_direct_publication.rs`.
  - `2421 crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`.

## Disposition

- `04_direct_publication.rs` is below the 2000-line WARN threshold.
- `direct_publication/day_input_and_helpers.rs` is above the 2000-line WARN
  threshold but below the 3000-line required-refactor threshold.
- Decomposition rationale: R7D8 inherited a hot publication/day-input helper
  surface that had already been split out of `04_direct_publication.rs` during
  the package. Completing H2637 parity required preserving the existing helper
  cluster while final blockers were resolved and full gates were rerun.
- Follow-on split intent: split `day_input_and_helpers.rs` by direct day-input
  responsibility after R7D8 closure, preferably into climate/hyetograph,
  hydrology storage/runoff, subsurface/frost carry, erosion seed, and shared
  runtime-surface helper modules. The split must be mechanical and must not
  change R7D8 parity behavior.
