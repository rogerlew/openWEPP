# PERFDEEP05 Endpoint

Evidence class: Ran.

## Final-Code Endpoint Results

Release binary:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: passed, final binary SHA-256
`6833a30b57ef7a96b409437a656b91037e9db7e0a3a77b24471bcdaf299a07a6`.

| Run | Env | Elapsed s | Max RSS KB |
|---|---|---:|---:|
| Default-disabled H2637 | none | 701.95 | 227712 |
| PERFDEEP05 opt-in H2637 | `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1` | 911.11 | 229820 |

Reference:

```text
PERFDEEP01 H2637 reference: 669.97 s
```

Comparison:

| Comparison | Result |
|---|---:|
| Opt-in minus PERFDEEP01 reference | +241.14 s |
| Opt-in/reference ratio | 1.360x |
| Opt-in minus final default-disabled | +209.16 s |
| Opt-in/default ratio | 1.298x |
| Opt-in improvement versus PERFDEEP04 profiled opt-in `1164.31 s` | -253.20 s, 21.7% faster |

## Interpretation

PERFDEEP05 improved the PERFDEEP03 opt-in lane-dense path by removing the
measured full resync hotspot, but the real H2637 endpoint remains materially
slower than the default-disabled path and the `669.97 s` activation reference.

No default activation is allowed.
