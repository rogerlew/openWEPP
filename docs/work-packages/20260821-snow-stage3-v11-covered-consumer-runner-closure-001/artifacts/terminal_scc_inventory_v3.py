#!/usr/bin/env python3
"""Executable exact-component SCC and follower inventory candidate."""

from __future__ import annotations

import json
from pathlib import Path

COMPONENTS = [
    {"id":"snow.ice","units":"kg m^-2","cardinality":"N_lane","index_order":"lane_id ascending","storage_map":"identity","residual":"end-begin-prescribed_solid-deposition+sub+melt-refreeze","active_tag":"snow_phase","forcing_class":"storage","order":0,"tolerance":"1e-9 kg m^-2","counterpart":"surface_liquid.generated_melt","followers":["receipt.snow"]},
    {"id":"snow.liquid","units":"kg m^-2","cardinality":"N_lane","index_order":"lane_id ascending","storage_map":"identity","residual":"end-begin-rain-melt+refreeze+route","active_tag":"snow_phase","forcing_class":"storage","order":1,"tolerance":"1e-9 kg m^-2","counterpart":"surface_liquid.generated_melt","followers":["receipt.snow"]},
    {"id":"snow.enthalpy","units":"J m^-2","cardinality":"N_lane","index_order":"lane_id ascending","storage_map":"cold_content_material_enthalpy","residual":"end-begin-prescribed_advection-integrated_rates-generated_phase_energy","active_tag":"snow_phase","forcing_class":"storage","order":2,"tolerance":"1e-6 J m^-2","counterpart":"soil.enthalpy","followers":["receipt.snow"]},
    {"id":"soil.enthalpy","units":"J m^-2","cardinality":"sum_ofe N_soil_layer(ofe)","index_order":"ofe_id then layer ascending","storage_map":"soil_contract_enthalpy","residual":"end-begin-internal_conduction-external_bottom","active_tag":"soil_phase","forcing_class":"storage","order":3,"tolerance":"1e-6 J m^-2","counterpart":"snow.enthalpy","followers":["receipt.soil"]},
    {"id":"vegetation.liquid","units":"kg m^-2","cardinality":"N_vegetation_component","index_order":"ofe_id,tile_id,stratum_id,component_id","storage_map":"identity","residual":"end-begin-prescribed_rain-generated_release-vapor","active_tag":"wet_dry","forcing_class":"storage","order":4,"tolerance":"1e-9 kg m^-2","counterpart":"surface_liquid.storage","followers":["bgc.transition","receipt.vegetation"]},
    {"id":"vegetation.temperature","units":"K","cardinality":"N_vegetation_component","index_order":"ofe_id,tile_id,stratum_id,component_id","storage_map":"heat_capacity_enthalpy","residual":"storage_delta-integrated_radiative_sensible_latent","active_tag":"wet_dry","forcing_class":"storage","order":5,"tolerance":"1e-9 K","counterpart":"carrier.air_temperature","followers":["bgc.transition","receipt.vegetation"]},
    {"id":"lse.surface_temperature","units":"K","cardinality":"N_ofe_tile","index_order":"ofe_id,tile_id","storage_map":"algebraic","residual":"net_radiation-sensible-latent-ground","active_tag":"surface_route","forcing_class":"algebraic","order":6,"tolerance":"1e-9 K","counterpart":"snow.enthalpy","followers":["receipt.lse"]},
    {"id":"carrier.air_temperature","units":"K","cardinality":"N_carrier_node","index_order":"canonical topology ordinal","storage_map":"algebraic","residual":"ordered_sensible_flux_sum","active_tag":"carrier_topology","forcing_class":"algebraic","order":7,"tolerance":"1e-9 K","counterpart":"vegetation.temperature","followers":["receipt.carrier"]},
    {"id":"carrier.specific_humidity","units":"kg kg^-1","cardinality":"N_carrier_node","index_order":"canonical topology ordinal","storage_map":"algebraic","residual":"ordered_vapor_flux_sum","active_tag":"vapor_direction","forcing_class":"algebraic","order":8,"tolerance":"1e-12 kg kg^-1","counterpart":"vegetation.liquid","followers":["receipt.carrier"]},
    {"id":"surface_liquid.storage","units":"kg m^-2","cardinality":"N_ofe_tile","index_order":"ofe_id,tile_id","storage_map":"identity","residual":"end-begin-generated_ingress-infiltration-runoff-runon","active_tag":"surface_route","forcing_class":"storage","order":9,"tolerance":"1e-9 kg m^-2","counterpart":"hydrology.water","followers":["receipt.surface_liquid"]},
    {"id":"hydrology.water","units":"kg m^-2","cardinality":"sum_ofe N_hydrology_layer(ofe)","index_order":"ofe_id then hydrology layer ascending","storage_map":"water_depth_times_density","residual":"end-begin-infiltration-root_supply-drainage","active_tag":"wet_dry","forcing_class":"storage","order":10,"tolerance":"1e-9 kg m^-2","counterpart":"surface_liquid.storage","followers":["receipt.hydrology"]},
    {"id":"bgc.transition","units":"exact","cardinality":"sum_ofe N_bgc_pool(ofe)","index_order":"ofe_id then canonical pool ordinal","storage_map":"follower","residual":"none_exact_beginning_pool_transition","active_tag":"mineral_feedback_absent","forcing_class":"follower","order":11,"tolerance":"exact","counterpart":"none","followers":["receipt.bgc"]},
]

EDGES = {
    "snow.ice":["snow.enthalpy","lse.surface_temperature"], "snow.liquid":["snow.enthalpy","surface_liquid.storage"],
    "snow.enthalpy":["snow.ice","snow.liquid","lse.surface_temperature","soil.enthalpy"], "soil.enthalpy":["snow.enthalpy","hydrology.water"],
    "vegetation.liquid":["vegetation.temperature","carrier.specific_humidity","surface_liquid.storage"], "vegetation.temperature":["vegetation.liquid","carrier.air_temperature","lse.surface_temperature"],
    "lse.surface_temperature":["snow.enthalpy","vegetation.temperature","carrier.air_temperature","carrier.specific_humidity","soil.enthalpy"],
    "carrier.air_temperature":["vegetation.temperature","lse.surface_temperature"], "carrier.specific_humidity":["vegetation.liquid","lse.surface_temperature"],
    "surface_liquid.storage":["hydrology.water"], "hydrology.water":["surface_liquid.storage","soil.enthalpy"], "bgc.transition":[],
}


def strongly_connected_components() -> list[list[str]]:
    index, stack, indices, low, on_stack, output = 0, [], {}, {}, set(), []
    def visit(node: str) -> None:
        nonlocal index
        indices[node] = low[node] = index; index += 1; stack.append(node); on_stack.add(node)
        for target in EDGES[node]:
            if target not in indices: visit(target); low[node] = min(low[node], low[target])
            elif target in on_stack: low[node] = min(low[node], indices[target])
        if low[node] == indices[node]:
            component = []
            while True:
                current = stack.pop(); on_stack.remove(current); component.append(current)
                if current == node: break
            output.append(sorted(component))
    for node in EDGES:
        if node not in indices: visit(node)
    return output


def main() -> None:
    ids = [component["id"] for component in COMPONENTS]
    required = {"id","units","cardinality","index_order","storage_map","residual","active_tag","forcing_class","order","tolerance","counterpart","followers"}
    schema_complete = len(ids) == len(set(ids)) and all(set(component) == required for component in COMPONENTS)
    graph_complete = set(ids) == set(EDGES) and all(target in EDGES for targets in EDGES.values() for target in targets)
    order_exact = [component["order"] for component in COMPONENTS] == list(range(len(COMPONENTS)))
    sccs = strongly_connected_components()
    result = {"schema":"openwepp-terminal-scc-inventory-v3","components":COMPONENTS,"edges":EDGES,"sccs":sccs,"schema_complete":schema_complete,"graph_complete":graph_complete,"order_exact":order_exact,"pass":schema_complete and graph_complete and order_exact}
    here = Path(__file__).resolve().parent
    (here / "terminal-scc-inventory-v3.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    (here / "terminal-scc-inventory-v3.md").write_text("# Terminal executable SCC inventory v3\n\nRan: package-local inventory validator and Tarjan SCC calculation.\n\n" + f"- pass: `{result['pass']}`\n- exact ordered components: `{len(COMPONENTS)}`\n- SCCs: `{json.dumps(sccs)}`\n- follower: `bgc.transition` (no feedback edge)\n")
    if not result["pass"]: raise SystemExit("SCC inventory validation failed")


if __name__ == "__main__":
    main()
