# Coverage/CRAP before

Status: complete; coverage expansion/decomposition required
Evidence mode: Ran

The first post-correction focused capture passed 26/26. Target coverage was
494/613 lines (80.587%), 605/764 regions (79.188%), and 22/28 functions, below
the package's 90% science-tier thresholds. Non-format named floors below 75%
were `WatershedChannelParseError::source` 0%, path parse 72.727%,
`parse_single_f64` 70.588%, fixed tuple parsing 73.684%, and float-token parsing
72.727%.

`parse_channel_block` was the eligible CRAP blocker at 36.059 (CC 36,
96.429% covered), so coverage alone cannot close it. Formatting/observability
rows were `WatershedChannelParseError::fmt` CRAP 132 and
`ChannelWarningCode::as_str` CRAP 42; tests will cover stable label/display
behavior before any reviewed exclusion is considered.

Commands exited zero: workspace coverage clean, targeted LCOV, targeted JSON,
and LCOV-backed `cargo crap`. Timings were 1.55, 20.76, 21.14, and 1.15
seconds. Raw evidence:

- `lcov-before.info`: SHA-256
  `31c171c9f76071fa702a4b1c1177f502f44762ce10821c148a94788b77b5b808`,
  210,903 bytes.
- `coverage-before.json`: SHA-256
  `f1e9071a35f17bde23a795890758612c462946c6819c0b77bcec7bb2e79ebeb4`,
  1,054,667 bytes.
- `crap-before.json`: SHA-256
  `f2c86582ea1a89d829ff511b6a6d149a08e48b9f213c6b7ff12810bcc41fe0a1`,
  2,788,845 bytes.

Capture identity: source
`675ef55135e4f89d35f822cdbc836354a4215a6c10c5cf035cbefe51192635dc`
(25,225 bytes, 875 lines) and focused test
`0f3e81d8a6f6c5db9ac7d7258feae1af58945e5f5a7324b32891b2f7458cdc3f`
(16,366 bytes, 496 lines).

## Closed pre-decomposition safety net

Test-only expansion then passed 38/38 against the same production source at
610/613 lines (99.511%), 759/764 regions (99.346%), and 28/28 functions. Every
named function exceeded 75%; the minimum was 96.970%. `parse_channel_block`
remained the sole CRAP row above 30 at exactly CC/CRAP 36, authorizing only its
behavior-preserving decomposition.

Commands were the same clean/LCOV/JSON/LCOV-backed-CRAP sequence and exited
zero in 1.31, 20.73, 21.51, and 1.04 seconds. Safety-net identity: source hash
`675ef55135e4f89d35f822cdbc836354a4215a6c10c5cf035cbefe51192635dc`
and test hash
`7999f66715eef99117426de1c32e5136f350a547d0f6c7c1e3dfad1fba3226ba`.

Raw safety-net evidence:

- `lcov-safety-net.info`: SHA-256
  `a60b7a2f21d19a0628e204ba816576b9f5af31fbbc1f41e33c88ef0317abfe2a`,
  211,917 bytes.
- `coverage-safety-net.json`: SHA-256
  `ea505c2f39fc607e53e8b9833b03e67ff873bdb38401c08275fc60b839788f4d`,
  1,069,985 bytes.
- `crap-safety-net.json`: SHA-256
  `b00eb2731c5ae30f6f6ffd7d86da8e864ccaca5389387e6e93d3609ec4605909`,
  2,788,988 bytes.
