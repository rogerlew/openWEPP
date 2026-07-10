# CRAP After

Ran: delegated final command, exit `0`.

```text
cargo crap --lcov /tmp/openwepp-cqr-b02-t03-target-final4.lcov --min 0 \
  --format json --output /tmp/openwepp-cqr-b02-t03-target-final4.json
```

The target has `51` CRAP rows, `0` above `30`, and maximum CRAP
`29.91740980561775` for `parse_rosetta_layer_row` at line `1021` (CC `29`,
89.706% line coverage). Every target function clears the 75% coverage floor;
the independent LLVM region calculation also has no function below 75%. No
`COVERAGE-EXCLUDE` is used.

The JSON SHA-256 is
`d9978fc116baf4b22d0d73d6c99f78ece953ed5b4ed5a39be48a7ecaf4ca1f2f`.
`cargo-crap` emitted its known off-target unmatched-source warning for this
target-only LCOV, but target data is present and complete.
