# Contract Reference — SC-OFEROUTE-002 (rev 1, draft)

Canonical path:
`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`

Draft class: **CONSOLIDATION** — the review question is provenance fidelity,
not new science. The draft claims to introduce NO new normative content
beyond (a) stable invariant IDs (`INV-OFEHYB-001..010`) and gap IDs
(`GAP-OFEHYB-001/002`), (b) obligation/BEI/guard-map organization, and
(c) the gap register carrying the executed Case-4 HOLD with its two
recorded design levers.

Provenance sources the reviewers must check the draft against:

- `SC-OFEROUTE-001` revision-history entries 28, 29, 30, 31 (verbatim in
  that contract; rev 32 is the pointer transfer).
- The pre-rev-32 hybrid Branch-and-Guard / Test-Vector / BEI rows of
  `SC-OFEROUTE-001` (in git history at `48129fac`).
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i0-scheme-design.md`
  (esp. §1 discrete form/ledger, §2 switching rationale + the recorded
  explicit cool-down fallback, §2.1-2.3 seam/dt/ledger rules).
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/ratification-evidence.md`
  (the HOLD numbers and counter evidence).
- The implementation:
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/{implicit_recession,cascade,kinematic_wave}.rs`
  and the runner selector plumbing.

Review lanes (per the authoring procedure):
- `review_agent_a.md` — completeness/fidelity: every normative rule in the
  provenance appears in the draft, unweakened; nothing new smuggled in.
- `review_agent_b.md` — code-vs-contract: every draft rule matches the
  current implementation and its retained vectors; guard map anchors real
  enforcement paths.
- `disposition.md` / `verification_agent_{a,b}.md` — per procedure after
  findings.
