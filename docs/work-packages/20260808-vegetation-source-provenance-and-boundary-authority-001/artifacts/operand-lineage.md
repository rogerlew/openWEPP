# Shared-Transfer Operand Lineage

Status: complete for authority admission

Evidence mode: Static

| Field | Units/basis | Cadence | Producer -> receiver | Authority status | Rejected aliases |
|---|---|---|---|---|---|
| topology tile / stratum cover | fraction of horizontal area | configuration/state | management -> vegetation | authoritative definition | summed cross-rank cover; implicit independence |
| canopy liquid start/end | `kg m^-2` ground area | interval | vegetation state | authoritative custody; physics missing | ground ponding; litter water; snow SWE |
| incident liquid | interval-integrated `kg m^-2` | interval | forcing/upper stratum -> vegetation | authoritative handoff | precipitation total from another phase/stage |
| downward liquid release | same | interval | vegetation -> lower stratum/ground recipient | authoritative generic transfer | stemflow/drip until separately authorized |
| canopy evaporation | same | interval | vegetation -> atmosphere/LSE | authoritative custody; constitutive gap | transpiration; soil/litter evaporation |
| stratum radiation receipt | interval-integrated `J m^-2`, band/direction/recipient | interval | LSE -> vegetation | authoritative lineage; constitutive gap | ground, litter, snow, soil, ponded-water receipt |
| root-water demand `D_s,l` | interval-integrated `kg m^-2`, hydrology layer | interval | vegetation -> hydrology | authoritative protocol; physiology gap | root-depth scalar; potential `Ep` |
| authorized withdrawal `U_s,l` | same | interval | hydrology -> vegetation | authoritative protocol; allocation-policy gap | demand; aggregate stress scalar; frozen water |
| competing withdrawal `W_comp,l` | interval-integrated `kg m^-2`, same transaction/area/layer basis as `U_s,l` | interval | hydrology-owned competing consumers -> Stage B ledger | required aggregate-admissibility operand; policy gap | omitted competitors; another interval or area basis |
| admissible liquid `A_l` | interval-integrated `kg m^-2`, same-snapshot transaction/area/layer basis | interval | hydrology -> Stage B guard | exact aggregate bound in `sum_s U_s,l + W_comp,l <= A_l` | total liquid store; frozen water; independently computed availability |
| actual transpiration `T_s` | same | interval | vegetation -> LSE/orchestrator | exact sum of `U_s,l` | potential demand; canopy evaporation |
| latent debit `Q_T,s` | interval-integrated `J m^-2` | interval | LSE -> ledger | exact `-h_v*T_s`; `h_v` authority required | other-surface latent; duplicate debit |
| dry-material transfer | interval-integrated `kg dry matter m^-2` | interval/event | vegetation -> residue | exact-once custody | carbon or nitrogen mass |
| carbon transfer | interval-integrated `kg C m^-2` | interval/event | vegetation -> biogeochemistry | exact-once custody | dry matter; atmospheric exchange |
| nitrogen transfer | interval-integrated `kg N m^-2` | interval/event | vegetation -> biogeochemistry | exact-once custody | dry matter/carbon; mineral demand |
| canopy snow store/release | `kg m^-2` water equivalent | interval | vegetation -> snow/frost/LSE | ownership only; `AUTHORITY_MISSING` | ground SWE; snow depth; rain storage |
| compatibility cover/LAI/height/`Ep` | field-specific | after Stage C | read-only adapter -> current consumer | non-authoritative until field reduction + real consumer | universal average; feedback into native state |

Future fixtures assign distinct numeric values to every plausible alias. Both
owners reconstruct accepted transfers from their own state/output surfaces;
producer self-consistency is supporting evidence only.
