# Supplement: Verification of openWEPP's Daily Linear Groundwater Reservoir

*Version 1.0 draft — 2026-07-16*

This supplement gives the methods, operand lineage, evidence identities, and
reproduction procedure behind the [main report](index.md). It is intended
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
[study protocol](research-objects/study-protocol.md) and
[realization freeze](research-objects/realization-freeze.md) retain
that preregistration.

The declared groundwater source and consumer set contains
12 paths. The
[path-currency result](research-objects/assure05-path-currency.json) records
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
1 d.

The H2637 reconstruction deliberately rejects these substitutions:

- the latest runoff-event baseflow for terminal-day baseflow;
- lateral subsurface flow for groundwater recharge or baseflow;
- channel `cbase` for generated groundwater baseflow;
- inferred terminal storage when a timing-qualified produced value exists;
- surface-router flow for groundwater discharge; and
- producer-only state as proof that the watershed consumer read the value.

## S4. Independent two-day calculation

The public-safe
[input object](research-objects/two-day-recurrence-input.json) declares a
1000 m2 hillslope, initial storage depth of
0.010 m, coefficients
`kb = 0.10 d^-1` and `ks = 0.05 d^-1`, and recharge
of 2.0 m3 followed by
4.0 m3. The
[reproduction procedure](research-objects/reproduce_groundwater_report.py)
uses Python's standard library and does not import or call
openWEPP.

The day-one storage, baseflow, and deep seepage are
12.0 m3,
1.20 m3, and
0.60 m3. The corresponding day-two values are
14.2 m3,
1.42 m3, and
0.71 m3. The maximum binary64-versus-decimal
arithmetic residual is 1.776356839400250e-15 m3. Separately,
the Rust recurrence test passed its preregistered
1.0e-12 m3 assertion allowance. The strict
[two-day result](research-objects/two-day-recurrence.json) preserves the
unrounded values.

## S5. Executable domain and consumer checks

The fresh focused selection contains
7 tests. It covers the recurrence timing,
over-export rejection, multi-OFE recharge aggregation, contributing-area
threshold, missing coefficient authority, hillslope-pass publication, and the
watershed consumer. The writer/parser and hand-constructed consumer checks are
separate interface tests; no one fresh nonzero payload traversed the complete
CLI adapter between them. The strict
[focused-test result](research-objects/assure05-focused-tests.json) and
[execution evidence](research-objects/assure05-production-evidence.json)
retain the command, run identity, exact test names, outcome, and realization.

The negative over-export vector uses
`kb = 0.80 d^-1` and
`ks = 0.30 d^-1`. Missing groundwater-coefficient
authority and inconsistent exports must fail closed; no inferred parameter
default is admitted.

## S6. H2637 acquisition and reconstruction

H2637 covers 731 d, of which
610 d carried active surface-routing water,
and 19 OFEs. The accepted runner and test executable
were built in an isolated target directory from the frozen realization. The
produced manifest, HBP, and pass-Parquet identities were checked before their
operands were used. An earlier run whose reused binary sidecar did not bind the
frozen source was rejected before scientific disposition and is recorded in
the package execution evidence, not substituted into this report.

The first reconstructed identity is terminal pre-export storage:

```text
S_N = S_0 + sum(D) - [sum(Qb) - Qb_N] - [sum(Qs) - Qs_N]
```

Produced `S_N` is 126.01452784040274 m3 and the
independent reconstruction is
126.01452784044524 m3. Their signed residual is
-4.249045559845399e-11 m3, compared with an
allowance of 1.260145278404028e-07 m3.

The complete post-export identity is:

```text
S_N - Qb_N - Qs_N = S_0 + sum(D) - sum(Qb) - sum(Qs)
```

The post-export storage is 120.97394672678662 m3 and
the full-run reconstruction is
120.97394672682913 m3. Their signed residual is
-4.250466645316919e-11 m3, compared with an allowance of
1.209739467267866e-07 m3.

The same independent procedure reconstructed the active surface-routing
ledger from source 374420.251156 m3, routed outlet
371254.384601 m3, end-window storage
3165.866555 m3, and numerical clamp
4.98e-14 m3. Its residual is
3.32e-09 m3, relative residual is
8.87e-15 unitless, and allowance is
3.74e-04 m3. These surface operands test the
separation between generated groundwater and active surface routing; they are
not a second groundwater-performance statistic.

The [H2637 result](research-objects/h2637-ledger.json) retains all reported
operands and reconstructions. The retained
[manifest](research-objects/manifest.json),
[HBP](research-objects/H2637.hbp), and
[pass-Parquet output](research-objects/H2637.pass.parquet) are the
exact inputs accepted by the public reproduction procedure. The
[execution evidence](research-objects/assure05-production-evidence.json)
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
[exact execution prompt](research-objects/20260716-codex-execute-assure05_prompt.md) and
output identity are retained in the
[agent-assistance packet](research-objects/agent-assistance-packet.json). Hidden
model/runtime configuration is unavailable, so the packet supports review and
repeatability assessment rather than bitwise regeneration of prose.

Coding-agent review can find defects but cannot supply accountable scientific,
reproduction, publication-steward, or release approval. The current structured
attribution and lifecycle projections are:

**Authorship and accountability.** Draft authors: Codex (AI coding agent). Accountable report lead: Not yet assigned. Material producers: None recorded.


**Assurance status.** This report is `DRAFT`. Independent scientific, reproduction/publication, and assurance-steward approval remain pending; no approval lock exists. It does not authorize public export, vendoring, or an application-fitness determination.


## Revision Log

| Version | Date | Changes |
| --- | --- | --- |
| 1.0 draft | 2026-07-16 | Replaced the architecture fixture supplement with the preregistered ASSURE-05 methods, fresh-evidence lineage, independent reproduction procedure, claim boundaries, and explicit human-approval hold. |
