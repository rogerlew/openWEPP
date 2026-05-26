# 20260526-hrref01-hairsine-rose-references-intake-001

## Status
- `state`: active
- `date`: 2026-05-26
- `timezone`: UTC

## Objective

Execute the references-intake step for the Hairsine-Rose multi-class sediment model
concept backlog item by adding canonical bibliography entries to
`references/annotated_bibliography.md`, performing first-pass rights classification,
and placing acquirable artifacts under `references/vendorable/` (redistributable) or
`references/copyrighted/` (restricted local cache) per
`references/README.md` and `docs/governance/reference-vendoring-policy.md`.

This package is **docs-only and metadata-only**. It does not author science
contracts, does not implement kernels, and does not generate comparator vectors.

## Why This Package Exists

The Hairsine-Rose multi-class sediment model backlog item
([`docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md`](../../backlog/20260526-hairsine-rose-multiclass-sediment-model.md))
enumerates ~14 references across foundational, independent-validation,
model-comparison, transport-capacity-adjacent, and post-fire-target categories. The
backlog item's promotion-from-backlog acceptance criterion #1 explicitly requires:

> Foundational HR papers (1991, 1992a, 1992b, plus at least one independent-group
> validation: Beuselinck 2002 or Heng 2009) are vendored or metadata-tracked per
> `references/README.md`.

This package satisfies that criterion. It does **not** itself promote the backlog
item — it produces one of the inputs the backlog-to-WP promotion needs.

Subsequent HR-related work packages (audit, contract authoring, kernel scaffolding)
will reference the bibliography entries added by this package and cite them as
authority anchors.

## Scope

### Included

- Add at minimum the following bibliography entries to
  `references/annotated_bibliography.md` (R-numbers assigned sequentially from R-17;
  R-16 is the current tail):
  - **Foundational primary (mandatory)**
    - Hairsine & Rose (1991) — *Soil Sci. Soc. Am. J.* 55(2)
    - Hairsine & Rose (1992a) — *Water Resour. Res.* 28(1) — Sheet flow
    - Hairsine & Rose (1992b) — *Water Resour. Res.* 28(1) — Rill flow
    - Rose, Williams, Sander & Barry (1983) — *Soil Sci. Soc. Am. J.* 47(5)
  - **Independent validation (at least one mandatory; all four recommended)**
    - Sander et al. (1996) — *J. Hydrol.* 178
    - Beuselinck, Hairsine, Sander & Govers (2002) — *Water Resour. Res.* 38(7)
    - Hogarth, Rose, Parlange, Sander & Carey (2004) — *J. Hydrol.* 294
    - Heng, Sander & Scott (2009) — *Water Resour. Res.* 45(5) W05423
  - **Model-comparison / sensitivity (recommended)**
    - Misra & Rose (1996) — *Eur. J. Soil Sci.* 47(4)
    - Tromp-van Meerveld et al. (2008) — *Water Resour. Res.* 44(6) W06401
    - Morgan, Quinton, Smith, Govers, Poesen, et al. (1998) — *ESPL* 23(6) — EUROSEM
  - **Adjacent transport-capacity (optional, recommended)**
    - Govers (1990) — IAHS Pub. 189
    - Govers (1992) — book chapter in *Overland Flow: Hydraulics and Erosion
      Mechanics*, UCL Press
    - Prosser & Rustomji (2000) — *Prog. Phys. Geogr.* 24(2)
  - **Post-fire scenario lineage (optional)**
    - Wagenbrenner, MacDonald & Rough (2006) — *Hydrol. Process.* 20(14)
    - Robichaud, Elliot, Pierson, Hall & Moffet (2007) — *Catena* 71(2) — ERMiT

- For each entry, populate the existing
  [annotated_bibliography.md](../../../references/annotated_bibliography.md) schema:
  - **Citation** (author, year, title, journal/publisher, volume/issue/pages)
  - **Local path** (`references/copyrighted/<basename>.pdf` if cached,
    `references/vendorable/<basename>.pdf` if redistributable,
    `external-print-source` if unacquired)
  - **Reference quality** (`verified-primary`, `verified-secondary`,
    `external-print-source`, or new tier introduced with rationale)
  - **Topic** (one paragraph)
  - **Key equations / concepts for HR adoption** with `[DIRECT]` / `[INFERENCE]`
    evidence labels
  - **Kernel mapping** (target SC-SED-HR-001 concept if the entry supports that
    contract authoring; otherwise `bibliographic-only`)
  - **Notes / caveats**
  - **OAR-6 compliance status** if applicable

- Update or supplement
  `references/rights_classification_first_pass_2026-05-11.md` with HR-paper
  redistribution determinations. Acceptable forms:
  - extend the existing first-pass file with a dated "2026-05-26 HR addendum"
    section, **or**
  - author a new `references/rights_classification_hr_2026-05-26.md` with the same
    schema and cross-link both files.

- Place acquired artifacts:
  - **Redistributable** (public-domain, open-access, author-self-archived with
    explicit redistribution permission): `references/vendorable/`
  - **Restricted** (subscription journals without confirmed redistribution rights):
    `references/copyrighted/` (gitignored)
  - **Not yet acquired**: leave `local_path: external-print-source` and document
    acquisition status in the bibliography entry's `Notes / caveats` field.

- Cross-link the backlog item to this WP:
  - Append a `## Work-package linkage` section (or equivalent) to
    `docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md` pointing to
    this WP directory.
  - Note in the backlog item which acceptance-criterion rows this WP closes.

- Update `docs/work-packages/README.md` index with the new WP entry under the
  appropriate chronological section.

### Explicitly Out of Scope

- No `SC-SED-HR-001` contract authoring or amendment (downstream WP).
- No openWEPP kernel implementation, no `run_erod_hairsine_rose` scaffolding.
- No comparator-vector authoring.
- No DOI/page-number forensic re-verification beyond what the bibliography schema
  requires for citation closure. Equation-anchor verification (specific page
  numbers cited inside an SC contract) is deferred to the contract-authoring WP.
- No acquisition of papers requiring institutional library access not already on
  this workstation. Such entries get `external-print-source` and acquisition
  deferred — the WP completes with annotated metadata, not necessarily with all
  PDFs in hand.
- No modification of `references/README.md` or
  `docs/governance/reference-vendoring-policy.md` workflow definitions.

## Deliverables

1. **Updated bibliography**:
   - `references/annotated_bibliography.md` with new entries R-17..R-(17+N-1)
     where N is the count of entries authored this cycle.
2. **Rights classification supplement**:
   - Either extension to
     `references/rights_classification_first_pass_2026-05-11.md` or a new
     `references/rights_classification_hr_2026-05-26.md` with cross-link.
3. **Vendored artifacts** (if any redistributable HR papers acquired):
   - `references/vendorable/<paper>.pdf`
   - `references/vendorable/<paper>.md` (text extract if produced)
4. **Local-cache artifacts** (if restricted PDFs acquired but not redistributable):
   - `references/copyrighted/<paper>.pdf` (gitignored; track only metadata)
5. **Backlog cross-reference**:
   - Updated `docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md`
     `## Work-package linkage` section pointing to this WP.
6. **WP-index update**:
   - `docs/work-packages/README.md` entry for this WP.
7. **Disposition artifacts**:
   - `artifacts/hrref01-bibliography-intake-evidence.md` — list of R-NN entries
     added with their local-path status and rights classification.
   - `artifacts/hrref01-acquisition-gaps.md` — any entries left at
     `external-print-source` with rationale and follow-up acquisition plan.
   - `artifacts/hrref01_disposition.md` — closeout state.
   - `artifacts/worker-handoff.md` — handoff for next HR-family WP.
   - `artifacts/owned-file-manifest.md` — list of files this WP creates/edits.
   - `artifacts/gate-results.md` — Phase 0-3 gate outcomes.

## Dependencies

- `docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md` (parent backlog)
- `references/README.md` (intake workflow)
- `references/annotated_bibliography.md` (target file; existing schema authority)
- `references/rights_classification_first_pass_2026-05-11.md` (rights schema)
- `docs/governance/reference-vendoring-policy.md` (vendoring policy authority)
- `docs/work-packages/README.md` (index conventions)

## Phase Plan

### Phase 0 — Bibliography skeleton

Add stub entries to `references/annotated_bibliography.md` for every reference
listed in the backlog item's References section. Each stub has at minimum
citation + `local_path: external-print-source` + `reference quality: pending-acquisition`
+ a placeholder `Topic` paragraph drawn from the backlog item's annotations.

Gate: all R-numbers from R-17 onward exist; the file parses as well-formed
Markdown.

### Phase 1 — Rights classification

For each new entry, determine redistribution status using the framework in
`docs/governance/reference-vendoring-policy.md` and the first-pass file's
existing rationale schema. Decision tree:

1. **Author-self-archived with explicit redistribution license** (CC-BY, etc.) →
   `vendorable/`.
2. **U.S. federal or agency public-domain** (USGS, ARS-USDA bulletins, USFS
   technical reports) → `vendorable/` (unlikely for HR-family papers which are
   mostly academic journal articles).
3. **Subscription journal, no open-access version found** → `copyrighted/`.
4. **Open-access journal article with explicit redistribution permission** →
   `vendorable/`.
5. **Conference proceedings or out-of-print book chapter** (e.g., Govers 1992) →
   `external-print-source`.

Record each decision in the rights classification supplement.

Gate: every new bibliography entry has a documented rights classification.

### Phase 2 — Artifact acquisition (best-effort)

Attempt to acquire PDFs for entries classified as `vendorable/` and
`copyrighted/`:

- Check existing on-disk caches (workstation library, prior research archives).
- Web-fetch open-access versions where available (publisher OA, author
  self-archive, ResearchGate with caution).
- Do **not** bypass paywalls or use unauthorized sources.
- For unacquired entries: leave `local_path: external-print-source` and add an
  `Acquisition status` line to the entry's `Notes / caveats` field.

Gate: every `vendorable/` and `copyrighted/` classified entry either has a
local file or an explicit acquisition-deferred note. No silent gaps.

### Phase 3 — Annotation enrichment

For each entry, enrich the placeholder Topic and add structured fields:

- **Key equations / concepts for HR adoption**: which HR governing-equation
  surface the entry supports (rainfall detachment, flow-driven entrainment,
  shielding factor, deposited-layer continuity, transport-capacity replacement,
  identifiability, post-fire applicability).
- Evidence-label each: `[DIRECT]` for equations / claims read from the source
  itself; `[INFERENCE]` for claims supported only by reasoning from the source.
- **Kernel mapping**: target SC-SED-HR-001 concept or `bibliographic-only`.
- **OAR-6 compliance status**: per existing schema.

Stub entries that received no PDF acquisition (Phase 2) still get enriched from
the backlog item's annotations and citation context, but their evidence labels
must remain conservative (`bibliographic-only` for kernel mapping; no
`[DIRECT]` claims that require reading equation content from a source not
acquired).

Gate: every entry has either rich enrichment (acquired source) or honest
bibliographic-only metadata (unacquired source). No fabricated equation citations.

### Phase 4 — Cross-linking and disposition

- Append `## Work-package linkage` section to
  `docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md` pointing to
  this WP. Note which backlog acceptance-criteria rows this WP closes.
- Add WP entry to `docs/work-packages/README.md` index.
- Author `artifacts/hrref01-bibliography-intake-evidence.md` listing R-NN entries
  with local-path status and rights classification.
- Author `artifacts/hrref01-acquisition-gaps.md` listing unacquired entries with
  follow-up plan.
- Author `artifacts/hrref01_disposition.md` with package state and any HOLD
  conditions (acquisition gaps do not block disposition; they are documented and
  forwarded to the next HR-family WP via `worker-handoff.md`).

Gate: backlog item cross-references this WP; WP index lists this entry;
disposition recorded.

## Exit Criteria

1. All foundational primary references (4) and at least one independent
   validation reference are present in `references/annotated_bibliography.md`
   with at minimum citation + reference-quality + topic + rights classification.
2. `references/annotated_bibliography.md` parses as well-formed Markdown.
3. Every new entry has a documented rights determination.
4. Every entry classified as `vendorable/` or `copyrighted/` has either a local
   file or a documented acquisition-deferred note.
5. Backlog item cross-references this WP.
6. WP index lists this WP.
7. Disposition artifacts written.
8. No fabricated equation citations; conservative evidence labels for entries
   without acquired PDFs.

## Autonomous Execution Caveats

This WP can complete Phase 0, Phase 1, and Phase 4 **fully autonomously** from
the backlog item's reference list. Phase 2 (artifact acquisition) is bounded by
what is actually available on this workstation or as open-access PDFs on the
web. Phase 3 (annotation enrichment) is fully achievable for any entry whose
PDF is acquired and conservatively achievable from citations alone for any entry
whose PDF is not acquired.

The package is **not blocked** if some PDFs cannot be acquired — those entries
get honest `external-print-source` quality classification and acquisition is
deferred to a follow-up WP. The exit criteria are framed to allow this.

## Security Impact and Review Gate

- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: docs-only references-intake package; no executable code paths
  changed, no kernel behavior affected. Acquisition step is read-only web fetch
  and local file placement.

## Cross-References

- Parent backlog:
  [`docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md`](../../backlog/20260526-hairsine-rose-multiclass-sediment-model.md)
- Intake workflow: [`references/README.md`](../../../references/README.md)
- Target file: [`references/annotated_bibliography.md`](../../../references/annotated_bibliography.md)
- Rights schema: [`references/rights_classification_first_pass_2026-05-11.md`](../../../references/rights_classification_first_pass_2026-05-11.md)
- Vendoring policy: [`docs/governance/reference-vendoring-policy.md`](../../governance/reference-vendoring-policy.md)
- ADR-0011 (contract-first; HR adoption follows this):
  [`docs/decisions/0011-architecture-first-top-down-science-contracts.md`](../../decisions/0011-architecture-first-top-down-science-contracts.md)
