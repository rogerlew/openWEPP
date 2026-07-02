# B01 Verdict Table

Protocol: campaign §4 (adjudication-first; the brief is a flag list, not an
authority). Grades: `conservation-forced` / `source-intent` / `convention` /
`unverified`. Brief-claim dispositions: `upheld` / `partially-upheld` /
`convention-not-defect` / `unsubstantiated`. Evidence detail and file:line
in `class-notes.md`. Evidence class **Ran** = direct source/contract reads
this package (legacy `/workdir/wepp-forest/src` HEAD `924ab16d` + openWEPP
main) plus verified explorer sweeps; no simulations were run for this
audit (all rows Static at the runtime-behavior level except where the
identity gates' standing Ran evidence is cited).

| # | Class | Claim grade | Brief-claim disposition | openWEPP disposition | Key evidence |
|---|---|---|---|---|---|
| B1 | Hourly q-cap bottom-OFE bypass | observation `conservation-forced`; "transport capacity" framing **not code-backed** (source cap is availability: `runoffin + rmloc + subrin`) | **partially-upheld** (real conservation bug; physics narrative overstated) | **not-applicable / correct-by-construction** — runoff is computed *as* the conservation residual (`runoff.rs:652-688`); the failure mode has no surface | Ran: `watbal_hourly.for:1009-1024`; `runoff.rs:652-688` |
| B2 | Snowmelt double-count in closure basis | `convention` (audit input-basis definition; production outputs unchanged) | **upheld as audit-basis error** (same water counted twice in *their audit's* algebra) | **correct-by-construction** — identity uses precipitation-only external input + typed signed `snow_coupling_m`; SWE reconciled in the snow producer; no dual counting. Consumer caveat: WAT still publishes `RM`; an RM-basis external audit re-inherits the double-count → usersum note (follow-up) | Ran: `storage.rs:804-814` |
| B3 | Missing interception-storage export | `conservation-forced` for legacy (real state invisible to audits) | **upheld** (for legacy; `pintlv+resint` now exported in current source) | **not-applicable** — openWEPP has no interception-storage state (same-day flux only; `InterceptionStorage` column always `None`). Carryover fidelity = existing backlog item `20260512-residue-moisture-storage-full-state` | Ran: `01_publication.rs:340-343`; `watbal.for` write |
| B4 | Zero-input flux without Δstorage | `conservation-forced` | **upheld** (their new-kernel defect, caught at their gate) | **correct-by-construction** — storage assigned from the identity; overdraw fails nonnegativity; ledger-vs-state guard (`projection.rs:179-204`) enforces the state actually moved, per OFE-day | Ran: `storage.rs:800-845`, `projection.rs:167-204` |
| B5 | Rain-routing conflation (rain-on-snow) | observation `conservation-forced` *given their label-keyed melt cap*; Candidate-1 repair (rain always in rain channel) = **`convention`** | **partially-upheld** — the water loss was real in their pipeline; the channel-attribution rule is a convention, not forced | **correct-by-construction under ratified contract** — openWEPP deliberately encodes the legacy rain-into-melt lineage (`INV-SNOWFREEZE-015/021/023`, `INV-WATBAL-055`) with the hazard neutralized (availability includes retained rain; overdraw hard-fails). **Comparator hygiene:** rain-on-snow channel attribution vs post-`260514` legacy differs by design | Ran: `runoff_reconciliation.rs:143-158`; `winter.for:381` (dead alias site); SC invariant texts |
| B6 | Clamp+preserve interaction (WB-18×WB-30) | invariant "clamp+preserve without compensation is non-conserving" = `conservation-forced`; their Shape-A partition choice = `convention` | **upheld** (their new-kernel defect) | **correct-by-construction** — outputs derive from bounded stores; raw values diagnostic-only; overdraw hard-fails; no baseflow-preservation machinery exists | Ran: `infiltration_reconciliation.rs:1759-1791` |
| B7 | `QOFE = n × Q` denominators | `convention` (column semantics; physical volume correct on both sides via the cancellation) | **convention-not-defect** — the 2008 half-applied parallel edit is real (`source-intent` for the *inconsistency*), but "18-year defect" framing overstates: it is a denominator-semantics choice the ecosystem has now standardized | **contract-decision → `MOFEFID-B02`** — openWEPP publishes the pre-fix convention (`01_publication.rs:370-376`, matches our `wepp_260430` anchor); ecosystem + wepppy consumers moved to `QOFE = Q`; adjudicate openWEPP-native contract under ADR-0019 with `H.pass`-runvol invariance as the hard gate | Ran: both sources read |
| B8 | R01 cascade-tail rain-event over-counting | observation (`runoff > precipitation`) `conservation-forced`; repair `unsubstantiated` (failed its own 30/40 cohort gate; **unfixed in legacy production**) | **upheld (observation) / unsubstantiated (repair)** | **not-applicable** — openWEPP has no event counter (`RM` is an additive depth bucket; totals are depth sums, never event-gated). **Comparator hygiene:** legacy production still over-counts on H347-class cascades; totals deltas expected | Ran: summary-accumulator sweep (no counter exists) |
| B9 | Dry-day per-OFE residual | `unverified` (mechanism not on record; their package still active) | **unverified** | **correct-by-construction** on the enforced-identity basis — same per-OFE-day identity + guard pair on dry days, no special-casing. **Comparator hygiene:** legacy carries an open dry-day residual | Ran: guard reads |
| B10 | Winter mixed-melt day-end aggregation | `source-intent` (math defect real; fix dated in source; branch empirically unreachable — 0 mixed days / 21.7M rows in their cohort) | **upheld** (latent legacy defect, fixed) | **contract-decision (follow-up)** — `INV-SNOWFREEZE-015` ratifies the corrected *net* algebra while the implementation routes the *positive-parts sum* under SNOWSCI-S1 single-source rationale; these disagree on (empirically doubtful) mixed-sign days. Reconcile invariant text with implementation. Not a proven runtime defect | Ran: `winter.for:430-466`, `runoff_reconciliation.rs:1133-1162`, INV-015 text |
| B11 | Surface-pulse audit over-reach on `latqcc`-only days | `convention` (audit-tool design lesson) | **upheld** (their tool over-reached; they hardened it) | **not-applicable today** (no external WAT/PASS closure-audit tool exists) — carried as a **binding design constraint** on Lane C2 and any future exported-surface audit. Related A01-S4 gap: per-OFE closure is not externally reconstructable from our exports | Ran: tools sweep |

## Summary

- **No openWEPP production defect was found in any of the eleven classes.**
  Seven classes are correct-by-construction or without an existing surface;
  two are contract decisions (B7 → `MOFEFID-B02` QOFE adjudication; B10 →
  SC-SNOWFREEZE-015 text/implementation reconciliation); two produce
  standing comparator-hygiene entries (B8, B9 — legacy production carries
  those defects unfixed, so deltas are expected).
- **The skepticism directive earned its keep.** Three brief claims did not
  survive at face value: B1's "transport capacity" narrative is not what
  the legacy code enforces (availability cap); B5's Candidate-1 rule is a
  convention on which openWEPP has deliberately ratified the *opposite*
  (legacy-lineage) choice with the conservation hazard neutralized
  structurally; B7's "defect" framing reduces to column semantics.
- **Follow-ups spawned:** `MOFEFID-B02` (QOFE ecosystem-contract
  adjudication); SC-SNOWFREEZE-015 reconciliation note; usersum WAT `RM`
  consumer caveat; Lane C2 audit-tool design constraints (B11); A01 seeds
  (dormant `QcapSoftLimit`, tautological `closure_residual_m` naming,
  `INV-RUNOFFPART-029` case-classifier spec-vs-code question, external
  closure reconstructability gap).
