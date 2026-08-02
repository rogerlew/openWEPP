# Baseline Replay Evidence

Status: `PASS`

Evidence mode: **Ran**.

The `1.0` cells replayed EB-04W's baseline-B release runs exactly. The package
analysis reconstructed every frozen operator with zero numeric residual. A
separate byte-level comparison also found identical WAT parquet and snow-trace
hashes for all four lanes.

| Lane | WAT SHA-256 | Snow trace SHA-256 | Operator residual |
|---|---|---|---:|
| Mica Creek | `c24263562339c816d5b46e8db09c307cc3f5f0d94bff5115bc1ff7c1634aaa40` | `35217bbc5cde3eeafb3d47e5be2b744022752262fb25327121d274a1d7616b4f` | `0` |
| Niwot | `082e243e82bccfc6cb3ad26897bb8d8ebb47f7e42981427b5dddbecf2b985db3` | `6be8cf1057c3f6a20198a8211de99e318ad42501fac3bb6af088849651d9ab36` | `0` |
| Paradise | `4b3f768a2ea540b31c1daf88598172243600a6b6e446e8764fc4f7216245922f` | `3ad6154283dd57c6845e632311cf14e7f6a17967260b1e12d6b4fb2ad728fcf5` | `0` |
| Snowbird | `30014d20e2037642bec3b15a4a03c5609daf7396aff942054cb23d2b219b8ef6` | `09fac357a9464ecb33f513b561eda02e5a5e62552b9a0e226a140a74b3163239` | `0` |

The two hashes in each row apply independently to the EB-04W source output and
the EB-04W1 `p100` output. This proves that fixture copying, runfile rewriting,
and the package-local transformer are behavior-neutral at multiplier `1.0`.
