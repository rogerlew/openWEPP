# PL04 Symbol Alias Expansion Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL02 requires canonical alias continuity for PL schedule, growth, and decomposition surfaces.
- Canonical WEPP symbols remain authoritative; boundary aliases are explicit projections.
- Legacy baseline declaration anchors confirm mixed scalar, `(mxcrop,mxplan)`, `(mxres,mxplan)`, and `(mxgraz,mxcrop,mxplan)` PL families.

Ran:
- Extended `openwepp-sim-contract` canonical registry with PL schedule/growth/decomposition canonical symbols and deterministic alias templates.
- Added forward and reverse alias resolution assertions and ambiguity guards in integration tests.

## Contracted Expansion Rules

1. Canonical symbol authority
- Canonical names are preserved (no replacement with boundary-only names).

2. Deterministic alias forms
- Scalar/OFE families: `{symbol}`, `ofe{ofe}_{symbol}`.
- Slot-indexed families: `{symbol}_{idx4}`, `ofe{ofe}_{symbol}_{idx4}`.

3. Template token policy
- Allowed tokens remain `{ofe}` and `{idx4}` only.
- Unsupported tokens are typed errors.

4. Reverse lookup determinism
- Exact alias mapping is resolved first.
- Template matching is allowed only when it maps to a single canonical symbol.
- Ambiguous reverse matches return typed `AmbiguousBoundaryAlias`.

## Expansion Scope Closed by PL04

- Schedule controls: `lanuse`, `nowcrp`, `itype`, `imngmt`, `tilseq`, `conseq`, `drseq`, `jdplt`, `jdharv`, `jdstop`, `resmgt`, `mgtopt`, `rw`, `gday`, `gend`.
- Growth state: `vdmt`, `tlive`, `cancov`, `canhgt`, `lai`, `rtmass`, `rtd`, `sumgdd`, `hia`, `vdmx`, `isenes`, `ncount`.
- Decomposition/residue state: `rmagt`, `rmogt`, `rilrm`, `rigrm`, `smrm`, `rtm`, `iresd`, `iroot`, `senvin`, `fenvin`, `benvin`.

## Evidence Links

- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-canonical-symbol-alias-requirements.md`
- `/workdir/wepp-forest_260430_baseline/src/ccrpprm.inc`
- `/workdir/wepp-forest_260430_baseline/src/ccrpvr1.inc`
- `/workdir/wepp-forest_260430_baseline/src/ccrpvr2.inc`
- `/workdir/wepp-forest_260430_baseline/src/ccrpout.inc`
- `/workdir/wepp-forest_260430_baseline/src/cdecvar.inc`
- `/workdir/wepp-forest_260430_baseline/src/cperen.inc`
- `/workdir/wepp-forest_260430_baseline/src/cperen1.inc`
- `/workdir/wepp-forest_260430_baseline/src/crinpt1.inc`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs`
- `/home/workdir/openWEPP/tests/integration/sim_contract_symbol_alias_registry.rs`
