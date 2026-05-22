# HBP Provenance Pin Record

Static: provenance pins captured with exact commit and file-hash evidence.
Ran: SHA/commit evidence commands executed.
Status: complete.

## ADR-0012 Baseline Governance

Pinned static legacy baseline (ADR-0012):

- repo/worktree: `/workdir/wepp-forest_260430_baseline`
- commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- commit message: `Fix wshpas leap-day truncation and re-release wepp_260430`

HBP authority exception (also ADR-0012):

- HBP parser/serialization authority remains tied to `/workdir/wepp-forest`
  contract surfaces.
- Therefore ARCH18 records both baseline pin and active HBP authority pin.

## Active HBP Authority Pins

`/workdir/wepp-forest`:

- repo HEAD used: `028feb2317a35a9ad3e578c0e5798631fc0e61bd`
- `docs/contracts/hillslope-binary-pass-format.md`
  - last-touch commit: `720d14cd1f61164801205986d7990e6424b90686`
  - commit date: `2026-05-14T15:11:16-07:00`
- `docs/contracts/watershed-hillslope-pass-reader-contract.md`
  - last-touch commit: `3c854a98e46a32a66364c7558473454dfb2667fd`
  - commit date: `2026-05-14T13:13:21-07:00`

`/workdir/wepppyo3` reference implementation:

- repo HEAD used: `6c92e3fa70e45838e2a4778ee70ceae88db8e42b`
- `wepp_interchange/src/hill_hbp.rs`
  - last-touch commit: `3992e598067793117c300ab47ea12505d85478cb`
  - commit date: `2026-05-14T14:58:48-07:00`

## File Hash Evidence (sha256)

- `72c44b64749ad5db5efcf0ef7323f42cfab78b19ce112ef50ceef720d694bcdf`  `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`
- `8949793f5f9c2259579fd99ccc9ac0d5f551465eee8478dc278d8be20b22e04f`  `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md`
- `e79c5a970b22a37b2afca9804a240a1fc74f3eabee266a617cbedf9c3cdca313`  `/workdir/wepppyo3/wepp_interchange/src/hill_hbp.rs`
- `73c89ef4d81ad28e025521de1c9eb366d545cecc1396bb73c5746458cc534ef1`  `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/hbp.rs`
- `7f6d8e45277cf567397039aaf0fcce6b8f544ddacac6ae13e3dc8434ed5954d1`  `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/hbp.rs`

## Provenance Decision

ARCH18 uses pinned baseline governance plus explicit HBP authority SHA capture,
in line with ADR-0012 section on HBP contract authority.
