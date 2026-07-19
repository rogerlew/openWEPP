# `omarchy` TESTGATE runner

`manage.sh setup` builds a pinned Ubuntu 24.04 container on `omarchy`, registers
the repository-scoped `omarchy-openwepp-01` runner, and starts it with labels
`openwepp`, `omarchy`, and `trusted`. The runner is unprivileged, has all Linux
capabilities dropped, and receives neither the Docker socket nor host-home or
homelab mounts. Registration tokens travel only over stdin and are not retained
by the scripts.

```bash
bash tools/ci/omarchy-runner/manage.sh setup
bash tools/ci/omarchy-runner/manage.sh status
```

The only named volume holds registration state and is mounted read-only for
jobs. Work, Cargo downloads, target output, home, diagnostics, and `/tmp` use
size-bounded tmpfs mounts. A root-owned completion hook repeatedly terminates
runner-owned job processes until quiescent, then clears job, target, dependency,
home, temporary, and diagnostic state after every attempt. A killed runner
loses those surfaces when Docker restarts it. The container root filesystem is
read-only.

The image verifies the official runner archive, the Rust installer, and the
controller's `markdown-doc` binary by SHA-256. Rust and the repository's gate
tools are pinned in the image; normal jobs verify them instead of installing
them. The clean-workspace test inventory also receives Python 3.12, PyArrow
22.0.0, PHP 8.3, and the commit-pinned `uk2us` converter with the repository's
reviewed spelling rules. Bootstrap creates the ignored repo-local `.venv` with
system packages enabled, and the completion hook removes it with the workspace.
Runner updates are deliberate image revisions so a job cannot replace the
persistent control plane.

Execution evidence, Cargo output, and planner temporary files share the bounded
26 GiB executable tmpfs at the short fixed paths `/t/e`, `/t`, and `/t/p`.
Concurrency is one and the completion hook purges the mount after every job, so
the fixed names stay fresh while keeping Unix-domain socket fixtures below
Linux's path limit. Repository-snapshot verifier fixtures are serialized within
the full profile so their disposable build trees cannot exhaust the mount.

`manage.sh remove` deregisters the runner, stops and removes its container, and
deletes the dedicated registration-state volume. Job surfaces are tmpfs and
therefore have no retained recovery state.
