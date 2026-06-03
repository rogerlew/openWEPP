# Contract Implementation Evidence

Status: completed

Evidence mode: static

## Canonical Contract Amendments

- Static: `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  adds `REF-SUBHYD-LEGACY-HOURLY-SSH`, `ui_ssh(i)`, `INV-SUBHYD-027`, guard
  mapping, alias mapping, and the HPHYS0257 hourly horizontal-conductivity
  addendum.
- Static: `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  adds `REF-WATBAL-LEGACY-HOURLY-SSH`, required-input and branch-surface
  references to hourly `wb19_lateral_ssh_####`, and the `ui_ssh(i)` alias map.
- Static: both contract amendments cite pinned baseline source paths under
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Authority Statement

- Static: HPHYS0257 authorizes a distinct hourly lateral conductivity surface
  only; it does not authorize heuristic `latqcc` damping, daily lane changes,
  or storage compensation.
