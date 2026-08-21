# Review Finding Disposition

Status: `FINAL / DUAL REVIEWS AND DUAL VERIFICATIONS COMPLETE; HOLD`

Evidence class: `Static + Ran`. Four fresh read-only agents completed against
the typed-owner follow-on: two independent reviews and two independent
verifications. No agent edited the worktree.

| Independent gate | Result | Material finding |
| --- | --- | --- |
| Nash — Rust code review | `HOLD` | The typed endpoint is opt-in only; the ordinary runner is unchanged. The endpoint commits a typed candidate into `committed_shadow`, not live production owners; terminal-liquid custody, carrier geometry/ledger derivation, restart/replay identity, event joins, and failure scenarios remain open. |
| Carver — Rust QA review | `HOLD` | Focused validation passes, but ordinary consumer binding, shadow-backed custody, restart/rollback coverage, mandatory scenarios, and line-count governance remain blockers. Workspace all-target Clippy and frost are not clean on unrelated baseline/fixture failures. |
| Rawls — independent verification | `HOLD` | Historical pre-hardening verification: Nix fmt/check, package 4/4, typed endpoint 1/1, V11/Child 2C 2/2, restart 26/26, and targeted library Clippy pass. The normal runner does not construct `DirectV11SnowStage3OwnerExecutor`; no production-path closure claim is valid. |
| Hilbert — independent verification | `HOLD` | Nix focused gates and targeted Clippy pass; the typed endpoint is real only through the explicit seam/fixture. Restart-before/at/after, publication rollback, and mandatory failure evidence are absent; critical profile remains incomplete. |

The following findings are accepted only as bounded implementation evidence:

| Finding | Disposition | Evidence/owner |
| --- | --- | --- |
| Shared-air equations, sealed exposure identity, reciprocal-LW and poison rejection | `accepted for increment; not authority closure` | `evaluate_shared_carrier` and focused authority tests / Child 2C implementation owner. |
| Canonical tick support/tolerance selection and `ERR-CT-021` | `accepted for increment; not replay closure` | `locate_terminal_event` and focused test / coupled-time owner. |
| Stage/commit cloning, typed owner receipt, and delayed row release | `accepted as opt-in seam` | `DirectFrameExecutor::run_publication_stream_with_snow_stage3_terminal_handoff_and_owner_executor` / orchestrator owner. |
| Concrete V11/LSE/BGC/soil-thermal execution | `accepted as typed opt-in endpoint; production HOLD` | `DirectV11SnowStage3OwnerExecutor` invokes `DirectV11RealConsumerStack`, and the endpoint test reaches commit / orchestrator owner. |
| Ordinary hillslope runner consumption | `blocker` | Runner still calls the ordinary interleaved stream and constructs no typed V11 owner bundle / requires separately authorized receiver integration. |
| Live owner custody and terminal liquid transfer | `blocker` | Current executor retains a shadow candidate and does not join `terminal_liquid_kg_m2` into the real receiving surface-liquid owner / owner-transaction authority. |
| Carrier geometry, independent ledgers, event/support identity, and replay schema | `blocker` | Current inputs validate caller-supplied conductances/ledgers and omit complete parent/segment/ordinal identities / science-contract owner. |
| Restart, failure injection, publication rollback, and mandatory scenarios | `blocker` | Existing tests cover focused stage/commit and tamper rejection only; full before/at/after and late-failure closure is absent / package owner. |
| Line-count governance | `blocker` | `v9_real_consumer_shadow.rs` is 3,151 lines and its tests are 2,032; the current artifact is reconciled and a refactor package is required. |
| Authority and critical profile | `blocker` | Final Nix frost run is 390/391: one unchanged v13 contract marker; combined authority selection remains 17/18 with an unchanged registry-string guard. |

The dual review and dual verification gates are therefore satisfied as review
activities, but their unanimous `HOLD` findings prevent `COMPLETE`. The normal
selector/default, CoE production owner, and publication authority remain
unchanged.

### Post-hardening review disposition — 2026-08-21

Two additional independent read-only reviews were run after the identity and
restart hardening. Both remained `HOLD` and confirmed that the remaining
ordinary-runner, live-custody, terminal-liquid, durable-publication, and full
restart/rollback gates are genuine blockers. The primary review also identified
that the prior scheduler callback releases rows one at a time and cannot prove
external publication atomicity; this remains explicitly unclosed. The QA review
confirmed the current package target has six tests and the touched-crate
library Clippy gate passes.

The final delta after those reviews adds only fail-closed guards and their
supporting evidence: exact 5 m / 0.005 m exposure geometry, full carrier/event
support-receipt equality, candidate-set identity over event inputs and
tolerances, computed reciprocal-longwave matching, predecessor receipt-chain
continuity, cursor/ending-owner semantic binding, repeated segment rejection,
and empty-owner-payload rejection. These changes preserve the final `HOLD`
disposition; they do not promote shadow state or claim production closure.

The final delta review pair also returned `HOLD`: Carson identified the still
unsealed wind/conductance and caller-ledger authority, incomplete full-run
restart/publication semantics, the 600 ms support and remaining carrier/event
identity limits, and duplicated scheduler transaction logic; Franklin confirmed
the ordinary-runner/live-custody/line-count blockers, reconciled the current
six-test package run, and recorded the `cargo deny check` evidence gap. The
current artifact counts are authoritative after the final guard tests:
1,226 handoff lines, 2,032 shadow-test lines, and 431 integration-test lines.
After that review, the final source delta also rejects nonzero remainders below
the 600 ms LSE support minimum and full canopy cover; the focused package run
remains 6/6. The final frost run is 390/391, with only the unchanged v13
contract marker failure.
