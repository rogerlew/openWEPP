# Protected operand and output map

Static: prospective operand map retained. Ran: exploratory A public-output
reconstruction below; treatment scientific admission remains pending.

| Field | Operand/source | Units/basis | Independent comparison |
|---|---|---|---|
| Lane-D source | WAT5 hourly accepted source depths, associated WAT Area per OFE | mm and m2; sum depth/1000*area | compare manifest total_source_m3 and reject mm-as-m poison |
| Outlet | HBP 24 hourly volume bins; PASS runvol | m3, terminal outlet only | sum HBP bins, compare PASS and manifest total_routed_outlet_m3 |
| End storage | manifest total_end_window_storage_m3, accepted routing book | m3, complete routed topology | source + clamp - independently reconstructed HBP outlet - storage |
| Clamp | manifest total_clamp_m3 | m3, diagnostic custody correction | exact parity, unchanged no artificial normalization |
| Peak | maximum HBP hourly volume /3600; PASS peakro | m3/s, hourly reporting basis | compare both output surfaces |
| Topology/area | authored exact CompleteOwner seed and WAT Area | ordered OFE identity, m2 | exact count/order and area association |
| Day/support/trials | real committed qualification snapshot and adaptive telemetry | counts, actual source semantics | exact A/F/R except identified lower-level mechanism counts |
| Scientific output files | each emitted nonmanifest HBP/Parquet/output file | original serialization | SHA256 per path, byte exact |
| Run manifest | full raw JSON retained | field-defined units | explicit volatile-field allowlist only, to freeze from baseline schema before comparative admission |

Authority: existing SC-OFEROUTE-001/SC-WATBAL-001 routing closure, SC-SNOWENERGY-001
INV088/C056, SC-LANDSURFACEENERGY-001 INV164/C020. Preserve all inner mass/energy,
phase, liquid, soil and receipt independent reconstruction and forced-reference
tests required by those contracts. Output equality alone does not satisfy inner
process closure. Calibration/identifiability NOT_APPLICABLE: no process parameter,
empirical calibration or observation operator changes.

Runner reuses assert_real_lane_d_public_closure with existing source tolerance
1e-12 m * summed area and publication relative comparison 1e-9. These tests are
inherited authority checks, not new physics tolerances. Raw operands are emitted
so independent verifier reconstructs without reading producer counters.

## Actual A evidence and claim boundary

Read-only independent analysis by feed_forward_design and parent reconstruction
used `A-admission-01.log` exported operands. That log contains three exploratory
processes (first two lacked requested memory environment); it is not a single-PID
measurement. All scientific identities agree. Exclusive admission02 is required
before collector identity freeze. No exploratory timing enters comparative data.

WAT Area comes from `H83.wat.parquet/Area`; WAT5 depth comes from
`H83.wat-subhourly.parquet/hourly_authoritative_runoff_depth_mm`, deduplicated by
OFE/hour with bit-identical repeated-value validation. HBP bins come from the
parsed real HBP event; PASS runvol/peakro come from the produced Parquet. See
controlled_mechanism_experiments.rs:257 and stage3_runner_qualification.rs:2340.

| Independently recomputed quantity | A value |
|---|---:|
| sum WAT Area * sum hourly WAT5 / 1000 | 0.8488061229561459 m3 |
| Difference from manifest source | -1.887379141862766e-15 m3 |
| Source tolerance (1e-12 m * 100 m2) | 1e-10 m3 |
| sum HBP bins = PASS runvol = manifest outlet | 0.8471105124736579 m3 |
| Manifest end storage; clamp | 0.0016956104824910018 m3; 0 m3 |
| WAT5 source + clamp - HBP outlet - storage | -2.993699038666975e-15 m3 |
| max HBP bin /3600 = PASS peakro | 0.00022481503806426537 m3/s |

SC-OFEROUTE-001:473-496 binds router-internal/seam 1e-9 relative and assembled
1e-6*max-operand bounds, with explicit reconciled-storage tautology warning;
725-755 binds accepted 24-bin source and real consumption. Public comparisons
retain 1e-9*max(abs(left),abs(right),MIN_POSITIVE), not a new scale floor of one.
Storage/clamp remain producer-ledger operands, not independent mesh reconstruction.
Peak is the maximum hourly mean, not instantaneous discharge. This public source/
outlet check does not independently establish snow/energy/ET/subsurface closure;
those remain mandatory contract tests and complete-carrier/forced-complete oracles.

## Frozen manifest comparison design

`reproduction/admit_identity.py` enumerates only run-root paths, checksum-map path
keys, invocation timestamp, and five executable-provenance fields. Provenance
exceptions require exact arm-specific executable path/hash, actual sidecar path/
hash and checkout; they are separately retained and checked, not ignored. Every
other manifest field and nonmanifest output hash is exact, including signed zero,
apart from the separately reviewed exact construction-work leaf below.
`test_collectors.py` ran four offline tests: scientific/control/signed-zero poisons,
foreign provenance/path rejection, exploratory A identity agreement, and lawful
one-sided stencil enumeration. This tests the comparator, not treatment admission.

## Reviewed F construction-work leaf, before comparative timing

F-day-frame-attribution.md and dual GO in F-implementation-review-a.md /
F-day-frame-review-b.md bind exactly
`/direct_runtime_counters/day_frame_constructions`. This is a strict
mechanism-dependent telemetry qualification, not a volatile-field exemption.
The raw leaf remains in records and binds the exact admission rule. No raw
manifest is rewritten, and the containing counters object is never omitted.

| Actual admission | D constructions | P completed providers/carriers | N owner lanes | D-2*N*P |
|---|---:|---:|---:|---:|
| raw/A-admission-02.log | 1205 | 400 | 1 | 405 |
| raw/F-admission-01.log | 805 | 200 | 1 | 405 |

Each removed carrier execution avoids two existing hydrology adapters, each
seeding all N authenticated owner-frame lanes. D_A-D_F=400=2*(400-200).
The unchanged noncarrier residue405 establishes the bounded work relation;
it is not a physical balance or an accepted-day count. Other direct-runtime
counters remain exact, including commits0, compute operations10, phase entries26
and state mutations10. Detailed carrier/result/custody and all scientific
output/closure gates remain independently required.

reproduction/run_series.py qualifies this leaf only with balanced zero-error
Provider=Carrier counts, no dropped records, allowed integer N=1/10/19,
checked nonnegative arithmetic, and exact raw-leaf match. N=1 pins the observed
1205/400 and805/200 populations. N=10/19 requires independently matched fresh
admissions and equal actual residues under D-2*N*P, plus authenticated lane
cardinality and unchanged native branch/batch/invocation populations. No405
constant or future scale population is inferred. All other leaf/output parity
rules are unchanged. No comparative timing had run when this amendment was
added; source attribution and protocol review do not themselves admit a run.

Reproduction clarification: source kits now include15 compiler-read JSON
documents, with exact-base canonical adjuncts; see authority-input-reproduction.md.
Full root document/schema tests require a complete exact-base checkout, not
only the narrow runtime kit. Actual isolated F/R `.venv` symlinks both resolve
to `/workdir/openWEPP/.venv`; this local validation dependency is explicit and
is not a scientific-output normalization or claimed portable bundled venv.
