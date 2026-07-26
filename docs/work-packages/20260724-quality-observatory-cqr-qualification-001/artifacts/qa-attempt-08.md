# QA Attempt 8

Evidence class: Ran.

- Provider run:
  [`30200514260`](https://github.com/rogerlew/openWEPP/actions/runs/30200514260)
- Source/workflow head:
  `1168bae1dfb21dd1a3be840d4381877aa54d9795`
- Qualification TESTGATE run: `30198502723`
- Result: `COMPLETE`
- Quality evidence ID:
  `5cb7c5ea9471ab536ce7b9c9270992b68c0ab35b3c729e6d6c5095b57692baea`

QA itself passed and published the exact 11-file artifact, but CQR selection-only
intake rejected it as `INVALID`: `adjudicated-crap-report.json` was formatted
JSON rather than canonical compact JSON. CQR did not launch collection.

This is a publication-contract defect, so the QA result cannot qualify the
end-to-end handoff despite its valid tests, measurements, and evidence ID.
The next changed head canonicalizes the externally produced CRAP JSON before
any digest/identity binding and makes the independent QA verifier reject a
noncanonical report.

Complete QA evidence is retained at
`/home/workdir/openWEPP-quality-history/20260726-quality-observatory-run-30200514260`.
The failed CQR intake receipt is retained package-locally as
`cqr-quality-evidence-intake.json`.
