# Independent component-temperature dependency-replay verification B

Status: `PASS — POST-REVERT STATE VERIFIED`

Evidence mode: `Static + Ran + Expected-red`

Reviewer: `rust_qa_reviewer`

Current post-revert Rust manifest:
`2813f6e8faabb9408bac5e59b9271626ff5bcdc7fe49ab6dda810d3a1c3eee0d`

This terminal verification supersedes the earlier authority-readiness
verification against `767bc190...1583`. The implementation and release
recommendation is `HOLD`, not `GO`: the reviewed v31 candidate was correctly
rejected and fully reverted after its frozen release conjunct failed.

## Findings

No blocking or non-blocking finding.

## Exact release failure

The raw candidate log independently hashes to
`00ba46b4dfafd63958a187015bb1e065c5f5d928db80f8ed5b0f97c2a1e28e4e`.
It records one unchanged candidate source identity,
`039a312502a5e6ef442b1e81ac78b988141199f6283fedcc86518ba78ba61abc`,
before and after run 1. The cached candidate binary independently hashes to
`f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`.

Run 1 exited `101` before emitting a `STAGE3_LANED_RELEASE_PROBE` JSON record.
The candidate-only release assertion found no authentic completed `N=2,S=6`
sweep with the required `58/14/16/28` logical/anchor/replay/complete
accounting. The log explicitly records
`COMPONENT_DEPENDENCY_CANDIDATE_RUNS_2_AND_3 NOT_RUN_AFTER_FAILED_CONJUNCT`.
That is the required behavior under the prospectively frozen conjunctive
protocol; the failed run is not relabeled and the two absent runs are not
reported as passing evidence.

The retained baseline log independently hashes to
`52292cb7c6ddcb7cae087cbd5ba8dbe3e8bb5265abc6d42446ece95f916ffe04`.
Its raw total/potential tuples reconstruct the documented medians
`4,903,570/353,431 us` and exact subtractive ceilings
`4,803,570/253,431 us`. No keep/revert term was changed after candidate
execution.

## Candidate review closure and rejection

Both final implementation reviews approve the exact 16-file candidate
manifest
`edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`.
Their final sections close all eleven accepted graph, direct-edge,
connectivity, custody, lifetime, source-error, rollback, audit, parity, and
performance-readiness findings. The recorded candidate gates include focused
`14/14`, full LSE `154/154`, authentic parity `1/1`, feature diagnostics,
checks, formatting, and diff hygiene.

Those closures apply to the reviewed candidate bytes. They do not turn the
failed release conjunct into a pass and do not claim that v31 connectivity is
present after reversion. The disposition correctly records
`FAIL_REVERTED` for release while retaining the accepted findings as closed for
the rejected candidate.

## Full revert and structural expected red

Static production search finds none of the seven revision-31 symbols:

- `CoveredComponentTemperatureDependencyGraph`;
- `ValidatedCoveredComponentReplaySweepBase`;
- `ValidatedCoveredComponentProbeReplay`;
- `CoveredComponentDependencyReplayAudit`;
- `covered_component_temperature_probe_residuals`;
- `begin_covered_component_dependency_replay_audit`; and
- `take_covered_component_dependency_replay_audit`.

The three extracted LSE test-support files and three runner qualification files
introduced by the candidate are absent. A broader source search found no
component-dependency replay production residue. The cached failed binary is
forensic evidence, not retained source or active production connectivity.

Ran: the exact structural seam test fails only because all seven required
declarations are absent. This is the intended post-revert `EXPECTED_RED`; it is
not implementation evidence. The complete LSE nextest suite passes `140/140`
without the candidate. `cargo fmt --all -- --check` and `git diff --check`
pass. No heavy release command was rerun during this bounded verification.

## Current-manifest and frozen-baseline reconciliation

The independently rebuilt sorted stream of every existing changed/untracked
Rust path contains 179 entries and hashes to the current manifest
`2813f6e8faabb9408bac5e59b9271626ff5bcdc7fe49ab6dda810d3a1c3eee0d`.
It is byte-identical to the saved terminal verifier stream.

The current authority test
`tests/integration/land_surface_energy_balance_authority_contract.rs` hashes to
`9250d0dd7e5335cb866bc7d4057fcc291e2746436d715bdaddb2c205a04a2451`.
The independently reconstructed pre-v31 form hashes to
`912bb3deae3708f681a82417a631ebf6dcb7079e84ab64542ebbba00e8772096`.
Substituting only that reconstructed line in the 179-entry manifest stream
reproduces the frozen aggregate exactly:
`78d756be1fa11ed85ee92b7d19e6c04427b01b122efaf7804d1b55d60536bbbe`.

This proves the documented 23 authorized post-baseline authority-test patches
explain the aggregate difference. It does not make a false byte-exact
whole-workspace restoration claim. Restored production hosts match their
archived pre-v31 identities and no semantic v31 mechanism remains.

## Disposition and HOLD legitimacy

The component readiness matrix, disposition, package gate results, final
disposition, and hold-legitimacy audit agree on these terminal facts:

- revision 61 passed timing but failed the per-run RSS conjunct in two of
  three runs and was fully reverted;
- revision 31 passed candidate correctness/review but failed its authentic
  release-audit conjunct on run 1 and was fully reverted;
- earlier bounded validation-once, custody, cache, replay, topology, terminal
  package, and physical-evidence routes were retained or rejected under their
  prospective gates; and
- exact-workspace correctness, broad warnings-denied Clippy, and long-run
  release/performance qualification remain failed or unestablished.

The authentic `N=2,S=6` condition cannot now be manufactured, weakened, or
silently replaced. Further work requires owner authority either to revise the
representative workload/budgets and define a new protocol, or to authorize a
materially different canonical solver/evaluation architecture with new
science-contract obligations. After the continued revision-61 and v31 cycles,
no credible bounded route remains inside the present authority.

## Non-blocking debt / follow-up

None inside this verification scope. The first HOLD-lift action is the owner
choice recorded in `artifacts/hold-legitimacy-audit.md`; it is not a verifier
or implementation cleanup task.

QA pass statement: the v31 failure, conjunctive early stop, complete revert,
expected-red seam, manifest reconciliation, and candidate-finding disposition
are truthful and internally consistent. Terminal `HOLD` is legitimate. This
is not implementation, release, performance, or package-completion approval.
