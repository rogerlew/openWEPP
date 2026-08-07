# Contract Version-Pin Reconciliation

Status: `PASS / stable identity and registry structure reconciled`.

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

Independent review passed at exact clean `cc6a17db7`: the inventory is 37
files and 38 occurrences; stable identity plus unchanged substantive markers
is nonweakening, while current package tests continue to pin v130 directly.
The 38 substitutions are exact and no v129 contract-version marker remains.

The first focused execution ran 35 of 158 selected tests before fail-fast: 34
passed and one existing shadow-observability test failed because it separately
required the registry index to contain both `v129` and obsolete v129 summary
prose. That test's package realization assertions are still current. Before
editing it, this package prospectively admits only an exact registry-row
extraction asserting the stable contract ID, canonical path, `in_review`, and
`draft`. This follows `tests/AGENTS.md`: registry tests bind structure and
lifecycle; detailed authority remains in the canonical contract.

Independent review passed at exact clean `55c7eef19` and required unique-row,
positional field checks. The implementation extracts the sole exact
SC-SNOWFREEZE-001 row and binds its ID, canonical path, `in_review`, `draft`,
and ten-field shape. Package-side realization assertions are byte-unchanged.

The dedicated shadow-observability binary passed `6/6`. A no-fail-fast run of
all 37 changed integration binaries then passed `158/158` in `282.131 s`.
Exact diff inspection confirms 38 identity substitutions plus only the
reviewed registry-row restructuring. A fresh complete terminal suite remains
required. Both failed runs remain evidence and are not converted into passes.
