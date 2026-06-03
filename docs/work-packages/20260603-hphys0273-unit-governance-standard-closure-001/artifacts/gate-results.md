# Gate Results

Status: completed
Evidence mode: ran

Static: HPHYS0273 is docs/governance-only; no Rust production code or runtime
projection changed under this package.

Ran:

```text
markdown-doc lint --path docs/specifications/unit-governance.md \
  --path docs/specifications/science-contract-authoring-procedure.md \
  --path docs/specifications/science-contracts/kernel-process-contract-profile.md \
  --path docs/specifications/science-contracts/index.md \
  --path docs/work-packages/README.md \
  --path docs/work-packages/20260603-hphys0273-unit-governance-standard-closure-001
✅ 29 files validated, 0 errors, 0 warnings
```

Post-review fix rerun:

```text
markdown-doc lint --path docs/specifications/unit-governance.md \
  --path docs/specifications/science-contract-authoring-procedure.md \
  --path docs/specifications/science-contracts/kernel-process-contract-profile.md \
  --path docs/specifications/science-contracts/index.md \
  --path docs/work-packages/README.md \
  --path docs/work-packages/20260603-hphys0273-unit-governance-standard-closure-001 \
  --path docs/work-packages/20260603-hphys0274-boundary-symbol-unit-registry-closure-001 \
  --path docs/work-packages/20260603-hphys0275-boundaryvalue-dimensional-typing-remediation-001 \
  --path docs/work-packages/20260603-hphys0276-unit-conversion-helper-and-raw-literal-guard-001 \
  --path docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001 \
  --path docs/work-packages/20260603-hphys0278-output-unit-metadata-registry-alignment-001 \
  --path docs/work-packages/20260603-hphys0279-sc-contract-unit-compliance-lint-001
✅ 167 files validated, 0 errors, 0 warnings
```

Final artifact consistency check:

```text
rg -n "Status: queued|queued placeholder|Evidence mode: not-run" \
  docs/work-packages/20260603-hphys0273-unit-governance-standard-closure-001
# no output

for pkg in docs/work-packages/20260603-hphys027{4,5,6,7,8,9}-*; do
  rg -q "docs/specifications/unit-governance.md" "$pkg/package.md"
  rg -q "docs/specifications/unit-governance.md" \
    "$pkg"/prompts/active/*_kickoff_agent_prompt.md
done
# no output

markdown-doc lint ... HPHYS0273 through HPHYS0279
✅ 167 files validated, 0 errors, 0 warnings
```
