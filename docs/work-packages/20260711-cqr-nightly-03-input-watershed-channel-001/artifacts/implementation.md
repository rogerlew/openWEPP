# Implementation

Static + Ran: attempted a mechanical extraction of header, mode, parameter,
rating-curve, control-override, and per-channel blocks after module-level
coverage percentages passed, but before the per-logical-function floor was
actually closed. The attempt preserved parse/guard/warning order and passed
focused tests and clippy. Independent Review A found no semantic drift; Review B
identified the sequencing violation.

The attempt is not landed. After Review B identified the pre-existing
`G-CHN-013` error-contract mismatch, both owned implementation paths were rolled
back byte-for-byte to scaffold `a7d07708`:

- target SHA-256: `e0b04f88051fd7030446313bb2a853fe4d08100b4c2911c8845bed757f28d154`;
- focused test SHA-256: `46b18d420e23a3b83709e3e5107d4d931b559b33d99c05c928380faa2bd91fb8`.

Attempt-only patch evidence is
`/tmp/openwepp-cqr-20260711-t03-attempt.patch` (`35332` bytes). No production or
test edit remains.
