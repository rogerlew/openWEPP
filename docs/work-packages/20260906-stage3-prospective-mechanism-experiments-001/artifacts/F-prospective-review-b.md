# F prospective runtime/API review B

Later authority correction: `F-role-coverage-review-b.md` explicitly supersedes
FB-2's existential full-physical all-five-role requirement. C056 binds every
real group, not existence of every role in one fixture. The replacement must
retain all-five helper coverage, the primary independent baseline multiset and
the focused complete oracle for every actual invocation. The original review
and failed test remain historical; no replacement runtime pass is claimed here.

Static: independent preactivation QA review, 2026-09-06. No build, test,
comparator or workload was run by this reviewer. First findings were submitted
without consulting review A. Reviewed isolated F base
`2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3`, the typed scaffold and four helper
tests, plus the proposed test-only complete-carrier oracle. Source hashes below
identify the inspected cut; ongoing implementation is not approved as executed.

## Findings and current disposition

| ID / severity | Path at inspected cut | Finding and disposition |
| --- | --- | --- |
| FB-1 / P2, resolved statically | `src/snow_stage3_v11_terminal_feed_forward_tests.rs:34` | The initial candidate loop omitted private soil and validated-owner custody. The corrected oracle calls an owning-module `cfg(test)` comparator that exhaustively binds every candidate field without `..`, compares private soil candidate/continuation and resident/candidate byte maps, and compares every seal variant. Typed equality and canonical bytes remain primary, with derived Debug supplemental. Executable complete-oracle proof is still pending. |
| FB-2 / P2, resolved statically | `src/snow_stage3_v11_adaptive_production_tests.rs:28` | The corrected authentic oracle test now requires Full/Retry/Half1/Half2/Root and both Discovery/Exact modes. The plan explicitly requires additional authentic fixture coverage if this fixture lacks a role; assertions may not be weakened or replaced with synthetic labels. Actual coverage must still pass. |
| FB-3 / Low | `src/snow_stage3_v11_terminal_feed_forward_tests.rs:18` | Derived-Debug comparison is a supplemental bit-distinguishing check only where complete derived field coverage and finite floats are established. Preserve typed equality, canonical owner bytes and explicit scalar `to_bits` checks; a custom/abbreviated Debug or nonfinite float cannot support the claimed full bitwise oracle. No Debug projection may become runtime custody. |

`src/` above is `crates/openwepp-hillslope-orchestrator/src/` in isolated F.
These findings are concrete current-scope oracle work; none is a waiver or a
request to make the unwritten behavior pass its tests before authoring it.

## Interface and test assessment

`CoveredTerminalFeedForwardRequestV1` contains physical/beginning inputs and
structurally lacks ending hint and coupling ordinal. The exhaustive test pattern
has no `..`, so adding either forbidden field fails compilation. The provider
enum selects feedback or feed-forward before evaluation; no error-dependent
mode switch is proposed. Feedback conversion is needed only for the retained
feedback path, observation projection and the explicitly test-only legacy
reference. The real typed carrier entry must receive the feed-forward request.

The concrete helper corpus is appropriately bounded and useful:

- Repeated equal physical requests and the independent exact invocation each
  reach a provider; provider poison returns with no selected trial and unchanged
  persistent/joint inputs.
- Outer lane/interval/support/mode failures precede provider invocation.
- All five role labels exercise the actual common invocation helper, comparing
  one typed call with both results of the two-call feedback path; flux and
  boundary floats use exact bits. Feedback receives sequential ordinals and a
  second-call hint.
- Competing malformed support/role/attempt/beginning-digest poisons outrank a
  downstream NaN at the named boundary-join error, with exactly one refused call.

The fixtures use synthetic seven-owner bytes and constructed boundaries, as the
test plan states. They do not prove actual LSE/vegetation/soil/WB14 result parity,
typed owner rejection, complete rollback or publication/restart correctness.
The separate authentic complete-owner oracle is the proper consumer-level
addition and remains a required executable gate.

The proposed oracle executes two fresh physical carrier references from the
same immutable beginning/child inputs, supplies the first canonical preview as
the second hint, compares the candidate against both results, and checks exact
zero reference coupling delta. It is included under `cfg(test)` and has an
explicit activation session in the same thread as the real fixture. The
measured compact path must never activate it. No scientific arithmetic,
adaptive/event cadence, batch or canonical-final work may be removed by F.

## Required admission work and owners

| Obligation | Owner / evidence required before measurement |
| --- | --- |
| Complete reference | implement_f: execute the corrected FB-1/FB-2 oracle; every physical/custody/receipt/ledger/diagnostic result field, both reference calls, exact-zero preview deltas and actual role/mode/support/attempt records. |
| Guard and failure closure | implement_f + parent: source-real provider failure mapping, every forcing/topology/owner/beginning binding, stale/foreign/reuse/partial-output rejection, transition failure and independent exact/final poisons; typed precedence and byte-identical complete-owner rollback with zero selected state, retained arena entry, installed owner, receipt or publication. |
| Independent evaluations | Parent: actual A/F invocation keys and multiplicity, each removed same-invocation execution joined to unchanged results, independent equal-payload invocations, unchanged batch and Initial/history/FinalAccepted cadence. The old 400/200 observation is context only. |
| Science and consumer | Parent: real runner/control/output parity, protected mass/energy/phase/liquid/receipt/WB14 reconstruction, produced source/outlet/storage/clamp closure and restart evidence. Helper parity is not a substitute. |
| Exact runtime cut | Parent/comparator: focused tests and full applicable critical-cut correctness, format/warnings-denied lint, authority/anti-evasion as applicable, line-count disposition, immutable dual implementation reviews and terminal verifications. |

All runtime obligations above are NOT RUN by this reviewer. The package must
complete them or give the authorized evidence-backed arm disposition; prospective
approval does not defer a current-scope scientific requirement out of the package.

## Reviewed identities

| File | SHA-256 |
| --- | --- |
| retained `F-implementation-plan.md` | `863b759c96e5eebcb4b48401c7624695dbe48fb92e5f23d8d1417a01d068454c` |
| retained `F-required-reading.md` | `a8427df1d6fb46426a374b04bfc429c90ab57cecfb5283b72a3833b60028884e` |
| retained `F-test-plan.md` | `e17b3846250ab9f599329d655e4253d15d463c76d3680e7a8891a66c0dbd8804` |
| isolated `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_feed_forward_tests.rs` | `cd1507d69e13391e70f6beeec5470cc5256300445e13743928eb0476ad498cfb` |
| isolated `hydrology/support_helpers_mod/runoff_reconciliation/terminal_carrier_provider.rs` | `aea2e47f8f134dfe6c9f764b2033334665c7ed0d700130a9811f57f42f285c77` |
| isolated `snow_stage3_v11_terminal_feed_forward_tests.rs` | `4fbd55a350afab3ebb994fc6872cde0f68dc415236352031de52c9053cc7cef3` |
| isolated `snow_stage3_v11_adaptive_production_tests.rs` | `6a1cc7a02caf0d6eb1a889695cac93522f8de5bbaa1d7200e9bac098a0446954` |
| isolated `v11_covered/carrier_phase.rs` | `d7b8d6e0ea8fde3c07815281f93dcc37291f108c6c28055417d6f9a12f2ab105` |

Isolated Rust paths are relative to the orchestrator `src/` directory.

QA decision: GO for prospective F interface/test design and implementation of
the declared behavior. No open prospective design blocker remains at this cut.
Complete-carrier and scientific admission remain pending executable proof.
No measurement, package-completion or production-promotion approval is given.
