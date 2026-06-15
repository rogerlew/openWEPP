# CQR06 Line Count Governance Checklist

Evidence class: Static

Target file:

- Path: `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`
- Before: `1617` lines.
- After: `2527` lines.

Governance disposition:

- `>= 2000` line WARN threshold: triggered.
- `>= 3000` line block threshold: not triggered.
- WARN owner: CQR06 package disposition.
- Follow-on intent: consider a future behavior-preserving file/module split for
  WB19 lateral, drainage, and WB14 ksat-adjustment helpers after this CRAP
  reduction lands. This package intentionally excluded file splitting.
