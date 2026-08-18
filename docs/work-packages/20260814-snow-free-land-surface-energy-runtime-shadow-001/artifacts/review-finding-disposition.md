# Review Finding Disposition

Status: `executing / all material findings accepted`

Evidence class: `Static + Ran`

The independent science review inspected exact commit `dfc7cf971` and returned
HOLD. Its six findings are accepted as Child-3 implementation defects. The
passing 33-test LSE and 73-test integration suites are retained but do not
override the endpoint review.

| Finding | Disposition | Correction route |
|---|---|---|
| `B-CRITICAL-001` final E04 release not used as ingress | Accepted | Derive typed hydrology ingress only from the accepted fixed-cap E04 ledgers; prohibit caller replacement and poison wrong/missing/duplicate/potential releases. |
| `B-CRITICAL-002` unbound physical inputs and absent public E01--E03 | Accepted | Project strict V8/LSE configuration, state, forcing, topology, hydrology and soil-thermal snapshots into the solver and execute whole-column radiation inside that projection. |
| `B-CRITICAL-003` caller placeholder companion tiles | Accepted | Solve every configured open and covered tile from one immutable beginning snapshot, authorize once, and rebuild all tiles under fixed caps. |
| `B-HIGH-004` ground-only energy reconstruction | Accepted | Expose primitive component, shared-air, reciprocal longwave and directional shortwave operands and validate every local tile before one OFE weighting. |
| `B-HIGH-005` synthetic rollback hashes | Accepted | Hash deterministic actual owner/envelope bytes before and after each injected failure using real owner identities. |
| `B-HIGH-006` covered oracle not consumed by Rust | Accepted | Load the committed digest-bound covered fixture in ordinary Rust tests and compare potential, capped, warm-start and failure outputs under the frozen rules. |

The Rust review also identified two accepted protocol defects already under
correction: component/occupancy bindings must carry exact configured vertical
rank, and both mineral-N authorizations and finalized uses must be restored to
the immutable request order before positional BGC protocol validation.

Its two additional high findings are also accepted. Every Newton step must
obey the canonical strict-decrease backtracking rule, including when the
current normalized residual is already below one. Singular, backtracking-limit
and iteration-limit failures must remain typed and carry the complete rejected
diagnostic and actual rollback lineage through the public runtime. The stale
write-set, checkpoint and exact line-count evidence is accepted governance
debt and will be reconciled against the terminal diff.

No finding is deferred or rejected. No new authority package or model identity
is required. Performance, broad heavy gates and terminal verification remain
withheld until focused remediation gates and fresh reviews pass.

## Terminal Update — 2026-08-18

All original material findings were accepted and corrected. Fresh production
Rust and science reviews returned PASS; summaries are preserved in
`fresh-review-rust-final.md` and `fresh-review-science-final.md`.

Heavy execution exposed a new authority finding: immutable V3/V5 fixtures do
not regenerate from checked-in calculators. Disposition:
`accepted / unresolved / HOLD`. Rewriting hashes would cascade through V3--V8
authority identities and is not authorized in Child 3. Full workspace Nextest
remains FAIL; no finding is deferred or rejected.
