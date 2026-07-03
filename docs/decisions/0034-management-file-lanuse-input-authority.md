# ADR-0034: Management-File `lanuse` Input Authority (first-class landuse modes)

Status: **Accepted** (ratified 2026-07-02 by Codex after ADR-0034 review
disposition; operator selected Option A on 2026-07-02)
Deciders: Roger Lew, Codex
Ratification provenance:
`docs/planning/disturbed-forest-fidelity-strategy.md` (campaign strategy),
`docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
(Increment-1 foundation), and Codex ADR-0034 review disposition on 2026-07-02.
Relates: `SC-OFEROUTE-001` (routing operands + activation gate),
`SC-SUBHYD-001` / `INV-SUBHYD-032` (`ksatadj`), the growth–canopy contract
(future canopy phenology), `ADR-0011` (contract-first), `ADR-0017`
(legacy-as-flag), `ADR-0024` (source-intent authority).

## Context

openWEPP inherits WEPP's landuse handling, in which forest, shrub, and grass —
disturbed (burned) or not — are encoded as **cropland** management records
(`landuse=1`, perennial), with forest/range physics carried through cropland
fields (`cancov`, `inrcov`, `rilcov`, `rrinit`, `rdmax`, `xmxlai`, `hmax`,
`cuthgt`, `decfct`, `dropfc`, `extnct`). This is a legacy workaround: WEPP's
native forest/rangeland branches never finished in production, so everything was
run as cropland. A second, coupled workaround disables frost for non-ag
(`ksflag=0`) as a broad lever to accommodate the forest conductivity model
(`ksatadj`).

The disturbed-forest fidelity campaign brings this landuse family into openWEPP
as first-class, contract-governed physics (canopy phenology, litter, `ksatadj`
soil conductivity, Papanicolaou routing operands). That raised a ratification
question: **where does the "opt-in physics is authorized by the management-file
`lanuse` block, not by the `.run` file, and not inferred from legacy cropland
fields" principle live** — a new standalone contract, or folded into an existing
physics contract (`SC-OFEROUTE-001`)?

The deciding observation is a **layer** distinction openWEPP already draws:

- **Science contracts** (`docs/specifications/science-contracts/`, `SC-*`) govern
  **physics** — state surfaces, invariants, laws — per domain.
- **Interface contracts** (`docs/contracts/`) govern **input/output structure and
  provenance** — the `.run` runfile, HBP, parquet.

Input authority (where operands come from, that they are present/typed/
authorized, that nothing is inferred) is **not physics**. It is an interface
concern that **multiple** physics domains consume (routing → `SC-OFEROUTE`,
soil/`ksatadj` → `SC-SUBHYD`, canopy → growth–canopy contract, PMET/erodibility).
The enforceable boundary is narrow: this contract may govern operand source,
schema identity, typed presence, reproducibility, and fail-closed provenance
rules; it does **not** define equations, physical-domain bounds, process
invariants, or how the operands alter state. Those remain in the consuming
science contracts.

### Boundary test (why a standalone contract)

A surface earns its own contract when **all three** hold: (1) it has coherent,
self-contained invariants; (2) it is consumed by **more than one** existing
domain; (3) it is a **distinct concern-layer** from the domains that consume it.
The management-file/`lanuse` input authority passes all three (structure +
provenance invariants; consumed by routing/soil/canopy/PMET; interface-layer, not
physics). The mirror case keeps the test honest: `ksatadj` physics does **not**
earn a new contract — it belongs in `SC-SUBHYD` — because it is single-domain
physics (fails #2 and #3). **Physics goes in the domain contract; cross-domain
input structure gets its own interface contract.**

## Decision

1. **Management-file authority.** For openWEPP-native managements, the
   **management file** is the opt-in authority for new-physics operands. The
   `.run` file points to the management sidecar and remains reproducibility
   metadata; it carries **no** hidden physics selectors.
2. **First-class `lanuse` modes.** `lanuse` selects a physical landuse mode
   (forest / shrub / grass / …), not a cropland masquerade. Each mode's landuse
   record carries its physics operands as first-class, typed parameters.
3. **Cropland-encoded forest/range fixtures are compatibility inputs, not
   authority** for new physics. They may be migrated or interpreted by an
   explicit adapter, but must not be the authority for new landuse physics.
4. **No inference without a bridge contract.** Deriving new-physics operands
   (e.g. Papanicolaou roughness) from legacy cropland fields such as row width,
   ridge spacing, or `rrinit` is **disallowed** unless a separate, ratified
   **bridge contract** defines that mapping.
5. **Fail-closed.** Active new-physics behavior fails closed until the
   management/`lanuse` record (and its authoritative `(texture × class)`
   parameterization lookup) supplies the required operands.
6. **This input authority is governed by a new standalone INTERFACE contract**
   (`docs/contracts/openwepp-management-lanuse-authority-contract.md`), **not**
   folded into a physics contract. Physics contracts (`SC-OFEROUTE-001`,
   `SC-SUBHYD-001`, the growth–canopy contract) reference it for operand
   provenance as concrete operands are bound; they retain authority over the
   *physics* of what the operands do.

## Scope of ratification

Ratifying ADR-0034 accepts the authority model and the skeleton contract's
`LANUSE-AUTH-1..6` rules only. It does **not** ratify a concrete `lanuse`
operand schema, concrete field names, MOFE/multi-OFE cardinality, bridge-mapping
mechanics, runtime wiring, default activation, or changes to any consuming
physics contract's operand definitions. Those remain WS-1 deliverables under
schema ID `openwepp-management-lanuse-v1` and later domain-contract amendments.
The management-lanuse contract therefore remains `draft-normative` / skeleton
until WS-1 populates and promotes the concrete schema.

## Alternatives considered

- **Extend `SC-OFEROUTE-001`'s activation section** — rejected: a **layer
  violation** (input-structure authority inside a physics law) that under-scopes
  the principle (canopy/litter/soil authority would be stranded in the *routing*
  contract, and non-routing physics would reference a routing contract for their
  inputs).
- **ADR only, defer the contract** — rejected: only delays the standalone-vs-fold
  choice and risks ad-hoc `lanuse` structure emerging without the guardrail.

## Consequences

- A new interface contract is authored (skeleton now; concrete `lanuse` operand
  schema is designed in the WS-1 foundation work-package of the disturbed-forest
  campaign).
- Consuming physics contracts add provenance cross-references when WS-1 or a
  domain-contract amendment binds concrete `lanuse` operands; this ratification
  does not amend `SC-OFEROUTE-001`, `SC-SUBHYD-001`, or any future growth–canopy
  contract.
- The legacy `ksflag=0` frost-off lever is **not** carried: openWEPP keeps frost
  on (`ksflag=1`) and augments `ksatadj` to be sensible independently (a separate
  SUBHYD/campaign decision; referenced here only as the coupled legacy workaround
  this authority model removes from the input-authority layer).
- Default activation of any `lanuse`-mode physics remains a **separate later
  gate** with its own no-regression + magnitude evidence (consistent with
  `ADR-0033`).
