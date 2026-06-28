# Line-Count Governance

Evidence mode: Ran.

Command:

```bash
find crates tests -name '*.rs' -print0 | xargs -0 wc -l | sort -nr | head -n 30
```

Disposition:

- New Rust test file `tests/integration/snowdensity10_3_15_default_activation_active_cap.rs`: `187` lines.
- Touched production file
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`:
  `2596` lines. This is a pre-existing WARN-size file over `2000` lines.
- No touched file newly crosses the `3000` line hard-refactor threshold.
- Existing unrelated 3000+ files remain outside this package scope:
  `scheduler.rs` and `tests_mod/direct_runtime.rs`.

Outcome: PASS with WARN for the existing touched builder file size.
