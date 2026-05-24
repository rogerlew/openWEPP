# Verification Agent B

Status: pass
Evidence mode: Ran

## Ran
Runtime and behavior verification:

1. Strict fixture execution
- Command: `openwepp-cli-hill --run-dir <fixture> --run-file case.run --output-dir <out> --policy strict`
- Result: pass; `H5.wat.dat`, `H5.plot.dat`, and manifest emitted.

2. Strict unknown-sidecar behavior
- Command: same with unknown-sidecar fixture and `--policy strict`
- Result: hard-fail with `LSB-E-009`.

3. Compat unknown-sidecar behavior
- Command: same with unknown-sidecar fixture and `--policy compat`
- Result: pass with warning `LSB-W-002`.

4. Missing required sidecar behavior
- Command: remove `frost.txt`, run strict and compat
- Result: both hard-fail with `LSB-E-007`.

5. Launcher boundary execution
- Command: `open_wepp_runner run-hillslope --engine openwepp ...`
- Result: pass; required outputs emitted.

6. Release-lint boundary execution
- Command: `open_wepp_runner release lint --release-dir /tmp/cli01_release_lint_uQLT9H`
- Result: pass.
