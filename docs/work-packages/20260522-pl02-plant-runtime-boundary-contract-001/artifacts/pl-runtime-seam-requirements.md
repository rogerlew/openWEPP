# PL Runtime Seam Requirements

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Seam requirements must preserve strict typed-failure posture and deterministic runtime ordering.

Ran:
- Derived explicit seam requirements from parser executable profile and baseline runtime branch/order behavior.

## Requirements for `PL-MAN-SEAM-001` (PL03 target)

1. Input contract:
- Consume typed `ManagementParseOutput` only; no raw-text reparsing inside runtime adapters.
- Preserve datver and section/schedule identity metadata for diagnostics and parity tracing.

2. Branch/domain guards:
- `landuse=2` remains typed reject in current executable profile.
- `imngmt` domain remains `1..3`; unsupported options are typed domain failures.
- `resmgt` and `mgtopt` domains remain version-aware and explicit.

3. Projection strictness:
- Required PL runtime fields are mandatory; missing required fields fail typed.
- Non-finite runtime-critical projected values fail typed.
- No implicit defaulting of absent required fields.

4. Ordering contract handoff:
- Adapter outputs must be structured to preserve scheduler ordering obligations:
  - decomposition-first same-day management effect path,
  - growth/decomp transition coupling path,
  - landuse/imngmt branch routing.

5. Error taxonomy requirement:
- PL seam must define typed error codes (new PL seam family) with stable IDs and message sites.
- Error IDs must distinguish: missing required field, invalid option domain, non-finite projected value, illegal branch state.

6. Test obligations (PL03 + PL07):
- positive path: `.man` fixture projects complete PL runtime surface.
- negative paths: missing required controls, invalid schedule domains, invalid date domains, and illegal branch combinations each return typed errors.

## Evidence Links

- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:47`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:57`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:78`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:44`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:47`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:49`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:324`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:1082`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:1121`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:1204`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:881`
