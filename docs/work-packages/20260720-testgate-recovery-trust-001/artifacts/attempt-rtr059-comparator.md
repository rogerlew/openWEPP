# RTR-059 Changed-Head Comparator Attempt

Evidence class: Ran unless labeled Static.

## Subject

- Head: `a2446adcd1be74de98a29333ab33bc80af8ab13d`
- Comparison base: `63c0f3ea8fcd192976ebaa0b7eeed9ed17cd88e1`
- Release planner SHA-256:
  `db0a1d8b48e93783c96632391a5f81210ca00f90eb7e078d18951f89bff5a3aa`
- Artifact root:
  `/home/workdir/testgate-recovery-trust-01-final-rtr059.bBBR3t`
- Intent plan:
  `9d27fa498d57a5699ab40f884b7d595e1a5cfb23e5d7aa273644c381cbf99d96`
- Terminal plan:
  `db9f75c9a207d03e1301943e66f0ee5457fbbda18ebeaa3f6eef51ff1e51c694`
- Package authority chain:
  `ccd11b612e9490d71b2781781a39292c4787a8d0593027444b5d0c686f445eb9`

## Result

Ran: LIGHT passed 6/6. Pre-HEAVY audit
`337fec31ad6c891df1f6102d7ab0434d0c4edc0d4f41f8b1974362cd4c9c3a95`
was `READY` with all 10 checks PASS.

Ran: receipt
`2ee57361119b196534abd9653fd5a37205e0bcf6a2ab75cf0eb2310b1210aa21`
sealed 13 PASS, one FAIL, one prerequisite-BLOCKED, and zero retries.
Source mutation was unchanged. Planner wall time was 344.483 seconds, HEAVY
ledger wall time was 1,308.635 seconds, and total wrapper wall time was
1,718.819 seconds.

Ran: ordinary full Nextest executed once for 985.473 seconds: 2,302/2,304
tests passed, two failed, and seven configured tests were skipped. CRAP did
not launch; its node was prerequisite-BLOCKED with no coverage output.

The two failures were:

- `executor::coverage_tests::ready_audited_heavy_preserves_import_and_final_receipt_bindings`;
- `verifier::tests::coverage_tests::ready_audit_verification_preserves_order_and_exact_verdict`.

Static: both duplicated `DurableLedger::new` fixtures joined
`CARGO_MANIFEST_DIR` with `../..` without canonicalizing it. The resulting
absolute paths retained parent components, so production correctly returned
`GATE-AUDIT-LEDGER-PATH`. RTR-060 owns the fixture correction. This failed
head was not rerun.
