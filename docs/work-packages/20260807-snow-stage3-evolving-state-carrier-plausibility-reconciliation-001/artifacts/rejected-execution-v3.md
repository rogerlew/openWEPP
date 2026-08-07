# Rejected Execution V3

Status: `INVALID_CONSUMER / no scientific result / retained`.

The comparator runner invoked `attempt-003` once at clean
`8513994778326bb02b9836c822843c624d6376d0`. After retained verification and
trace parsing began, the analyzer exited status `1` after `120.618 s`, maximum
RSS `67,400 KiB`, with:

`RuntimeError: inactive joined non-formulation fingerprints are not zero`.

Cause: the analyzer conflated empty resolved support with the separate
`operator_not_selected` state. Retained operator-selected days with
`no_resolved_snow_at_day_start` legitimately have zero tuples and distinct,
nonzero operator-specific fingerprints. All four sites contain such rows. The
correct guard permits either paired zero sentinels or paired distinct nonzero
operator fingerprints, and rejects mixed sentinel applicability or a nonzero
cross-operator alias.

No result directory or scientific metric was produced. Sibling logs:

- stdout: `0` bytes,
  SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- stderr: `1,158` bytes,
  SHA-256 `65d76e62d72241ac62f159d8d7e85cd2795131d62f6fca51c492e70af6d03e55`;
- timing: `74` bytes,
  SHA-256 `f7bdb60ad91e774c7d09c4a1f065158b010ddb42f803554879e9ae09e402696e`.

Prospective recovery uses reviewed immutable `attempt-004`; rejected attempts
remain untouched.
