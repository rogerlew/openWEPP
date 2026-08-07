# Review Disposition

Status: `contract review PASS / first tool review HOLD amended / re-review pending`.

Evidence class: `Static`.

| Finding | Disposition | Correction/evidence | Remaining authority impact |
| --- | --- | --- | --- |
| Different forcing hashes invalidated same-forcing attribution | `accepted` | Frozen 2x2 source/forcing matrix and `FORCING_IDENTITY_DIFFERENCE` precedence; v130 contract correction required | No result run until re-review and v130 gate |
| Historical trace mislabeled schema v5 | `accepted` | Corrected to exact schema v4 | None |
| Checkpoint selection could be result-biased | `accepted` | Frozen all 62 first-parent commits into 14 consecutive build-input digest checkpoints | Conditional execution only after endpoint source effect |
| Build/execution custody incomplete | `accepted` | Isolated local clones, `--locked --offline`, per-checkpoint Cargo target, environment scrub, manifests, no overwrite | Runner implementation/test pending |
| Legacy/explicit selector compatibility unfrozen | `accepted` | Legacy selector fixed for interval; explicit absent; current-source equivalence arms required | Execution pending |
| Historical binary unavailable | `accepted` | `HISTORICAL_BINARY_NOT_RETAINED`; rebuilt endpoint called exact-source semantic replay | May justify HOLD only if replay is impossible |
| Contract test write set incomplete | `accepted` | Added root `Cargo.toml` and existing publication parity test | Contract implementation pending |
| Sequential-only run could not prove protected outputs | `accepted` | Every enabled arm paired with disabled control; same-pair HBP/PASS/WAT identity required | Execution pending |
| Line-count risk missing | `accepted` | Baselines frozen; no Rust currently planned; conditional diagnostics constrained to extracted modules | Recount if Rust changes |
| Input/adapter/tolerance/aggregation freeze missing | `accepted` | Added exact `protocol-freeze.json`, windows, hashes, adapters, tolerances, aliases, operand lineage | Dual re-review pending |
| Current scalar contract predicate incompatible with corrected custody | `accepted` | Prospectively require forcing-matched v130 predicate and distinct outcomes | Contract edit before results |
| Earliest difference was overclaimed as causal | `accepted` | Descriptive localization separated from controlled-substitution causality | None |
| Historical boundary could be invisible to current-only instrumentation | `accepted` | If needed, instrument parent/child twins identically and prove neutrality against unmodified endpoints | Conditional |
| Conservation/anti-tautology placeholders | `accepted` | Independent consumer, primitive/mass/cold/conduction closure, per-WY effects, adversarial aliases required | Tool/tests pending |
| HOLD legitimacy absent | `accepted` | Explicit allowed/prohibited state machine frozen | None |
| Ambient `CARGO_HOME` remained writable | `accepted` | Package-local content-hashed non-credential cache seed and recorded identity required | Runner implementation pending |
| Conditional checkpoint trigger/lane ambiguous | `accepted` | Any out-of-tolerance per-WY source delta triggers the affected forcing lane; both lanes run if both trigger | Endpoint execution pending |
| S/F/Q predecessor gate not bound to its forcing | `accepted` | Parametric same-forcing all-WY-plus-median predicate; prior development S/F/Q requires E11-to-E01 | V130/test pending |
| Schema v4 cannot prove v6 primitive/support closure | `accepted` | Old cells narrowed to aggregate custody; v6 retains primitives; localization requires neutral instrumented twins | Tools/tests pending |
| Causal predicates remained placeholders | `accepted` | Six descriptive and six matching single-substitution causal classes frozen with equifinality fallback | Conditional observability only |
| Mandatory v130 phase ordering contradicted package | `accepted` | V130/test/gate/fresh review now precede tools and any result execution | Pending execution |
| INV-097 absent from guard/boundary maps | `accepted` | Added explicit governance/evidence guard and boundary rows plus section-scoped test | Reverification pending |
| Checkpoint trigger omitted median failure | `accepted` | Aligned package/protocol to per-WY-or-median trigger for each forcing | Reverification pending |
| TOL-019 retained superseded predecessor tolerance | `accepted` | Narrowed TOL-019 to sign/support; TOL-020 solely owns predecessor reproduction | Reverification pending |

No result execution may begin with an undispositioned review finding.

Agent A and Agent B independently verified every accepted amendment at exact
clean commit `8135e3b90ec8f1d696c603ece588e868ca000f3c`; both returned `PASS` and
focused contract tests passed `12/12`. Tool implementation is admitted. Model
execution remains gated by tool tests and custody verification.

## First Tool Review

Agent A and Agent B reviewed exact clean commit
`1b035592d48fedf2a214b7e01c76d717dbd25c6c` without running the model or
inspecting new results. Both returned `HOLD`.

| Finding | Disposition | Result-blind amendment |
| --- | --- | --- |
| Multi-gigabyte traces were materialized | `accepted` | Stream JSONL bytes in exact date order and retain only daily scalars |
| V6 reconstruction trusted producer-derived terms | `accepted` | Bind the prior reviewed independent primitive radiation, turbulent, precipitation-advection, mass, cold, support, cadence, and continuity consumer; add an adversarial primitive/derived mutation |
| Current daily comparison used the superseded shadow scalar | `accepted` | Reconstruct `legacy_sequential_complete_j_m2` from independent external terms plus active conduction and close it to the v6 complete-arm/hourly publication |
| Retained v4/v6 endpoint replay was absent | `accepted` | Stream and hash both immutable anchors; prove fresh E00 and E11 daily/WY/median replay before versioned-estimand classes |
| Classification omitted exact forcing hashes and failure classes | `accepted` | Parameterize success/failure classes by exact forcing SHA and retain frozen precedence |
| Verifier trusted receipt-provided inventory | `accepted` | Independently require all cells/modes, builds, clones, Cargo manifests/targets, fixtures, arms, runtime manifests, sidecars, traces, protected outputs, semantic inputs, current HEAD, and allowed artifact roots |
| Conditional checkpoint obligation had no executable path | `accepted` | Add independent trigger receipt, exact 14-checkpoint build/run command, per-checkpoint controls, streaming reconstruction, first-transition localization, and retained verification |
| Build-affecting ambient environment was incompletely scrubbed | `accepted` | Scrub Rust/Cargo/C/linker overrides, select and record an explicit host target and linker, and recheck clones after build, run, and verification |
| Legacy/explicit semantic equivalence was result-only | `accepted` | Normalize both selector spellings to the same effective operator and prove every other semantic input equal before execution |
| Frozen `2be275fa...` digest had 63 characters | `accepted` | Corrected result-blind to the independently recomputed 64-character digest `294c6ece758c7f4496b46b3eb059b25ec23cb5aa197ebc63bad02da9973d5c32` |
| V4 applicability context was incorrectly required true | `accepted` | Retain the typed Boolean as context while allowing legitimate false/no-snow rows; zero remains a numeric aggregate rather than proof of support |
| Environment scrub remained denylist-shaped | `accepted` | Replace it with a minimal runtime/build allowlist, deterministic locale, explicit Cargo home/target/host/linker, and sanitized Git/LFS execution |

No result execution is admitted until fresh Agent A and Agent B review the
amended exact clean commit and both return `PASS`.
