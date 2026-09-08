# Focused independent prospective re-review B

Static: re-reviewed the corrected primary-checkout v33 authority cut, original
`prospective-review-b.md`, `prospective-finding-disposition.md`, package/handoff,
updated authority/protocol, source/build manifest, authentic corpus, expected-red
record, preimplementation gate, required-reading map, binding-index/test diff,
and the canonical and detached capture source needed to check the remaining B
findings. Reviewer A's re-review was not read.

Ran:

- `nix develop --offline -c cargo nextest run --test land_surface_energy_balance_authority_contract version_thirty_three_binds_experimental_residual_jacobian_policy`
  from `/workdir/openWEPP`: PASS, 1 passed/26 skipped, exit 0. The build emitted
  one inherited dead-code warning; this focused command was not warnings-denied.
- `sha256sum /tmp/openwepp-rj-corpus-v1-combined.json`: PASS, exact documented
  digest `b8f6a923065e7b396f84831ae6e3435cded05fae324b92d13c05ac646b783fec`.
- Independently decoded the JSON envelope with `.venv/bin/python`: schema
  `openwepp.covered-residual-corpus.v1`; four `first` and four `last` snapshots,
  each split 2 Potential/2 FixedFinal, with distinct lifecycle identities.

Role/session: prospective reviewer B focused re-review; configured/requested
effort: secondary QA/science review; effective runtime setting: UNOBSERVED.
Reviewed identity: dirty corrected authority/package cut relative to scaffold
commit `827a7470e058a5e09f9feced9375b776e5959789`; corpus identity above; detached
worktree `/tmp/openwepp-residual-jacobian-FC2T1v/J` remains dirty and has no
post-A-source capture-patch identity in the package manifest.

## Original B-finding reconciliation

- **B-H1 fixed:** `numerical-methods.md:103-128` now gives the correct
  `i<o/i=o/i>o` recurrence, same-layer sun/shade/wet zeros, own-stem direct
  emission/sensible dependency, ground propagation, and receiving zero-area
  behavior. This matches `physics.rs:386-409`.
- **B-H3 fixed:** `numerical-methods.md:130-181` now binds executed positive
  `dry_stem`, each affected row's residual/normalizer derivative, the
  `max(z,1 W m^-2)` floor, active versus structural zero operands, ordered
  applicability/integrity errors, and no-recovery/no-partial behavior.
- **B-M1 fixed:** `numerical-methods.md:253-274` now enumerates symmetric field
  classes, units/bases, exact handling for unlisted fields, and a mandatory
  tighter canonical threshold plus independent A0 closure.
- **B-M3 fixed and ran:** the binding test now checks stable method tokens,
  exact INV/OBL exposure and definition-locator mappings, lifecycle, and
  non-production semantics; the focused test passed above.
- **B-H2, B-H4 and B-M2 are improved but not fully resolved**, as detailed in
  the remaining findings below.

## Remaining findings

### RR-B-H1 — HIGH — Phase-A has not frozen a baseline oracle-support result before derivative implementation

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:204-251`;
`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authentic-corpus.md:25-28`;
`artifacts/preimplementation-contract-gate.md:8-18`.

Evidence: the corrected authority now defines a deterministic `D_k`/basin /
Richardson/U algorithm and deterministic directional arithmetic, resolving the
original algorithm ambiguity. But no package artifact contains the baseline-A
nine-scale estimates, admitted samples, sign/floor/branch masks, chosen basin,
`J_oracle`, U, or supported/unsupported entry set for the eight authentic
states. The detached tree contains no oracle implementation or oracle self-tests;
only the future requirements exist in prose. Consequently the authority's
requirement that “No basin in baseline A freezes that entry unsupported before J
exists” is not yet evidenced, and the original B-H2 correction requiring a
baseline-only eligibility mask has not happened. Adding the oracle alongside J
would recreate candidate-aware population discretion even if the formulas are
sound.

Required disposition: implement the independent primal-only oracle and its four
specified self-tests without the derivative capability, run it on the frozen
corpus, and retain exact per-entry/per-direction baseline admission and oracle
records before any J implementation. Bind the oracle source/build/result
identity. Only then may a candidate comparison be unable to change the admitted
population. Focused re-review the resulting Phase-A evidence.

### RR-B-H2 — HIGH — The corpus/capture source identity and exact detached write set remain incomplete

Location: `docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/source-and-build-manifest.json:5-49,51-68`;
`artifacts/authentic-corpus.md:3-17`.

Evidence: the manifest authenticates restored A-source-05, then names a dirty
detached path, but it does not hash or bundle the post-A observation patch or the
release capture binary that produced the corpus. The actual new capture owner
`crates/openwepp-land-surface-energy/src/solver_residual_corpus_capture.rs` is
absent from `initial_exact_write_set`, even though it defines the wire codec,
validation, first/last selection and session lifecycle. The write set also lists
`physics.rs` and `solver.rs` twice and generally gives file-level prose rather
than the package-required exact functions. Thus the corpus output hash is
verified, but its generating source/build identity and declared write scope are
not. This leaves B-H4's source/isolation correction incomplete.

Required disposition: capture and verify a recoverable observation-source
series manifest/patch from exact A-source-05, including the capture module,
wiring/hooks, runner entrypoint, toolchain/build target and binary where one
exists. Reconcile a deduplicated exact file/function write set separating
already-landed observation support from future derivative edits. Preserve the
current corpus bytes and link them to that source/build identity; recapture only
if identity reconciliation finds a semantic mismatch.

### RR-B-M1 — MEDIUM — The frozen cost protocol names a nonexistent span and does not define the 75% coverage ratio

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:276-305`.

Evidence: separate monotonic counters and the tenfold complete-residual ratio
resolve the heterogeneous-counter issue. However, the exclusive span inventory
contains `jacobian_v33_stem` and `jacobian_fd_other`, while the formula for `f`
uses `A.jacobian_fd_stem`, which is not a declared span. That formula divides by
`complete_runner`, so it is the Amdahl modifiable fraction, not the separately
required fraction of baseline Jacobian-construction wall time. No exact formula
defines the >=75% broad-coverage gate. B-M2 is therefore only partially fixed.

Required disposition: declare an A-side `jacobian_fd_stem` exclusive subspan (or
an exact derivation from named spans), define both
`f_Amdahl = replaced_A_time / complete_runner_time` and
`coverage = replaced_A_jacobian_time / total_A_jacobian_construction_time`, and
freeze corpus-class weighting for both before candidate timing.

### RR-B-M2 — MEDIUM — The expected-red executable test no longer specifies capability behavior

Location: `docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/expected-red.md:6-16` and
`/tmp/openwepp-residual-jacobian-FC2T1v/J/crates/openwepp-land-surface-energy/src/solver_stem_jacobian_tests.rs:1-6`.

Evidence: the retained compiler E0425 is valid evidence that the named capability
was absent before J. The subsequently “converted” test is only an unconditional
`panic!`; it has no API call, expected mask/order/assembly/counter behavior, or
negative predicate. Candidate work can make it green by deleting the panic,
without implementing any contract obligation. It therefore does not satisfy the
protocol's statement that expected-red tests demonstrate missing derivative
capability, assembly and counters or pin the required negatives.

Required disposition: retain the E0425 record, but replace the unconditional
panic seam with compile-gated or scaffolded behavior tests whose assertions are
already fixed and which become green only through the named capability,
hybrid-assembly participation and counters. Add the listed lifecycle/error /
structural-fallback negatives before the implementation under test can satisfy
them; never weaken/delete the assertions to obtain green.

### RR-B-L1 — LOW — The N=1 directional-vector rule is internally inconsistent

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:238-247`.

Evidence: all eight vectors are declared to have infinity norm one, but for
`N=1` the text says signed single-coordinate vectors repeat without defining how
`e1`, `e0+e1`, and `e0-e1` map. Treating `e1=e0` produces norm two and zero for
the last two vectors. Arbitrary-cardinality authority requires a deterministic
N=1 rule.

Required disposition: explicitly list the eight N=1 vectors (duplicates are
acceptable if separately identified) and ensure each nonzero vector has the
declared norm, or state that zero directions are allowed and their separate
acceptance rule.

## Non-blocking debt / follow-ups

- The corpus truthfully contains only authentic N=2/S=6 smooth early/late
  Potential/FixedFinal states; exact/near-bound, zero-area, kink and alternate
  cardinality cases remain correctly assigned to contract-derived tests. Do not
  broaden corpus claims without new frozen identities.
- Full formatter, warnings-denied Clippy, tests, deny, consumer, closure and
  line-count gates remain future implementation gates. The focused binding test
  is not evidence for them.
- The focused binding run exposed one dead-code warning in an inherited
  orchestrator field. Preserve it as classified prior debt unless the exact
  terminal diff makes it relevant; do not represent this non-`-D warnings` run as
  a Clippy/warnings gate.

## Recommendation

**HOLD.** The mathematical row/normalization/result corrections are materially
improved and four original B findings are closed, but derivative implementation
is not ready. Phase A still lacks the candidate-independent oracle/support mask,
and the authentic corpus is not bound to its post-A capture source/build identity
or a reconciled exact write set. The remaining cost, expected-red and N=1
definitions should also be corrected while the protocol is still prospective.
After RR-B-H1 and RR-B-H2 are resolved and the medium/low amendments land,
obtain focused independent re-review of the newly stable cut. Production HOLD
remains unchanged.
