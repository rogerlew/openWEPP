# Independent terminal review A

Static: primary-checkout v33 authority/package cut and detached Phase-A source
at `/tmp/openwepp-residual-jacobian-FC2T1v/J`; Ran: focused oracle selector and
positive-stem transaction/oracle tests, corpus/manifest digest checks, and
evidence-bundle verification. Role/session: terminal reviewer A; configured /
requested effort: primary correctness review; effective runtime setting:
UNOBSERVED. Reviewed identity: scaffold commit
`827a7470e058a5e09f9feced9375b776e5959789` plus the current primary diff;
A-source-05 identity `e9899b0e...`; detached oracle blobs
`3ae38b0830472d404b4378c7fea82a08172182f4` and
`8acff12d8e571914c6ced1f69973c0d8e99d3ad6`; active corpus
`a05ca09031c28f863bc8dd8f3cc8d605459a683abc2af6e0f37dd84c713068c0`;
custody manifest
`488eaff372f48802efc1ccf59a7deb8d5fdd27aacf25d06900f5896e0c8293ad`.

## Findings

### TRA-001 — MEDIUM — terminal corpus and expected-red records contradict the corrected cut

Location: `artifacts/authentic-corpus.md`, `artifacts/expected-red.md`, and
`artifacts/preimplementation-contract-gate.md`.

Evidence: `authentic-corpus.md` documents only the superseded zero-stem corpus
and ends by saying a positive-stem capture is still required, while
`oracle-results.md`, `worker-handoff.md`, and the external fixture establish the
later positive-stem corpus. The preimplementation gate likewise still calls the
invalid `b8f6a923...` corpus a PASS and leaves corrected review pending.
`expected-red.md` says an executable expected-red assertion remains and must be
turned green rather than deleted, but the captured/current
`solver_stem_jacobian_tests.rs` contains no such test. These are false current
state statements in required closure artifacts, not merely historical prose.

Required disposition: update the corpus artifact to include the active corpus
and clearly retain the first capture only as quarantined history; reconcile the
preimplementation gate with the 0/16 Phase-A stop; and state exactly that only
the retained E0425 record exists because J was never implemented. Do not claim a
surviving behavior test.

### TRA-002 — MEDIUM — Rust line-count and warnings evidence is incorrectly marked not applicable

Location: `artifacts/line-count-governance.md`, `artifacts/gate-results.md`, and
detached `solver_covered_evaluation.rs` / `solver_litter_phase.rs`.

Evidence: Phase A changed and retained Rust source in the detached tree.
`solver_covered_evaluation.rs` is 2,685 lines and therefore requires the
repository's WARN rationale and follow-on split intent, but the line-count
artifact says no Rust source changed. Both focused review reruns emitted two
`unused_mut` warnings introduced by the observation hook in
`solver_litter_phase.rs:245,249`; no warnings-denied result is recorded. The
package may preserve this research source without implementing J, but cannot
close by calling its applicable source-governance checks N/A.

Required disposition: inventory the retained detached `.rs` files, disposition
the 2,685-line WARN file with rationale/split intent, and either remove the two
new warnings and run the focused warnings gate or record that gate honestly as
an unresolved package-closure HOLD.

### TRA-003 — MEDIUM — Phase-A custody does not bind the exact v33 protocol bytes or a retained run log

Location: `artifacts/source-and-build-manifest.json` and
`/tmp/openwepp-rj-phase-a-terminal-bundle/manifest.json`.

Evidence: the bundle correctly preserves the active corpus and the exact oracle
/capture source bytes, and its manifest verifies. Its protocol member is only a
one-line pointer to mutable primary-checkout v33/package documents; neither
those authority bytes nor their digests are in the compact source/build
manifest. Its result member records only `0/16`, not the command output or the
per-column/row basin decisions. The implementation test also prints whatever
support count it observes but asserts only `candidates > 0`, so the retained
test result does not itself pin `supported == 0`. My rerun independently
reproduced `0/16`, which supports the present review, but the declared custody
is weaker than the package's exact experiment/evidence identity rule.

Required disposition: bind hashes for the exact v33 authority/protocol files
used by Phase A and retain a compact exact command/result log (or a deterministic
machine-readable per-column support report) in a verified bundle. The absence
of a J executable is correctly and explicitly recorded; no candidate binary
should be invented.

## Correctness assessment and residual risk

The mathematical target, coordinate order, quotient-rule normalization,
cross-occupancy longwave map, own-stem/shared-heat/ordinary-ground rows,
same-layer exact zeros, represented-snow identities, and pre-entry structural
fallback/no-post-entry-recovery boundary are coherent with the inspected
canonical primal source. The corrected shared-heat pre-floor `y` rule,
nonfinite validation precedence, and deterministic N=1 directions resolve the
prior authority defects.

The active corpus is authentic for the ordinary covered potential/final
transaction: eight exact-bit records, four per pass, N=2/S=2/D=25, two positive
executed dry-stem areas in every record, distinct real solver lifecycle joins,
and exact base replay. The focused transaction/oracle rerun passed and printed
`0/16`; the selector self-test passed. The bundle and corpus hashes also match
the package. The oracle's smooth-mask helper does not explicitly test the
shared-heat pre-floor `y<1/y>1` class, but because every column already fails a
mandatory affected-row basin, that omission cannot turn any of these 16 columns
into an admitted column. It would be blocking before any future J comparison.

No analytic derivative, hybrid assembly, J full solve, consumer comparison,
closure, local-cost, end-to-end timing, memory, or multi-OFE evidence exists.
The package consistently labels those scientific/performance gates NOT RUN;
that is honest and is not a failed J result.

Stopping before J is correct under the kickoff. The frozen method requires a
complete affected-column reference, and each authentic stem column lacks at
least one resolvable mandatory entry. Implementing J, retuning v33 after that
observation, expanding to leaf temperatures, or timing a zero-participation
hybrid would violate the prospective admission rule. `INCONCLUSIVE_WITH_BOUND`
is therefore the correct scientific outcome: it neither accepts nor rejects
the analytic representation, and production remains HOLD. The proposed next
experiment correctly identifies a new authority-first oracle-conditioning /
precision study with an early 12/16 rejection threshold.

## Verdict

**HOLD for package closure on TRA-001 through TRA-003; no scientific blocker to
the bounded `INCONCLUSIVE_WITH_BOUND` / stop-before-J decision itself.** After
the required artifact, line-count/warnings, and custody corrections, request a
focused independent re-review. No production promotion is admissible.
