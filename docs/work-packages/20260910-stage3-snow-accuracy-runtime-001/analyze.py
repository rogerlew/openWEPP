#!/usr/bin/env python3
"""Physical projections and independent arithmetic; absent evidence stays unresolved."""
import argparse
import hashlib
import json
import math
from pathlib import Path
import struct

FUSION = 333_600.0


def number(value):
    if isinstance(value, str):
        value = struct.unpack(">d", bytes.fromhex(value.removeprefix("0x")))[0]
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        raise ValueError("missing/nonfinite physical number")
    return value


def layer_storage(layer, parameters, overlap):
    depth = number(parameters["depth_m"])
    fraction = overlap / depth
    liquid = number(layer["theta_m"]) + number(parameters["residual_theta"]) * max(
        depth - number(layer["frozen_depth_m"]), 0.0)
    frozen = number(layer["frozen_water_m"])
    return 1000 * fraction * liquid, 1000 * fraction * frozen


def ledger_balance(row):
    """Reconstruct storage and boundary sums, never use producer residual fields."""
    v = {k: number(x) for k, x in row.items() if isinstance(x, (int, float))}
    m_terms = [v["ending_ice_kg_m2"], v["ending_liquid_kg_m2"],
               -v["beginning_ice_kg_m2"], -v["beginning_liquid_kg_m2"],
               -v["solid_precipitation_kg_m2"], -v["liquid_precipitation_kg_m2"],
               -v["deposition_kg_m2"], v["sublimation_kg_m2"], v["terminal_liquid_kg_m2"]]
    # Reconstruct H from cold content and liquid, not supplied enthalpy/residual.
    e_terms = [-v["ending_cold_content_j_m2"], FUSION*v["ending_liquid_kg_m2"],
               v["beginning_cold_content_j_m2"], -FUSION*v["beginning_liquid_kg_m2"],
               FUSION*(v["terminal_liquid_kg_m2"]-v["liquid_precipitation_kg_m2"]),
               v["terminal_liquid_sensible_enthalpy_j_m2"]]
    e_terms += [-v[k] for k in ("shortwave_j_m2", "longwave_j_m2", "sensible_j_m2",
        "latent_j_m2", "vapor_material_enthalpy_j_m2", "precipitation_advection_j_m2",
        "soil_heat_j_m2", "interlayer_active_conduction_j_m2", "interlayer_lower_conduction_j_m2")]
    mass, energy = math.fsum(m_terms), math.fsum(e_terms)
    return {"mass_kg_m2": mass, "energy_j_m2": energy,
            "mass_allowance": max(1e-9, 1e-12*math.fsum(map(abs, m_terms))),
            "energy_allowance": max(1e-6, 1e-12*math.fsum(map(abs, e_terms)))}


def bind_output(key, ledger, output, support, configuration):
    if key != ledger["receipt_sha256"] or key != output["physical_ledger_receipt_sha256"]:
        raise ValueError("foreign ledger hash")
    for field in ("support", "lane_id", "ofe_id", "beginning_snow_owner_sha256",
                  "ending_snow_owner_sha256", "source_receipts_sha256"):
        if output[field] != ledger[field]:
            raise ValueError("foreign output/ledger " + field)
    if output["support"] != support or output["ofe_id"] != configuration["ofe_id"]:
        raise ValueError("foreign accepted support/topology")
    for public, primitive, tolerance in (
        ("mass_kg_m2_ofe_ground", "terminal_liquid_kg_m2", 1e-9),
        ("sensible_enthalpy_j_m2_ofe_ground", "terminal_liquid_sensible_enthalpy_j_m2", 1e-6),
        ("refreeze_kg_m2_ofe_ground", "refreeze_kg_m2", 1e-9)):
        if abs(number(output[public])-number(ledger[primitive])) > tolerance:
            raise ValueError("published transfer differs from physical operand")
    tiles = {t["tile_id"]: number(t["fraction_ofe_ground"]) for t in configuration["tiles"]}
    destinations = output["destinations"]
    ids = [d["tile_id"] for d in destinations]
    if len(set(ids)) != len(ids) or set(ids) != set(tiles):
        raise ValueError("missing/duplicate/foreign destination")
    for d in destinations:
        if d["ofe_id"] != output["ofe_id"] or number(d["tile_fraction"]) != tiles[d["tile_id"]]:
            raise ValueError("foreign destination topology")
        for field in ("mass_kg_m2_tile_ground", "sensible_enthalpy_j_m2_tile_ground"):
            number(d[field])
    for tile_field, aggregate, tolerance in (
        ("mass_kg_m2_tile_ground", "mass_kg_m2_ofe_ground", 1e-9),
        ("sensible_enthalpy_j_m2_tile_ground", "sensible_enthalpy_j_m2_ofe_ground", 1e-6)):
        reconstructed = math.fsum(d["tile_fraction"]*d[tile_field] for d in destinations)
        if abs(reconstructed-number(output[aggregate])) > tolerance:
            raise ValueError("destination mass/energy reconstruction")


def project(directory):
    directory = Path(directory)
    process = json.loads((directory / "receipt.json").read_text())
    result = {"schema": "snow_accuracy_physical_projection_v1", "policy": process["policy"],
              "case": process["case"], "execution_valid": process["execution_valid"],
              "provenance":{"process_receipt_sha256":hashlib.sha256((directory/"receipt.json").read_bytes()).hexdigest(),
                  "analyzer_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest()},
              "days": [], "audits": [], "unresolved": [
                  "melt-derived portion of delivered snow liquid (rain mixing)",
                  "fine-grid routed outlet peak and timing", "independent complete-system soil/atmospheric energy audit",
                  "snow-liquid receiver credit beyond sealed destination allocation"]}
    if not process["execution_valid"]:
        result["disposition"] = "EXECUTION_INVALID"
        return result
    for name, expected in process["artifact_sha256"].items():
        if hashlib.sha256((directory/name).read_bytes()).hexdigest() != expected:
            raise ValueError("changed admitted artifact: " + name)
    run = json.loads((directory / "run.json").read_text())
    obs = json.loads((directory / "observations.json").read_text())
    fixture = Path((directory / "fixture-path.txt").read_text().strip())
    admitted_inputs = json.loads((directory/"inputs.json").read_text())["files"]
    for name, expected in admitted_inputs.items():
        if hashlib.sha256((fixture/name).read_bytes()).hexdigest() != expected:
            raise ValueError("changed admitted input: " + name)
    seed = json.loads((fixture / "case.run.snow_stage3_v11_owner_seed.json").read_text())
    initial_hydrology = seed["checkpoint"]["phase"]["committed"]["scientific"]["direct_hydrology"]
    parameters = {l["lane_id"]: l["subsurface_layers"] for l in initial_hydrology["lanes"]}
    configurations = {o["ofe_id"]: o for o in seed["lse_configuration"]["ofes"]}
    days = [r for r in obs["physical"] if r.get("kind") == "accepted_day"]
    if [r["day_index"] for r in days] != list(range(run["days_requested"])):
        raise ValueError("daily observation coverage mismatch")
    ledgers = obs.get("physical_ledgers")
    if not isinstance(ledgers, dict):
        raise ValueError("physical ledgers absent")
    seen = set()
    previous_snow = {}
    for day in days:
        owners, transfers = day["owners"], day["transfers"]
        cursor = day["day_index"] * 86_400_000_000_000
        release = {}
        for support in transfers:
            start, end = (int(support["support"][k]) for k in ("start_ns", "end_ns"))
            if start != cursor or end <= start:
                raise ValueError("accepted support gap/overlap")
            cursor = end
            for output in support["snow_liquid_outputs"]:
                key = output["physical_ledger_receipt_sha256"]
                if key in seen or key not in ledgers:
                    raise ValueError("missing/duplicate accepted physical ledger")
                seen.add(key)
                ledger = ledgers[key]
                bind_output(key, ledger, output, support["support"], configurations[output["ofe_id"]])
                audit = ledger_balance(ledger)
                audit.update(day=day["day_index"], ofe=output["ofe_id"], receipt=key,
                             support_start_ns=start, support_end_ns=end)
                audit["pass"] = abs(audit["mass_kg_m2"])<=audit["mass_allowance"] and abs(audit["energy_j_m2"])<=audit["energy_allowance"]
                result["audits"].append(audit)
                release[output["lane_id"]] = release.get(output["lane_id"], 0.0)+number(output["mass_kg_m2_ofe_ground"])
        if cursor != (day["day_index"]+1)*86_400_000_000_000:
            raise ValueError("incomplete physical day")
        snow = dict(day["snow"]["lanes"])
        hydro = {l["lane_id"]:l for l in owners["hydrology"]["lanes"]}
        soil = {o["ofe_id"]:o for o in owners["soil_thermal"]["owner"]["state"]["ofes"]}
        if len(hydro)!=run["ofes"] or set(snow)!=set(hydro):
            raise ValueError("lane physical coverage")
        for lane, s in snow.items():
            ofe = f"ofe-{lane}"
            values = {"swe_mm": 1000*math.fsum(number(l["mass_swe_m"])+number(l["liquid_water_m"]) for l in s["layers"])+number(s["detached_retained_liquid_kg_m2"]),
                      "snow_liquid_allocated_mm":release.get(lane,0.0)}
            for field, label in (("cumulative_melt_kg_m2","generated_melt_mm"),
                                 ("cumulative_sublimation_kg_m2","sublimation_mm"),
                                 ("cumulative_deposition_kg_m2","deposition_mm")):
                values[label]=number(s[field])-number(previous_snow.get(lane,{}).get(field,0.0))
                values["cumulative_"+label]=number(s[field])
            top, liquid, frozen = 0.0, 0.0, 0.0
            for layer, param in zip(hydro[lane]["subsurface_layers"],parameters[lane],strict=True):
                depth=number(param["depth_m"])
                overlap=max(0.0,min(top+depth,0.4)-top)
                a,b=layer_storage(layer,param,overlap)
                liquid+=a;frozen+=b;top+=depth
            if top<0.4:
                raise ValueError("root reporting zone unavailable")
            values.update(root_liquid_mm=liquid,root_frozen_mm=frozen,root_total_mm=liquid+frozen)
            top=0.0
            for layer, config in zip(soil[ofe]["ordered_layers"],configurations[ofe]["soil_interface_layers"],strict=True):
                if layer["layer_id"]!=config["layer_id"]:
                    raise ValueError("soil node identity mismatch")
                depth=number(config["thickness_m"])
                values[f"soil_temperature_K_at_{top+depth/2:.6f}m"]=number(layer["temperature_k"])
                top+=depth
            result["days"].append({"day":day["day_index"],"lane":lane,"ofe":ofe,"values":values})
        previous_snow=snow
    result["disposition"]="PARTIAL_PHYSICAL_AUDIT" if all(a["pass"] for a in result["audits"]) else "PHYSICAL_AUDIT_FAIL"
    return result


if __name__ == "__main__":
    parser=argparse.ArgumentParser()
    parser.add_argument("directory",type=Path)
    parser.add_argument("output",type=Path)
    args=parser.parse_args()
    args.output.write_text(json.dumps(project(args.directory),indent=2)+"\n")
