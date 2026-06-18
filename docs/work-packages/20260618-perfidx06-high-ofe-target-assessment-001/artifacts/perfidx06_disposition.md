# PERFIDX06 disposition

Evidence: Ran + Static.

## Verdict

PERFIDX06 does not close the performance target.

| Target | Verdict | Evidence |
| --- | --- | --- |
| `<=10x` legacy | Not closed | primary no-UI ratio is `73.12x` |
| `<=5x` legacy | Not plausible under current approach | would require about `14.6x` more improvement |
| More incremental id-table work | Not the right next move | PERFIDX05 showed the write/guard id path is dual-write-bound and net-negative |
| Redesign need | Yes | remove logical `BTreeMap`/symbol-keyed runtime surface from hot execution path |

## Headline Numbers

Ran:

- openWEPP H2637 no-UI endpoint: `666.82s`, rc `0`;
- openWEPP H2637 with-UI endpoint: `667.44s`, rc `0`;
- legacy H2637 no-UI median: `9.12s`, rc `0`;
- legacy H2637 with-UI median: `11.54s`, rc `0`;
- primary no-UI ratio: `73.12x`;
- with-UI ratio: `57.84x`.

PERFIDX06 confirms the package expectation: PERFIDX04 was real progress, but the remaining
gap is still one order of magnitude above the `<=10x` target.

## Recommended Next Move

Do not continue with narrow write-side id migrations under the read-mirror design. They
should be treated as a known losing class unless a specific package proves it avoids
logical-map dual-write cost before touching production code.

The next performance package should be a redesign/scoping package for an
array-authoritative hot-path state representation. It should explicitly answer:

- how the scheduler, Wb11 hydrology, writeback, and decomposition guards run without
  symbol-keyed `BTreeMap` access in the per-day/per-OFE hot path;
- how publication/export reconstructs the contract-visible logical surface without
  repeating PERFIDX03's export seam failure;
- how legacy-style fixed-array execution can preserve openWEPP typed guards and contract
  diagnostics without allocating or formatting on success paths;
- what minimum H2637 prototype must be measured before broader migration.

## Closure

PERFIDX06 is successful as an assessment package: the endpoint, profiler, wall-clock ladder,
same-machine legacy ratio, bottleneck analysis, and target verdict are recorded. No
production code or science contract changed.
