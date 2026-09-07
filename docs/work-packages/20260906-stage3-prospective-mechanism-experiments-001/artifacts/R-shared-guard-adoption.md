# SG1 canonical adoption record

Static: exact reviewed authority adoption only; runtime/scientific admission
and measurement gates remain pending. Retained production HOLD.

Canonical `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
now ends with the exact reviewed `EXP-STAGE3-20260906-R-SG1` subsection,
lines 3286–3346. SHA-256:
`7a002bcac2ad640716b52f4bd59326f241d5fd3084d26e56f6a703eb496ed61d`.
The previous canonical SHA was
`c993020e96aec0dac934acd5ccfa3050f8173a90bcd7f860b88b8da7405418cd`.
This adoption appends 62 lines including the separating blank line; no
historical text, registry, source, runtime selector or physical guard changed.

## Contract-first approval chain

- Proposed text: `R-shared-guard-authority.md`, SHA-256
  `856ff186550815276e2baf205f7d1a4f127bfa07e63ed6d43a4155e53ff7131d`.
- Derived source proof: `R-shared-guard-source-proof.md`, SHA-256
  `48e5dcfe7ea904e19bf434023a87e87da0efde87900879cb4718a719551c72de`.
- Corrected formatted R-only runtime tests:
  `638de1d0e5d04c32f4e8a5e4f10a9fe2fca2f2f964ed2515499b061ed162e2e2`;
  runtime plan `70d64ad77e98323ea16d48cd7fb3f46711f98b107f10d460765c705f1c3b0aa3`.
- Independent A GO: `R-implementation-review-a.md`, section
  “Final SG1 derived rollback cut: prospective adoption GO”.
- Independent B GO: `R-shared-guard-review-b.md`, final concrete-cut addendum.
- Parent explicitly confirmed application after both exact-cut approvals and
  completion of main A full/debug readers. Canonical application used
  `apply_patch`; no active A authority reader was modified mid-run.

The proof artifact's older test hash and rollback-assertion limitation remain
historical. The corrected test cut explicitly snapshots nested input float
bits/tags and actual base/capture/caps/frozen/custody. Both reviewers accepted
that prospective correction without claiming execution. Their addenda and
this record supply the corrected-cut status without rewriting frozen proof
history. Private graph proof remains a separate obligation.

Ran: exact proposed-subsection versus canonical-tail `diff -u` passed;
canonical `git diff --check` passed; canonical full-file hash recorded above.
No runtime tests, builds, isolated-R copy, or runtime/source edits were
performed by the authority author. Parent must check the canonical hash before
authorizing any R copy. This record does not claim a wet-error witness,
historical noncrossability PASS, or experiment completion.
