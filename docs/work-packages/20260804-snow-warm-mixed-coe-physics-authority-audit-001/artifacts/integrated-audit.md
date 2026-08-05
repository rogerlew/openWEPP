# Integrated Audit

Status: complete

Evidence mode: Static + Ran

## Outcome

Primary frozen-matrix disposition:
`BASELINE_FIDELITY_WITH_AUTHORITY_GAP`.

Current Rust reproduces the normative pinned post-2007 CoE term arithmetic and
the audited daily-midpoint/interval-start-pack call ordering. It does not
reproduce the older Chapter 3 formula because the pinned lineage materially
changed `A`, `B`, `C`, `D`, and the daily gate. Rust's typed `1e-12 m` inactive
threshold, inactive drift, and contract-authorized downstream redistribution
and routing are outside this exact term-generation claim. No same-input Rust
transcription defect was found.

Production sufficiency is not established. Full energy-balance formulations
use snow-surface temperature, cold-content satisfaction, resolved net
radiation, and gradient/stability-aware turbulent exchange before energy
becomes melt. Current CoE does not consume that state. Existing Stage 3 does,
but canonical contracts intentionally preserve CoE melt ownership and discard
positive Stage 3 energy excess rather than converting it to melt.

That omission alone does not invalidate an empirical model: Ohmura 2001
explains why temperature-index methods can proxy multiple energy terms, and
Walter et al. 2005 acknowledges contexts where greater process resolution may
not be justified. The authority gap is narrower and decisive under the frozen
matrix: the material 2007/2008 departures from the Chapter 3 equation have no
cited independent validation or bounded transferability authority. Current
contract ownership makes the path intentional but does not supply that missing
scientific support.

## Quantitative Result

| Site | Eligible hours / days | Positive-term leader | Positive `C` share | Net `C_open` / `C_canopy` (m) | Max applied latent equivalent (`W m^-2`) |
| --- | ---: | --- | ---: | ---: | ---: |
| Mica Creek | 73613 / 3416 | `C` | 52.53% | -0.678 / 4.924 | 217.56 |
| Niwot | 93287 / 4148 | `C` | 41.61% | -4.368 / 5.001 | 228.63 |
| Paradise | 136554 / 5876 | `C` | 53.38% | -0.162 / 14.423 | 306.37 |
| Snowbird | 91251 / 3991 | `C` | 50.44% | -1.851 / 8.906 | 255.80 |

Ran: all four terms and both frozen subcomponent identities reproduce within
`1e-12 m`; the actual maximum residual is `9.941202185450096e-18 m`, and all
daily aggregate residuals are exactly zero. Frozen identities, including the
pinned Git blob, reproduced. The complete machine-readable site counts,
depths, magnitudes, and hashes are in `quantitative-audit.json` and
`execution-receipt.json`.

## Relationship To 21L

21L remains diagnostic and chronology-confounded. This audit sharpens, but
does not convert, its `cmelt` dominance into causation: the 2008 canopy
air-temperature branch is the only net-positive `C` subcomponent in signed
all-hour site sums, while the open wind/dewpoint subcomponent is net negative
at every site. This is not a decomposition of positive-only `C`. It identifies
an authority priority; it does not validate removal, coefficient change, or
replacement.

The widespread raw positive-melt exposures with interval-start density below
`350 kg m^-3`, frozen dewpoint, and same-hour snowfall describe when the
empirical generator is active. They do not identify density after same-hour
mixing or prove routed loss because cap, holding capacity, daily
redistribution, and downstream liquid routing intervene.

## Frozen-Matrix Application

- `CURRENT_AUTHORITY_SUFFICIENT`: rejected because the specific material
  post-handbook changes lack cited independent validation or a bounded
  transferability claim; the contract's intentional ownership decision does
  not close that scientific-authority gap.
- `RUST_TRANSCRIPTION_DEFECT`: rejected because exact same-input reconstruction
  and static pinned-source comparison pass.
- `BASELINE_FIDELITY_WITH_AUTHORITY_GAP`: selected because Rust is faithful to
  the pinned empirical lineage while physical production sufficiency is not
  established.
- `UNRESOLVED_EVIDENCE`: rejected as the primary outcome because identities,
  chronology, and missing physical operands are sufficient to classify the
  seam, although they are insufficient to select a replacement.

## Bounded Next Action

The smallest justified successor is a contract-first CoE/Stage-3 melt-owner
authority reconciliation. It should first decide whether the current empirical
CoE claim can be explicitly bounded and retained or whether melt ownership
must consume the existing state-resolved Stage-3 control volume. It must
prioritize the 2008 `C_canopy` branch, the daily midpoint-temperature
gate, `A`'s embedded albedo lineage, and the exact moment cold content is
satisfied. Only after contract adjudication may a separate package propose
tests or production changes.

This package authorizes no correction, coefficient tuning, default change,
counterfactual implementation, or promotion claim.
