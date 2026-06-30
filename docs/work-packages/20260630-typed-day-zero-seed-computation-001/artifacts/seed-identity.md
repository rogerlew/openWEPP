# Seed Identity

Evidence mode: Static/Ran.

## Focused Shadow Identity

Ran:

```text
cargo nextest run -p openwepp-runner publication_wb11_seed
```

Result: `29` tests run, `29` passed.

The focused tests compare the typed projections to the existing surface seed
outputs:

- lane substeps: WB18 and WB19 substep scalars and multi-OFE carry activation;
- rainfall/hyetograph normalization: breakpoint cardinality, synthesized
  zero-cardinality event points, and hyetograph-integrated rainfall depth;
- initial storage: saturation, per-layer theta/field-capacity/upper-limit, and
  WB11 totals by exact `f64::to_bits()`;
- fine-frost refresh: scalar frost-depth distribution and fine-layer aggregate
  frozen-depth sums;
- residue/`Ws`: defaulted residue interception and neutral water stress;
- WB12 reconciliation: rainfall, storage, precipitation, runoff carryover, and
  forward-solver flag values.
- ET-demand: Priestley-Taylor branch demand/flags and EVAPPM branch demand,
  branch flags, selected PMET diagnostics, and condensation storage-return
  publication.
- `efflen`/`m`: default `efflen = slplen` and `m = 1.5`;
- WB16 compatibility: default `ealpha = 1.0` and compatibility flag.

## Endpoint Seed-Authority Identity

The production consumer cutover was proven by endpoint output identity:

- H2637 HBP/loss/PASS/WAT/plot are byte-identical against clean `5b139058`.
- cli01 HBP/loss/WAT/plot are byte-identical.
- The focused multi-OFE/Wave-2 fixture passes and reports the expected Wave-2
  manifest state.

The direct H2637 manifest reports `compatibility_edge_invocations=0`, so the
typed seed authority reaches the real downstream direct publication consumer.
