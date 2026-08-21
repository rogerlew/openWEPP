# Operand lineage

Status: complete / pre-implementation authority

Evidence mode: Static

The table records the normalization, physical basis, source identity, and
consumer for every Child 2C boundary operand. Digests name the receipt or
contract identity that a later runtime implementation must bind; they are not
claims that a production consumer already exists.

| Operand | Units/basis | Normalization or denominator | Physical basis | Source identity/digest | Producer -> consumer | Status |
|---|---|---|---|---|---|---|
| reference temperature, humidity, pressure | `K`, `kg kg^-1`, `Pa`; sealed forcing interval | none | above-canopy reference node | `SC-SNOWFREEFORCING-001` | forcing -> carrier | authoritative |
| exposure-projected transfer wind | `m s^-1` at virtual 5 m | neutral log-law transfer geometry; roughness `0.005 m` | exposed modeled surface | `SC-SNOWENERGY-001@14`; `exposure-provider-digest` in vector | forcing/exposure -> carrier/V11/Stage 3 | authoritative precondition |
| shared canopy-air temperature/humidity | `K`, `kg kg^-1`; one node | conductance-weighted node denominator `sum(g)` | carrier control node | `SC-VEGETATION-001@26`; `SharedCanopyAirNodeV1` | carrier -> V11/Stage 3 | authoritative |
| sensible exchange `H_i` | `W m^-2`; surface area basis | `rho*c_p*g_H,i` | surface-to-node bulk transfer | Child 2C equation block | carrier -> owning surface | authoritative |
| vapor exchange `V_i` | `kg m^-2 s^-1`; surface area basis | `rho*g_q,i` | surface-to-node bulk transfer | Child 2C equation block | carrier -> owning surface | authoritative |
| canopy component longwave | `W m^-2`; weighted component area | `sum(w_j)=1`; `sigma*T_j^4` | V11 leaf/stem radiating surfaces | `SC-VEGETATION-001@26`; component lineage | V11 -> carrier ledger | authoritative |
| snow/canopy reciprocal longwave | `W m^-2`; surface area basis | complementary `(1-f_sky)` | canopy/snow radiative boundary | `SC-SNOWENERGY-001@14` | radiation owner -> carrier | authoritative |
| snow SWE/liquid/cold content | `kg m^-2`, `kg m^-2`, `J m^-2` | none; area-normalized control volume | Stage 3 snow state | `SC-SNOWENERGY-001@14` | Stage 3 -> carrier/ledger | authoritative |
| snow mass ledger | `kg m^-2`; interval amount | start + solid precip - melt - sublimation + deposition | snow-ice control volume | `state_ledger` vector | terminal state -> oracle | independent reconstruction |
| liquid ledger | `kg m^-2`; interval amount | start + rain + melt - refreeze - runoff | retained surface liquid | `state_ledger` vector | terminal state -> oracle | independent reconstruction |
| vapor net ledger | `kg m^-2`; signed interval amount | deposition - sublimation | snow-vapor boundary | `state_ledger` vector | terminal state -> oracle | independent reconstruction |
| energy ledger | `J m^-2`; interval energy | external + canopy + snow - delta storage | first-law area control volume | `state_ledger` vector | terminal state -> oracle | independent reconstruction |
| event proposed/accepted tick | canonical unsigned `u128 ns` strings | no floating conversion on wire | parent-relative chronology | `SC-COUPLEDTIME-001@3` | terminal solve -> event receipt | authoritative |
| active minimum supports | canonical unsigned `u128 ns` strings | maximum across active physical participants | adopter support domain | Child 2B receipt + participant receipts | owners -> coordinator | authoritative |
| owner beginning/ending digests | canonical identity bytes | exact equality on rollback/restart | complete-owner transaction | `SC-VEGETATIONTRANSACTION-001@15` | candidates -> parent commit | authoritative |
| diagnostic melt alias/raw 10 m wind/copied residual | diagnostic or prohibited | never enters accepted ledger | anti-alias poison | named vector poison fields | any producer -> rejected | prohibited |

The independent reference model reconstructs the node, flux, component
longwave, mass, liquid, energy, event-time, and rollback outcomes from these
separate operands. It does not consume a Rust value, producer residual, or
runtime publication field.
