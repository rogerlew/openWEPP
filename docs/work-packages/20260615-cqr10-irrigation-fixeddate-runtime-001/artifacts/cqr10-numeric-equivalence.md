# CQR10 Numeric Equivalence

Status: complete.

Static: production edit type is behavior-preserving decomposition. No
constants, units, parser normalization, science contracts, output formulas, or
public API surfaces were intentionally changed.

Static: protected numeric expressions preserved:

- `irrigation_count_to_f64` conversions remain the source for count-like
  fixed-date symbols.
- sprinkler rate, depth, and nozzle finite/domain guards retain the same field
  strings and thresholds.
- furrow supply rate, start, end, and depletion-tail guards retain the same
  field strings and thresholds.
- furrow accumulation order remains:
  `total_duration += depletion_tail`, then `active_duration = end_s - start_s`,
  then `total_duration += active_duration`, then
  `total_volume += supply_rate * active_duration`.
- schedule record advancement remains one event at a time, wrapping after the
  declared OFE count.

Ran: focused characterization asserts exact projected sprinkler values, exact
projected furrow totals, and exact fail-closed field/allowed strings.

Ran: `cargo test --workspace` exited `0`, including irrigation runtime
integration coverage.

Conclusion: no numeric-equivalence issue found.
