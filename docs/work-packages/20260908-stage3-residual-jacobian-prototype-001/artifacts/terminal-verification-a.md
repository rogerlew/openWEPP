# Independent terminal verification A

Static: root and package instructions; `role-verification.md`; testing strategy
sections 7, 9, 10, and 18; science obligations; the v33 LSE contract cut;
package protocol, manifest, oracle, corpus, gate, treatment, handoff, and
disposition records; detached Phase-A oracle/capture source. Ran: the commands
listed below.

Role/session: terminal verifier A; configured/requested effort: independent
bounded verification; effective runtime setting: UNOBSERVED. Reviewed identity:
primary scaffold HEAD `827a7470e058a5e09f9feced9375b776e5959789`
plus the dirty documented v33/package cut; detached HEAD
`e89befa4678eadec039b3e7f7fe0a176af8e9dc5`; Phase-A bundle manifest SHA-256
`488eaff372f48802efc1ccf59a7deb8d5fdd27aacf25d06900f5896e0c8293ad`.
Assigned scope: authority/binding/unit checks, Phase-A byte identity, oracle
self-test and authentic transaction, and independent inspection of the 0/16
reduction. No source or configuration was edited.

## Independent checks

- `.venv/bin/python tools/check_sc_binding_exposure.py --strict
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
  from `/workdir/openWEPP`: exit 0; PASS, directory-v1, 18 binding-exposure
  rows and 102 definitions.
- `bash tools/release/check_sc_unit_compliance.sh --path
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`:
  exit 0; PASS, no unit-compliance findings.
- `nix develop --offline -c cargo nextest run --test
  land_surface_energy_balance_authority_contract
  version_thirty_three_binds_experimental_residual_jacobian_policy`: exit 0;
  PASS 1/1 with 26 skipped. One existing orchestrator dead-code warning was
  emitted; this was not a warnings-denied gate.
- `.venv/bin/python tools/agents/evidence_bundle.py verify
  /tmp/openwepp-rj-phase-a-terminal-bundle`: exit 0. The manifest digest and
  sidecar both equal `488eaff3...8293ad`; all declared blobs passed inventory,
  size, and digest verification. I separately hashed all 12 captured source
  files in the live detached tree; every digest matched its bundle entry.
  `git hash-object` independently reproduced the manifest's two Phase-A source
  blob IDs: oracle `3ae38b0830472d404b4378c7fea82a08172182f4`
  and owning test `8acff12d8e571914c6ced1f69973c0d8e99d3ad6`.
- Independent corpus inspection reproduced SHA-256
  `a05ca09031c28f863bc8dd8f3cc8d605459a683abc2af6e0f37dd84c713068c0`,
  509,132 bytes, 8 records (4 Potential and 4 FixedFinal), 4 first and 4 last,
  N=2/S=2/D=25 in every record, and 4,636 finite exact-tagged binary64 words.
  Both stem areas are positive in all eight records (0.72 and 0.4176); all
  eight lifecycle tuples are distinct and contain nonzero solve/parent/scope
  joins.
- `nix develop --offline -c cargo test -p openwepp-land-surface-energy
  solver_stem_jacobian_tests::v33_oracle_selector_self_tests_precede_candidate_code
  -- --exact --nocapture` from the detached tree: exit 0; PASS 1/1. It exercises
  the smooth cubic, constant/roundoff rejection, kink rejection, and deliberately
  wrong derivative rejection.
- With `OPENWEPP_RESIDUAL_CORPUS_PATH=/tmp/openwepp-rj-terminal-verifier-a.json`,
  `nix develop --offline -c cargo test -p openwepp-land-surface-energy
  solver::tests::covered_v8_block_matches_frozen_joint_solution -- --exact
  --nocapture`: exit 0; PASS 1/1 and printed
  `baseline v33 dry-stem oracle support: 0/16 columns`. The independently
  regenerated corpus was byte-identical to the retained corpus: the same
  SHA-256 and 509,132-byte size.
- Static inspection confirms that the audit reconstructs each base through the
  canonical primal evaluator, checks exact replay, independently enumerates the
  affected/zero mask, probes all nine frozen scales, and counts a column only
  when every required affected row selects a basin. It loops two positive-stem
  columns over eight records, hence exactly 16 candidates. The candidate source
  `solver_stem_jacobian.rs` and all named v33 derivative/integrity symbols are
  absent. For the published first failing row, I recomputed all seven
  consecutive-triple ratios from the retained estimates: approximately
  `10.155, 0.00798, 33.333, 0.0227, 10.160, 0`, and undefined; none lies in
  `[2.5,5.5]`.

## Findings and disposition

- **TV-A-001 — MEDIUM — package status records are not yet mutually current.**
  `package.md` says COMPLETE while its last checklist row still requires dual
  terminal review/verification; `preimplementation-contract-gate.md` still
  calls the superseded zero-stem `b8f6a923...` corpus PASS and says corrected
  re-review is pending; `kernel-profile-compliance-checklist.md` still labels
  test vectors pending. These stale rows do not alter the independently
  reproduced Phase-A result, but they prevent treating the present publication
  set as a reconciled terminal cut. Correction: reconcile these status tables
  and freeze the actual terminal publication identity after both reviews and
  verifications land.
- **TV-A-002 — LOW — the authentic transaction test reports but does not assert
  the historical 0/16 result.** Its assertions require only `candidates > 0`;
  `supported` is printed. Exact source/result custody and this verifier's output
  authenticate the current negative result, but a future rerun could pass while
  changing it. Correction: retain the historical bundle as the authority for
  this disposition, and if v33 remains executable test authority, assert the
  frozen `candidates == 16` and `supported == 0` or emit a machine-checked result
  record. Do not tune v33 from the observation.

Accepted prospective fixes inspected here are present: the shared-heat floor
uses pre-floor `y`; nonfinite executed stem area follows existing validation;
the N=1 vector mapping is norm-one; and the cost taxonomy declares separate
baseline stem-FD and broad-coverage formulas. The former prospective HOLDs are
therefore not evidence against the corrected Phase-A numerical result. No J was
implemented, so candidate/full-solve/cost/timing/memory/scale gates being NOT
RUN is a legitimate frozen early-stop consequence, not a deferred PASS.

Uncertainty: effective model/runtime settings and workflow-total reading
exposure are unobserved. This verification does not claim bit-reproducible
rebuildability beyond the captured bytes, derivative correctness, candidate
equivalence, performance, scale behavior, or production readiness.

Verdict: **PASS for the bounded Phase-A claim**: the retained authentic
positive-stem transaction reproduces byte-identically and the frozen
baseline-only v33 oracle reports 0/16 admitted columns, so stopping before J and
retaining production HOLD is supported. **Terminal package publication remains
HOLD until TV-A-001 is reconciled**; that documentation closure does not change
the verified `INCONCLUSIVE_WITH_BOUND` scientific result.
