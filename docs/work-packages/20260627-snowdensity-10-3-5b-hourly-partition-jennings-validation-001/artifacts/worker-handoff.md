# Worker Handoff

Status: complete
Evidence mode: Static/Ran

## Current State

SNOWDENSITY-10.3.5b is complete. The direct-runtime hourly precipitation phase
partition now has an explicit opt-in `harder_pomeroy_hourly` candidate using
`openwepp-meteorology`, while default behavior remains `legacy_rst`.

## Important Results

- Full Jennings validation ran against the local file2 corpus:
  - rows read: `17,810,805`
  - rows scored: `11,711,058`
  - stations scored: `6,883`
  - Harder-Pomeroy hourly accuracy: `0.903141`
  - legacy `RST` 0 C accuracy: `0.858331`
- Mean predicted station temp50 is `1.527938` C; observed mean temp50 is
  `0.973472` C.
- Humidity contrast has the expected sign: observed high-minus-low temp50
  `-0.883105` C; predicted `-0.770058` C.

## How To Select The Candidate

For package-bound diagnostics only:

```text
OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=harder_pomeroy_hourly
```

Absent, empty, or `legacy_rst` selects the default. Invalid values fail closed.
No parser/runfile/user CLI selector was added.

## Follow-On Recommendation

Use the opt-in candidate in the next snow-depth remediation/adjudication package
for maritime and mixed-forest over-accumulation signatures. Do not default
activate from this package alone; Jennings validates precipitation phase, not
snow-depth response under openWEPP accumulation/density/melt coupling.
