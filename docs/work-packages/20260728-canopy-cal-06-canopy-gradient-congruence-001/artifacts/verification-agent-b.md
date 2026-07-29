# Terminal Verification B

Status: `PASS`

Evidence class: `Ran + Static`

Verification scope: exact staged CAL-06 results, tools, figures, reviews,
finding disposition, write set, and proposed final disposition. This
verification was performed independently of Verification A's calculations.

## Commands and results

Ran:

```text
.venv/bin/python \
  docs/work-packages/20260728-canopy-cal-06-canopy-gradient-congruence-001/tools/validate.py
```

Result:

```text
PASS: 261 runs; 259 forest/member executions; 2 open controls; 95,526
climatology rows; 783 observation scores; 13 verdict cells; 6 SVG figures
```

Ran an independent read-only Python calculation over the retained CSV, JSON,
and SVG objects. Result:

```text
runs=261; forest/member=259; open controls=2
execution-manifest rows=261
all seven forest lanes retain the identical 37-member set
period operands=24,012: ALL=261, calendar-year=11,745,
water-year=12,006
Marcell conifer winter-cover min/median/max =
0.9391899373747821 / 0.9391899373747821 / 0.9391899373747821
published ensemble summary is identical
Marcell observation matches=31,542
Harvard deciduous bulk-density matches/member=425..429
Harvard open bulk-density matches=400
all 259 forest runs retain both predictive sources as NULL_AUTHORITY_MISSING
all 261 erosion outputs remain NULL_NOT_EMITTED
all five downstream cells are NOT_EVALUATED / NOT_ADVANCED
cal06-snow-response.svg SHA-256 =
7ed285a4a171e9b9d74e15a6e24a69149c9812dc4130b95b0685cb3fd2bdab9c
all four embedded snow-figure source-table bindings rehash exactly
```

Ran:

```text
sha256sum \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/accepted-calibration-ensemble.csv
```

Result:

```text
83e749a3961604e4592f2a2217db30965c8bbb59f4752d0ff6d85fbac61fd986
```

The source object contains 37 data rows and matches
`result-manifest.json`.

Ran:

```text
git diff --check
markdown-doc lint --path \
  docs/work-packages/20260728-canopy-cal-06-canopy-gradient-congruence-001
```

Result: diff check passed; package Markdown validated 24 files with zero
errors and zero warnings.

## Independent determinations

1. `PASS — inventory and ensemble retention.` Both the execution manifest and
   result table contain exactly 261 passing runs: 259 forest/member
   executions and two single open-control executions. Each of the seven
   forest lanes retains the same 37 CAL-04B member identities. The accepted
   ensemble SHA-256 matches its CAL-04B authority object; no best-member
   selection or replacement occurred.

2. `PASS — retained reconstruction operands.` The 24,012 data rows in
   `run-period-operands.csv` equal 92 rows for each run: one all-period, 45
   calendar-year, and 46 water-year rows. The terminal validator reconstructs
   every run summary, all nine ensemble summaries, and all 13 nonzero
   observation-summary groups. My separate Marcell-conifer winter-cover
   calculation exactly matches the published ensemble minimum, median, and
   maximum.

3. `PASS — Harvard observation semantics.` Harvard comparison uses HF237-01
   daily bulk density only. Deciduous members retain 425--429 exact-date
   matches and the open control retains 400. Vertical density profiles remain
   `NOT_EVALUATED_SCALE_MISMATCH`. Bound Harvard SWE remains
   `INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION` with zero matches, and the
   unbound mixed/hemlock comparison remains `NOT_EVALUATED`. No source unit
   was silently relabeled.

4. `PASS — cell counts, nulls, and advancement.` The Marcell-only snow cell
   independently totals exactly 31,542 metric/member matches. Needle and
   fine-woody predictive sources remain `NULL_AUTHORITY_MISSING` for all 259
   forest runs. Erosion output is `NULL_NOT_EMITTED` for all 261 runs.
   Residue, frost, ET, runoff, and erosion cells are explicitly
   `NOT_EVALUATED / NOT_ADVANCED`; none is converted to zero or promoted by
   downstream compensation.

5. `PASS — figure identity and accessibility.` The terminal validator parses
   all six SVGs, checks their accessible title/description metadata, and
   verifies every declared source-table digest. My independent
   `cal06-snow-response.svg` check reproduces its file digest and all four
   embedded table bindings.

6. `PASS — reviews, findings, and exact write set.` Both independent terminal
   reviews and Verification A are `PASS`. Every execution and review finding
   is accepted and corrected; none remains undispositioned. The kickoff prompt
   is archived. The CAL-06 changes are confined to this package,
   `docs/work-packages/README.md`, the canopy roadmap, and the fulfilled
   CAL-06 figure contract. No production Rust, canonical contract, fixture,
   observation, or public schema changed. The unrelated gate-planner audit is
   explicitly excluded. No Rust line-count gate applies; the five current
   package-local Python tool counts are reconciled.

7. `PASS — closure readiness.` Terminal validation, dual review, finding
   disposition, dual verification, focused tests, runner tests, deterministic
   rendering, figure inspection, Markdown, line-count, and diff-hygiene gates
   are passed. The proposed disposition matches the evidence:

   ```text
   COMPLETE / BOUNDED GRADIENT CHARACTERIZATION /
   DOWNSTREAM ADVANCEMENT WITHHELD
   ```

## Verification disposition

`PASS`. CAL-06 is ready for the mechanical final status transition. Its
result supports bounded within-model canopy-gradient characterization and the
CAL-07 handoff only. It does not establish CAL-04B transferability, empirical
canopy-amplitude validity, predictive needle/fine-woody authority,
residue/frost adequacy, downstream advancement, Southern Hemisphere
robustness, or assurance publication.

## Plot-only presentation follow-on re-verification B

Status: `PASS`

Evidence class: `Ran + Static`

This section independently re-verifies the user-directed plot/sidecar
follow-on and both presentation re-reviews. It supersedes only the original
figure-identity details above; the scientific verification and bounded final
disposition remain unchanged.

Ran the current terminal validator:

```text
.venv/bin/python \
  docs/work-packages/20260728-canopy-cal-06-canopy-gradient-congruence-001/tools/validate.py
```

Result:

```text
PASS: 261 runs; 259 forest/member executions; 2 open controls; 95,526
climatology rows; 783 observation scores; 13 verdict cells; 6 SVG figures; 6
Markdown sidecars
```

I copied the renderer and its three input tables into a clean temporary
package root and reran it there. All six generated SVGs matched the canonical
files byte-for-byte. Their current SHA-256 identities are:

```text
fb1b17fd375a447446c99748bf56213a3a3c461bfcb7625e82072cf5da12cf80  cal06-canopy-chronology.svg
5997213bb954863465caf5e77a537eafd9cf65a7eac690986e829797c1b202a4  cal06-congruence-verdict-matrix.svg
250156290ffe27a13f661a77b7640b3734433b15526111e3a7f4a9ece3d47b08  cal06-downstream-consequences.svg
0382387b6635bd95ee55f54dc9522cc13754a57a3375636acf409b598ca723c8  cal06-litter-residue-frost.svg
40757569f3f6282ea423f1ff7c8a7e6624dbb68edfbabcd9c5074e713a851f68  cal06-seasonal-ordering-amplitude.svg
5ac1f9e3fa13913c7aff76db6267c3dcca67bb3ec18329c1a2371fd2c7c38264  cal06-snow-response.svg
```

Independent XML inspection confirms that visible SVG text is limited to
legends, axes, units, ticks, panel labels, plotted categories, and the verdict
matrix's categorical cell/status/advancement values. Captions, interpretation,
source-completeness prose, observation-residual narrative, and gated
consequence prose are absent. Each SVG retains `role="img"`,
`aria-labelledby="title desc"`, nonvisual accessible title/description
metadata, and an exact digest binding to its plotted source table.

The figure directory contains exactly six SVGs and six same-basename Markdown
sidecars. All local sidecar links resolve. The sidecars carry captions,
ancillary information, source links, units and scope, and preserve the
required scientific boundaries: Harvard SWE
`INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION`, Harvard profile
`NOT_EVALUATED_SCALE_MISMATCH`, the unbound mixed/hemlock comparison,
predictive-source `NULL_AUTHORITY_MISSING`, gated `NOT_ADVANCED`, erosion
`NULL_NOT_EMITTED`, and the distinction between null and measured zero.

The five manifest-bound scientific outputs retain their exact declared byte
counts and SHA-256 identities. The validator independently reconstructs the
run, summary, score, and verdict inventories, so the presentation follow-on
does not alter the scientific tables, member ensemble, operator exclusions,
null semantics, advancement gates, or package claim. Both follow-on
scientific/code re-reviews are `PASS`, and the accepted presentation finding
is fully dispositioned.

The scoped repository state remains confined to this package,
`docs/work-packages/README.md`, the canopy assurance roadmap, and the CAL-06
figure contract. No production Rust, canonical science contract, fixture,
observation, or public schema is in the CAL-06 write set. The unrelated
gate-planner audit remains excluded.

Follow-on Verification B is `PASS`. No determinism, presentation,
accessibility, source-binding, sidecar-semantics, scientific-integrity,
write-set, or disposition blocker remains. CAL-06 is ready for the mechanical
final status transition with:

```text
COMPLETE / BOUNDED GRADIENT CHARACTERIZATION /
DOWNSTREAM ADVANCEMENT WITHHELD
```

## Harvard downstream legend-correction re-verification B

Status: `PASS`

Evidence class: `Ran + Static`

The retained Harvard lane inventory is exactly `open`, `deciduous`, and
`mixed`; both the execution manifest and daily climatology contain no Harvard
conifer lane. Independent SVG text extraction confirms that the downstream
legend contains those three available strata and no `conifer`. The paired
sidecar states the same availability boundary without changing the
model-response-only, `NOT_ADVANCED`, or `NULL_NOT_EMITTED` semantics.

A clean temporary-root render reproduced the corrected downstream SVG
byte-for-byte at SHA-256
`6303080140c882d2bad216aafe8b0da1b506ca2a981dbd18774e1e5b2d995157`.
This digest supersedes only the downstream digest in the preceding follow-on
section; all other figure digests are unchanged. The current validator passes
the full package. Temporary fault injections independently proved its
regression guard: adding `conifer` failed with
`downstream legend advertises unavailable Harvard conifer lane`, and removing
`mixed` failed with `downstream legend is missing plotted Harvard mixed lane`.

All five manifest-bound science outputs retain their exact byte counts and
SHA-256 identities, and all six SVGs retain accessibility and source-binding
metadata. Legend-correction re-verification B is `PASS`; no other legend,
sidecar, science, or disposition regression remains.
