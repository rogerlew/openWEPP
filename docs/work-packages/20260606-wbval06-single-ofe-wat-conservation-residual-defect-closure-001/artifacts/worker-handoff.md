# Worker Handoff

Status: corrected

Evidence mode: executed

Current handoff: WBVAL06 is closed as corrected on the **openWEPP side** — the
daily interception flux `I` is now published to `H.wat` as `Interception`. One
follow-on is required before single-OFE WB closure is auditable on the stated
acceptance surface: closure was demonstrated under openWEPP's own identity audit,
not the **totalwatsed3** audit, which does not yet consume the `Interception`
flux.

Follow-on (next work package, wepppy-side):

- Goal: add the `Interception` flux as a first-class outflow in the totalwatsed3
  daily closure so WB closure is auditable from totalwatsed3 on openWEPP output.
- Observable gap: totalwatsed3 closes `P - (Runoff + Lateral + ET + Percolation)
  - ΔStorage` with no interception-flux outflow; on openWEPP post-WBVAL06 output
  it would show a residual ≈ `+I` (~26.8 mm), because openWEPP's published `ET`
  excludes interception and totalwatsed3 ignores the new `Interception` column.
- In-scope (wepppy): `totalwatsed3.py` closure + schema and
  `tools/totalwatsed3_daily_closure_audit.py`; consume `hillslope_wat.Interception`
  as an outflow (`... + Percolation + Interception ...`).
- Hard constraint: do **not** change ET (`Ep`/`Es`/`Er`) — it is
  producer-authoritative physics; interception is a separate first-class outflow.
- Authority: `SC-WATBAL-001` closure identity (`... + S - I - Q - ET - D - Qd`)
  and the openWEPP `H.wat` `Interception` column.
- Acceptance: totalwatsed3 audit closes on openWEPP post-WBVAL06 output for years
  `2..6` within tolerance; only then is single-OFE rung-1 closure complete.

If package closes `HOLD`, the handoff must name:

- Defect or boundary ID.
- Observable failure and failing fixture.
- Suspected mechanism.
- In-scope write set for the owning follow-on.
- Correction authority.
- Acceptance target.
- Legitimate `HOLD` conditions.

Forbidden relay: no handoff may name only a next diagnostic step.

Static:

- The openWEPP-side defect (`H.wat` omitted the interception flux) is closed; the
  follow-on above is a wepppy-side totalwatsed3 audit-consumer change, not a
  reopening of openWEPP physics.
- Stage-2 snow science remains a separate protected boundary if future work
  raises snow magnitude questions; it was not implicated by WBVAL06 closure.

Ran:

- Corrected validation passed for all 22 WAT emitters.
