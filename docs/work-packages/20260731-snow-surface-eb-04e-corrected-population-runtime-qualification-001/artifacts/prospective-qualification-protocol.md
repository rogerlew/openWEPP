# EB-04E Prospective Qualification Protocol

Status: `frozen before result execution`

Evidence class: `Static`

Frozen: `2026-07-31`

## Candidate And Population

Candidate source is Git HEAD `44c6c9cc2e4447064fbbbf70935cf581d60d49b0`,
which contains EB-04C and EB-04D. The exact runner binary is rebuilt and hashed
before execution. The population is the immutable EB-04 set: five SNOTEL open
controls, Harvard open/hardwood, Marcell open/deciduous/conifer, and two
Sleepers diagnostic frost lanes. Each receives B, L, S, and LS exactly once.

The prior EB-04 fixture hashes, lane roles/strata, selectors, and non-target
environment are binding inputs. Drift fails before execution.

## Physical Gates

- complete inventory: 12 lanes, 48 cells, one attempted result per key;
- runtime: return code zero plus present WAT and research trace;
- chronology: sequential day indexes beginning at zero and WAT/trace row-count
  equality;
- snow mass: maximum daily reconstruction `<=1e-9 m`;
- surface energy: maximum independent reconstruction `<=1e-6 J m^-2`;
- cold content: maximum independent reconstruction `<=1e-6 J m^-2`;
- latent/mass: roundoff-aware hourly reconstruction ratio `<=1` with absolute
  floor `1e-6 J m^-2`;
- hourly/daily aggregation: shortwave, longwave, and latent energy
  `<=1e-6 J m^-2`; vapor mass `<=1e-9 kg m^-2`;
- vapor/sublimation: `<=1e-6 kg m^-2`;
- layer SWE and depth reconstruction: `<=1e-9 m`, with exact eight-field,
  finite, physically signed vectors;
- thermal domain: every populated layer temperature is `(-273.15, 0] deg C`.

Every gate is per-cell and population-reduced by conjunction. A producer-carried
residual cannot replace the independent reconstruction.

## Claim Boundary

EB-04E may state which cells complete, whether physical ledgers close, how often
the EB-04C shallow-pack branches occur, whether EB-04D-represented fragments
survive, and the magnitude/range of modeled mechanism operands. It may not read
observations for scoring, calculate factorial contrasts or interactions, judge
empirical improvement, calibrate, promote, activate defaults, or modify EB-04.

## Execution And Retry

The matrix is executed once. Semantic, scientific, or typed runtime failures
are retained and never retried. An infrastructure-only interruption before a
cell starts may be resumed only if the attempt ledger proves that no completed
cell is rerun and the exact source/binary/input identities remain unchanged.
No such resume is presumed.

## Disposition

`PASS` admits scaffolding of EB-04R. Any failed inventory, identity, runtime,
physical, thermal, layer, evidence, review, or verification gate produces
`HOLD`, with no observation scoring.
