use cal04b_executor::{arg_value, read_climate};
use openwepp_plant_phenology::{
    ForestCanopyParameters, ForestCanopyState, GsiDailyForcing, GsiDate, GsiParameters,
};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn number(args: &[String], name: &str) -> Result<f64, Box<dyn Error>> {
    Ok(arg_value(args, name)?.parse()?)
}

fn exact(value: f64) -> String {
    format!("{value:.17e}")
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let climate_path = PathBuf::from(arg_value(&args, "--climate")?);
    let output_path = PathBuf::from(arg_value(&args, "--output")?);
    let (latitude, climate) = read_climate(&climate_path)?;
    let parameters = ForestCanopyParameters {
        gsi: GsiParameters {
            minimum_temperature_inactive_c: number(&args, "--tmin-inactive")?,
            minimum_temperature_unconstrained_c: number(&args, "--tmin-unconstrained")?,
            vapor_pressure_deficit_unconstrained_pa: number(&args, "--vpd-unconstrained")?,
            vapor_pressure_deficit_inactive_pa: number(&args, "--vpd-inactive")?,
            photoperiod_inactive_hours: number(&args, "--photo-inactive")?,
            photoperiod_unconstrained_hours: number(&args, "--photo-unconstrained")?,
        },
        summer_foliar_biomass_kg_m2: number(&args, "--bf")?,
        maximum_leaf_area_index: number(&args, "--lai")?,
        evergreen_fraction: number(&args, "--fe")?,
        structural_canopy_cover_fraction: number(&args, "--cs")?,
        structural_biomass_kg_m2: number(&args, "--bs")?,
        canopy_cover_coefficient_m2_kg: number(&args, "--bb")?,
    };
    let mut state = ForestCanopyState::new_uninitialized();
    let mut writer = BufWriter::new(File::create(output_path)?);
    writeln!(writer, "year,ordinal,gsi21,evergreen_biomass,deciduous_biomass,foliar_biomass,structural_biomass,total_aboveground_biomass,lai,cover")?;
    for row in climate {
        let result = state.advance(
            parameters,
            GsiDailyForcing {
                minimum_temperature_c: row.tmin,
                vapor_pressure_deficit_pa: row.vpd,
                latitude_degrees: latitude,
                date: GsiDate {
                    year: row.year,
                    ordinal_day: row.ordinal,
                },
            },
        )?;
        let canopy = result.canopy;
        writeln!(
            writer,
            "{},{},{},{},{},{},{},{},{},{}",
            row.year,
            row.ordinal,
            exact(result.gsi.growing_season_index),
            exact(canopy.evergreen_foliar_biomass_kg_m2),
            exact(canopy.deciduous_foliar_biomass_kg_m2),
            exact(canopy.live_foliar_biomass_kg_m2),
            exact(canopy.structural_biomass_kg_m2),
            exact(canopy.live_foliar_biomass_kg_m2 + canopy.structural_biomass_kg_m2),
            exact(canopy.leaf_area_index),
            exact(canopy.canopy_cover_fraction)
        )?;
    }
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::exact;

    #[test]
    fn exact_csv_render_round_trips_small_fraction_bits() {
        let value = 0.007_482_993_197_278_913_f64;
        let reparsed = exact(value).parse::<f64>().expect("valid exact decimal");
        assert_eq!(reparsed.to_bits(), value.to_bits());
    }
}
