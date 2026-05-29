# HPHYS0201 Gate Results

Status: completed  
Evidence mode: Ran

## Documentation validation gates
1. `wctl doc-lint` (default staged mode) -> pass
   - Result summary: `files_scanned=0`, `errors=0`, `warnings=0`
   - Logs:
     - `/tmp/hphys0201_20260529T232700Z/gates/wctl_doc_lint.stdout.log`
     - `/tmp/hphys0201_20260529T232700Z/gates/wctl_doc_lint.stderr.log`
2. `markdown-doc lint` (scoped HPHYS paths) -> pass
   - Result summary: `files_scanned=81`, `errors=0`, `warnings=0`
   - Logs:
     - `/tmp/hphys0201_20260529T232700Z/gates/markdown_doc_lint_hphys.stdout.json`
     - `/tmp/hphys0201_20260529T232700Z/gates/markdown_doc_lint_hphys.stderr.log`
     - `/tmp/hphys0201_20260529T232700Z/gates/markdown_doc_lint_hphys.exitcode`

## Verification command outputs
- Queue entries and sequence verification:
  `/tmp/hphys0201_20260529T232700Z/verification/readme_hphys_entries.log`
- Follow-on package measure wording verification:
  `/tmp/hphys0201_20260529T232700Z/verification/followon_package_measures.log`
