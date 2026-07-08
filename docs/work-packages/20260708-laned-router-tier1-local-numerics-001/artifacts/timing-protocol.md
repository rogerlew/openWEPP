# Timing Protocol

Status: `EXECUTED`

Protocol:

- Build command:
  `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Binary:
  `target/release/openwepp-cli-hill`
  `5b6788c795600d6329a46bb12b52f3c3107938ca29e5e3d0726cbf91075fa01e`,
  `9947912` bytes, mtime `2026-07-08 12:21:30.429767052 -0700`.
- Fixture: `tests/fixtures/laned_shadow_h2637` copied to `/tmp`, patched like
  the integration helper: `ow-lanuse-1`, 19 native cropland landuses, 19
  `routing_coefficients` blocks `500.0 0.0 0.0 0.0 0.0`.
- Timing command shape:
  `/usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill --run-dir ...`.
- Repeat count: 3 endpoint runs, profiling off.
- Slot profile: `OPENWEPP_LANED_SHADOW_PROFILE=1` on default-active run.
- Perf: `perf stat -d` around the same release command.
- Pre-change comparison binary: detached worktree at commit `46532c28`, built
  with the same release command and run once on a copied `/tmp` fixture.
