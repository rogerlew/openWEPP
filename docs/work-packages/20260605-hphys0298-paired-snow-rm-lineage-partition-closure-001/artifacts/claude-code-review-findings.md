# Claude Code Review Findings — HPHYS0298

Reviewer: Claude Code (independent review).
Verdict: **STRONG APPROVE.** The science path delivered a definitive,
reconstruction-grade, `file:line` root cause that answers "which model is
defective," refutes the prior acceptance hypothesis, and was sharpened by a dual
review that changed the verdict. No production patch, no compensation, honest
`HOLD`.
Evidence mode: static (ledger, contracts, reviews) + ran (verified ledger
values, observe-identity, no-production-patch).

## End-goal result (definitive)

For all nine H1/H7/H39 snow/`RM` windows: **`OPENWEPP-DEFECTIVE`, first divergent
cut-point `hourly-forcing`, first divergent symbol `hrsnow`** (`hrrain,hrsnow`
for H39 2013 days 97-112). Quantified from `paired-lineage-ledger.json`:

| symbol | baseline | openWEPP | delta | baseline src | openWEPP src |
|---|---:|---:|---:|---|---|
| `hrsnow` | 68.569 mm | 6.857 mm | 61.712 mm | `winter.for:412` | `mod.rs:4606` |
| `hrrain` | 12.506 mm | 14.115 mm | -1.609 mm | `winter.for:410` | `mod.rs:4530` |

openWEPP produces ~10% of the baseline hourly snowfall — a precipitation-phase
partition defect at the top of the winter forcing chain. Snowpack, melt, `RM`,
and storage residuals all inherit it. This satisfies acceptance criterion A
(root cause pinpointed by `file:line` in both models, quantified delta).

## Why this is the headline of the arc — the magic was foreclosed by evidence

HPHYS0296 classified 6 of these windows as `corrected-negative-melt-candidate`,
the bucket the correlational acceptance gate was poised to accept as "openWEPP
correct, baseline buggy." HPHYS0298 proves all six are **openWEPP-defective at
`hrsnow`**. Had the correlational acceptance path been taken (accept on "contains
material negative melt + internal closure"), six real openWEPP defects would have
been buried as "acceptable semantic divergence." The insistence on reconstruction
plus independent attribution over correlational acceptance (CLAUDE-0296-001 /
the A-F acceptance criteria) is what made this catchable. This is the
load-bearing evidence that the magic/science distinction mattered.

## The dual review ran and was decisive

After five consecutive packages with the dual-review gate off (0292-0296), review
ran here and **changed the root-cause verdict**:

- A-001/B-001: the classifier checked raw melt *before* hourly forcing and
  omitted the raw-snow comparison. As originally run, it would have
  mis-attributed the divergence to **melt** instead of upstream `hrsnow`. The fix
  reordered to forcing-first and reclassified all nine windows to
  `hourly-forcing`. **Without the review, this package would have shipped the
  wrong root cause.**
- A-002: removed zero-fill of missing trace fields into closure and the
  `wb13_rm_mm == 0` fallback; added `trace-gap` fail-closed handling.
- A-003/B-002: added per-symbol `source_provenance` (canonical/openWEPP symbol,
  unit, values, deltas, source path/line) to every ledger row.
- A-004/B-003: observe-identity initially omitted the instrumented observe-off
  lane; runner now executes pinned-release / instrumented-observe-off /
  instrumented-observe-on.

This is the strongest argument in the sequence for restoring the dual-review
gate: it just corrected a root-cause verdict that would otherwise have been wrong.

## Validity foundation (verified)

`baseline-observe-identity` runs three lanes — pinned release
(`wepp_260430_hill`), instrumented observe-off, instrumented observe-on — with
**byte-identical SHAs** per hillslope (H1/H7/H39). This proves the baseline
instrumentation is non-intrusive, which is the validity precondition for paired
comparison. There is a runnable, instrumentable pinned baseline, so true paired
root-causing (acceptance criterion B reconstruction) is now a capability, not
just contract-invariant checking.

## Findings

### CLAUDE-0298-001 [NOTE, criterion C] — Verdict rests on baseline `hrsnow` as porting authority
`OPENWEPP-DEFECTIVE` here means "openWEPP fails to replicate baseline `hrsnow`."
That is well-founded — `hrsnow` is upstream of the only known baseline defect in
this area (downstream negative-melt), so the baseline precipitation-phase
partition is the legitimate porting target and authority. The verdict should
state this explicitly (porting-fidelity defect against an un-impeached baseline
routine) rather than default to "openWEPP differs from baseline." Full
independent physical authority (e.g., the documented precip-phase threshold
equation) would make it airtight, but is not required to act here: the follow-on
fix is to replicate baseline `winter.for:410-412` partition behavior.

### CLAUDE-0298-002 [POSITIVE] — Discipline held end to end
No production physics change, no downstream WB13/17/18/19 compensation, nothing
accepted or excluded (all nine `OPENWEPP-DEFECTIVE`, suite stays open), `Q`
remains closed. The package produced a defect ledger that names which model is
wrong, where, and by how much — the intended deliverable of this thread.

### CLAUDE-0298-003 [reference, stale memory] — "No reference binary" note is outdated
A standing note that "no oracle/reference binary exists" is contradicted by this
package, which runs and instruments `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`.
Paired instrumented baseline observation is now an available capability and
should be recorded as such.

## Disposition under the A-F acceptance criteria

- A (mechanistic root cause, both models): **met** — `hrsnow`, `file:line`, both
  sides.
- B (reconstruction / controlled experiment): **met** — paired instrumented runs
  with byte-identical observe-identity.
- C (independent correctness authority): **partially met** — baseline taken as
  porting authority for `hrsnow` (sound; see CLAUDE-0298-001). No magic, because
  the verdict is `OPENWEPP-DEFECTIVE` (no acceptance claimed).
- D (per-window disposition): **met** — all nine `OPENWEPP-DEFECTIVE`, stay
  failing, fix the producer.
- E (scope discipline): **met** — per-window ledger.
- F (auditable re-tiering): **N/A** — nothing accepted.

## Bottom line

The arc converged. After a long sequence that chased melt magnitude, publication
lineage, and a near-miss with correlational acceptance, paired instrumented
observation cut to the root: openWEPP's hourly snow/rain partition (`hrsnow`,
`mod.rs:4606`) is the defect and the upstream source of the snow/`RM`/storage
residual. Next package: baseline-authoritative winter hourly snow/rain forcing
migration to replicate `winter.for:410-412`. This is what the correctness-
authority model was built to produce.
