# Coverage Closure

Ran: the ADR-0021 glue thresholds passed at historical HEAD `9970ac32`.

- aggregate line threshold: 87.0993% >= 85%;
- aggregate region threshold: 85.5374% >= 85%;
- per-function region floor: all 81 compiled production functions >= 75%;
- per-function CRAP bound: all compiled production functions <= 30;
- denominator: 81 compiled current-profile production functions; one
  `cfg(not(target_os = "linux"))` function omitted by the measured profile.

Static: no retained exception or denominator suppression was used.

Static: closure is pending one new changed-head measurement after the
dual-review test-oracle correction.
