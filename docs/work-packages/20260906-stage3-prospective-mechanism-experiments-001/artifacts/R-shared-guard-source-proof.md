# R shared wet-guard source proof

Static: proposed SG1 derived source proof for independent dual review. NOT
canonical adoption, runtime execution, general wet noncrossability, measurement
admission, or production promotion. No runtime/test/canonical edit or build was
performed for this artifact. Root/work-package AGENTS apply (find-agents run).
The entire R-shared-guard-authority.md draft was read before writing.

## Exact cut and scope

Paths below are relative to isolated R `crates/openwepp-land-surface-energy/src/`.
Read evaluator 1021–1082, 1270–1518, 2002–2120; replay 1–117;
covered_liquid 124–232; replay_tests 1–163, 220–330, 730–789, 1218–1285;
existing solver_tests condensation and multirank fixture bindings.

- solver_covered_evaluation.rs: c23618821c6c2e3a9bc954959585ba42ab93afe452015859d88747cee6e637a2
- solver_component_dependency_replay.rs: 0c557403ded163aa67867a28d3eecdfd9444908da3666ab3469670f4befdb7ce
- covered_liquid.rs: 56d1dde3832175db9a2cd60f8859ed001982e4867b4ad84c1190545396803d18
- solver_component_dependency_replay_tests.rs: 36bf6a8cbe4e8faaa9e093097a91d458d970822337ebaf8d2ee5312cbc445036

The comparison is replay versus complete at the SAME lawful signed probe,
validated input/caps, frozen branches and successful sweep base. It is not
base-versus-probe identity for changed wet operands. No pressure-margin or
successful-base-implies-wet-probe-success theorem is asserted.

## Five named families

| Family | Exact shared body and operands | Copy/recompute and order |
| --- | --- | --- |
| route.prepare[o] | replay:67 calls covered_liquid:124 prepare_covered_liquid(immutable occupancy, incident_rain). Guard operands are beginning store, interception fraction, capacity coefficient, stemflow fraction, LAI, SAI and incident rain. | The evaluator:2034 top-down loop copies successful preparation/ledger only when graph route_reachable is false. Otherwise the same helper runs first. Wet probe at rank k recomputes k and all lower ranks. Sun/shade/stem changes no route operand, so the route trio is copied. |
| route.wet[o] | replay:68 calls evaluator:1021 covered_wet_flux(column, occupancy, preparation, block[8], shared canopy T/q, frozen). Reads pressure, gas constant, wet conductance, class/stem areas, preparation wet fraction/store, interval, and the exact occupancy frozen wet branch. Saturation is evaluated inside this same body before branch selection. | First wet call follows preparation in the route helper, before first finalization. Same copy selector as route.prepare; a reachable route is never partly copied. A potentially failing saturation call is recomputed, not assumed safe from temperature bounds. |
| route.finalize[o] | replay:77 calls covered_liquid:177 finalize_covered_liquid(preparation, wet_flux * interval, block[8], Potential/FixedAuthorizationFinal selected by the same caps.is_some()). Includes identical reference-temperature handling, evaporation/condensation split, drainage, enthalpy and result.validate(). | First finalization follows first wet via `?`. evaluator:2055 updates next incident rain in the same throughfall + initial drainage + second drainage order and accumulates stemflow in rank order. A copied ledger supplies exactly these operands; reachable lower ranks consume exactly the resulting complete-probe prefix. |
| occ.wet[o] | evaluator:1365 calls the same evaluator:1021 body with context.liquid = liquid_preparations[o], the same block[8], shared canopy T/q, occupancy and frozen context. | ALWAYS recomputed in the later occupancy evaluator. It remains after sun current, shade current, sun maximum, shade maximum and vapor calculations, before hydraulics. It is not coalesced with route.wet. |
| occ.liquid[o] | evaluator:1500 calls the same covered_liquid:177 body with context.liquid, wet_e * interval, twet, and the identical caps-selected pass. | ALWAYS recomputed after hydraulics/energy/tolerance arithmetic at its original later source position. Route-match check remains evaluator:2113 after the occupancy result. It is not substituted by the first ledger. |

For a recomputed route, equality follows inductively down the prefix: same top
rain; same preparation operands; same wet operands/branch; same finalization
operands; therefore identical result or identical first typed error. A returned
error exits by `?` before later nodes. The second calls use the exact same
preparation and probe inputs, but retain their distinct original positions.
This proves result-or-error equivalence, not noncrossability.

## Earlier skipped guards and first-error precedence

Capture exists only after successful complete base evaluation. Exact validated
borrows bind occupancy/column constants, caps and frozen state; the consumed
signed proof changes exactly one admitted component temperature. The normative
graph plus complete edge oracle determines copied nodes, not a success heuristic.

For a copied route trio, wet temperature and shared canopy T/q are unchanged.
All immutable preparation/gas/liquid operands above are unchanged. No upstream
wet-dependent route is skipped: a changed upper wet temperature reaches every
lower preparation. Prefix equality therefore proves copied incident rain too.
The frozen wet entry is the successful base natural branch, or the same already
fixed branch; the existing frozen-base recapture comparison explicitly checks
preparation and every ledger bit against the actual production base capture.
Thus earlier copied preparation/wet/finalization guards still succeed with their
identical operands, so removing those calls cannot expose a later error first.

Copied current/maximum leaf calls have identical leaf temperature, beta, canopy
q, gas environment, biochemical inputs, conductance constants and class inputs
(evaluator:1295–1350). Wet fractions are not leaf_trial_state operands. For a
changed sun/shade leaf PC1 requires the affected beta bits equal one; the current
call executes at its original position and maximum uses that successful current
state under INV-163. Unproved thermal beta selects complete BEFORE capability
creation. Wet/stem changes no leaf-call operand. These facts preserve earlier
skipped-leaf success; they do not prove success for a changed current call.
Other guards are not skipped by this implementation. SG1 does not waive their
separate normative implication obligations.

## Frozen derived test bindings and limits

`replay_probe_corpus` (220 onward) independently constructs canonical signs/h,
uses actual base.capture, checks frozen-base recapture, and compares actual
dispatcher with complete under identical frozen state. For successful eligible
probes it compares full evaluation and capture fields, including every first
ledger/preparation field and every second occupancy ledger float via to_bits.
The no-complete-fallback count is asserted before Result matching. The natural
Err/Err catch-all retains exact typed identity and returns immediately at the
first error; it does not manufacture or claim a wet error witness.

`replay_fixed_and_boundary_corpus` binds potential/fixed, beginning store zero
and exact capacity, plus ordinary wet fixture, beta boundaries and zero PAR.
These labels describe actual constructed input boundaries; incident rain may
wet a zero-beginning-store case, so zero store is not falsely called zero wet
fraction. `assert_boundary_component_replay` binds the authentic zero-area
fixture. Full-field checks include wet fraction, vapor, both drainage roles,
enthalpy, incident/ending stores and ground transfer, not only residuals.

Actual N2/S6 `observe_runtime_base` invokes that full all-coordinate corpus on
the first real potential and fixed base; flags become true only after every
signed coordinate completes successfully. Upper wet probes therefore exercise
all lower route descendants in both postures. The separate node-oracle consumer
requires both flags; its extra work is not timing evidence. Existing N2
condensation fixture solver_tests:1806–1833 explicitly checks positive upper
condensation/drainage and lower incident-rain bit identity, but is a complete
evaluation test, not itself a replay differential condensation witness.

Genuine assertion limit referred to parent/test owner: natural Err/Err currently
checks beginning column through derived PartialEq, not an explicit every-field
bit snapshot of input/base/caps/frozen/capture. Immutable borrowing structurally
prevents their mutation, and success node/capture parity is bitwise, but the
catch-all must not be described as an already-executed byte-exact rollback
assertion. Test owner should identify an existing sufficient assertion or offer
a bounded assertion-only refinement for review; no new physical obligation or
synthetic crossing is inferred. Runtime evidence and final dual disposition are
parent-owned and pending for SG1. No historical wet noncrossability gate passed
by virtue of this artifact.
