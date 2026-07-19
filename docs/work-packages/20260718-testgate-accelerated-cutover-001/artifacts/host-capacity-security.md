# `omarchy` Host Capacity And Security

Ran: read-only SSH and provider intake on 2026-07-18 PDT.

## Capacity

- Tailnet address: `100.73.176.77`; SSH hostname `omarchy`.
- Host: Arch Linux, kernel `6.18.7-arch1-1`, x86_64.
- CPU: 16 online logical processors.
- Memory: 32,589,248 KiB reported by `/proc/meminfo`.
- Root filesystem: 497,991,680 KiB total; 467,791,816 KiB available.
- Docker client/server: `29.2.1`; overlayfs storage; seccomp and cgroup
  namespace security options active.
- No existing containers or Docker volumes were present at intake.
- SSH principal `roger` is UID 1000 and belongs to the `docker` group.

## Isolation Decision

Static: Arch Linux is not in GitHub's supported self-hosted runner OS list.
Docker is the only installed isolation mechanism; no `virsh`, `incus`, or QEMU
binary was found. The implementation therefore uses an Ubuntu 24.04 container
as the supported runner userspace. The runner process is non-root and receives
only dedicated work/cache volumes plus outbound network access. It does not
receive the Docker socket, host homes, unrelated homelab paths, privileged
mode, host networking, or reusable registration credentials.

The repository is public. Workflow admission therefore permits only trusted
pushes to `main` and manual dispatches whose exact commit is on `origin/main`.
No `pull_request` or `pull_request_target` event is routed to this runner.

## Pinned Intake

- Ubuntu 24.04 image manifest digest:
  `sha256:4fbb8e6a8395de5a7550b33509421a2bafbc0aab6c06ba2cef9ebffbc7092d90`.
- GitHub Actions runner: `2.335.1`.
- Official runner linux-x64 archive SHA-256:
  `4ef2f25285f0ae4477f1fe1e346db76d2f3ebf03824e2ddd1973a2819bf6c8cf`.
- Rust toolchain target: `1.92.0` with Rustfmt and Clippy.
- Gate tools: cargo-nextest `0.9.138`, cargo-deny `0.19.6`,
  cargo-llvm-cov `0.8.7`, cargo-crap `0.2.2`.

## Provisioned Receipt

Ran: 2026-07-18 PDT / 2026-07-19 UTC.

- Container image ID:
  `sha256:cae074269ed8afd33aeeca7ad66143733d0ee379fd8baa546c9afd1f8e09aaf6`
  (`767,010,889` bytes). This revision adds Ubuntu's repository-pinned
  `ripgrep 14.1.0-1`, required by the planner's execution-context identity.
  `manage.sh` rejects any rebuilt image with a different
  identity until the reviewed lock is deliberately updated. The Ubuntu base,
  runner archive, Rust installer, and copied Markdown tool are digest-pinned;
  Ubuntu apt repository contents are recorded by the image identity but are
  not claimed to be byte-reproducible after cache eviction.
- Acceptance run `29671679629` exposed two immutable-image launch defects
  before gate execution: Rustup tried to update a named channel on the
  read-only toolchain store, and the runner rejected a completion-hook path
  without a supported script extension. The rebuilt image pins
  `RUSTUP_TOOLCHAIN=1.92.0-x86_64-unknown-linux-gnu` and installs/configures the
  hook as `openwepp-job-completed-hook.sh`; the runner returned online and idle
  on that exact image.
- Container/service identity: `openwepp-actions-runner`, Docker restart policy
  `unless-stopped`, running as image user `runner` (UID 10001). Runner
  self-update is disabled; an update requires a reviewed pinned-image change.
- Provider identity: repository-scoped runner `omarchy-openwepp-01`, version
  `2.335.1`; state `online`, `busy=false` at receipt time.
- Provider labels: `self-hosted`, `Linux`, `X64`, `openwepp`, `omarchy`,
  `trusted`.
- Runtime limits: 16 CPUs, 28 GiB memory, 4096 PIDs, all Linux capabilities
  dropped, `no-new-privileges`, bridge networking, not privileged.
- The container root filesystem and sole named registration-state volume are
  read-only during jobs. Work (16 GiB), Cargo (4 GiB), target (8 GiB), home
  (512 MiB), diagnostics (256 MiB), and `/tmp` (1 GiB) are bounded tmpfs
  mounts. Only target is executable, because Cargo must execute freshly built
  build scripts and test binaries there; work, Cargo source cache, home,
  diagnostics, and `/tmp` remain `noexec`. Host bind mounts are absent; no
  Docker socket, host home, or homelab data path is present.
- A root-owned runner completion hook repeatedly terminates non-control-plane
  processes owned by the runner UID until quiescent and deletes work, Cargo,
  target, home, `/tmp`, and writable diagnostics after every job. A standalone
  container probe used a TERM trap that forked a replacement process; repeated
  cleanup terminated both generations and left all six writable surfaces
  empty. The earlier direct probe proved the rest of `/runner-state` rejects
  writes.
- Container logs showed authenticated outbound GitHub connectivity and
  `Listening for Jobs` at `2026-07-19T00:23:42Z`.
- Direct tool verification passed for Rust 1.92.0, Nextest 0.9.138,
  cargo-deny 0.19.6, cargo-llvm-cov 0.8.7, cargo-crap 0.2.2, and
  markdown-doc-cli 0.1.0.
- Clean removal deleted the provider registration, container, and dedicated
  registration volume. Reprovisioning created exactly one online/idle runner
  with `disableUpdate=true`; the final image/sizing reprovision again reported
  `online`, `busy=false`. Stale or missing provider state now fails closed.
