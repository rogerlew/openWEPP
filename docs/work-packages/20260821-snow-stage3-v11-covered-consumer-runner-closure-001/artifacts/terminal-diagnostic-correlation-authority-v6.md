# Terminal diagnostic correlation V6 compiler-indexed projection authority

Status: `CANDIDATE / NO SOURCE IMPLEMENTATION AUTHORITY`

Base: `6836e4cae6bab3a70767d64ab3e6a96e990745fe`. V5 remains an
unchanged governed HOLD. Last qualified physical implementation remains
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.

The machine-readable `terminal-diagnostic-correlation-v6-schema.json` is
primary authority. It defines a purpose-built diagnostic DTO graph, not a
recursive serialization of `CoveredCarrierPhaseResultV1` or its live graph.
Only selected leaves cross into DTOs. Every nonprimitive DTO field names a
non-null nested DTO schema and the generator proves root reachability equals
the complete declared DTO set.

## Compiler index

The pinned stable toolchain rejects rustdoc JSON because `-Z unstable-options`
is nightly-only, and the pinned shell has no rust-analyzer binary. V6 therefore
uses the same pinned rustdoc compiler's `--document-private-items` HTML index.
Rustdoc supplies actual module paths, private item declarations, field/variant
anchors and compiler-rendered types. Each binding records the rustdoc page and
anchor as compiler item ID, actual fully qualified path, exact field/method
path, compiler-window type hash, output DTO type ID, stage, owner/access plan
and page hash. Filename-derived module guesses are forbidden.

The generated compiler binding report includes expanded leaf bindings for all
three exact `TerminalState` fields, all fourteen exact `TerminalLedger` fields,
all 23 named stack snapshot members and every selected carrier top-level leaf.
Caller-local joint, clock and provider-call count are separately named DTO
locations. No lexical `last_*`, wildcard, `NamedBytes`, open map, or whole-
carrier encoding is authorized.

## Retained numerical semantics

`SelectedTerminalTrialEvidenceV6` contains exact beginning/ending terminal
state DTOs, the complete terminal ledger DTO, coupling selection and the
hydrology-complete ending-joint digest. `PairDecisionV6` contains exactly
COARSE, FINE_1 and FINE_2 selected trials with COARSE+FULL/RETRY,
FINE_1+HALF_1 and FINE_2+HALF_2 mappings. The five-component error order,
binary64 bits/finiteness, exact left-fold maximum and first-bitwise-equal winner
are retained.

The later floor admission is a separate `TrialAdmissionV6`: proposed support
and duration, required half duration, exact 600 ms minimum, explicit
`BELOW_CARRIER_DOMAIN`, narrow outcome witness and equal provider counts.
`ZeroTerminalIngressEvidenceV6` independently binds hydrology terminal-liquid
supply, WB14 terminal-liquid credit and surface-liquid terminal ingress.

The evidence-sufficiency matrix maps every final claim to exact DTO fields.
The computed resolution report is produced only after all closure, projection,
snapshot, selector, output and private-access checks succeed. Four negative
fixtures independently prove unresolved, ambiguous, stale and missing-private-
access failures are rejected.

## Gate

No package artifact is implementation authority. Two independent reviews of
one exact frozen V6 candidate are required. Either HOLD stops before source
edits. Two GO-to-evidence reviews would authorize only a later exact-file
implementation intent, not an owner helper, diagnostic seam, capture, matrix,
final v21 review, temporal/Batch/event/receiver/restart/runner/Child-3/cutover
work. `BelowCarrierDomain` remains authoritative.
