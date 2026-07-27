# Worker Handoff

Status: `HOLD / EXTERNAL ASSURANCE LIFECYCLE ACTION REQUIRED`

Evidence class: `Ran + Static`

`CAL04B-NATIVE-001` production and frozen replay work is complete. Do not edit
its physics, reopen calibration population, or access Harvard.

First actionable item: close defect
`ASSURANCE-CANOPY-README-IDENTITY-001` in a separate assurance-lifecycle work
package.

- Observable failure: exact-head
  `cargo nextest run --workspace --profile full` fails because
  `tests/fixtures/cancov_forest/README.md` no longer matches its generated
  identity.
- Suspected mechanism: commit `502dd745` changed the admitted README after the
  current machine-owned lock/review generation.
- In-scope authority/write set for the new package: the snow/frozen-soil
  assurance report, its dependency and review state, the generated identity
  transaction, and only directly required assurance lifecycle evidence.
- Required reading: `assurance/v2/README.md`,
  `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/report.yaml`,
  its `review.lock.json`, and the assurance amendment/identity contracts.
- Failing evidence: current README hash
  `b81fbe2efa5624e5018c18f24c55ada53d7c484ff020b19d6fa1deae8bd1dd7b`
  versus bound hash
  `703a138076900f24a3232457dfab8744e60f69ab196b4b361eeb12bbfedb268c`.
- Correction authority: a governed assurance lifecycle/evidence transaction,
  with any required human review-state adjudication; not this canopy package.
- Acceptance target: the assurance source and review state are current under a
  valid transaction, then the exact-head unfiltered full profile passes.
- HOLD legitimacy: this package changes none of the affected assurance
  surfaces, and the mismatch predates its authenticated base.
- Forbidden relay: do not stop after inspecting another function. Manual hash
  edits, `rebind-implementation` adoption of report evidence, and reverting the
  earlier research documentation are evasion.

After that defect closes, return here, rerun the unfiltered full profile, then
complete dual terminal verification, archive the prompt, lift the CAL-04B
prerequisite, and close.
