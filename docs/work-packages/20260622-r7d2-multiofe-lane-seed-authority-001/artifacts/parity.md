# Parity Evidence

Status: executed-held.

## Focused Fixture

- Ran: same-binary focused one-OFE default/direct production fixture after this
  package.
- HBP checksum matched:
  `cbe53a3ee5ac216782fc5db87dacb1dc40ff50f51dc2d5d0cf24171da4371760`.
- Loss checksum matched:
  `36efc6c896e79890c89d1593a95ae20c5d7e84f20e9cb487fe3503eb70676d5e`.
- PASS checksum matched:
  `1a62c2b09aa3507a90536283ee10bf70c9b88caf4c8647c304d67e8a5bff73d3`.
- WAT checksum matched:
  `2216771cc933074b071d75879822c150fabb04f814a895c98b2b68b5c25b051e`.

## H2637

- Ran: same-binary H2637 default and direct production after this package.
- Default compatibility: `633.27 s / 228828 KiB`.
- Direct production: `182.83 s / 627436 KiB`.
- HBP checksum mismatch:
  - default:
    `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8`
  - direct:
    `20037fdacd21c15abbfe0ffdaf7b75f98f053d045ef42d9f4bb66673c18f6366`
- Loss checksum matched:
  `9bdbabe532bfbc2f49d4a4ae5db24c6069e93384f306e71759c223a795a5be38`.
- Plot checksum matched:
  `4cdb19fecd36a3f074d5c900bc687eff7ce58f80a31c9cb7e5e0f5615ac5a783`.
- PASS checksum mismatch:
  - default:
    `9bc37769ec7a544641b903f038f59768c672e0f0b026333921723ebc9ae95a46`
  - direct:
    `3e9420a08c0188f1b87a5efc2fb6ba2634bf86afc28bf6940a158fe9296c6b04`
- WAT checksum mismatch:
  - default:
    `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`
  - direct:
    `6d6dc7e5e25bf2104ec7d568a59eb86eac24300f2050af856c81408f47194179`
- DuckDB bidirectional row differences:
  - WAT default-direct: `235961`; direct-default: `235961`
  - PASS default-direct: `12419`; direct-default: `12419`
- Direct manifest remains on the intended consumer path:
  `scheduler_kernel_executed=false`,
  `publication_source=direct-publication-frame`, `row_count=235961`,
  `compatibility_edge_invocations=0`.

## Residuals

- WAT joined field-diff counts after R7D2:
  `P=17233`, `RM=86125`, `Q=227860`, `Ep=235470`, `Es=235589`,
  `Dp=231934`, `UpStrmQ=209194`, `SubRIn=192445`,
  `Total-Soil=235961`, `SoilWaterTotal=235961`, `Interception=111895`.
- PASS joined field-diff counts after R7D2:
  `runvol=12372`, `sbrunv=12419`, `peakro=0`, `tdet=0`.
- Static diagnosis: day-1 direct ET/profile terms now vary by lane, but direct
  day-1 `Q` remains `45.2 mm` across lanes while default `Q` is near zero to
  `0.49 mm`. The direct R4K infiltration/depression handoff is zero, so R4A
  computes runoff as liquid input.
- Final residual:
  `HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT`.
