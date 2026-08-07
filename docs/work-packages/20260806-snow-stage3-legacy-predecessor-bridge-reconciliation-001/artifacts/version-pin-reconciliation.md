# Contract Version-Pin Reconciliation

Status: `prospective test-only write-set amendment / review pending`.

Evidence mode: `Ran terminal failure; Static disposition`.

At exact clean `de3c14933650ed7eb41a54ac86028af4448f7a1d`, workspace
quick ran 910 tests before fail-fast: 908 passed and two failed because old
integration guards required the canonical SC-SNOWFREEZE-001 file to contain
`contract_version: 129`. The contract is legitimately v130.

An exact inventory found 37 tracked `tests/integration/*.rs` files containing
that same assertion. These historical package tests already bind their owning
invariants, obligations, behaviors, source paths, and package evidence. They
do not own the latest canonical revision number.

The prospectively admitted correction is mechanical and test-only: in exactly
those 37 files, replace `contract_version: 129` with
`contract_id: SC-SNOWFREEZE-001`. Do not remove or alter any other assertion.
This preserves canonical file identity and avoids silently weakening the
process-specific guards, while allowing later additive contract revisions
without editing every historical package test.

Before the edit, an independent reviewer must verify the scope and nonweakening
argument. After the edit, require an exact 37-to-zero marker count, focused
execution of all changed integration binaries, and a fresh complete terminal
suite. The failed quick run remains evidence and is not converted into a pass.
