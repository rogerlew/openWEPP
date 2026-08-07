# Rejected Execution V2

Status: `INVALID_CONSUMER / no scientific result / retained`.

The required comparator runner invoked the exact `attempt-002` command once at
clean `39e82fbb49664f37e6d90860a8c78a93e5676050`. Retained hash verification
and trace parsing began, then the analyzer exited status `1` after `171.889 s`
with maximum RSS `33,384 KiB`:

`RuntimeError: joined fixed daylight differs in IEEE-754 representation`.

Cause: the package consumer treated Python `bool` as generic numeric because
`bool` subclasses `int`, then incorrectly applied the predecessor's float-bit
comparator to the exact boolean `daylight` field. This is an analyzer type-
dispatch defect, not trace disagreement or a scientific result. No result
directory, water-year metric, or decision class was produced.

Sibling wrapper logs are retained under `runner-logs-attempt-002/`:

- stdout: `0` bytes,
  SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- stderr: `774` bytes,
  SHA-256 `1d1a3b7d9299ad0ae114ae2f2418c6e2a61f8efa413906ab79c77d977057c4c4`;
- timing: `44` bytes,
  SHA-256 `99e71253823519ade3605b6b50a2eb833e2d6dd51976a293d15493c86dc8fb81`.

Prospective correction: dispatch only actual floats through IEEE-754 comparison
and compare booleans/strings exactly, matching the predecessor independent
consumer. Add a boolean joined-field vector, re-review, and use new immutable
`attempt-003`; never reuse or remove the rejected namespace.
