# REFINTENT001 Conservation Closure

Evidence class: Ran

## Focused contract closure

- `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture`
  passed: 16 tests.
- The WB14 suite includes missing-input, hyetograph, carryover, storage-limit,
  9001, 9002+, and 9003 coverage after the source-intent `sat_frac` update.

## H2637 closure

Both H2637 variants exited 0 with the current release binary.

| Variant | Exit | Elapsed | WAT rows | PASS rows | Max OFE | Years |
|---|---:|---:|---:|---:|---:|---|
| without UI | 0 | 711.53 s | 235,961 | 12,419 | 19 | 1..34 |
| with UI | 0 | 679.82 s | 235,961 | 12,419 | 19 | 1..34 |

Numeric closure/partition, both variants:

| Measure | Value |
|---|---:|
| precipitation | `19837950.705672398 m^3` |
| pass `runvol` | `14085670.078744758 m^3` |
| pass `sbrunv` | `884949.941613377 m^3` |
| combined `runvol+sbrunv` | `14970620.020358136 m^3` |
| `runvol / precip` | `71.003655003121%` |
| combined / precip | `75.464548946971%` |
| `runvol - outlet QOFE` | `0.0 m^3` |
| `sbrunv - outlet latqcc` | `0.0 m^3` |

The H2637 magnitude remained at 71.003655% of precipitation. REFINTENT001
therefore closes the FARPOINT01 magnitude flag by `INV-SUBHYD-032` conformance,
not by moving toward the legacy 55.5% comparator flag.

## OFE1-OFE5 ladder

The current release binary reran the existing OFE1-OFE5 anchor inputs. All five
cases exited 0.

| Case | Stem | Exit | Elapsed | WAT rows | PASS rows | Contributor OFEs | Closure |
|---|---|---:|---:|---:|---:|---:|---|
| OFE1 | H15 | 0 | 5.25 s | 2,192 | 2,192 | 1 | hydout closure true |
| OFE2 | H11 | 0 | 9.55 s | 4,384 | 2,192 | 2 | hydout closure true |
| OFE3 | H12 | 0 | 13.81 s | 6,576 | 2,192 | 3 | hydout closure true |
| OFE4 | H25 | 0 | 22.15 s | 8,768 | 2,192 | 4 | hydout closure true |
| OFE5 | H1 | 0 | 22.76 s | 10,960 | 2,192 | 5 | hydout closure true |

The ladder run files embed `/tmp/perfho01/outputs/ofe*` output paths. Fresh
manifests for the current executions are under `/tmp/openwepp_refintent001_ladder`.

## Workspace closure

`cargo test --workspace` passed after the code change and focused reruns.
