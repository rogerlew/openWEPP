# Stage 3 snow accuracy–runtime experiment

Status: experimental delivery complete; evidence retained and independently reviewed.
Model, integration and release qualification: **HOLD**. No production activation.

## Result and recommendation

Ran: R0, numerical profiles P1/P2, conservative bulk profiles B01/B1/B5, coupling
profiles C600/C1800, and the P1_STEP ablation in the authentic integrated runner.
The final frozen matrix contains **69 executions: 15 complete, 54 failed, no
timeouts**. All profiles complete the positive-vegetation warm continuity day;
none completes the other six development regimes. R0/C600/B01 complete genuine
10- and 19-OFE continuity cases. Their continuous 21-day and distinct held-out
14-day cases all fail. Failed prefixes have no speedup or OFE-day rate.

On the one-OFE warm case, six balanced pairs give **C600 1.3865×** R0 speed
(descriptive paired-log t95 interval 1.3803–1.3927×) and **B01 1.5758×**
(1.5199–1.6338×). B01 saves 1.911 s/pair on average; C600 saves 1.397 s.
All 30 timing executions, including six warmups, pass. The slow B01 pair
(R0 6.177773498 s; B01 3.657443827 s) is retained. The intervals are descriptive,
not distribution-free guarantees. No extension to twelve pairs is justified by
the one-OFE result.

The best supported local tradeoff is **B01**, which gives up internal snow
stratification and detailed terminal iteration while retaining an explicit bulk
water/enthalpy budget and real shared soil exchange. C600 preserves the process
representation but gives up the old embedded time-error estimate and discovery
schedule. Neither is a qualified application model. Numerical tolerances alone
have little measured warm-case benefit. The one-OFE gain cannot be transferred
to watershed throughput. Six balanced 10-OFE pairs give B01 only **1.0478×**
(1.0458–1.0499×), or **4.56% lower wall time**. C600 gives **1.0061×**
(0.9618–1.0524×), with no resolved advantage. Candidates and settings stayed fixed.

Recommend the next fidelity experiment address a **reduced joint snow–canopy–soil
exchange formulation**, including the repeated physical work that remains after
terminal discovery is removed. Preserve the existing process equations where
affordable, but explicitly evaluate a simpler energy/moisture representation and
exchange schedule. The evidence does not support another tolerance-only or
cache/wrapper experiment as the answer to the scale requirement. Exact allocation
of the remaining runtime among batch physical subsystems is still unresolved;
work counts are not exclusive process timers.

The machine-readable [frontier](artifacts/frontier-final016.json) retains every
case, individual physical-unit differences, work, partial balances, failure and
provenance. Its qualified frontier is empty. **Available banded values passing
is not a complete comparative-band pass, physical accuracy, or release approval.**

## Compact result table

Times below are full-observer diagnostic runner seconds for the one-OFE warm day;
only C600/B01 additionally have the balanced minimal-observer estimates above.
Every row is HOLD. Development counts are complete/failed; P1_STEP is one targeted
ablation, not a seven-case screening family. Generated melt is not melt delivered.
Peak/timing, whole-system conservation and reference limitations apply to all rows.

| Policy | Precision/model detail given up | Development complete/failed | Warm runner s; paired speedup | Shared physical maps vs R0 | Available maximum errors vs R0 | Local snow audit; peak RSS | Reference/qualification |
| --- | --- | --- | --- | --- | --- | --- | --- |
| R0 | Strict retained reference | 1/6 | 5.0623; reference | 400 | Reference only | Pass; 79.6 MiB | Cold reference fails; HOLD |
| P1 | Dimensioned coupling/integration and LSE step precision | 1/6 | 5.0135; unpaired | 400, 0% reduction | Outlet +0.00248343 mm; soil <2e-10 K | Pass; 79.7 MiB | Missing full bands; HOLD |
| P2 | Larger numerical allowances | 1/6 | 4.9987; unpaired | 400, 0% | Same available errors as P1 | Pass; 80.0 MiB | Missing full bands; HOLD |
| P1_STEP | Only LSE temperature step becomes 0.001 K | 1/0 | 5.0346; unpaired | 400, 0% | Same available errors/work as P1/P2 | Pass; 79.9 MiB | Warm causal ablation only; HOLD |
| B01 | One bulk volume below 0.1 mm total retained SWE | 1/6 | 3.2466; 1.5758× paired | 72, 82% | Outlet +0.00247794 mm; soil 0.00605975 K; root 0.00002616 mm | Pass; 79.6 MiB | Layer detail/timing lost; HOLD |
| B1 | Same model below 1 mm | 1/6 | 3.2551; unpaired | 72, 82% | Same available warm errors as B01 | Pass; 80.3 MiB | Broader domain unvalidated; HOLD |
| B5 | Same model below 5 mm, stress profile | 1/6 | 3.2760; unpaired | 72, 82% | Same available warm errors; cold failure after B work | Pass on warm case; 79.9 MiB | Failure origin unclassified; HOLD |
| C600 | Alternative sequential endpoint scheme, target 600 s | 1/6 | 3.6402; 1.3865× paired | 144, 64% | Available daily physical values and observed generation bins equal R0 | Pass; 79.9 MiB | No old LTE guarantee; HOLD |
| C1800 | Same scheme, target 1800 s | 1/6 | 3.6635; unpaired | 144, 64% | Same available warm values as C600 | Pass; 78.9 MiB | Target is not accepted minimum; HOLD |

All available final daily SWE, generated melt, sublimation/deposition and frost
values have zero difference in the completed comparisons. This tiny-snow warm
case has initial snow only 1.6587631546105544e-7 kg/m² and cumulative snowfall
9.582796944851426e-7 kg/m². Its 50 mm rain produces consequential runoff, but it
is not a meaningful deep-snow melt or winter-frequency test.

## Meaningful topology and continuous coverage

Areas are authenticated sums of nonuniform OFE geometry, not N times the first
OFE's area. These full-observer scale diagnostics are individual executions,
not paired scale-speedup estimates. The earlier B01 10-OFE smoke was 12.3403 s;
the slower 14.4575 s execution below is retained.

| Topology | Area m² | R0 / C600 / B01 runner s | Batch physical requests R0 / C600 / B01 | B01 maximum soil / root / daily outlet difference | Peak RSS R0 / C600 / B01 |
| --- | ---: | --- | --- | --- | --- |
| 10 OFEs | 5,500 | 12.8537 / 13.1671 / 14.4575 | 64 / 48 / 36 | 0.0215888 K / 0.00004784 mm / 0.00126525 mm | 326.7 / 316.8 / 326.6 MiB |
| 19 OFEs | 19,000 | 24.1658 / 23.1745 / 23.4999 | 64 / 48 / 36 | 0.0215888 K / 0.00004784 mm / 0.00313708 mm | 600.8 / 616.3 / 599.7 MiB |

The additional 10-OFE balanced series completes all 30 executions, including six
warmups. Mean paired wall differences are −0.584522 s for B01 and −0.085395 s
for C600. The slow C600 pair 3 (R0 12.850028247 s; C600 13.940467019 s) is
retained. The paired-log t95 intervals above are descriptive. Twelve pairs are
not warranted: uncertainty about this small C benefit cannot change the empty
qualified frontier or the several-thousand-fold workload gap. This is a distinct
topology series, not an extension of the one-OFE sample.

B01 produces and consumes 360/684 bulk lane receipts on 10/19 OFEs; C600 records
480/912 endpoint integrations. Accepted snow outputs cover every lane. Scalar
`Carrier` counters are zero on this **batch entry path**, which calls the same
shared physical engine and completion path. They do not mean zero physical work.
Report scalar requests, batch requests and shared-engine maps separately; never
sum them. LSE solves are 232/200/176 for R0/C600/B01 on both topologies. LSE sweeps
are 8637/8569/8527 on 10 OFEs and 16116/16048/16015 on 19. Thus substantial
physical iteration remains despite removing many batch requests. Dedicated proof
of each native-dispatch subclass's participation is not available.

The 21-day cold/transition and 14-day held-out intermittent windows advance state
continuously through their attempted supports, with no daily reset, but none
finishes. No seasonal drift, seasonal RSS growth, successful long-window restart,
or complete-window accuracy is inferred from those attempts. The six other
development regimes cover snow-free inactive canopy, persistent cold snow,
gradual melt, rain-on-snow, repeated inactive-canopy snow, and thin-snow insulation.
Their failures remain explicit rather than being replaced with easier forcing.

## Seven owner questions

1. **How much work does snow/transition handling request?** On the warm one-OFE
   reference, 200 evaluator entries request 400 complete scalar physical maps.
   Four sequential adaptive parent spans account for roughly 3.3 of 5.1 runner
   seconds, including induced coupled and snow-free successor work. This is not
   exclusive snow arithmetic. There are 64 Half1-disappearance refinements and
   72 terminal-floor entries; only four accepted snow ledgers. Produced/discarded
   work must not be relabeled accepted work. Exclusive accepted-versus-discarded
   downstream CPU allocation is not independently measured.
2. **Which precision/schedule matters?** The direct endpoint bypasses the old
   embedded terminal LTE loop. Merely changing its 1e-8 relative constant is not
   evidence of treatment. Warm work is driven by discovery/disappearance and
   physical solves. Nanosecond timestamps still represent a 60-second physical
   floor. P1_STEP reproduces the P1/P2 warm work reduction, identifying the LSE
   temperature-step criterion as sufficient for that observed numerical effect.
   This does not identify a decisive tolerance for the failed winter cases.
3. **What do tolerance relaxations buy?** P1/P2 retain 400 physical maps; LSE
   sweeps fall 2873→2573 and leaf calls 438048→385248, with little unpaired wall
   benefit. P1/P2/P1_STEP have the same warm work and available physical errors.
   P1/P2 still fail cold cases and sometimes fail earlier than R0.
4. **What additional change comes from B/C?** B01 reduces maps to 72, sweeps to 1233
   and leaf calls to 163184; C reduces maps to 144, sweeps to 1593 and leaf calls
   to 223520. B changes representation and the accompanying exchange procedure,
   so it is not a perfectly independent representation-only factor. C600 and
   C1800 still hit the 60s accepted floor in this warm case; their gain largely
   removes discarded discovery work, not 600/1800s accepted timesteps.
5. **Do effects survive continuous evolution, vegetation, runoff and scale?**
   Positive vegetation and substantial rain-generated runoff execute successfully.
   Genuine 10/19-OFE treatments execute, but their cost differences are far smaller
   with a paired 4.56% B01 wall reduction and unresolved C600 advantage on 10 OFEs.
   Continuous winter/held-out validation fails; inactive
   canopy controls fail before establishing their intended comparison.
6. **Which policy meets each band?** None meets a complete band. All15 completed
   comparisons pass their available banded values; missing delivery, peak/timing,
   trajectory, restart and whole-system audit evidence prevents qualification.
   R0 agreement is not agreement with observations or established physical truth.
7. **What explains the remaining scale gap?** It is not resolved by tolerances
   alone or by removing scalar snow discovery. The multi-OFE physical sweeps
   largely remain. Exact exclusive downstream cost attribution is unresolved;
   a reduced joint physical formulation is the next modeling hypothesis, not a
   proven guarantee of application throughput.

## Method authority, frozen settings and selection

Owner authorization is retained in [authorization.md](artifacts/authorization.md).
Intake HEAD was the exact checkpoint f8783de68ae97529eeacf87d31db7df65f063453;
the primary checkout was clean. Primary runtime source, branch, unrelated work,
and historical FAIL/HOLD were preserved. Scoped local commits are authorized;
no push, branch switch, deployment or production activation is authorized.

Root/crate/science-contract/work-package instructions, implementation/science
obligations, testing strategy, numerical architecture and affected canonical
sections were read. Prospective experimental authority was added **before**
dependent implementation in SC-SNOWENERGY-001, SC-COUPLEDTIME-001 and the LSE
nonlinear-solve/soil-coupling sections. These amendments identify approximations
without replacing R0 authority or relaxing conservation/identity/domain guards.

- R0 has no newly introduced F, R or inactive-Jacobian treatment. It is a retained
  composition, not repository HEAD or observational truth.
- P1/P2 use absolute-plus-relative dimensioned numerical scales: relative 1e-4/1e-3,
  ice/liquid 1e-4/1e-2 kg/m², integrated energy 0.1/1 J/m² and temperature 0.001/0.01 K.
  Effective controls take the maximum with looser R0 controls; outer relative
  5e-3 and temperature 0.01 K are not tightened. Water-depth scales divide mass
  by 1000; heat/vapor flux scales use the actual support duration. Conservation
  constants are separate and unchanged; LSE residual admission remains finite
  and unchanged. P1_STEP changes only the LSE temperature-step scale to 0.001 K.
- B's state is total retained water W=I+L (including detached liquid), enthalpy
  H=Lf L−cold content and represented depth D. Thresholds are 0.1/1/5 mm with
  active exit 1.2×entry. The one-volume phase/enthalpy solve uses ci 2100,
  Lf 333600 and cw 4218 in SI units and a shared implicit snow/soil heat exchange
  with actual top-soil capacity/conductance. Rain/snow material enthalpy,
  vapor material enthalpy, refreezing and drainage are explicit. Layer→bulk
  conversion preserves W/H/D; conversion back preserves water/enthalpy with
  canonical layer geometry and an 8-epsilon relative depth arithmetic bound.
  Stratification is lost. Existing surface carrier radiation, albedo, roughness,
  vapor and canopy treatments are retained with the declared lagged bulk surface
  temperature. One declared bulk step is used per real published support;
  outer subdivisions keep B, never silently fall back to R0.
- C uses sequential endpoint exchanges targeted at 600/1800 s, bounded by forcing
  discontinuities, actual material events, finite/nonnegative/phase/soil guards
  and the 60 s floor. It does not coarsen rainfall/runoff wholesale and does not
  claim the old embedded LTE; those unavailable diagnostics are explicitly null.
- Startup identity is immutable: R0=None, P1=1, P2=2, C600=3, C1800=4,
  B01=5, B1=6, B5=7, P1_STEP=8. Experimental restart state retains identity and
  B hysteresis; foreign identities fail. An unsupported candidate never retries
  through another policy or publishes a successful partial day.

[cases.json](artifacts/cases.json) and [metrics.json](artifacts/metrics.json) were
frozen before policy outcomes. Tight/coarse allowances use absolute+relative×
abs(R0), not candidate-dependent denominators. Root storage uses a fixed 0.4 m
root zone with residual liquid/frozen water; available soil nodes are at 0.1,
0.3,…,1.1m. The consequential disappearance criterion is snow inventory ≤0.1 mm for 1800 s; runoff generation is
≥0.1mm/h for600s. No time warp, post-hoc threshold change or crossing-time
interpolation is used to manufacture an event pass.

Selection was nondominated cost/error then fastest complete-band policy. None
qualifies, so no combination is eligible and no interaction speedup is claimed.
C600/B01 were frozen exploratory extension representatives using the shorter
cadence/smaller trace-domain tie rule before held-out inspection. Build016 B
re-execution after inspected build015 held-out failures is a new exploratory
**implementation revision**, not confirmation of an unchanged held-out candidate.
Equations, thresholds, forcing, bands and selected representatives were not tuned.

## Physical measurement and hard limits

Daily WAT records are per OFE. WAT Q/QOFE are cumulative-length-normalized
publication diagnostics, not local generation depth; they are unbanded and never
area-averaged into runoff generation. PASS runvol×1000/actual total area supplies
daily outlet depth. Its hourly maximum is explicitly hourly and unbanded, not an
instantaneous event peak. SWE/frost/storage comparisons retain per-OFE identity.
Complete day×OFE keysets, stable positive areas, duplicate/foreign rows and source
hashes are checked. Final committed snapshots also expose frost and thaw depth
in metres. Only the final day is compared under the frozen frost/thaw allowance;
intermediate thaw trajectories remain unavailable.

WAT5 generation is integrated conservatively onto the frozen 300 s reporting grid
over common observed support only. Individual bin differences, cumulative signed
bias and observed grid maxima remain visible. A coarse source bin implies a
constant within-bin mean, not recovered fine physics. Missing tails are
unobserved, never zero-filled. These diagnostics do not establish a full-day
hydrograph or routed event-peak/timing band. Maximum observed generation-rate
changes are about 1.02e-5 mm/h for one-OFE B and 5.23e-6 mm/h on multiple OFEs.

Independent `fsum` reconstruction of primitive accepted snow operands verifies
local mass and energy residuals. Maximum residuals across completed comparisons
are approximately 1.73e-16 kg/m² and 5.51e-11 J/m². Accepted ledger counts are 4/40/76
for 1/10/19 OFEs. Summed residual magnitudes are explicitly unweighted local-OFE
quantities, not hillslope-wide budgets. Bulk receipts recompute their governing
step and bind equal/opposite snow/soil heat, beginning inventory, lane and support.
**Whole-system independent energy and native receiver closure remain unresolved.**

P1/P2's0.002483431 mm outlet increase independently reconciles with terminal
routing storage: outlet increases0.0002483431225443 m³ while end storage decreases
0.0002483431224566 m³ on100m²; source changes about8.7e-14m³. This validates metric
bookkeeping, not a causal trajectory explanation. Snow liquid allocation includes
rain and is not relabeled melt-derived delivery.

No observation-based calibration/accuracy is claimed. R0's failed cold windows
and lack of a verified finer-time trajectory limit reference adequacy. Local
independent balances/analytical tests do not cure that reference limitation.
Full workflow restart across trace/resolved/reappearance transitions and mixed
B/resolved batch acceptance are not executed successfully; serialization and
mixed-temperature projection tests cover narrower obligations.

## Workload budget

Recovered planning target:5000 hillslopes×10OFEs×100years×365.25days=
1,826,250,000 OFE-days; mean500µs CPU/OFE-day and550µs wall/OFE-day.
The CPU target gives182.625s per10-OFE century. The prior210s wall ceiling has
headroom above the200.8875s arithmetic product of550µs. Sixteen workers at an
assumed70% efficiency and10% orchestration/I/O yield a hypothetical24.9h campaign,
not measured parallel performance.

Completed scale diagnostics cost roughly 1.2–1.5 process CPU seconds/OFE-day in this warm
regime, thousands of times the mean target. A workload-mixture lower bound with
all other days costing zero allows only roughly0.04% such days before using the
CPU budget; this is a conditional illustration, not an estimated storm frequency.
No century runtime is extrapolated from the stress day, and failed cases have no
throughput rate. Memory evidence is one-day process RSS/high-water behavior;
failed long windows cannot establish seasonal memory stability.

## Validation and independent disposition

Ran on frozen build016:

| Check | Result and scope |
| --- | --- |
| Full workspace release compilation | PASS,7m51s; exact Cargo artifact selected and frozen |
| Rust formatting | PASS, all 41 changed Rust files |
| Full workspace nextest profile full | FAIL:4015passed,202failed,84skipped,532.274s; no fail-fast |
| Failure-set comparison | Same202 named failures as earlier candidate013; not proof of R0 inheritance |
| Fresh-process policy tests |15/15PASS: nine startup/restart identity cases, four actual conservation-poison cases, two cadence chronology/null-LTE cases |
| Wet/dry multilayer bulk projection |1/1PASS on actual code, scalar and batch projections; immutable source and W/H/D checked |
| Independent bulk analytical tests | Earlier12PASS plus explicitB receipt-poison/two-step inventory PASS; governing math source unchanged in final016 |
| Python evidence tests |17/17 PASS, including failure retention, nonuniform areas, foreign rows, grid partition invariance, batch counter basis and final-only thaw projection |
| Workspace doctest command | PASS; not a simulation workflow |
| Warnings-denied lint | FAIL: all-targets blocked by10 LSE test errors; library command reports1143 orchestrator errors, including introduced/unclassified findings; runner lint not independently passed |
| Full/minimal observation | Three diagnostic pairs PASS exact inputs/outputs/snapshot/parent/carrier/LSE/method work; minimal physical records absent |
| Warm one-OFE reproducibility | Exact outputs/state/work across17R0,11C600,11B01 executions, including observer pairs |
| Source/reproduction |707 candidate source hashes and701 retained R0 hashes match; final patch reconstructs all 41 changed files |

The full profile explicitly skips named empirical campaigns and ignored tests;
it is not blanket release qualification. Individual failures and JUnit are retained.
A0 independent conservation evidence is partial: local snow balances pass, but
complete receiver and energy closure remain unmet. A1 local invariants and typed
guards have partial evidence. Applicable A3 constitutive obligations are not fully
qualified. Prospective amendment review and R0 reference adequacy are separate
from those gate classifications; transfer and full restart remain unqualified. No conservation, custody, domain or chronology guard
was relaxed to pass a comparison. Coverage/CRAP is not an admission claim.
No TESTGATE or gate planner was dispatched, repaired or extended. Terminal
whitespace checks pass for authored files. Unmodified raw patch context lines
and the verbatim authorization trailing blank line trigger ordinary diff-check
warnings; their exact evidence bytes are preserved.

The two required independent reviewers dispositioned the method/evidence plan
before comparisons and verified accepted fixes. Modeling review accepted bulk
geometry, shared batch receipt use/all-lane join/non-event installation and
one-volume ephemeral projection; it found no additional concrete numerical bug
in final016. Evidence review independently replayed timing statistics and all 105
main process artifact identities, checked observer/reproducibility and corrected
its unsupported inference from zero scalar counts. Both verified the corrected
separate counter reporting. Evidence review also audited all 30 additional
10-OFE timing executions and exact output/state/work reproducibility across
15 R0, nine C600 and nine B01 scale executions. Their substantive qualification
reservations are retained above.

Final modeling reviewer (`model_review`) disposition: “Approve experimental
delivery once archive retention is complete.” No remaining concrete modeling or
reporting error was identified in the corrected record. Final evidence reviewer
(`evidence_review`) disposition: “Final evidence-delivery QA PASS (qualification
HOLD unchanged).” That reviewer independently verified all 7,608 archived file
bindings, 2,349 objects, archive hash/size and embedded index equality, inspected
restore/changed-file refusal evidence, and verified the receipt overwrite fix.
Both approvals concern the executed experiment, not model acceptance.

## Failures, corrections and terminal scope

| Retained attempt/finding | Correction or disposition |
| --- | --- |
| Initial thread-local observation missed worker calls | Shared observation capture corrected; missing counters never treated as nonparticipation proof |
| Early P/C/B compilation/diagnostic failures | Logs and source snapshots retained; final source is explicitly build016 |
| B cold vapor/enthalpy and redundant layer geometry | Corrected producer accounting/geometry; shared validators preserved |
| Global B-policy receiver participation changed resolved lanes | Bound participation to immutable beginning domain and authenticated actual B receipts; R0/B01/B1 resolved-path parity restored |
| Build015 B10/19 missing real carrier context | Build016 consumes one shared phase's per-lane B receipts and common finisher; no fresh per-lane physical solve; both topologies now complete |
| Non-event B batch discarded selected phase | Install actual B non-event endpoints with all-lane joint state; create a group only for real events |
| Generic multilayer trial projection conflicted with bulk equilibrium | One canonical equilibrium trial layer; original layers remain receipt authority; wet/dry scalar/batch guard passes |
| Multi-OFE WAT projection assumed one daily hillslope row | Per-OFE identity/area validation and separate PASS outlet projection; previous failed analysis retained |
| Scalar count zero misread as absent batch physics | Inference withdrawn; separate scalar/batch/shared-map fields and regression test |
| Cold R0/B01 dependent-output instability | Same60s support1200..1260s, work and zero B participation; baseline-path failure evidence |
| B5 parent-finalization source mismatch after65bulk steps | INTEGRATION_FAILURE, CAUSAL_ORIGIN_UNCLASSIFIED; same guard elsewhere underR0 does not prove inherited cause |
| Native positive retained-ice drain receiver incompatibility | Separate static interface limitation; no schema downgrade, fake zero-duration support, carry reset or fallback |

Terminal runtime diff is41 Rust files across LSE, hillslope orchestration,
persisted restart and runner, delivered as an isolated patch. Primary changes are
this package/locator and four narrow science-authority documents. All authorized
experiment families and frozen case extensions executed; evidence gaps and
failures prevent model/integration acceptance. Experimental execution, evidence
retention and the two required final reviews are complete. No model, integration
or release qualification is granted.

## Reproduction and retained identity

R0 is base e89befa4678eadec039b3e7f7fe0a176af8e9dc5 plus
[R0-composition.patch](artifacts/R0-composition.patch) and
[R0-untracked-source.tar.gz](artifacts/R0-untracked-source.tar.gz).
[R0-source.json](artifacts/R0-source.json) binds701 source/manifest/toolchain files.
Strict/common-observer neutrality is in [common-neutrality.json](artifacts/common-neutrality.json).
The primary review checkpoint is distinct from this composition.

Final source: [PBC-final-build016.patch](artifacts/PBC-final-build016.patch) and
[source hashes](artifacts/PBC-final-build016-source.json). Runner SHA256:
`68268f7583b298ce87dd1bcf29d6047a521de8dab16d704cfa2e4f2570889f4b`.
Cargo test artifact features include runner default/test-fixture-authority and
orchestrator default/persisted-restart-v1/restart-authority-evidence. The runner
is an actual integrated-runner test executable, not the standalone bulk oracle.
Runtime release sidecars truthfully say source commit unknown; composition is
bound separately. Retained binaries/source copies/build directories are local
scratch assets, not committed production executables.

Use a new source/target directory. Export the base with `git archive`, apply the
R0 composition patch with `patch -p1`, unpack its untracked-source archive, and
verify all R0 hashes. Apply the final016 patch. For authority checks, copy current
SC-SNOWENERGY-001.md, SC-COUPLEDTIME-001.md and the full current LSE root contract
and fragment directory into that source, replacing the old LSE monolith.
Do not copy another checkout's `.git` file or share its Cargo target directory.
From the reconstructed source, the build command is:

```sh
nix develop /workdir/openWEPP --command env \
  CARGO_TARGET_DIR=/tmp/openwepp-snow-reproduction-target \
  CARGO_PROFILE_RELEASE_LTO=false RUST_MIN_STACK=67108864 \
  cargo test --workspace --release --offline --no-run \
  --message-format=json-render-diagnostics > /tmp/snow-reproduction-build.log 2>&1
```

[freeze_executables.py](freeze_executables.py) selects exact compiler-artifact
executables, rejects overwrite and verifies copied bytes. [freeze_source.py](freeze_source.py)
binds the patch and binary. Run [run_matrix.py](run_matrix.py) once normally and
once with `--extension`; both freeze the complete job list before execution.
[series.py](series.py) takes `--candidate C600 --candidate B01 --source <source.json>`
and optional `--ofes 10`, with two warmups and six alternating pairs. The original
one-OFE script is retained separately because its hash belongs to that protocol.
[observer_impact.py](observer_impact.py) runs declared full/minimal diagnostic pairs.
[collect.py](collect.py) uses fresh processes, CPU0, explicit startup policy,
180s timeout, finalized exit/identity receipts and independent wall/CPU/RSS fields.
It does not silently retry. CPU0 was supported, governor powersave, with16available
CPUs; compiler and heavy jobs were outside measured windows.

Recompute physical projections/frontier with [summarize.py](summarize.py), which
validates exact matrix completeness and each artifact identity before analyzing.
Use repo `.venv/bin/python`; this machine required
`LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu` for standalone PyArrow imports outside
the Nix shell. Python scripts use only the admitted raw files; regenerate
projections after analyzer changes. Raw fixtures/outputs, build/test logs,
protocols, receipts, source proofs and superseded attempts are retained in
[raw-evidence-final016.tar.gz](artifacts/raw-evidence-final016.tar.gz), with its
[path/content index](artifacts/raw-evidence-final016-index.json). The archive is
31,678,445 bytes, binding 7,608 files and 270 fixture directories through 2,349
unique content objects. SHA256:
`9ab77154a777887084d339ef2728d85ac76766d3bc70d7982a2973c46e10f222`.
Full object verification and restoration to a fresh temporary root both passed;
every restored file hash matched, and a changed existing file was refused without
overwrite. ELF binaries and build/source-copy directories are excluded; source
patches, build commands and exact original executable identities are retained.

From the repository root, verify without writing:

```sh
.venv/bin/python docs/work-packages/20260910-stage3-snow-accuracy-runtime-001/restore_evidence.py \
  docs/work-packages/20260910-stage3-snow-accuracy-runtime-001/artifacts/raw-evidence-final016.tar.gz \
  --root / --verify-only
```

Omit `--verify-only` to restore original `/tmp` paths needed by the raw receipts;
changed existing files are never overwritten. Use `--root /tmp/snow-evidence-copy`
for an isolated copy instead; original absolute receipt paths then require the
corresponding prefix when inspecting that copy. The immutable raw archive includes
superseded analyzers; use this package's current scripts for final projections.
Balanced [one-OFE](artifacts/balanced-one-ofe-final016.json) and
[10-OFE](artifacts/balanced-10ofe-final016.json) summaries are also directly available.
