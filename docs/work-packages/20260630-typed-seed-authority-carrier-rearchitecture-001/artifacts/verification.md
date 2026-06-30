# Verification

Evidence mode: Static/Ran.

## Static Verification A

Static:

- Verified `direct_publication_day_zero_seed_surface` clones a lane seed
  `HillslopeWritebackSurface`, merges day-one climate, and calls
  `seed_wb11_runtime_surface_inputs`.
- Verified `seed_wb11_runtime_surface_inputs` computes WB11/WB18/WB19/WB12/WB16
  and MOFE03 seed state by reading and mutating the surface.
- Verified direct lane construction and `DirectProductionDayInputBuilder`
  consume the day-zero surface through surface readers and `from_seed`
  constructors.

Conclusion: Phase 1 cannot be completed by wiring existing helpers without
retaining symbol-map seed authority.

## Static Verification B

Static:

- Searched for existing parse-derived typed authority constructors. Existing
  direct production authority structs are downstream runtime consumers, not the
  setup projection authority.
- The partial frost typed authority is constructed inside
  `DirectProductionSnowFrostAuthority::from_seed`; it does not solve the full
  lane seed authority.
- `ParsedHillslopeRunInputs` remains local to runner setup and is immediately
  projected into `HillslopeWritebackSurface` fragments.

Conclusion: the correct prerequisite is to factor typed projection APIs and keep
surface-writer adapters only for compatibility replay and transition identity
checks.

## Ran

```text
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-seed-authority-carrier-rearchitecture-001 --format json
markdown-doc validate --path docs/work-packages/README.md --path docs/work-packages/20260630-typed-seed-authority-carrier-rearchitecture-001
git diff --check
```

Result: `markdown-doc lint` scanned `9` files with `0` errors and `0`
warnings; `markdown-doc validate` validated `9` files with `0` errors;
`git diff --check` produced no findings.

Note: `wctl doc-lint --path
docs/work-packages/20260630-typed-seed-authority-carrier-rearchitecture-001`
and explicit package-file paths returned `0 files validated`; this disposition
therefore relies on direct `markdown-doc` evidence.
