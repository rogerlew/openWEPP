# TESTGATE Host Capacity And Security

## `forest1` Pivot Intake

Ran: read-only SSH and provider intake on 2026-07-19 PDT.

- SSH hostname: `forest1`; x86_64 with 48 online logical processors.
- Memory: 188 GiB total, 64 GiB available at intake, and 8 GiB swap.
- Root/Docker filesystem: 938 GiB total with 306 GiB available.
- Docker server: `29.4.2`.
- Existing unrelated service containers remain out of scope. The runner gets no
  Docker socket, host bind, host network, privileged mode, or service/data
  mount, and the setup performs no host-global package installation.
- Reviewed runner envelope: 32 CPUs pinned to CPU IDs 0-31, 48 GiB memory with
  swap disabled at the same hard limit, 8,192 PIDs, 40 GiB executable target
  tmpfs, 24 GiB work tmpfs, and the same read-only-root/capability-drop/
  completion-purge design proven on `omarchy`. This preserves at least 12 GiB of
  the intake `MemAvailable` for host co-tenants even if the runner reaches its
  hard limit.
- Image construction is a separate non-registering controller operation using
  the default Docker driver and per-build resource controls capped at CPUs 0-7
  and 24 GiB memory with swap disabled. A SHA-256-verified archive installs only
  the reviewed image on `forest1`. Setup requires 60 GiB immediately available
  and refuses an absent or nonmatching reviewed image identity.
- Read-only coexistence intake found no container named
  `openwepp-actions-runner` and none of the dedicated `openwepp-runner-state`,
  `openwepp-runner-work`, `openwepp-runner-cargo`, or
  `openwepp-runner-target` volumes on `forest1`.
- Provider target: repository-scoped runner `forest1-openwepp-01` with labels
  `self-hosted`, `Linux`, `X64`, `openwepp`, `forest1`, and `trusted`.
- The `omarchy-openwepp-01` registration was offline at pivot intake and is not
  eligible for the new exact workflow label.

### Controller Image-Build Intake

- Controller hostname `forest`: 48 logical CPUs, 125 GiB RAM, and 114 GiB
  available at intake.
- Docker 29.6.2 uses the default non-privileged `docker` Buildx driver;
  Buildx 0.35.0 and BuildKit 0.31.2 expose per-build `--resource` controls.
- The rejected `forest1` Docker-container builder probe confirmed its cgroup
  limits but also proved that driver is privileged. The exact builder and its
  volume were removed, and the mutable `moby/buildkit:buildx-stable-1` image and
  layers pulled by the probe were deleted before adopting controller-local
  construction.

### Controller Image-Build Receipt

Ran: 2026-07-19 PDT.

- `manage.sh build-image` completed successfully in 599 seconds with the
  reviewed default-driver limits: CPUs 0-7, 24 GiB memory, and swap disabled at
  the same hard limit.
- The loaded candidate is `openwepp-actions-runner:2.335.1`, image ID
  `sha256:a0dbc987aa4ea42041e1148739a04ee8b2ce805e38d0197c16d3f4545baf7f6d`,
  size `4,231,049,535` bytes, created at `2026-07-19T07:05:55-07:00`.
- The manager, workflow envelope, integration contract, and this receipt bind
  that exact image ID before archive creation or transfer. No image archive or
  runner artifact had been sent to `forest1` at receipt time.
- A disposable local container probe used a read-only root filesystem, dropped
  every capability, enabled `no-new-privileges`, and confirmed Rust 1.92.0,
  Nextest 0.9.138, cargo-deny 0.19.6, cargo-llvm-cov 0.8.7, cargo-crap 0.2.2,
  Python 3.12.3, pandas 3.0.3, PyArrow 22.0.0, PHP 8.3.6, markdown-doc-cli
  0.1.0, and both runner control scripts on the exact candidate.
- The image archive was `4,265,653,760` bytes with SHA-256
  `804b66d3fdadbf93f96e601886e349f6430c41f76504c90c99528ba46de6537f`.
  The digest matched before and after transfer; `forest1` loaded the exact
  reviewed image ID, and both temporary archives were removed on return.

### `forest1` Provisioned Receipt

Ran: 2026-07-19 PDT.

- Provider ID `23`, name `forest1-openwepp-01`, version `2.335.1`; state
  `online`, `busy=false`, with exactly `self-hosted`, `Linux`, `X64`,
  `openwepp`, `forest1`, and `trusted`. The stale `omarchy-openwepp-01`
  registration was deleted.
- Container `openwepp-actions-runner` is running from the exact reviewed image
  with restart policy `unless-stopped`, 32 CPUs pinned to `0-31`, 48 GiB memory
  and equal memory-plus-swap limit, and 8,192 PIDs.
- Docker inspection confirms a read-only root, all capabilities dropped,
  `no-new-privileges`, bridge networking, non-privileged mode, no bind mounts,
  and one read-only `openwepp-runner-state` volume. The six bounded tmpfs
  surfaces match the reviewed 40/24/8/2/1/0.25-GiB design.
- A live unprivileged probe rejected writes to the root and registration state,
  confirmed the Docker socket absent, and confirmed each bounded job surface
  writable. A disposable exact-image cleanup probe terminated a process that
  respawned on `TERM` and emptied every writable surface. The live provider
  remained online and idle afterward.
- Existing service containers remained running after setup; none is mounted
  into or reachable through a host namespace shared with the runner.

## Original `omarchy` Intake And Evidence

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

## Original `omarchy` Provisioned Receipt

Ran: 2026-07-18 PDT / 2026-07-19 UTC.

- Original `omarchy` container image ID was superseded by the reviewed
  `forest1` pivot candidate recorded above. This revision retains Ubuntu's repository-pinned
  `ripgrep 14.1.0-1` and adds Python 3.12's `python` alias, system-visible
  pandas `3.0.3`, PyArrow `22.0.0`, PHP `8.3.6`, and `uk2us` at commit
  `6ce03a96a9466bed029fb0287786cd903f1876d6` with hashed executable and spelling
  rules. These tools satisfy the clean-workspace inventory without job-time
  executable installation.
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
  read-only during jobs. Work (16 GiB), Cargo (4 GiB), target (40 GiB), home
  (512 MiB), diagnostics (256 MiB), and `/tmp` (1 GiB) are bounded tmpfs
  mounts. Only target is executable, because Cargo must execute freshly built
  build scripts and test binaries there; work, Cargo source cache, home,
  diagnostics, and `/tmp` remain `noexec`. Host bind mounts are absent; no
  Docker socket, host home, or homelab data path is present.
- Acceptance run `29673766929` proved that the original 8 GiB target bound was
  too small for a clean workspace inventory: compilation stopped with
  `No space left on device` before plan selection or gate execution. The target
  tmpfs bound was raised to 20 GiB; it remains isolated, purge-on-completion,
  and dynamically allocated under the unchanged 28 GiB container memory cap.
- Acceptance runs `29673913049` and `29674327727` exposed two distinct target
  lifecycle defects. The first retained concurrent reconstruction and executor
  trees. Direct sharing then reused binaries containing the temporary
  snapshot's compile-time source path, so path-aware tests in the second run
  could not find the deleted snapshot. Reconstruction now uses an isolated
  target tree that is deleted before preflight; preflight, gate execution, and
  local receipt reconstruction then use one retained repository-root tree.
  This bounded sequential lifecycle prevents concurrent full build trees and
  path-bound binary reuse. The GitHub-hosted immutable-envelope verifier still
  reconstructs independently on a clean hosted worker.
- Exact-candidate run `29677049559` proved that four concurrent independent
  repository-snapshot verifier fixtures could fill the 20 GiB target mount and
  that `/cache/target/e/execution/.work/tmp` remained too long for the longest
  Unix-domain socket fixture. The target mount was 26 GiB at `/t`, with
  execution paths `/t/e` and `/t/p`; full-profile executor/verifier snapshot
  fixtures are serialized without removing or skipping any test.
- Exact-candidate run `29677779525` then proved that one serialized immutable-
  envelope reconstruction can peak near the 26 GiB ceiling because it compiles
  a complete independent workspace beside the retained primary full-suite
  target. The dynamically allocated `/t` ceiling is now 40 GiB; the container
  retains its 28 GiB memory limit and host-backed swap boundary, and the same
  completion hook purges the entire mount after every job.
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
  the `llvm-tools-preview` component, and markdown-doc-cli 0.1.0. The
  unprivileged live runner also reported Python
  3.12.3, pandas 3.0.3 and PyArrow 22.0.0 under
  `/usr/local/lib/python3.12/dist-packages`, PHP 8.3.6, and the pinned `uk2us`
  help surface; a disposable `--system-site-packages` virtual environment
  imported the same pandas and PyArrow versions.
- Clean removal deleted the provider registration, container, and dedicated
  registration volume. Reprovisioning created exactly one online/idle runner
  with `disableUpdate=true`; the final image/sizing reprovision again reported
  `online`, `busy=false`. Stale or missing provider state now fails closed.
