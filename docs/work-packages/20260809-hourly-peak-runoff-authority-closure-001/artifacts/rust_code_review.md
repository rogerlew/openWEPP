# Rust Code Review

Status: `complete`

Verdict: `PASS`

Evidence class: `Static` for exact commit
`669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`; `Ran` checks below were executed
at that exact `HEAD`.

Narrow reopen range:
`36cd4ca04..669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`.
Prior implementation/contract/test correctness remains bound to reviewed
commit `33831787b7029b28b0716c8458f08a11899db446` and is unchanged by this
ADR/source-guard increment.

## Findings

No Critical, High, Medium, or Low Rust/science-authority correctness finding
was identified in the narrow reopen scope.

## ADR-0036 Amendment Assessment

The amendment at
`docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md:13-19`
truthfully reconciles the accepted decision with
`SC-WATBAL-001#INV-WATBAL-102..104` and `SC-SED-001#INV-SED-014`. It limits
legacy shards without paired hourly water to explicit compatibility fallback
semantics rather than allowing a scalar legacy peak to regain native
authority.

The affected decision sections remain mutually consistent:

- D2 (`ADR-0036:172-174`) distinguishes the frame-internal maximum-hour depth
  rate in `m/s` from the exactly-once public/HBP area conversion to `m3/s`.
- D3 (`ADR-0036:186-198`) retains triangular reconstruction only for shards
  lacking the modeled hourly pair and requires a current serialized peak to be
  derived from `max_h(V_h / 3600 s)`.
- D4 (`ADR-0036:200-216`) makes the water/sediment integrals and peak derive
  from the same hourly payload, retires the analytical APPMTH/rainfall-envelope
  operator, prohibits rescaling toward another peak, and excludes legacy
  fallback from a current acceptance claim.
- D5 (`ADR-0036:218-225`) preserves ADR-0017 Investigation-tier comparator
  posture without permitting a comparator or legacy scalar to replace native
  WB16 authority.
- Alternative 4 (`ADR-0036:273-277`) now rejects a separate analytical peak as
  equal native authority instead of describing it as the reason for a second
  peak value.

The equations and units match the canonical contracts and production boundary:
internal `peakro_depth = max_h(q_hourly(h) / 3600 s)` and public
`peakro = max_h(V_h / 3600 s)` after one area conversion. No production
arithmetic, guard precedence, typed error, serialization layout, or fallback
implementation changes in this increment.

## Integration Source-Guard Assessment

`tests/integration/peak_hourly_authority_contract.rs:128-149` adds assertions
without removing or weakening the existing runtime, contract, duration, unit,
and public-area checks in the same test.

The positive assertions bind the ADR's current maximum-hour production rule,
internal depth-rate equation, rejection of an independent estimator, and
compatibility-only fallback. The negative assertions reject the former
separate-estimator paragraph, allowed-inequality statement, and rescaling
alternative. The required and retired checks cover different sides of the
authority transition, so deleting only the old wording or adding only a loose
hourly-peak phrase cannot satisfy the guard.

Static inspection confirms the asserted phrases occur in the authoritative
amendment/D4 text and that none of the retired authority remains elsewhere in
the ADR. The guard is deterministic and local-only, and its placement in the
existing integration target introduces no new harness or dependency surface.

## Ran Evidence

- `cargo nextest run --test peak_hourly_authority_contract`: PASS, 4/4;
  nextest run `10c3d308-0135-454b-8a96-4155a41cbb44`.
- `markdown-doc lint --path
  docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md
  --format plain`: PASS, 0 errors and 0 warnings.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check 36cd4ca04..669269ee4`: PASS.

## Residual Risk And Missing Tests

- The ADR assertions search the whole document rather than extracting D4 by
  heading. Exact retired-string checks plus static review are adequate for the
  current change, but a heading-scoped helper would reduce the chance that
  unrelated future prose satisfies a required marker.
- The ADR checks share the publication/erosion test function. A separate test
  would improve failure localization without changing coverage.
- The initially reported dangling `The` before the amended D3 paragraph was a
  non-blocking editorial finding and is resolved by descendant `a8a96498e`.
  The semantic review target remains `669269ee4`.
- Previously recorded scope limits remain: the warmed H2637 suite does not
  prove frost-active Lane D/WB16 coupling, and pure-melt coverage is split
  across R4K and downstream WB16 tests rather than one end-to-end vector.

## Approval

`PASS` for Rust integration-test and ADR science-authority correctness at exact
commit `669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`. The amended decision matches
the canonical maximum-hour/unit/fallback contract, and the new source guard
protects both required authority and removal of the contradicted text. No
code-review blocker remains.
