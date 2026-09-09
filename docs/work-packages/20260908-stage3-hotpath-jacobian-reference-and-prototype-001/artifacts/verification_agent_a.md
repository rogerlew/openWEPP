Static: authority, requirement legitimacy, changed bindings, selected behavioral
tests and current terminal claims inspected. Ran: independent frozen-release
behavioral verification, identity hashes and arithmetic/data checks below.

Role/session: `/root/terminal_verifier_a`, package_verifier A. Configured effort:
medium (role setting); effective runtime effort: UNOBSERVED. Writes limited to
this artifact. No source, configuration, measurement-series or external changes.

## Reviewed identity and scope

Measured J cut3 source identity:
`7b5133c93de4c827d54945e265b94235834bffef4eb21ec62942b3c377930d57`.
Formatting-only terminal cut4:
`0c582dd73d12434faf2d0394f320c5247633f1966538b347844dbe78bb9df70a`.
Independently hashed terminal manifest:
`17fbc3a85c9c53088734975f19999890e61d9cf52ba7cebe280e512f46f04764`.
Independently hashed canonical inactive-jacobian.md:
`e8917d559b9f33d3939c4db4a63567017961751a05984e83d2b4cc83ebf7f6d6`.

Read root/common instructions, assigned package/handoff, role-verification,
assurance template, testing strategy sections 7, 9, 10 and 18, correctness
authority model, science instructions/obligations, owner authorization,
v34 amendment, changed entry/bindings, selected J tests and the current reading,
preimplementation, review/finding, gate, treatment, custody and disposition
records. This is an independent terminal requirement/claim verification, not a
replacement full-source physics review. The unchanged full dependency proof
and broader owner readings retain the separately authored source reviews.

## Independently executed checks

From `/tmp/openwepp-hotpath-jacobian-jmvOZh/J`, invoked the measured release
libtest binary directly, without rebuilding or starting a performance series:

```sh
RUST_MIN_STACK=67108864 OPENWEPP_INACTIVE_JACOBIAN_CORPUS=/tmp/openwepp-hotpath-jacobian-jmvOZh/authentic-corpus-v2-cut2.json /workdir/.cache/openwepp/targets/J-fd4da896059c/release/deps/openwepp_land_surface_energy-29dc6221b95c33c1 --include-ignored --test-threads=1 --skip inactive_jacobian_local_cost inactive_jacobian inactive_reference solver_residual_corpus_capture::tests solver_accepted_operand_audit::tests
```

Exit 0: **23 passed, 0 failed, 0 ignored, 149 filtered**, 0.07 s. Own output is
retained in this session tool record; no separate raw log was written. This
executes the authentic independent affine reference, all-three-family column
and capability tests, real GenericV3 nonunit mapping, source-real exclusions,
atomic matrix/scale failures, cost-integrity mutant, accepted-operand overflow
and ownership checks, and corrected capture lifecycle/wire/count negatives.
The test source explicitly requires actual FD retention for exclusions and
ordinary base errors/complete matrix-RHS comparison for capability admission;
these are not merely predicate-name or artifact-presence checks.

`sha256sum` independently confirmed the executable is
`69b87a51a6c87ca49fe09a81ebe901cfddc38b635a467928f66c6bcad9a1a093`,
and the supplied cut2 corpus is
`000e59189d62fdc20a89fcefb80ecf62fdb060b7b3e8c8afa99a37bd92c932ec`.
Direct execution checks behavior of those bytes in this environment; it does
not claim a new Nix rebuild, performance replication or full-workspace gate.

Read-only `.venv/bin/python -c` JSON/arithmetic checks independently counted
six valid exit-zero teardown child rows and 60 run records. Pair0 raw active
maxima are A 83788 / J 88276 KiB. Their difference 4488 exceeds
max(4096, 0.05*83788)=4189.4 by 298.6 KiB. Recomputed the median of the 12
one-OFE paired `(A-B)/A` values: 0.004229221419309608, consistent with the
reported 0.422922%. An initial exact-float equality assertion failed because
the recorded equivalent `1-B/A` evaluation differs by 1.56125e-17; the bounded
arithmetic recheck passed (2e-16 absolute allowance, not a scientific threshold
change). No timing observations or analysis file were modified.

`git diff --stat`, `git status --short` and selected authority/test diffs confirm
the primary tracked changes are authority/bindings/package evidence, not an
installed production-crate treatment. This verifier did not independently
rehash every archive member or reproduce the 16-file patch; verifier B owns
those recovery/exact-diff checks.

## Requirement legitimacy, accepted fixes and limits

CAP-01/02 and scale/column CAP-03 obligations have fresh behavioral evidence
above, not recycled cut1 tests. CLOS finite-overflow/ownership and COST checksum
integrity corrections likewise execute; source-real exclusion evidence closes
the predicate-only weakness. Actual-runner observation/audit/consumer evidence
is reused from the exact measured-cut records and independent source reviews,
not claimed as independently rerun here.

The owner-approved inactive target, canonical complete-column/scaled assembly,
positive-zero support and retained invalid-base guards match the claims. There
is no active-foliage, full soil-interior/WB14 ingress-partition, production
activation or broad scale-performance claim. READ-A01 is disclosed as a late
parent reading correction, independently re-reviewed CLOSED; it is not rewritten
as preimplementation compliance. TERM-A01 and QA-DOC-01 are independently closed
in the final reviewer supplements; current claims distinguish actual cut3
execution from explicitly labeled historical pending records.

Non-deferral explicitly checked: broad required gates retain FAIL, including
the timed-out retry and unattributed failures. They prevent `complete`; neither
focused passes nor inherited examples subtract those failures. Engineering
REJECTED_IMPLEMENTATION is supported by an implemented and measured candidate
failing the frozen memory criterion. Authorization section 11 directs stopping
tuning once uncompetitive; the named alternative/reference ladder is not a
mandatory new experiment after this actual implementation-level cost rejection.
Competitive-case 10/19-OFE timing is consequently NOT QUALIFIED/untriggered,
not a retroactively deferred passing gate. Both 10-OFE admissions alone are
not scale performance. No newly justified persistent-growth or lifetime-bounded
memory conclusion is inferred from ten runs.

The next reduced-system assembly is expressly a separately authorized action
requiring new numerical authority and a decisive full-solve reconstruction
test, not a current implementation or automatic HOLD lift. Calibration is
not applicable to unchanged process parameters. No external-authority suite
posture/cohort binding mutation was found that would trigger AUTH11 evasion
checks for this diff. No scientific gate is weakened by this disposition.

Findings: no new blocker to the bounded rejected/executed-HOLD disposition.
Final dual-review/dual-verifier bookkeeping and scoped commit are separate
terminal obligations; this artifact cannot supply the other independent roles.

Verdict: **PASS for verifier A's bounded requirement/claim verification of
REJECTED_IMPLEMENTATION with executed-HOLD and production HOLD.** Not package
`complete`, full-green correctness, production qualification or a promotion.
