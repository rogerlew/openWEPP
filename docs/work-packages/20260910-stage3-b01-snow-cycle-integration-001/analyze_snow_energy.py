#!/usr/bin/env python3
"""Committed-only diagnostic decomposition; not whole-system closure qualification."""
import argparse
import hashlib
import json
import sqlite3
from fractions import Fraction
from decimal import Decimal
from pathlib import Path
import ijson


def binary64_numbers(value):
    if isinstance(value, Decimal): return float(value)
    if isinstance(value, list): return [binary64_numbers(v) for v in value]
    if isinstance(value, dict): return {k:binary64_numbers(v) for k,v in value.items()}
    return value


def wire(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)


def exact(value):
    return Fraction.from_float(value) if isinstance(value, float) else Fraction(value)


def dyadic(value):
    return value["sign"]*int(value["coefficient_hex"],16)*Fraction(2)**value["exponent2"]


def soil_energy(value, version):
    if version == "v2": return exact(value["enthalpy_hi_j_m2_ofe_ground"])+dyadic(value["enthalpy_carry"])
    if version == "v1": return exact(value["enthalpy_j_m2_ofe_ground"])
    raise ValueError("unknown soil version")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("run", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    args.output.mkdir(exist_ok=False)
    config = json.loads((args.run / "configuration.json").read_text())["lse"]
    fractions = {(ofe["ofe_id"], tile["tile_id"]): exact(tile["fraction_ofe_ground"])
                 for ofe in config["ofes"] for tile in ofe["tiles"]}
    db = sqlite3.connect(args.output / "operands.sqlite")
    db.execute("create table records(kind text,key text,body text,primary key(kind,key))")
    duplicate_counts = {}
    admitted = set()
    reports = []
    with (args.run / "observations.json").open("rb") as source:
        for row in ijson.items(source, "physical.item"):
            row = binary64_numbers(row)
            kind = row["kind"]
            if kind in ("soil_trial_primitive", "candidate_subslab_primitives"):
                row.pop("capture", None)
                key = row["receipt"]["receipt_sha256"] if kind == "soil_trial_primitive" else row["receipt_sha256"]
                body = wire(row)
                previous = db.execute("select body from records where kind=? and key=?", (kind,key)).fetchone()
                if previous is not None:
                    if previous[0] != body: raise ValueError(f"conflicting {kind} {key}")
                    duplicate_counts[kind] = duplicate_counts.get(kind,0)+1
                else: db.execute("insert into records values(?,?,?)", (kind,key,body))
            elif kind == "accepted_day":
                day = row["day_index"]
                report = {"day_index":day,"subslabs":0,"lanes":{},"soil_trial_joins":0}
                for identity in row["accepted_subslab_identities"]:
                    key=identity["receipt_sha256"]
                    if key in admitted: raise ValueError("duplicate committed subslab")
                    admitted.add(key)
                    result=db.execute("select body from records where kind=? and key=?",("candidate_subslab_primitives",key)).fetchone()
                    if result is None: raise ValueError(f"missing committed primitive {key}")
                    candidate=json.loads(result[0])
                    for field in identity:
                        if candidate[field] != identity[field]: raise ValueError(f"candidate identity {field}")
                    if candidate["day_index"] != day: raise ValueError("candidate day")
                    report["subslabs"]+=1
                    support=candidate["support"]
                    dt=Fraction(int(support["end_ns"])-int(support["start_ns"]),10**9)
                    for lane,ledger in candidate["physical_outcome_ledgers"].items():
                        totals=report["lanes"].setdefault(lane,{"supports":0,"duration_s":0,"energy_j_m2":{},"maximum_local_mass_residual_kg_m2":0,"maximum_local_ice_reference_energy_residual_j_m2":0,"maximum_destination_operand_difference_j_m2":0})
                        totals["supports"]+=1;totals["duration_s"]+=float(dt)
                        names=["shortwave_j_m2","longwave_j_m2","sensible_j_m2","latent_j_m2","soil_heat_j_m2","precipitation_advection_j_m2","vapor_material_enthalpy_j_m2","interlayer_conduction_j_m2","refreeze_fusion_j_m2"]
                        for name in names:totals["energy_j_m2"][name]=totals["energy_j_m2"].get(name,0)+ledger[name]
                        get=lambda name:exact(ledger[name])
                        mass=get("ending_ice_kg_m2")+get("ending_liquid_kg_m2")-get("beginning_ice_kg_m2")-get("beginning_liquid_kg_m2")-get("solid_precipitation_kg_m2")-get("liquid_precipitation_kg_m2")-get("deposition_kg_m2")+get("sublimation_kg_m2")+get("terminal_liquid_kg_m2")
                        # Ice-at-0C reference for this local diagnostic only.
                        energy=-get("ending_cold_content_j_m2")+333600*get("ending_liquid_kg_m2")+get("beginning_cold_content_j_m2")-333600*get("beginning_liquid_kg_m2")-sum(get(n) for n in names)-333600*(get("liquid_precipitation_kg_m2")-get("terminal_liquid_kg_m2"))+get("terminal_liquid_sensible_enthalpy_j_m2")
                        totals["maximum_local_mass_residual_kg_m2"]=max(totals["maximum_local_mass_residual_kg_m2"],float(abs(mass)))
                        totals["maximum_local_ice_reference_energy_residual_j_m2"]=max(totals["maximum_local_ice_reference_energy_residual_j_m2"],float(abs(energy)))
                        primitive={n:Fraction(0) for n in ["shortwave_j_m2","longwave_j_m2","sensible_j_m2","latent_j_m2"]}
                        for destination,receipt in candidate["destination_receipts"]:
                            if destination[0] != ledger["ofe_id"]:continue
                            area=fractions[tuple(destination)]
                            if "V11Canopy" in receipt:
                                value=receipt["V11Canopy"];h=value["sensible_to_canopy_air_w_m2"];le=value["latent_energy_to_canopy_air_j_m2"]
                            elif "OpenSnow" in receipt:
                                value=receipt["OpenSnow"]["candidate"];h=value["sensible_outward_w_m2"];le=value["latent_energy_outward_j_m2"]
                            else:raise ValueError("unknown destination class")
                            primitive["shortwave_j_m2"]+=area*dt*exact(value["snow_absorbed_shortwave_w_m2"])
                            primitive["longwave_j_m2"]+=area*dt*exact(value["snow_net_longwave_w_m2"])
                            primitive["sensible_j_m2"]-=area*dt*exact(h)
                            primitive["latent_j_m2"]-=area*exact(le)
                        for name,value in primitive.items():
                            totals["maximum_destination_operand_difference_j_m2"]=max(totals["maximum_destination_operand_difference_j_m2"],float(abs(value-get(name))))
                        soil_source=ledger["source_receipts_sha256"][4]
                        result=db.execute("select body from records where kind=? and key=?",("soil_trial_primitive",soil_source)).fetchone()
                        if result is not None:
                            trial=json.loads(result[0]);receipt=trial["receipt"]
                            if receipt["support"] != support or receipt["lane_id"] != int(lane):raise ValueError("soil trial identity")
                            if exact(receipt["snow_heat_j_m2"])+exact(receipt["soil_heat_j_m2"]) != 0:raise ValueError("soil heat pair")
                            version,begin=next(iter(trial["beginning_soil"].items()))
                            end=trial["ending_soil"][version]
                            actual_version,actual_owner=next(iter(candidate["ending_soil"].items()))
                            ofes=[v for v in actual_owner["ofes"] if v["ofe_id"]==ledger["ofe_id"]]
                            if len(ofes)!=1:raise ValueError("actual soil OFE identity")
                            layers=[v for v in ofes[0]["ordered_layers"] if v["layer_id"]==begin["layer_id"]]
                            if len(layers)!=1:raise ValueError("actual soil layer identity")
                            cross_side = get("soil_heat_j_m2") - exact(receipt["snow_heat_j_m2"])
                            totals["maximum_snow_ledger_vs_soil_receipt_j_m2"] = max(totals.get("maximum_snow_ledger_vs_soil_receipt_j_m2",0), float(abs(cross_side)))
                            q=exact(receipt["soil_heat_j_m2"])
                            exact_update=soil_energy(end,version)-soil_energy(begin,version)-q
                            actual_update=soil_energy(layers[0],actual_version)-soil_energy(end,version)
                            for label,value in [("maximum_exact_trial_soil_update_residual_j_m2",exact_update),("maximum_actual_recipient_vs_trial_j_m2",actual_update)]:
                                totals[label]=max(totals.get(label,0),float(abs(value)))
                            if receipt["experimental_bulk"] is None:
                                snow=trial["snow_inputs"];soil=trial["soil_configuration"]
                                resistance=exact(snow["depth_m"])/(2*exact(trial["snow_conductivity_w_m_k"]))+exact(soil["thickness_m"])/(2*exact(soil["thermal_conductivity_w_m_k"]))
                                cs=2100*exact(snow["ice_kg_m2"]);cg=exact(soil["areal_heat_capacity_j_m2_k"])
                                predicted=dt*(exact(snow["temperature_k"])-exact(begin["temperature_k"]))/resistance/(1+dt/(2*resistance)*(1/cs+1/cg))
                                totals["maximum_independent_CN_heat_difference_j_m2"]=max(totals.get("maximum_independent_CN_heat_difference_j_m2",0),float(abs(predicted-q)))
                            report["soil_trial_joins"]+=1
                        else:
                            retained=[v for name in ["snow_soil_heat_receipts","terminal_snow_soil_heat_receipts","adaptive_terminal_snow_soil_heat_receipts"] for v in candidate[name].values()]
                            if not any(v["receipt_sha256"]==soil_source for v in retained): raise ValueError(f"unresolved soil source {soil_source}")
                reports.append(report)
                db.commit()
    receipt=json.loads((args.run/"receipt.json").read_text())
    result={"schema":"committed_snow_energy_diagnostic_v1","qualification":"NOT_WHOLE_SYSTEM_CLOSURE","run_receipt_sha256":hashlib.file_digest((args.run/"receipt.json").open("rb"),"sha256").hexdigest(),"execution_valid":receipt["execution_valid"],"duplicate_candidates":duplicate_counts,"days":reports}
    (args.output/"diagnostic.json").write_text(json.dumps(result,indent=2)+"\n")
    print(json.dumps(result,indent=2))

if __name__=="__main__":main()
