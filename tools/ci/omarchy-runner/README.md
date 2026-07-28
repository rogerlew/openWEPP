# `forest1` quality-observatory runner

`manage.sh build-image` builds the pinned Ubuntu 24.04 image on the controller.
After its exact identity is reviewed, bound, and installed, `manage.sh setup` registers the
repository-scoped `forest1-openwepp-01` runner and starts it with labels
`openwepp`, `forest1`, and `trusted`. The runner is unprivileged, has all Linux
capabilities dropped, and receives neither the Docker socket nor host-home or
homelab mounts. Registration tokens travel only over stdin and are not retained
by the scripts.

The directory name is retained as a compatibility path for the pinned workflow
and image inputs. `OPENWEPP_RUNNER_HOST`, `OPENWEPP_RUNNER_NAME`,
`OPENWEPP_RUNNER_SITE_LABEL`, and `OPENWEPP_RUNNER_LABELS` remain explicit
operator overrides; the reviewed defaults select `forest1`.

```bash
bash tools/ci/omarchy-runner/manage.sh build-image
# Review the reported image, update EXPECTED_IMAGE_ID, then:
bash tools/ci/omarchy-runner/manage.sh install-image
bash tools/ci/omarchy-runner/manage.sh setup
bash tools/ci/omarchy-runner/manage.sh status
```

`build-image` is deliberately separate from registration. It uses the
controller's non-privileged default Docker BuildKit driver with per-build
resource controls for 8 CPUs and 24 GiB with swap disabled, prints the resulting
image identity, and cannot contact or register `forest1`. After review and
rebinding, `install-image` transfers a Docker archive, verifies its SHA-256 on
both hosts, loads it, and requires the exact reviewed image identity. Temporary
build and transfer files are removed on return.
`setup` fails unless that already-present image exactly matches the reviewed
`EXPECTED_IMAGE_ID`, the host has at least 60 GiB available, and the provider
ultimately reports one online, idle runner with the exact label set.

One named volume holds registration state and is mounted read-only for jobs. A
second volume is mounted read-write at `/quality-history` for optional quality
observations and bounded controller evidence; setup fixes ownership for the
unprivileged runner and ordinary removal deliberately preserves it. Work,
Cargo downloads, target output, home, diagnostics, and `/tmp` use
size-bounded tmpfs mounts. A root-owned completion hook repeatedly terminates
runner-owned job processes until quiescent, then clears job, target, dependency,
home, temporary, and diagnostic state after every attempt. A killed runner
loses those surfaces when Docker restarts it. The container root filesystem is
read-only.

The image verifies the official runner archive, GitHub CLI 2.96.0 archive,
Rust installer, and controller's `markdown-doc` binary by SHA-256. Rust,
GitHub CLI, the repository's gate tools, and Rust's LLVM coverage component are
pinned in the image; normal jobs verify them instead of installing them. The
clean-workspace test inventory also receives Python 3.12, pandas 3.0.3,
PyArrow 22.0.0, PHP 8.3, and the commit-pinned `uk2us` converter with the
repository's reviewed spelling rules.
Bootstrap creates the ignored repo-local `.venv` with system packages enabled,
and the completion hook removes it with the workspace. Runner updates are
deliberate image revisions so a job cannot replace the persistent control
plane.

Quality execution evidence, Cargo output, and temporary files share the bounded
40 GiB executable tmpfs at the short fixed paths `/t/e`, `/t`, and `/t/p`.
Concurrency is one and the completion hook purges the mount after every job, so
the fixed names stay fresh while keeping Unix-domain socket fixtures below
Linux's path limit. Repository-snapshot fixtures are serialized within
the full profile so their disposable build trees cannot exhaust the mount.

The runtime is capped at 32 of 48 CPUs, 48 GiB memory with swap disabled, and
8,192 PIDs. The pre-launch memory check preserves at least 12 GiB of observed
host headroom for co-tenants before the runner is admitted.

`manage.sh remove` deregisters the runner, stops and removes its container, and
deletes the dedicated registration-state volume. Job surfaces are tmpfs. The
separate quality-observatory history volume and digest-indexed uploads
retain only optional quality evidence and bounded controller records. Retired
TESTGATE history is neither mounted nor modified.
