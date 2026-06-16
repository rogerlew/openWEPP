# CQR31 Coverage Closure

Before LCOV:

- `FNF: 119`
- `FNH: 76`
- `LF: 1625`
- `LH: 1090`

After LCOV:

- `FNF: 133`
- `FNH: 90`
- `LF: 1750`
- `LH: 1224`

Coverage posture:

- Target function coverage improved from `68.78787878787878` to `100.0`.
- Target-file line coverage increased from `1090 / 1625` to `1224 / 1750`.
- New helper coverage is sufficient for CQR closure because every extracted
  helper remains CRAP `<= 30`.

Characterization posture:

- Existing WB13 publication unit tests covered the guarded behavior before the
  refactor.
- Additional production characterization was not added because the existing
  suite already covers WB13 publication values, missing-symbol guards,
  negative-domain guards, source-precedence rules, row keys, and run-span
  provenance.
