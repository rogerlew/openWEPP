# Implementation Review Findings

Status: `CORRECTION REQUIRED`

Evidence class: `Static + Ran`

Reviewed implementation commit:
`43ac2a430a70e82c348ad8cf9f85fd69dba2389f`.

## Disposition

| Finding | Disposition |
|---|---|
| `O_NOFOLLOW` silently degraded when unavailable | `ACCEPTED`; fail closed and test unavailable primitives |
| Ancestor visibility checks were absolute/path-following | `ACCEPTED`; use descriptor-relative no-follow identity checks |
| `observe()` could leak an internally acquired guard | `ACCEPTED`; close only internally owned guards on every exit |
| Durability and ancestor-finalization tests were incomplete | `ACCEPTED`; assert exact file/parent sync and ancestor swaps |
| Python validated, then Rust reopened the ordinary path | `ACCEPTED`; prospectively expand the write set for inherited-FD binding |

The first four findings are corrected in
`7a99228a340618069f6067a815264f2657d8fdc9`. Focused Python tests pass
39/39. The cross-process finding remains a hold until the prospectively
reviewed Rust descriptor-binding correction and its tests pass.

No canonical retry is authorized while this status is
`CORRECTION REQUIRED`.
