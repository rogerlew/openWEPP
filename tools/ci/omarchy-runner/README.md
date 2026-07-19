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
them. Runner updates are deliberate image revisions so a job cannot replace
the persistent control plane.

`manage.sh remove` deregisters the runner, stops and removes its container, and
deletes the dedicated registration-state volume. Job surfaces are tmpfs and
therefore have no retained recovery state.
