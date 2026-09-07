# Source-kit reproduction verification

Ran: 2026-09-06. A source kit reconstructs all 10,131 declared executable-input
manifest rows from exact base `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`, its
tracked patch and new-source archive. Final result: **PASS**, after explicit
restoration of four recorded read/write permission modes that Git does not store.
No build, network access, branch/worktree change or main/F/R source edit occurred.

## Identity and command

Kit: `artifacts/reproduction/A-source`; source identity:
`3ef4346e9bb84553acfad4407ea6fd616bedccc570278e8142edacbab5e2bb58`.
Tracked patch SHA-256:
`3618fe2893f91db03c390998e7fcfa4497203e0e1655a6f81235ebc15a9ff47e`.

```sh
.venv/bin/python docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/reproduction/verify_source_kit.py --repo /workdir/openWEPP --kit docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/reproduction/A-source --restore-recorded-permissions
```

Exit 0. Preserved reconstructed source:
`/tmp/openwepp-source-kit-verify-m8_rkbk_/source`.
The same command accepts subsequently frozen F/R kit paths; those arms have
**NOT RUN** in this artifact. Scratch is preserved, not required custody.

## Checks and actual results

The independent verifier checks the manifest hash, patch hash, full base commit,
sorted unique canonical paths, complete reconstructed path membership, each
present file's byte length/content SHA-256/POSIX mode/symlink target, every
declared deletion, and every base path removed by the patch. It extracts only
the declared source-input roots from `git archive`, applies the patch, then
extracts the new-source archive with path/link containment checks. It does not
read current main/F/R source to fill a missing input and does not use the changed
source snapshot as a substitute for reconstruction.

| Check | Result |
|---|---|
| Present rows | 10,131 checked, all match |
| Deleted rows / removed base paths | 0 / 0 |
| New-source archive paths | 0; exact agreement with kit metadata |
| Missing / extra source paths | 0 / 0 |
| Byte length / content / symlink mismatches | 0 / 0 / 0 |
| Final mode mismatches | 0 |
| Builds executed | None |

Initial invocation omitted `--restore-recorded-permissions` and exited 1.
Its preserved source is `/tmp/openwepp-source-kit-verify-yrxrc7l_/source`.
It matched all contents but found these four modes at `0664`, where the manifest
records `0644`:

- `crates/openwepp-runner/tests/fixtures/snow_stage3_v11_owner_seed_frozen_litter_v3.json`
- `crates/openwepp-runner/tests/fixtures/snow_stage3_v11_owner_seed_frozen_litter_v4.json`
- `crates/openwepp-runner/tests/fixtures/snow_stage3_v11_owner_seed_two_day.json`
- `tests/fixtures/watershed/p102-sediment-active/runs/H1.source.run.snow_stage3_v11_owner_seed.json`

Git records executable status, not every local permission bit. Reproduction
therefore uses explicit umask `0002`, restores recorded read/write bits from the
manifest with the named option, and checks final modes. The option never repairs
a disagreement in executable bits and logs every restoration. All four above
were restored in fresh scratch; no content was substituted or kit modified.
The initial four failures remain recorded rather than reported as an original
recipe pass.

Scope limit: this proves the source kit's declared executable-input manifest;
documentary outputs are intentionally excluded. It is not a build or executable
reproduction claim. Compilation/runtime fixtures outside this declared manifest
remain the broader package's input-custody obligation.

## Owned paths and governance

Before edits, `tools/agents/find-agents --for` was run for both owned new paths:
`artifacts/reproduction/verify_source_kit.py` and this artifact. Applicable
instructions were root `AGENTS.md` and `docs/work-packages/AGENTS.md`, read along
with this package's `package.md`. These are package-local verification changes;
no production or canonical-authority edit is included.
