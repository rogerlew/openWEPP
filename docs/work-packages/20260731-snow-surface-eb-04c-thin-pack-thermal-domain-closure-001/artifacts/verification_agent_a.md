# Verification Agent A

Status: `PASS`

Evidence: `Static + Ran`

Independent science/runtime verification confirmed:

- exact libsnobal authority for total `m_s <= 1 kg m^-2`, lower
  `m_l < 1 kg m^-2`, and lower equality;
- native `0.001 m` SWE comparison sides and source branch ordering;
- suspension before thermal partition or exchange and continuing one-volume
  exchange after lower-volume collapse;
- retained CoE mass/phase ownership and aggregate mass, liquid, refrozen mass,
  and cold-content closure;
- above-boundary absolute-zero fail-closed behavior;
- real-runner consumption of all four diagnostics; and
- replay identity and outcomes for 22 unique cases, including all retained
  runfile/trace hashes, 6 suspension branches, 16 collapse branches, zero
  forbidden thermal errors, 20 complete trajectories, and two later EB-04D
  geometry occurrences.

Ran independently:

- focused contract/runtime tests: `23/23`, run
  `87209f0a-9b68-4732-acf3-747dd0187516`;
- native-SWE helper: `1/1`, run
  `01814bf0-bdf3-4780-8ed1-3e9a03d81c73`;
- authority anti-evasion guard: PASS;
- required-suite obligation guards: `3/3`, run
  `479c4209-8429-4a54-8015-f5b7e316ac06`;
- formatting and diff hygiene: PASS.

Retained JUnit receipts independently confirm quick `2119/2119`, frost
`325/325`, and the clean canonical full rerun `2168/2168`. The replay-time
binary is bound by its recorded unchanged pre/post-run hash; later validation
rebuilt the target binary without invalidating that replay evidence.
