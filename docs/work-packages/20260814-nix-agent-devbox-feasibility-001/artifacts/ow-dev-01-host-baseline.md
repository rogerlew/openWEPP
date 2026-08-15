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
- Nix was absent at initial intake. The user subsequently installed the
  recommended multi-user distribution: Nix `2.35.2`, active `nix-daemon`, and
  the `nixbld` build-user group.
- `/etc/nix/nix.conf` now enables `nix-command flakes`; the previous file is
  retained as `/etc/nix/nix.conf.codex-backup-20260814`.
- Ran: daemon store information query succeeded and `nix eval --expr '1 + 1'`
  returned `2`.
- GitHub CLI authentication is active for `rogerlew` over HTTPS. Git's global
  `github.com` and `gist.github.com` helpers already invoke
  `/usr/bin/gh auth git-credential`; no plaintext Git credential file was
  present. A secret-safe `git credential fill` confirmed username `rogerlew`
  and a nonempty credential without recording its value.

The live `/etc` override is host state, not yet a tracked deployment mechanism.
Phase 0 must add a report-only host check so drift is visible.
