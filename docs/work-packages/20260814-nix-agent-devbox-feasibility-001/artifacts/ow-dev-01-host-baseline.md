# `ow-dev-01` Host Baseline

Status: `Ran / pre-Nix baseline`

Date: `2026-08-14`

- CPU: Intel Core i7-13620H, 10 physical cores, 16 logical CPUs.
- RAM: 30 GiB visible; 8 GiB swap.
- `/workdir`: encrypted ext4 on NVMe, approximately 1.3 TiB available at intake.
- `/tmp`: encrypted ext4 on NVMe, approximately 503 GiB available at intake,
  mode `1777`, mounted `nosuid,nodev` and executable.
- `systemd-tmpfiles-clean.timer`: active, daily.
- Host override: `/etc/tmpfiles.d/tmp.conf` contains
  `q /tmp 1777 root root 3d`.
- A scoped `systemd-tmpfiles --clean --dry-run --prefix=/tmp` produced no
  deletion candidates at intake.
- Rust and Nix were not installed in the ordinary user environment at intake.

The live `/etc` override is host state, not yet a tracked deployment mechanism.
Phase 0 must add a report-only host check so drift is visible.
