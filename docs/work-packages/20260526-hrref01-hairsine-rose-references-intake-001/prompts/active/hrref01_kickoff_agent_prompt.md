# HRREF-01 Kickoff Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in `package.md` sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without
requesting additional user direction unless hard-blocked. Acquisition gaps in
Phase 2 are not hard blocks — document them and continue.

Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260526-hrref01-hairsine-rose-references-intake-001/package.md
- /workdir/openWEPP/docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md
- /workdir/openWEPP/references/README.md
- /workdir/openWEPP/references/annotated_bibliography.md
- /workdir/openWEPP/references/rights_classification_first_pass_2026-05-11.md
- /workdir/openWEPP/docs/governance/reference-vendoring-policy.md

## What to author this cycle

1. **Phase 0 — bibliography skeleton**: append new entries to
   `/workdir/openWEPP/references/annotated_bibliography.md` starting at R-17.
   The current tail is R-16. Reference list (mandatory subset highlighted):

   **Foundational (mandatory; assign R-17..R-20)**
   - Hairsine, P. B., & Rose, C. W. (1991). Rainfall detachment and deposition:
     Sediment transport in the absence of flow-driven processes. *Soil Sci. Soc.
     Am. J.*, 55(2), 320-324.
   - Hairsine, P. B., & Rose, C. W. (1992a). Modeling water erosion due to
     overland flow using physical principles: 1. Sheet flow. *Water Resour.
     Res.*, 28(1), 237-243.
   - Hairsine, P. B., & Rose, C. W. (1992b). Modeling water erosion due to
     overland flow using physical principles: 2. Rill flow. *Water Resour.
     Res.*, 28(1), 245-250.
   - Rose, C. W., Williams, J. R., Sander, G. C., & Barry, D. A. (1983). A
     mathematical model of soil erosion and deposition processes: I. Theory for
     a plane land element. *Soil Sci. Soc. Am. J.*, 47(5), 991-995.

   **Independent validation (at least one mandatory)**
   - Sander, G. C., Hairsine, P. B., Rose, C. W., Cassidy, D., Parlange, J.-Y.,
     Hogarth, W. L., & Lisle, I. G. (1996). Unsteady soil erosion model,
     analytical solutions and comparison with experimental results. *J.
     Hydrol.*, 178(1-4), 351-367.
   - Beuselinck, L., Hairsine, P. B., Sander, G. C., & Govers, G. (2002).
     Evaluating a multiclass net deposition equation in overland flow
     conditions. *Water Resour. Res.*, 38(7), 14-1 to 14-11.
   - Hogarth, W. L., Rose, C. W., Parlange, J.-Y., Sander, G. C., & Carey, G.
     (2004). Soil erosion due to rainfall impact with no inflow: A numerical
     solution with spatial and temporal effects of sediment settling velocity
     characteristics. *J. Hydrol.*, 294(4), 229-240.
   - Heng, B. C. P., Sander, G. C., & Scott, C. F. (2009). Modeling overland
     flow and soil erosion on nonuniform hillslopes: A finite volume scheme.
     *Water Resour. Res.*, 45(5), W05423.

   **Model-comparison / sensitivity (recommended)**
   - Misra, R. K., & Rose, C. W. (1996). Application and sensitivity analysis
     of process-based erosion model GUEST. *Eur. J. Soil Sci.*, 47(4), 593-604.
   - Tromp-van Meerveld, H. J., Parlange, J.-Y., Barry, D. A., Tromp, M. F.,
     Sander, G. C., Walter, M. T., & Parlange, M. B. (2008). Influence of
     sediment settling velocity on mechanistic soil erosion modeling. *Water
     Resour. Res.*, 44(6), W06401.
   - Morgan, R. P. C., Quinton, J. N., Smith, R. E., Govers, G., Poesen, J. W.
     A., Auerswald, K., Chisci, G., Torri, D., & Styczen, M. E. (1998). The
     European Soil Erosion Model (EUROSEM): A dynamic approach for predicting
     sediment transport from fields and small catchments. *ESPL*, 23(6), 527-544.

   **Adjacent transport-capacity (optional, recommended)**
   - Govers, G. (1990). Empirical relationships for the transport capacity of
     overland flow. *IAHS Pub.*, 189, 45-63.
   - Govers, G. (1992). Evaluation of transporting capacity formulae for
     overland flow. In: A. J. Parsons & A. D. Abrahams (eds), *Overland Flow:
     Hydraulics and Erosion Mechanics*, UCL Press, 243-273.
   - Prosser, I. P., & Rustomji, P. (2000). Sediment transport capacity
     relations for overland flow. *Prog. Phys. Geogr.*, 24(2), 179-193.

   **Post-fire scenario (optional)**
   - Wagenbrenner, J. W., MacDonald, L. H., & Rough, D. (2006). Effectiveness
     of three post-fire rehabilitation treatments in the Colorado Front Range.
     *Hydrol. Process.*, 20(14), 2989-3006.
   - Robichaud, P. R., Elliot, W. J., Pierson, F. B., Hall, D. E., & Moffet, C.
     A. (2007). Predicting postfire erosion and mitigation effectiveness with a
     web-based probabilistic erosion model. *Catena*, 71(2), 229-241.

   Use the existing R-01..R-16 entries as a structural template. Schema:
   `**Citation**:`, `**Local path**:`, `**Reference quality**:`, `**Topic**:`,
   `**Key equations / concepts for HR adoption**:`,
   `**Kernel mapping**:`, `**Notes / caveats**:`, `**OAR-6 compliance status**:`.

2. **Phase 1 — rights classification**: extend
   `/workdir/openWEPP/references/rights_classification_first_pass_2026-05-11.md`
   with a `## 2026-05-26 Hairsine-Rose addendum` section, **or** author a new
   `/workdir/openWEPP/references/rights_classification_hr_2026-05-26.md` with
   cross-links from both files. Per-entry decision tree is in `package.md`
   Phase 1.

   Default expectation: most HR-family papers are subscription journals
   (*Water Resources Research*, *Soil Science Society of America Journal*,
   *Journal of Hydrology*, *Earth Surface Processes and Landforms*) and will
   be classified `copyrighted/` or `external-print-source` depending on
   acquisition status. Govers (1992) book chapter and Wagenbrenner /
   Robichaud (USFS-coauthored) may have agency public-domain or open-access
   considerations — verify rather than assume.

3. **Phase 2 — best-effort acquisition**: scan the workstation for any
   pre-existing PDFs of these references. Do not bypass paywalls. Open-access
   author self-archives are acceptable if the publisher's policy permits and
   the source page documents the permission. Entries with no available source
   stay at `local_path: external-print-source` with an `Acquisition status`
   note in `Notes / caveats`.

4. **Phase 3 — annotation enrichment**: for each entry whose PDF is acquired,
   add `[DIRECT]` claims with section/equation references and target
   `SC-SED-HR-001` concept hooks. For entries without acquired PDFs, keep
   `Kernel mapping: bibliographic-only` and do not fabricate equation
   citations. Honest evidence labels are non-negotiable per repo-root
   `CLAUDE.md` §"Truthfulness About Work Performed".

5. **Phase 4 — cross-linking and disposition**:
   - Append `## Work-package linkage` to
     `/workdir/openWEPP/docs/backlog/20260526-hairsine-rose-multiclass-sediment-model.md`
     pointing to this WP directory and noting which backlog acceptance-criteria
     rows this WP closes (specifically row #1 "Reference closure").
   - Add WP index entry to `/workdir/openWEPP/docs/work-packages/README.md`.
   - Author the four artifact files listed in `package.md` §Deliverables item 7.

## Required outputs this cycle

1. R-17..R-(17+N-1) entries added to `references/annotated_bibliography.md`
   where N ≥ 5 (4 foundational + at least 1 independent validation).
2. Rights classification record (extension or new file).
3. Vendored or local-cached artifacts for acquirable PDFs.
4. Backlog cross-reference update.
5. WP-index update.
6. Disposition artifacts under `artifacts/`.

## Non-goals reminder

- Do **not** author or amend any `SC-*` contracts.
- Do **not** touch any Rust code under `crates/`.
- Do **not** fabricate DOIs, page ranges, or equation references for unacquired
  sources.
- Do **not** modify `references/README.md` or the vendoring policy file.

## Hard-block conditions

- Vendoring-policy contradiction: if a paper appears to qualify for both
  vendorable and restricted under different criteria, stop and surface the
  ambiguity rather than picking one silently.
- Schema drift: if the existing R-01..R-16 schema appears incompatible with
  HR-family entries (e.g., HR uses different equation-anchor conventions),
  surface and resolve before authoring the new entries.

## Closeout condition

Disposition is `COMPLETE` when exit criteria #1-#7 of `package.md` are met,
regardless of acquisition gaps. Acquisition gaps are documented in
`artifacts/hrref01-acquisition-gaps.md` and forwarded via `worker-handoff.md` —
they do not hold disposition.
