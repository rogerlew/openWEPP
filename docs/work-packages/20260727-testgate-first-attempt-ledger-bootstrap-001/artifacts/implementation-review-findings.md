# Implementation Review Findings

Status: `RESOLVED`

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
39/39. The cross-process correction was prospectively authorized at
`33572b795c3c21d10fb063b6477df81407fbc44f`, implemented at
`4f99bd38e8a3b2b5ab0f2c3b4968a35f551e3328`, and completed after adversarial
review findings at `bd64bc1b8adcd9dd2db789e7770d2126f5f5bfc6`.

Two independent implementation re-reviews dispositioned the exact corrected
commit `GO`. No canonical retry is authorized until dual terminal verification
also passes.

## Re-review Closure

| Finding | Closure evidence |
|---|---|
| Rust reopened the ledger pathname after Python validation | exact inherited FD, mandatory transition-only `--resume-fd`, device/inode match, retained `BoundAttemptLedger` |
| Bound admission followed `metadata()`/`canonicalize()` | no-follow metadata plus repeated full no-follow validation; same-inode final/ancestor race tests |
| `resume-fd` could be ignored outside transition | absent, LIGHT, and HEAVY stage rejection tests |
| Bound appends were not serialized | `Mutex<File>` covers read/verify/predecessor/append/sync; 32-writer chain test |
| Invalid-FD and exact-transport tests were incomplete | directory descriptor rejection and exact Python CLI/`pass_fds` inode assertion |

No `unsafe`, ledger-schema change, Rust preflight relaxation, CAL population,
or Harvard access occurred.
