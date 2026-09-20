# Grid40 continuation correctness result review

Reviewer: `/root/grid40_correctness` (independent replacement correctness and
scientific-result scope; no implementation, authority, adviser, or experiment
author role).

## Findings

No blocking correctness, science-contract, numerical-integrity, or
result-evidence finding remains for this fixed pair.

**Scientific result, not an implementation defect:** the treatment is a valid
negative result for the frozen b0..40 policy. It reaches a typed
`Rejected BacktrackingLimit` at iteration 14 and backtracking count 297. It is
not an accepted endpoint, proof of root nonexistence, or a production-policy
result.

**Nonblocking diagnostic limitation:** because every iteration-14 strict trial
is domain-invalid, no rejected-trial step norm exists. The terminal step norms
are bit-identical to the prior accepted iteration-13/b36 step. The sidecar's
disjunctive provenance text is truthful, but final reporting must not describe
those very small values as the rejected iteration-14 direction. Likewise, the
inherited fixed residual labels are not branch-aware; row 22 is a represented-
snow temperature identity in kelvin, not ground energy closure.

## Evidence class and result identities

**Static:** I reviewed the unchanged final T source authority and prior
correctness GO, final common pair freeze, baseline admission, treatment trace,
sidecar, receipt and row-dimension map, the independent QA baseline and result
reviews, the offline cost reconciliation, and the final preservation record. I
did not change source, numerical authority, configuration, the evaluator, or a
measured result.

**Ran:** I independently compared the baseline's 95 ordinary records to the
frozen trace with typed and network-order binary64 equality. For treatment I
ran only `continuation-correctness-reconstruct.py` over retained JSON evidence.
It performs arithmetic, branch/row mapping, domain-distance, linear-system and
primitive-balance reconstruction. It never loads or calls Rust, a solver, an
evaluator, or a surrogate evaluator. The script hashes to
`382282edc32fb56b0fe5144b69845b5d128ba56b774a816459538c9d64f97612`;
its result `continuation-correctness-reconstruction.json` hashes to
`4455f13dfa853796b70419e1b67436999c7d5399881ffe902aab400e68bae7ec`.
One batched read-only inspection subcommand failed before execution because I
mistyped its working directory as `/workWEPP`; the other two independent reads
in that batch succeeded, and the command was rerun correctly. The first patch
attempt for this accounting note then missed its exact text context and made no
change; the corrected patch succeeded. There were no reconstruction assertion
failures or expected-negative child commands. These add two mixed tooling
outcomes to the package floor.

Both arms bind unchanged freeze
`a13048e5df6c41e034746c2d4d52b9468ab04a1aa03a6ce796491eceeaa2f901`,
T `fae897fe90f6bbadcaf298607c9d23fd986cd778f9dd56195f7a5510cbd5c4be`,
binary `e26cc780e7065211714a37653e55f9038cee3a5a32a6964de7122cd80cf0f272`,
and input `527208c3e5ad07f1744f1bd92c9e4ac17cf7efd9469968a97d7964c5db482dfd`.
The baseline reproduces all 95 ordinary records exactly, including signed-zero
bits and order, and retains the expected iteration-6 E034/backtracking-65
refusal. This establishes matched baseline correspondence; it is not baseline
convergence.

The treatment receipt has exit 0, no stop or integrity error, and 4.115309141
seconds elapsed. Independent QA reconciled closed event lifecycles with zero
error, denial, budget stop or in-flight operation. Actual costs are 15 bases,
15 Jacobian assemblies, 14 updates, 863 signed probes, 29 ordinary evaluations,
682 complete evaluations and 3185 domain-predicate entries, all below the
frozen treatment ceilings. The accepted strict exponents are
`[1,2,5,5,12,20,21,22,23,25,26,29,30,36]`; their sum plus terminal exhaustion
contribution 40 reconstructs backtracking count 297.

## Branch-aware endpoint and first obstruction

All 15 accepted-base frames retain two occupancies with gas branches
`[Inactive, ExactZeroPar]`, zero sun area, positive shade area,
`ConstitutiveLaw` wet/root branches, and the represented-snow lower boundary.
Across all 435 base residual rows, captured `raw / applied_normalizer` reproduces
the captured normalized value bit-for-bit using each frame's own normalizer.
The original branch-aware 29-row map therefore remains applicable throughout
this treatment trajectory.

At iteration 14, the upper occupancy's active shade-leaf temperature is
`273.15000000000674 K`, only `6.764366844436154e-12 K` above the liquid-vapor
domain floor. Its Newton direction is `-11.26490267669643 K`, so a lawful
positive factor must be no greater than `6.004816054407216e-13`. Every captured
b0..40 trial coordinate is bit-exact to `x + 2^-b * delta` and is domain-invalid.
At b40, factor `9.094947017729282e-13` gives
`273.1499999999965 K`, or `3.467448550509289e-12 K` below the floor; coordinate
7 is the sole b40 violation. Thus the fixed search grid ends before a lawful
factor and correctly returns its typed backtracking refusal.

Offline arithmetic projects b41 above the bound, but b41 was outside this fixed
policy and was never passed to the canonical Rust domain-admission function or
evaluator. The reported projection uses only the reconstruction script's
offline bound arithmetic. No residual, descent, acceptance, convergence, or
follow-on behavior at b41 is known or claimed, and no additional evaluator call
is authorized.

The endpoint's largest normalized residual remains row 22:

`(275.65248803811716 K - 265.15 K) / 1e-9 K = 10502488038.117178`.

This is the represented-snow ground-temperature identity and the arithmetic is
bit-exact to the trace. The complete residual vector therefore fails acceptance
by many orders of magnitude; the terminal shade-energy row also remains
`-31.61197642012256 W m^-2 tile` while the active shade temperature is nearly
at its lower bound. Tiny prior-step diagnostics cannot convert this refusal
into scientific success.

The final captured scaled linear system is numerically solved accurately:
reconstructed matrix infinity norm `20000000012.683525`, maximum equation
residual `2.980232238769531e-7`, relative-RHS residual
`2.83764402106742e-17`, and normwise backward error
`8.458074372187226e-19`. These operands do not support a linear-solve failure as
the proximate obstruction.

## Independent primitive reconstruction

Using the captured final-base coordinates and primitive inputs, I independently
reconstructed shade stomatal conductance/vapor transfer, q1, q2, both active
root-layer laws, root sums, water scales and all six hydraulic rows for each
occupancy. All 12 reconstructed hydraulic rows are bit-exact to the trace.

For the upper occupancy, shade vapor is
`1.722635955884222e-9`, q1 shade is `0.0012146879663786734`, q2 is
`1.1656834595993929e-9`, and root-source sum is
`-3.8261256083177646e-8 kg m^-2 tile s^-1`; its water normalizer is
`2.2146879663786735e-12` in the same units. For the lower occupancy the
corresponding values are `1.7329216997416018e-9`,
`0.0012494661652394123`, `1.1544450420070503e-9`,
`-3.928977341625396e-8`, and `2.2494661652394125e-12`.
Both active root-layer laws are negative, so the uncapped Potential request and
finalized-use amounts are exactly zero after the canonical positive-part and
tile-fraction/time conversion. No unit, area, hydraulic-source, or negative-use
defect explains the refusal. These are failed-iterate diagnostic rates and
requests, not accepted fluxes or global conservation evidence.

## Residual risk and missing tests

- No accepted treatment candidate exists. Root existence, convergence under a
  different grid or method, and behavior at b41 remain unproved.
- The treatment assesses this one original-start case only. It does not qualify
  broader derivatives, production globalization, reader/E008/RQ1/A-001,
  physical interval advancement, restart, conservation, or cadence release.
- T's source bytes were unchanged across both arms, but writable modes during
  baseline and the later 0444 hardening remain a documented custody limitation.
- Strict warnings-denied Clippy remains FAIL on matched inherited debt. The
  experiment does not turn that into a lint PASS.

## Verdict

**APPROVE the pair as a complete, valid fixed-policy negative experiment.** The
matched baseline and treatment evidence satisfy the frozen source/input/cost
contract, and the first treatment obstruction is independently established as
the active shade-temperature domain floor lying just beyond the b40 factor.
The scientific disposition is **Rejected BacktrackingLimit**, not Accepted.
Both original arm allowances are consumed; no retry, b41 probe, alternate
input, new method, reader run, or downstream continuation follows from this
verdict.
