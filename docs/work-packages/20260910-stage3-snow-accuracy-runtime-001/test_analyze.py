import unittest
import copy
import analyze


class IndependentPhysicalArithmetic(unittest.TestCase):
    def test_root_water_counts_residual_liquid_and_ice_once(self):
        layer={"theta_m":0.02,"frozen_depth_m":0.05,"frozen_water_m":0.01}
        parameters={"depth_m":0.2,"residual_theta":0.1}
        liquid,ice=analyze.layer_storage(layer,parameters,0.2)
        self.assertAlmostEqual(liquid,35.0)
        self.assertAlmostEqual(ice,10.0)
        half=analyze.layer_storage(layer,parameters,0.1)
        self.assertAlmostEqual(sum(half),22.5)

    def test_independent_melt_energy_and_poison(self):
        row={k:0.0 for k in ("beginning_liquid_kg_m2","ending_liquid_kg_m2",
             "beginning_cold_content_j_m2","ending_cold_content_j_m2",
             "solid_precipitation_kg_m2","liquid_precipitation_kg_m2",
             "deposition_kg_m2","sublimation_kg_m2","terminal_liquid_sensible_enthalpy_j_m2",
             "longwave_j_m2","sensible_j_m2","latent_j_m2","vapor_material_enthalpy_j_m2",
             "precipitation_advection_j_m2","soil_heat_j_m2",
             "interlayer_active_conduction_j_m2","interlayer_lower_conduction_j_m2")}
        row.update(beginning_ice_kg_m2=1.0,ending_ice_kg_m2=0.75,
                   terminal_liquid_kg_m2=0.25,shortwave_j_m2=83_400.0)
        good=analyze.ledger_balance(row)
        self.assertEqual(good["mass_kg_m2"],0.0)
        self.assertEqual(good["energy_j_m2"],0.0)
        row["shortwave_j_m2"]+=0.1
        bad=analyze.ledger_balance(row)
        self.assertGreater(abs(bad["energy_j_m2"]),bad["energy_allowance"])

    def test_output_crossbinding_poison(self):
        support={"start_ns":"0","end_ns":"60000000000"}
        ledger={"receipt_sha256":"ledger","support":support,"lane_id":1,"ofe_id":"ofe-1",
          "beginning_snow_owner_sha256":"begin","ending_snow_owner_sha256":"end",
          "source_receipts_sha256":["source"],"terminal_liquid_kg_m2":1.0,
          "terminal_liquid_sensible_enthalpy_j_m2":2.0,"refreeze_kg_m2":0.0}
        output={k:v for k,v in ledger.items() if k in ("support","lane_id","ofe_id",
          "beginning_snow_owner_sha256","ending_snow_owner_sha256","source_receipts_sha256")}
        output.update(physical_ledger_receipt_sha256="ledger",mass_kg_m2_ofe_ground=1.0,
          sensible_enthalpy_j_m2_ofe_ground=2.0,refreeze_kg_m2_ofe_ground=0.0,
          destinations=[{"tile_id":"open","ofe_id":"ofe-1","tile_fraction":1.0,
            "mass_kg_m2_tile_ground":1.0,"sensible_enthalpy_j_m2_tile_ground":2.0}])
        configuration={"ofe_id":"ofe-1","tiles":[{"tile_id":"open","fraction_ofe_ground":1.0}]}
        analyze.bind_output("ledger",ledger,output,support,configuration)
        for field, value in (("lane_id",2),("beginning_snow_owner_sha256","foreign"),
          ("source_receipts_sha256",[]),("mass_kg_m2_ofe_ground",float("nan")),
          ("refreeze_kg_m2_ofe_ground",1.0)):
            bad=copy.deepcopy(output);bad[field]=value
            with self.assertRaises(ValueError):analyze.bind_output("ledger",ledger,bad,support,configuration)
        bad=copy.deepcopy(output)
        bad["mass_kg_m2_ofe_ground"]=3.0;bad["destinations"][0]["mass_kg_m2_tile_ground"]=3.0
        with self.assertRaises(ValueError):analyze.bind_output("ledger",ledger,bad,support,configuration)
        for field,value in (("tile_fraction",0.5),("tile_id","foreign"),("ofe_id","foreign")):
            bad=copy.deepcopy(output);bad["destinations"][0][field]=value
            with self.assertRaises(ValueError):analyze.bind_output("ledger",ledger,bad,support,configuration)
        bad=copy.deepcopy(output);bad["destinations"]*=2
        with self.assertRaises(ValueError):analyze.bind_output("ledger",ledger,bad,support,configuration)
        with self.assertRaises(ValueError):analyze.bind_output("wrong",ledger,output,support,configuration)

    def test_missing_and_nonfinite_are_not_zero(self):
        for value in (None,True,float("nan"),float("inf")):
            with self.assertRaises(ValueError):analyze.number(value)
        self.assertEqual(analyze.number("0x3ff0000000000000"),1.0)


if __name__=="__main__":unittest.main()
