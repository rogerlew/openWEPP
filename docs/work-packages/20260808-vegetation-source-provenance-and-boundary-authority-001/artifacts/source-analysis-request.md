# Source-Analysis Request

Status: frozen

Evidence mode: Static

Request ID: `VEG-SOURCE-FIREWALL-001`

Frozen source identity: local read-only checkout `/workdir/RHESSys` at commit
`f9d1bbf8d161aa55b6a51061dc320188ead44962`.

Authorized writer: the delegated source-aware analyst. The analyst may write
only `artifacts/approved-spec.md` and must return only that path and its SHA-256
digest to the parent agent. The response remains quarantined and unavailable to
the parent contract author until the independent compliance reviewer returns
`PASS` in `artifacts/compliance-review.md`.

## Objective

Produce a self-contained, sanitized semantic inventory that lets an independent
contract author define vegetation ownership and coupling boundaries without
seeing RHESSys source expression. Inspect the frozen checkout only to establish
observable process/state roles, cadence, boundary units where ascertainable,
mutation ownership, dependency ordering, and gaps. RHESSys behavior is
comparison evidence, not production authority.

## Required Inventory

Cover each family below even when the result is `reject` or `defer`:

1. canopy-stratum identity, vertical ordering, projected cover, overlap, leaf
   and woody area, height, and rooting distribution;
2. shortwave and longwave receipt, attenuation, transmission, and surface
   partition among canopy strata, ground, litter, snow, and soil;
3. rainfall interception storage, throughfall, stemflow, drainage, and wet-
   canopy evaporation;
4. aerodynamic and stomatal conductance, atmospheric demand, transpiration
   demand, layer-resolved root-water request, and realized-uptake feedback;
5. evergreen/deciduous phenology, photosynthesis, maintenance and growth
   respiration, allocation, turnover, mortality, and litter/coarse-woody-
   debris transfers;
6. carbon and nitrogen pool/flux boundaries needed to avoid duplicate custody,
   while keeping full soil biogeochemistry and nutrient routing deferred;
7. canopy-snow interception/storage/release behavior and its coupling to the
   ground snowpack, explicitly identifying whether the source organization
   supports a single-owner boundary;
8. timestep/call ordering and feedback or iteration needs across potential
   vegetation response, hydrologic allocation, and vegetation finalization;
9. degenerate states, domain failures, non-convergence behavior, and observable
   conservation identities; and
10. compatibility reductions that an existing aggregate canopy/ET/litter
    consumer would need, without treating the reduction as a second model.

## Required Entry Shape

For every process or state family, report:

- a neutral semantic name chosen independently of source identifiers;
- source coordinates limited to repository-relative file path plus function or
  module locator and the frozen commit (no excerpts or line-by-line account);
- evidence label `LITERATURE`, `CODE-OBSERVED`, or `INFERENCE` for every claim;
- inputs, outputs, units, cadence, mutated owner, prerequisite ordering, and
  downstream dependencies;
- external literature citation explicitly present in the inspected material,
  or `none found`; do not infer a citation from an equation's familiarity;
- unresolved ambiguity, known limiting case, and conservation relevance;
- licensing disposition: `inspection/comparison only`, `eligible for
  literature-based independent derivation`, or `direct translation prohibited`;
- recommended outcome: `adopt boundary concept`, `independently re-derive`,
  `compare only`, `reject`, or `defer`; and
- proposed openWEPP owner and consumer from this allowed vocabulary: native
  management, vegetation, land-surface energy, soil hydrology, snow/frost,
  residue/biogeochemistry, or hillslope orchestrator.

Equations or numerical constants may appear only when the artifact supplies an
external literature citation or separately named existing openWEPP canonical
authority. Otherwise describe only the semantic relationship and mark the
formula/constant `NON-PROMOTABLE`.

## Mandatory Sanitization Rules

Do not include source excerpts, comments, original local variable names,
distinctive function names except in the audit-only source-coordinate field,
statement order, branch-by-branch narration, reversible pseudocode, patches,
close mechanical descriptions, code-only formulas/constants, or instructions
to reproduce a named function. Do not copy table layouts, naming schemes, or
control-flow structure from the source. Summarize independently by process
family and ownership boundary.

The artifact must state that the checkout has no adequate repository-level
license grant for direct or closely translated incorporation, and that public
availability or community practice does not change that limit. Do not make a
legal conclusion beyond the repository evidence already supplied by the
package.

## Required Artifact Sections

1. identity and scope;
2. sanitization declaration;
3. process/state inventory;
4. ordering and shared-transfer ledger;
5. canopy-snow boundary evidence;
6. literature and authority anchors;
7. licensing/provenance dispositions;
8. rejected and deferred material;
9. open questions and non-promotable gaps; and
10. independent-authorship sufficiency statement.

The final section must answer whether an author who sees only this artifact can
design a differently structured typed contract without recovering source
expression. Return no substantive content through chat: return the artifact
path and SHA-256 digest only.
