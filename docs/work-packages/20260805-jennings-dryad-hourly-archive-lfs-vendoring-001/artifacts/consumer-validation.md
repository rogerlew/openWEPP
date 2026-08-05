# Consumer Validation

Ran: 2026-08-05 with the real `openwepp-snowbench jennings-phase` consumer,
the vendored hourly observations, and the source-native file 3 thresholds.

Repeated full-corpus executions produced identical stable scientific fields:
row counts, station count, humidity normalization count, both confusion
matrices, and both headline accuracies.
- Rows read/scored/skipped: `17,810,805` / `16,203,036` / `1,607,769`.
- Stations scored: `6,883`.
- Harder-Pomeroy hourly accuracy: `0.9233792975588032`.
- Legacy 0 C accuracy: `0.8854962119444776`.

The retained 2026-06-27 report scored `11,711,058` rows and is not identical.
Static history strongly attributes the difference to commit `62063495`, which
added a bracketed
bisection fallback to the hydrometeor solver after that report, so valid warm
unsaturated rows that previously failed convergence are now scored. The
Jennings consumer source itself has not changed since commit `835625ba`.

Disposition: pass for custody. Repeated current stable scientific fields are
identical,
the consumer reads the vendored path, and this package changes neither Rust
code nor scientific results. The historical report is not relabeled as a
current-head expectation. JSON bytes are not deterministic: pre-existing
unordered station-map reduction perturbs aggregate threshold floating-point
values. Fixing that Rust behavior is outside this custody-only package.
