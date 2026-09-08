# Independent terminal review A

Static: root and work-package governance; reviewer procedure; package, active
kickoff/resume authorization, experiment protocol, worker handoff, treatment
results, compact results, gate results, source/build manifest, rearchitecture
handoff, final disposition, line-count record, exact current Git status/diff,
and named current `/tmp/openwepp-stage3-resume-20260908` evidence. Ran: read-only
`git status`, `git diff`, `rg`, `find`, `jq`, and `sha256sum`; no scientific,
gate, or performance workflow rerun.

Role/session: terminal reviewer A; configured/requested effort: high; effective:
UNOBSERVED. Reviewed identity: dirty uncommitted package cut observed 2026-09-08,
with eight tracked files modified and current campaign/source artifacts untracked;
no stable terminal manifest or revision identifies the complete cut. Assigned
scope: authority/correctness/precedence, admission and scientific claims, F/R
decisions, scale disposition, evidence freshness/expiry, and non-deferral.

## Independent checks

- Static: the frozen protocol hash still equals
  `40033ea2e536984ae5a7455e99509e7e760624b78cdb08ee347597fe2f5f115f`.
- Ran: parsed the retained scratch `A05F06-timing` records: 12 measured records
  per arm, all exit 0 with empty parse-error arrays and one executable hash per
  arm. The reported 22.44% F median gain and 0.776 CPU ratio agree with the
  compact result and scratch evidence at a plausibility/reconstruction level.
- Ran: inspected the A/R, six-pair memory, and three-process ten-run teardown
  scratch result/analysis files and their SHA-256 values. Their broad direction
  supports F faster, R slower, and no obvious peak-memory reversal. This does
  not cure package retention or identity defects below.
- Static: protocol section `Prospective F construction-work qualification`
  requires positive matched native work for N=10/19 and says sampling, timing,
  memory, and decision criteria are unchanged. The 10-OFE observation P=0 for
  both arms does not satisfy that scale qualification.
- Static: no calendar expiry is defined. Freshness is identity-bound. Therefore
  evidence does not expire merely with elapsed time, but it is stale or
  unreviewable when the controlling manifest/handoff does not bind the actual
  executable/source/input/evidence identities.
- Static: line-count evidence reports no nonexempt Rust file at or above 3000
  lines, but labels itself provisional and requires refresh against terminal
  manifests.

## Findings

### TRA-001 — HIGH — no stable, retained terminal evidence identity

Location: `artifacts/results.jsonl`, `artifacts/source-and-build-manifest.json`,
`artifacts/worker-handoff.md`, `artifacts/rearchitecture-handoff.md`, and
`artifacts/raw/`.

Evidence: the compact results cite raw timing only at `/tmp/...`; the primary
timing, memory, teardown, freeze, and analysis files inspected by this reviewer
remain outside the package. The source/build manifest still says
`active-premeasurement`, identifies A-source-04/F-source-05/R-source-05, marks A
admission in progress and R not run, and does not bind the measured A05/F06 and
R05 executable hashes or campaign evidence hashes. The worker handoff contains
a newly added terminal summary but retains a contradictory pre-measurement
section and obsolete binary identities. The rearchitecture header says F/R
measurements are pending and its closing paragraph says final classifications
remain pending. Consequently the claimed terminal cut has neither a consistent
current-state record nor durable primary evidence, and exact-cut freshness
cannot be established.

Correction: retain bounded primary raw/freeze/analysis evidence in the package
or a durable content-addressed location authorized by the package; bind final
source kit, executable, environment/input, series, and evidence hashes in a
terminal manifest; reconcile all controlling documents to that one identity;
then request focused re-review. Disposition: accepted correction required;
OPEN.

### TRA-002 — HIGH — mandatory F scale obligation is unmet, not passed

Location: `artifacts/experiment-protocol.md:145-155`,
`artifacts/treatment-results.md:8-18`, `artifacts/gate-results.md:146-152`, and
`artifacts/final-disposition.md:5-9`.

Evidence: F satisfies the standalone timing predicate and is reported without a
meaningful memory increase, so the protocol defines it as competitive. The
protocol requires three pairs at both 10 and 19 OFEs and additionally requires
fresh positive matched native-provider scale qualification. The sole 10-OFE
admission produced provider=carrier=0, so it rejected that fixture as evidence;
no three-pair scale campaign was run and 19 OFE is explicitly NOT RUN. Calling
results complete and the modeling decision complete silently treats a failed
current-scope required gate as a stopping rule. Repository non-deferral rules
require execution, a prospectively reviewed scope amendment, or HOLD with a
named blocker/defect-shaped follow-on. The present `EXECUTED HOLD pending review`
does not name scale as a package blocker and simultaneously calls the required
results complete.

Correction: supply authenticated 10/19-OFE fixtures that enter the matched
native provider branch and execute the frozen three-pair scale obligations, or
retain package HOLD with this specific unmet requirement and a named owner/link;
do not represent F scale as an admitted or completed result. The one-OFE
`USEFUL_ARCHITECTURE_BUILDING_BLOCK` signal may remain explicitly bounded to one
OFE, but is not scale-qualified. Disposition: OPEN.

### TRA-003 — HIGH — current evidence-harness gate is knowingly failing

Location: `artifacts/gate-results.md:139-145` and
`artifacts/reproduction/run_series.py`.

Evidence: the collector changed from one cwd to arm-specific cwd values after a
real provenance rejection. Gate results state the historical collector unit
tests now FAIL and call this an evidence-harness closure finding. No replacement
current-cut test evidence is named. This is a changed validation runner/evidence
boundary, hence critical under the testing strategy, and cannot be waived as a
fixture-replacement explanation. A syntax check does not prove arm selection,
identity binding, or rejection behavior.

Correction: update only the authorized fixture/binding as appropriate, run the
collector tests including negative swapped/stale-cwd and identity cases, retain
the results, and independently re-review the correction. Disposition: OPEN.

### TRA-004 — MEDIUM — decision language overstates scientific proof

Location: `artifacts/treatment-results.md:7-10` and
`artifacts/rearchitecture-handoff.md:74-78`.

Evidence: phrases such as `exact protected baseline`, `exact
outputs/closure/custody`, and “F proves ... can save 22.44%” are not bound in the
controlling result record to specific current admissions and retained evidence.
The compact results contain only timing summaries and one invalid-for-F scale
record. Full-workspace, broad Clippy, release golden, and collector tests remain
FAIL, while primary scientific evidence is referenced indirectly or from older
cuts. The performance result supports a measured one-OFE association for the
admitted experimental cut, not a generally scaled causal claim.

Correction: link each scientific/admission claim to the exact current-cut log
and identity, and phrase the performance conclusion as a one-OFE controlled
experimental result with failed scale qualification. Disposition: OPEN.

## Disposition and uncertainty

No accepted fix was available to recheck in this review. The reported F and R
timing directions are supported by the scratch records inspected; R=`REJECTED`
is a reasonable bounded engineering decision. F=`USEFUL_ARCHITECTURE_BUILDING_BLOCK`
is supportable only as a one-OFE, non-production, non-scale-qualified signal
after identity/retention reconciliation. Production HOLD is correct and must
remain.

Uncertainty: no heavy workflow was rerun; `/tmp` evidence is mutable and may
disappear; the worktree was dirty and lacked a terminal content manifest;
configured/effective model session metadata was unavailable. Review freshness
ends upon any substantive change to source kits, collector, protocol, results,
or controlling disposition artifacts.

Verdict: **BLOCKED**. Scientific/performance direction is plausible, but terminal
acceptance fails non-deferral, durable evidence identity/freshness, and current
collector validation. Focused independent re-review is required after TRA-001
through TRA-003 are corrected or truthfully held as named blockers.

## Focused re-review of accepted fixes — 2026-09-08 PDT

Static: inspected the newly durable raw series/freeze files, current
`run_series.py` and `test_collectors.py`, and current treatment, architecture,
finding-disposition, gate, source/build-manifest, handoff, and final-disposition
documents. Ran: `/workdir/openWEPP/.venv/bin/python -m unittest
test_collectors.py` from `artifacts/reproduction` (PASS 8/8); `sha256sum` on all
six retained controlling result streams and both timing freeze records; `cmp`
of each retained result stream against the source scratch stream (all exact).

- **TRA-003 FIXED and independently rechecked.** The current collector suite
  passes 8/8 and its fixture now binds the retained A05 admission/identity.
- **TRA-001 PARTIALLY FIXED, still HIGH/OPEN.** The six controlling result
  streams and two timing freeze records are now durable under `artifacts/raw/`
  and byte-identical to the reviewed scratch evidence. However,
  `source-and-build-manifest.json` still binds A-source-04/F-source-05/R-source-05,
  labels A admission in progress and R not run, and omits the measured A05/F06
  executable and retained evidence hashes. `worker-handoff.md` still presents
  pre-measurement status, obsolete executable/source identities, “No timing
  sample has been started,” and all series as NOT RUN before a contradictory
  terminal paragraph. Exact-cut identity/freshness therefore remains
  unestablished. Retention alone is not the requested terminal binding.
- **TRA-002 ACCEPTED LIMIT but not fixed; HIGH/OPEN.** Finding disposition
  truthfully records the 10-OFE P=0 rejection and 19-OFE NOT RUN, but does not
  provide the frozen competitive-candidate scale evidence or convert this
  unmet current-scope obligation into a specifically blocked package
  disposition with owner/link. Bounded one-OFE wording cannot itself satisfy
  the non-deferral rule.
- **TRA-004 not demonstrably narrowed; MEDIUM/OPEN.** `treatment-results.md`
  retains the challenged phrases (“exact protected baseline,” “exact
  outputs/closure/custody,” and “gain is decisive”), while
  `rearchitecture-handoff.md` still says F “proves” the 22.44% saving. The
  finding-disposition assertion that wording was narrowed is not supported by
  the inspected documents. Replace these with an explicitly identity-bound,
  one-OFE invocation-local differential claim and durable evidence links.

Focused re-review verdict: **BLOCKED**. Durable raw evidence and the collector
fix are accepted. TRA-001 terminal binding/controlling-state reconciliation,
TRA-002 scale non-deferral disposition, and TRA-004 claim narrowing remain
required before this reviewer can pass the corrected stable cut.

## Second focused re-review — 2026-09-08 PDT

Static: re-read the updated source/build manifest, treatment results,
rearchitecture handoff, final disposition, worker handoff, finding disposition,
compact results, package status, and retained result streams. Ran: independently
hashed all three named source manifests and extracted their source identities;
parsed both retained timing streams for executable identity, arm cardinality,
exit/validity posture; searched controlling documents for superseded source and
NOT-RUN statements. No performance or scientific workflow rerun.

- **TRA-002 is now correctly dispositioned as HOLD.** `final-disposition.md`
  names the exact blocker (the authenticated multi-OFE fixture does not enter
  the native provider branch), owner (next authorized Stage3
  architecture/prototype package), and trigger (contract-reviewed authentic
  multi-OFE fixture exercising that branch). It explicitly withholds standalone
  and scaling recommendation. This is a truthful non-complete HOLD under the
  non-deferral rule; it is not PASS, completion, or deferral-as-evidence.
- **TRA-004 FIXED for the decision claim.** The architecture handoff now states
  the observed saving as one-OFE-only and expressly rejects a multi-OFE
  causal/scaling inference. Treatment results bind output/closure/custody to the
  admitted one-OFE workload, report uncertainty and lifecycle ranges, retain the
  failed engineering return allowance, and disclaim long-run boundedness.
- **TRA-001 remains HIGH/OPEN despite substantial progress.** The manifest now
  exactly matches the three named source-manifest hashes and identities and
  binds F/R measured executable hashes. It binds A executable
  `43b64b...` for the A05/F06 comparison, but the retained A/R timing stream uses
  a distinct A executable `22cdcd...`; that second controlling baseline identity
  is absent from the manifest. More importantly, `worker-handoff.md`, which
  repository governance defines as the single authoritative continuation view,
  still declares ACTIVE pre-measurement status, A-source-04/F-source-05, obsolete
  executable identities, A admission outstanding/R queued, “No timing sample has
  been started,” and timing/memory/teardown/scale NOT RUN before its contradictory
  final paragraph. This is not a reconciled current-state handoff. The manifest
  also does not content-bind the durable result/freeze hashes, though those
  hashes are independently observable in the retained files.

Second focused verdict: **BLOCKED for closure; PASS for the bounded HOLD
disposition itself.** The package may truthfully remain `EXECUTED HOLD` with F
limited to a one-OFE architecture signal and R rejected. It must not be marked
COMPLETE. Reconcile the authoritative handoff and bind both controlling A
executables (plus durable evidence identities) before terminal verification or
any stronger closure claim.
