# Gate Results

Status: `PASS`

Evidence mode: `Ran`

Working directory for every command was `/home/workdir/openWEPP`; openWEPP base
identity was `86faf6fd22421372c6d9874b7bd0b7e1cabd439f`.

## Exact Passing Commands

Each command below returned zero on the terminal audit bytes:

```text
markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001 --format plain
markdown-doc lint --path docs/work-packages/20260808-rhessys-east-coast-coupled-vegetation-slice-001 --format plain
markdown-doc lint --path docs/backlog/20260806-rhessys-derived-vegetation-crate.md --format plain
markdown-doc lint --path docs/backlog/TRACKER.md --format plain
markdown-doc lint --path docs/ROADMAP.md --format plain
markdown-doc lint --path docs/work-packages/README.md --format plain
markdown-doc lint --path references/annotated_bibliography.md --format plain
markdown-doc lint --path references/rights_classification_first_pass_2026-05-11.md --format plain
markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md --format plain
markdown-doc lint --path docs/specifications/science-contracts/index.md --format plain
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
git diff --check
cargo nextest run --test vegetation_boundary_authority_contract
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
```

The audit-package lint validated 43 files, the successor lint validated 34,
and each remaining Markdown invocation validated one file; all reported zero
errors and zero warnings. The science-contract unit check passed. The affected
contract-derived suite ran eight tests with eight passes, and the strict
Binding Exposure Index check found two fully consolidated rows.

The exact passing population/source/archive/contract assertion shell was:

```bash
set -euxo pipefail
matrix=docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/parameter-authority-matrix.md
fields=$(awk '/^## Field Inventory/{s=1;next}/^## Parser-Only/{s=0}s&&/^\| [0-9]+ \|/{n++}END{print n+0}' "$matrix")
defaults=$(awk '/^## Parser-Only/{s=1;next}/^## Profile Inventory/{s=0}s&&/^\| [0-9]+ \|/{n++}END{print n+0}' "$matrix")
profiles=$(awk '/^## Profile Inventory/{s=1;next}/^## Terminal Interpretation/{s=0}s&&/^\| [0-9]+ \|/{n++}END{print n+0}' "$matrix")
test "$fields" -eq 71
test "$defaults" -eq 53
test "$profiles" -eq 32
test "$(rg -c '^\| `SRC-' docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/source-function-state-inventory.md)" -eq 31
test "$(rg -c '^\| `CON-' docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/code-literature-concordance-matrix.md)" -eq 35
test "$(rg -c '^\| `DEV-' docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/code-literature-deviation-register.md)" -eq 30
test "$(rg -c '^\| `AUTH-RHEC-' docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/authority-gap-register.md)" -eq 16
test "$(rg -c '^\| `CIT-' docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/artifacts/citation-discovery-ledger.md)" -eq 32
test -z "$(git diff --name-only -- '*.rs' 'Cargo.toml' 'Cargo.lock')"
test -z "$(git status --short -- references/copyrighted)"
test -z "$(git -C /workdir/RHESSysEastCoast status --short)"
test -z "$(git -C /workdir/GIS2RHESSys status --short)"
test "$(git -C /workdir/RHESSysEastCoast rev-parse HEAD)" = 375c75b1cd2202217651dff43aa113d80b9c1118
test "$(git -C /workdir/GIS2RHESSys rev-parse HEAD)" = 6b20883dea7c9fd92f71ec69eaca015ebf6dfe18
test "$(sha256sum /workdir/RHESSysEastCoast/LICENSE | cut -d' ' -f1)" = 4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be
test "$(sha256sum /workdir/GIS2RHESSys/LICENSE | cut -d' ' -f1)" = 4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be
active=docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/prompts/active/20260808-rhessys-east-coast-code-literature-authority-audit-001_kickoff_agent_prompt.md
archived=docs/work-packages/20260808-rhessys-east-coast-code-literature-authority-audit-001/prompts/archived/20260808-rhessys-east-coast-code-literature-authority-audit-001_kickoff_agent_prompt.md
test "$(git show HEAD:$active | sha256sum | cut -d' ' -f1)" = "$(sha256sum "$archived" | cut -d' ' -f1)"
contract=docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md
rg -q 'contract_version: 2' "$contract"
rg -q 'INV-VEGETATION-052' "$contract"
rg -q 'BEI-VEGETATION-002' "$contract"
for i in $(seq 10 21); do
    n=$(printf '%03d' "$i")
    rg -q "GAP-VEGETATION-$n" "$contract"
done
```

It passed for 71 fields, 53 parser-only defaults, 32 profiles, 31 source
groups, 35 concordance rows, 30 deviations, 16 authority gaps, and 32 citation
rows. It also passed both source identities/licenses, clean external
worktrees, byte-identical prompt archival, contract version/invariant/BEI/gap
presence, absence of Rust/Cargo changes, and absence of restricted-full-text
changes.

## Diagnostic And Invalidated Runs

One initial assertion diagnostic exited 1 because its loop searched unpadded
`GAP-VEGETATION-10` names. The command was corrected to three-digit IDs and
passed; this was a test-command defect, not a product finding.

Terminal review identified that the affected contract-derived suite was
applicable. Its first run had six passes and two failures because the amendment
had not preserved exact historical guard phrases. After the contract repair,
the second run had seven passes and one registry-phrase failure. After that
repair, the identical exact command above ran eight tests with eight passes.
Only the final run supports disposition; the failed runs are retained here as
truthful invalidated-gate history.

No production Rust, test source, fixture, Cargo file, coverage/CRAP campaign,
workspace-wide correctness campaign, or comparator run was changed or claimed.
