# Gate results

Status: `EXECUTED HOLD`.

| Gate | Result | Evidence/disposition |
|---|---|---|
| starting identity and prompt intake | `PASS` | `exact-intake.md`; prompt `cmp` and SHA-256 recorded |
| contract/cadence gate | `PASS` | `contract-impact-and-preimplementation-gate.md`; no amendment required |
| assurance generation | `PASS` | `assurance-drift-census.md`; `verify-generation` passed at generation `41b142902d22e139ea732288ed40a504931a1fb54ab27c891d56891910229dd3` |
| current stale contract guards | `PASS` | `56/56`; `stale-contract-assertion-census.md` |
| Stage-3 support/event focused tests | `PASS` | `5` tests passed |
| typed boundary source guard | `PASS` | `3/3` passed |
| affected package compilation/format | `PASS` | `cargo check` and `cargo fmt --all -- --check` passed |
| workspace quick regression | `FAIL / baseline-and-environment debt` | terminal run `2c065e86-4846-47dd-a30c-cbf2a29aec66`: `3108/3118` passed; remaining failures are llvm-tools availability, pre-existing noncanonical legacy fixtures, and stale unrelated authority guards; Stage-3 line-count failure was corrected and targeted observability now passes |
| snow-covered V11/shared-carrier consumer | `BLOCKED` | current real consumer rejects snow-covered lower boundary |
| runner-owned 48-support consumer path | `BLOCKED` | no sealed capability/construction path exists |
| complete owner transaction and terminal receiver consumption | `BLOCKED` | only typed parcel construction exists |
| additive restart/equivalence | `NOT RUN` | outer restart is not implemented |
| positive/poison scenarios | `NOT RUN` | covered path is not executable |
| independent reviews A–D | `NOT RUN` | no independent reviewer result is claimed |
| terminal verifiers A/B | `NOT RUN` | no verifier result is claimed |
| comparator/performance | `NOT RUN` | current path cannot qualify the endpoint |

Any current-scope `BLOCKED` or justified `NOT RUN` prevents complete
disposition. The final disposition is therefore `EXECUTED HOLD`.
