# Codex Review Request — SC-OFEROUTE-002 dual-agent contract review

Task: dual-agent contract review of the NEW `SC-OFEROUTE-002` (Hybrid
Implicit-Explicit Kinematic-Wave Stepping Contract, rev 1, `status: draft`)
plus the `SC-OFEROUTE-001` rev-32 pointer transfer. Diff base: `48129fac`.
At authoring time the change set is the uncommitted working tree; if a
commit has landed it, review that commit (verify the base).

Repo: `/home/workdir/openWEPP`. Read `AGENTS.md`,
`docs/work-packages/AGENTS.md`,
`docs/specifications/science-contract-spec.md`,
`docs/specifications/science-contracts/kernel-process-contract-profile.md`,
and the package
`docs/work-packages/20260707-laned-router-hybrid-contract-authority-001/`
(`package.md` + `artifacts/science-contracts/SC-OFEROUTE-002/contract_ref.md`,
which lists the provenance sources and lane assignments).

## The review question

This is a CONSOLIDATION draft: it claims to carry the hybrid subsystem's
existing authority (SC-OFEROUTE-001 revs 28-31 + the T3 design record) into
one normative document with NO new normative content beyond stable IDs,
organization, and the recorded Case-4 HOLD + design levers. The review must
answer three things:

1. **Nothing dropped or weakened** (agent A lane): every normative rule in
   the provenance appears in the draft at equal or stronger strength.
   Specific high-value checks: the rev-29 double-collapse theorem statement
   and its fail-closed consequence; the rev-30 deficit-carry dispositions
   INCLUDING the C-L1 bounded all-dry drop and the C-M1 hour-partition
   guard; the rev-31 warm-seed acceptance conditions (branch-side, finite,
   positive, cold fallback) — note the draft compresses these; verify no
   condition was lost; the i0 §2.2 seam semantics (depth carries, discharge
   re-derived / installed); the dt policy; the "no filled-jump commit"
   prohibition.
2. **Nothing smuggled in** (agent A lane): flag ANY rule in the draft that
   is not traceable to the provenance. Known deliberate additions (declared,
   not smuggled): invariant/gap IDs, the obligations tables, the BEI rows,
   and the GAP-OFEHYB-001 lever notes (the explicit cool-down IS provenance
   from i0 §2; the spatial-predicate note and the "q-departure cannot
   discriminate" observation are assessment-class — decide whether they
   belong in a gap register or should be trimmed to the i0-recorded lever
   only).
3. **Code-vs-contract fidelity** (agent B lane): every draft rule matches
   the CURRENT implementation (post rev-31) and its retained vectors:
   `implicit_recession.rs` (solve chain, seeds, warm seeding, residual
   guard + dust floor), `kinematic_wave.rs` (Steffensen basin lock,
   deficit-returning variant + wrapper), `cascade.rs` (mask, hour-partition
   guard, carry, dispositions), runner plumbing (selector, manifest,
   counters — verify the counter NAMES in §Algorithm 6 match
   `ofe_routing::profile`). Verify the guard-map rows anchor real tests by
   their actual names.

Also verify structural compliance: required section order per the spec +
kernel profile; front-matter fields; the index row; and that the rev-32
pointer rows in `SC-OFEROUTE-001` leave no dangling references (grep for
consumers of the old row anchors).

Gates you may re-run (all PASSED at authoring): `markdown-doc lint` on the
touched docs; `python tools/check_sc_binding_exposure.py` on both contracts
(expect `PASS-DEFERRED`); `bash tools/release/check_sc_unit_compliance.sh`
on both (expect `PASS`). No production code changed in this package — cargo
gates are out of scope.

## Output protocol

Findings severity-ordered (High/Medium/Low) with `file:line` anchors;
explicit verdict per lane (GO / GO-WITH-AMENDMENTS / NO-GO for lifting
`status: draft` → `approved`). Write results to the procedure paths:

- `docs/work-packages/20260707-laned-router-hybrid-contract-authority-001/artifacts/science-contracts/SC-OFEROUTE-002/review_agent_a.md`
- `docs/work-packages/20260707-laned-router-hybrid-contract-authority-001/artifacts/science-contracts/SC-OFEROUTE-002/review_agent_b.md`

Do not modify the contract or production code; findings only. Note: the
draft's `status` stays `draft` until the disposition passes — approving is
the disposition's act, not the review's.
