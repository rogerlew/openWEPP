# Profile And Blockers

Evidence class: Static/Ran.

## Timing Blocker

- Reproduced current-code direct default-candidate timing failure:
  `112.99 s / 1083024 KiB`, budget `<=91.2 s`.

## Profiling

- Ran:
  `perf record -F 99 -g -o /tmp/r7h-iterative-completion/perf-direct/perf.data -- ... --direct-default-candidate`.
- Result: `115.52 s / 1083020 KiB`, `12606` samples.
- `perf report --children` attributed `45.55%` inclusive to
  `Wb11HydrologyKernel::require_shadow_fine_state_domains`; most of that cost
  was `alloc::fmt::format_inner`, `core::fmt::write`, and `String` growth for
  per-fine-layer indexed symbol construction.
- Mechanism: typed direct frost validation constructed five formatted
  `BoundarySymbol`s for every valid fine-layer guard check even though the
  success path does not need to clone or report the symbol.
- Classification: in-envelope performance defect. It is guard/report allocation
  overhead, not process-physics math.

## In-Envelope Correction Attempts

- Replaced hot valid-path formatted fine-layer symbols with
  `require_frost_fine_state_range`, which checks raw ranges first for typed
  direct `request=None` callers and constructs the indexed `BoundarySymbol`
  only on error. Compatibility/request-backed callers still use hot-symbol
  lookup.
- Focused test: `cargo test -p openwepp-hillslope-orchestrator r7g_ -- --nocapture`
  passed.
- Release H2637 direct default-candidate after the fix passed timing:
  `61.40 s / 1082876 KiB`.
- Explicit direct passed timing: `64.19 s / 1083260 KiB`.
