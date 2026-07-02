# Codex Review Request — MOFEFID-D7 Scaffold

**Type:** scaffold review (no code to review yet). Review the *framing,
scope, acceptance model, and governance* of the D7 D-val work-package before
execution begins. This is a review of a plan, not an implementation — surface
issues and challenge the framing; do not write the harness.

**Read:**
- `docs/work-packages/20260702-mofefid-d7-enhanced-wepp-dval-001/package.md`
- `docs/work-packages/20260702-mofefid-d7-enhanced-wepp-dval-001/artifacts/feasibility-findings.md`
- Context: `SC-OFEROUTE-001` (`INV-OFEROUTE-002`, `INV-OFEROUTE-011`); D01
  `validation-cases.json` + `source-manifest.md`; `references/copyrighted/Papanicolaou2018.md`
  §3.1 / Table 1 (copyrighted, gitignored — read locally, do not vendor).

## What to adjudicate

1. **Is the reproduce-enhanced-WEPP framing correct?** The scaffold claims
   this validates *method fidelity* (openWEPP reproduces Papanicolaou's model)
   and explicitly **not** validation against nature (their Ef-vs-observed
   stays a citation). Is that the right authority posture under ADR-0011 /
   ADR-0017, and is the language tight enough that no downstream reader will
   mistake it for observational validation?

2. **Is the acceptance model honest and adequate?** Per-case NS ≥ 0.85 (+
   peak/timing bands) to "reproduce"; a non-reproducing case (Iwagaki today)
   becomes a documented `GAP-OFEROUTE`, never a tuned pass. Is the NS
   threshold defensible? Is there a failure mode where a case could pass NS
   yet be physically wrong (e.g., right integral, wrong shape), and should the
   acceptance carry a shape/peak co-condition beyond NS?

3. **Cut-point mapping (S0) sufficiency.** The feasibility spike found Fig 5's
   magnitude implies the Walnut Creek hillslope, not the Case-1 plot, and
   units differ per column. Is S0 (prove like-for-like before any NS) framed
   strongly enough to prevent the HPHYS comparator-surface class of false
   result? Is anything missing (a dimensional guard, a per-column provenance
   requirement)?

4. **Operand-gap handling.** Cases 1–3 need texture-derived Green-Ampt params;
   Case 4's flume `k_o` is unspecified. Does the scaffold correctly keep an
   *operand-limited* verdict distinct from a *solver-defect* verdict (esp. for
   the Iwagaki lag)? Should S1 set an explicit rule for when an unresolved
   operand blocks a verdict vs. yields a bounded one?

5. **Copyright governance.** Committed tests use cited scalars + offline NS in
   artifacts referencing the xlsx by sha256; no series vendored. Is this
   sufficient and consistent with the C01/C03 precedent, or does the
   offline-NS-in-artifact step need a stricter reproducibility control (e.g.,
   a checked-in extraction script that reads the gitignored source at run
   time, like the auth setup-staged tests)?

6. **The Iwagaki shock finding.** The spike shows a ~5 s timing lag (NS ≈ 0.15)
   that survives cascade vs single-mesh and any `k_o`. Is treating this as a
   D7-S4 *attribution* (numerical diffusion vs friction regime vs celerity)
   with a possible feed-back to D4 — rather than a D7 fix — the right
   boundary? Does this warrant opening the GAP now (at scaffold) or only after
   S4 attribution?

7. **Sequencing / scope creep.** Is the S0→S5 staging right, and is anything
   over- or under-scoped? In particular: should D7 attempt all four cases, or
   land the tractable rise-to-steady cases as one package and split the shock
   investigation into its own?

## Output

Findings with severity + evidence (the review-artifact convention: surface
issues and reasoning, leave architecture/disposition choices open). A
`hold`/`ready-to-execute` disposition on the scaffold. Do not implement.
