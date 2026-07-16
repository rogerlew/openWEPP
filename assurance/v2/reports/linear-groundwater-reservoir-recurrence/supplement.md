# Supplement: Verification of openWEPP's Daily Linear Groundwater Reservoir

*Version 1.0 draft — 2026-07-16*

This supplement gives the methods, operand lineage, evidence identities, and
reproduction procedure behind the {{link:report|main report}}. It is intended
for scientific reviewers and researchers who want to inspect or reproduce the
reported software-verification results. It does not extend the claim envelope
of the main report.

## S1. Study question and evidence classes

The study asks whether the frozen openWEPP realization correctly implements
the authorized daily linear groundwater recurrence and transfers the generated
volumes through the named production consumer path. Four evidence classes are
kept distinct:

1. formulation traceability to `SC-GWBASEFLOW-001` and Srivastava et al.
   (2013);
2. a Rust assertion test plus independent arithmetic for a predeclared two-day
   case;
3. separate executable domain, serialization/parser, and consumer-contract
   checks; and
4. independent conservation reconstruction from a fresh H2637 production run.

The Priest River publication is prior empirical evidence for a related coupled
formulation. Its calibration and performance statistics are not openWEPP
results. H2637 is production integration evidence, not an observational
validation dataset.

## S2. Frozen realization and protocol

The assessed source realization is Git commit
`01ed70550a4e371e99afe35c4bdd4d9b667e812c`. Before result acquisition, the
ASSURE-05 protocol fixed the recurrence, operation order, input vector,
allowances, H2637 fixture, required operands, and rejected aliases. The
{{link:research-object:GW-OBJECT-STUDY-PROTOCOL|study protocol}} and
{{link:research-object:GW-OBJECT-REALIZATION-FREEZE|realization freeze}} retain
that preregistration.

The declared groundwater source and consumer set contains
{{quantity:GW-VALUE-PATH-COUNT}}. The
{{link:research-object:GW-OBJECT-PATH-CURRENCY|path-currency result}} records
the exact count. The freeze artifact binds every path by content digest and
records that none changed between the earlier integrated evidence realization
and ASSURE-05 intake.

## S3. Authorized recurrence and operand timing

For day `i`, the tested recurrence is:

```text
S_i  = S_(i-1) + D_i - Qb_(i-1) - Qs_(i-1)
Qb_i = kb S_i
Qs_i = ks S_i
```

`S_i` is accepted pre-export storage in cubic meters; `D_i`, `Qb_i`, and `Qs_i`
are daily-integrated cubic-meter recharge and export volumes. The coefficient
units are inverse days and the authorized interval is
{{quantity:GW-VALUE-INTERVAL}}.

The H2637 reconstruction deliberately rejects these substitutions:

- the latest runoff-event baseflow for terminal-day baseflow;
- lateral subsurface flow for groundwater recharge or baseflow;
- channel `cbase` for generated groundwater baseflow;
- inferred terminal storage when a timing-qualified produced value exists;
- surface-router flow for groundwater discharge; and
- producer-only state as proof that the watershed consumer read the value.

## S4. Independent two-day calculation

The public-safe
{{link:research-object:GW-OBJECT-TWO-DAY-INPUT|input object}} declares a
{{quantity:GW-VALUE-AREA}} hillslope, initial storage depth of
{{quantity:GW-VALUE-INITIAL-STORAGE-DEPTH}}, coefficients
`kb = {{quantity:GW-VALUE-KB}}` and `ks = {{quantity:GW-VALUE-KS}}`, and recharge
of {{quantity:GW-VALUE-DAY1-RECHARGE}} followed by
{{quantity:GW-VALUE-DAY2-RECHARGE}}. The
{{link:research-object:GW-OBJECT-REPRODUCTION-PROCEDURE|reproduction procedure}}
uses Python's standard library and does not import or call
openWEPP.

The day-one storage, baseflow, and deep seepage are
{{quantity:GW-VALUE-DAY1-STORAGE}},
{{quantity:GW-VALUE-DAY1-BASEFLOW}}, and
{{quantity:GW-VALUE-DAY1-DEEP-SEEPAGE}}. The corresponding day-two values are
{{quantity:GW-VALUE-DAY2-STORAGE}},
{{quantity:GW-VALUE-DAY2-BASEFLOW}}, and
{{quantity:GW-VALUE-DAY2-DEEP-SEEPAGE}}. The maximum binary64-versus-decimal
arithmetic residual is {{quantity:GW-VALUE-MAX-RESIDUAL-EXACT}}. Separately,
the Rust recurrence test passed its preregistered
{{quantity:GW-VALUE-TWO-DAY-ALLOWANCE}} assertion allowance. The strict
{{link:research-object:GW-OBJECT-TWO-DAY|two-day result}} preserves the
unrounded values.

## S5. Executable domain and consumer checks

The fresh focused selection contains
{{quantity:GW-VALUE-FOCUSED-TEST-COUNT}}. It covers the recurrence timing,
over-export rejection, multi-OFE recharge aggregation, contributing-area
threshold, missing coefficient authority, hillslope-pass publication, and the
watershed consumer. The writer/parser and hand-constructed consumer checks are
separate interface tests; no one fresh nonzero payload traversed the complete
CLI adapter between them. The strict
{{link:research-object:GW-OBJECT-FOCUSED-TESTS|focused-test result}} and
{{link:research-object:GW-OBJECT-PRODUCTION-EVIDENCE|execution evidence}}
retain the command, run identity, exact test names, outcome, and realization.

The negative over-export vector uses
`kb = {{quantity:GW-VALUE-GUARD-KB}}` and
`ks = {{quantity:GW-VALUE-GUARD-KS}}`. Missing groundwater-coefficient
authority and inconsistent exports must fail closed; no inferred parameter
default is admitted.

## S6. H2637 acquisition and reconstruction

H2637 covers {{quantity:GW-VALUE-H2637-DURATION}}, of which
{{quantity:GW-VALUE-H2637-DAYS-ROUTED}} carried active surface-routing water,
and {{quantity:GW-VALUE-H2637-OFE-COUNT}}. The accepted runner and test executable
were built in an isolated target directory from the frozen realization. The
produced manifest, HBP, and pass-Parquet identities were checked before their
operands were used. An earlier run whose reused binary sidecar did not bind the
frozen source was rejected before scientific disposition and is recorded in
the package execution evidence, not substituted into this report.

The first reconstructed identity is terminal pre-export storage:

```text
S_N = S_0 + sum(D) - [sum(Qb) - Qb_N] - [sum(Qs) - Qs_N]
```

Produced `S_N` is {{quantity:GW-VALUE-H2637-TERMINAL-STORAGE}} and the
independent reconstruction is
{{quantity:GW-VALUE-H2637-RECURRENCE-RECONSTRUCTED}}. Their signed residual is
{{quantity:GW-VALUE-H2637-RECURRENCE-RESIDUAL-EXACT}}, compared with an
allowance of {{quantity:GW-VALUE-H2637-RECURRENCE-ALLOWANCE-EXACT}}.

The complete post-export identity is:

```text
S_N - Qb_N - Qs_N = S_0 + sum(D) - sum(Qb) - sum(Qs)
```

The post-export storage is {{quantity:GW-VALUE-H2637-POST-EXPORT-STORAGE}} and
the full-run reconstruction is
{{quantity:GW-VALUE-H2637-FULL-RUN-STORAGE}}. Their signed residual is
{{quantity:GW-VALUE-H2637-POST-RESIDUAL-EXACT}}, compared with an allowance of
{{quantity:GW-VALUE-H2637-POST-ALLOWANCE-EXACT}}.

The same independent procedure reconstructed the active surface-routing
ledger from source {{quantity:GW-VALUE-H2637-SURFACE-SOURCE}}, routed outlet
{{quantity:GW-VALUE-H2637-SURFACE-OUTLET}}, end-window storage
{{quantity:GW-VALUE-H2637-SURFACE-END-STORAGE}}, and numerical clamp
{{quantity:GW-VALUE-H2637-SURFACE-CLAMP}}. Its residual is
{{quantity:GW-VALUE-H2637-SURFACE-RESIDUAL}}, relative residual is
{{quantity:GW-VALUE-H2637-SURFACE-RELATIVE}}, and allowance is
{{quantity:GW-VALUE-H2637-SURFACE-ALLOWANCE}}. These surface operands test the
separation between generated groundwater and active surface routing; they are
not a second groundwater-performance statistic.

The {{link:research-object:GW-OBJECT-H2637|H2637 result}} retains all reported
operands and reconstructions. The retained
{{link:research-object:GW-OBJECT-H2637-MANIFEST|manifest}},
{{link:research-object:GW-OBJECT-H2637-HBP|HBP}}, and
{{link:research-object:GW-OBJECT-H2637-PARQUET|pass-Parquet output}} are the
exact inputs accepted by the public reproduction procedure. The
{{link:research-object:GW-OBJECT-PRODUCTION-EVIDENCE|execution evidence}}
binds them to produced-file hashes and executable provenance.

## S7. Reproduction

From the repository root, reproduce the analytical result with:

```console
.venv/bin/python -B assurance/v2/reports/linear-groundwater-reservoir-recurrence/procedures/reproduce_groundwater_report.py analytical --input assurance/v2/reports/linear-groundwater-reservoir-recurrence/inputs/two-day-recurrence-input.json
```

Compare canonical JSON output with the staged research object. From the staged
version directory, reconstruct H2637 with:

```console
python3 -B research-objects/reproduce_groundwater_report.py h2637 --manifest research-objects/manifest.json --hbp research-objects/H2637.hbp --parquet research-objects/H2637.pass.parquet
```

The procedure first checks both produced-file hashes against the manifest and
then emits canonical JSON to standard output. It does not modify report
sources.

The deterministic assurance builder resolves typed quantities, tables,
figures, citations, and research-object links from `report.yaml`. Its external
staging root must first contain the referenced model-science narrative:

```console
stage="$(mktemp -d)"
mkdir -p "$stage/usersum"
cp usersum/hillslope-hydrology-and-sediment-physics.md "$stage/usersum/"
cargo run --quiet -p openwepp-assurance -- build --report linear-groundwater-reservoir-recurrence --staging-root "$stage"
cargo run --quiet -p openwepp-assurance -- check --report linear-groundwater-reservoir-recurrence --staging-root "$stage"
```

Building the same current source into unrelated staging roots must produce
byte-identical trees. Publication into `usersum` is a separate,
human-authorized lifecycle operation.

## S8. Claim-to-evidence map

| Claim | Primary evidence | Boundary |
| --- | --- | --- |
| Authorized recurrence and domain | `SC-GWBASEFLOW-001`; Srivastava et al. (2013) | Daily linear formulation only |
| Operation order and analytical values | Input, independent procedure, strict two-day result, focused tests | Synthetic software-verification case |
| Domain and fail-closed behavior | Negative guard and missing-authority tests | Tested invalid cases, not parameter plausibility |
| H2637 ledger closure | Fresh produced manifest and files, independent procedure, strict result | One deterministic fixture; no observations |
| Groundwater interface continuity | Production HBP writer/parser and separate watershed-consumer check | Adjacent interfaces verified; complete CLI adapter traversal remains open |
| Surface-router exclusion | Static source proof and independently closed surface ledger | Named active source builder |

## S9. Authorship, review, and publication boundary

Codex prepared this draft and its mechanical analysis procedure under the
ASSURE-05 protocol. The
{{link:research-object:GW-OBJECT-EXECUTION-PROMPT|exact execution prompt}} and
output identity are retained in the
{{link:research-object:GW-OBJECT-AGENT-PACKET|agent-assistance packet}}. Hidden
model/runtime configuration is unavailable, so the packet supports review and
repeatability assessment rather than bitwise regeneration of prose.

Coding-agent review can find defects but cannot supply accountable scientific,
reproduction, publication-steward, or release approval. Until named human
principals approve the exact locked source, the report remains `DRAFT`, export
and vendoring remain unauthorized, and the protected public `usersum` surface
must remain unchanged.

## Revision Log

| Version | Date | Changes |
| --- | --- | --- |
| 1.0 draft | 2026-07-16 | Replaced the architecture fixture supplement with the preregistered ASSURE-05 methods, fresh-evidence lineage, independent reproduction procedure, claim boundaries, and explicit human-approval hold. |
