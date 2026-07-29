use openwepp_plant_phenology::{
    ForestCanopyParameters, ForestCanopyState, GsiDailyForcing, GsiDate, GsiParameters,
};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone)]
struct Member {
    id: String,
    gsi: GsiParameters,
}

#[derive(Clone)]
struct Forcing {
    site: String,
    date: String,
    year: i32,
    doy: u16,
    latitude: f64,
    tmax: f64,
    tmin: f64,
    tdew: f64,
}

fn parse_f64(value: &str) -> Result<f64, Box<dyn Error>> {
    Ok(value.parse()?)
}

fn read_members(path: &Path) -> Result<Vec<Member>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    input
        .lines()
        .skip(1)
        .map(|line| {
            let fields: Vec<_> = line.split(',').collect();
            if fields.len() != 7 {
                return Err(format!("invalid ensemble row: {line}").into());
            }
            Ok(Member {
                id: fields[0].to_owned(),
                gsi: GsiParameters {
                    minimum_temperature_inactive_c: parse_f64(fields[1])?,
                    minimum_temperature_unconstrained_c: parse_f64(fields[2])?,
                    vapor_pressure_deficit_unconstrained_pa: parse_f64(fields[3])?,
                    vapor_pressure_deficit_inactive_pa: parse_f64(fields[4])?,
                    photoperiod_inactive_hours: parse_f64(fields[5])?,
                    photoperiod_unconstrained_hours: parse_f64(fields[6])?,
                },
            })
        })
        .collect()
}

fn read_forcing(path: &Path) -> Result<Vec<Forcing>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    input
        .lines()
        .skip(1)
        .map(|line| {
            let fields: Vec<_> = line.split(',').collect();
            if fields.len() != 8 {
                return Err(format!("invalid forcing row: {line}").into());
            }
            Ok(Forcing {
                site: fields[0].to_owned(),
                date: fields[1].to_owned(),
                year: fields[2].parse()?,
                doy: fields[3].parse()?,
                latitude: parse_f64(fields[4])?,
                tmax: parse_f64(fields[5])?,
                tmin: parse_f64(fields[6])?,
                tdew: parse_f64(fields[7])?,
            })
        })
        .collect()
}

fn saturation_vapor_pressure_kpa(temperature_c: f64) -> f64 {
    0.6108 * (17.27 * temperature_c / (temperature_c + 237.3)).exp()
}

fn vpd_pa(forcing: &Forcing) -> Result<f64, Box<dyn Error>> {
    let value = 1_000.0
        * (0.5
            * (saturation_vapor_pressure_kpa(forcing.tmax)
                + saturation_vapor_pressure_kpa(forcing.tmin))
            - saturation_vapor_pressure_kpa(forcing.tdew));
    if !value.is_finite() || value < 0.0 {
        return Err(format!("invalid VPD for {} {}", forcing.site, forcing.date).into());
    }
    Ok(value)
}

fn parameters(member: &Member, evergreen_fraction: f64) -> ForestCanopyParameters {
    ForestCanopyParameters {
        gsi: member.gsi,
        summer_foliar_biomass_kg_m2: 0.8,
        maximum_leaf_area_index: 5.0,
        evergreen_fraction,
        structural_canopy_cover_fraction: 0.2,
        structural_biomass_kg_m2: 6.0,
        canopy_cover_coefficient_m2_kg: 2.0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        return Err("usage: cal07-executor ENSEMBLE FORCING OUTPUT".into());
    }
    let members = read_members(Path::new(&args[1]))?;
    let forcing = read_forcing(Path::new(&args[2]))?;
    if members.len() != 37 {
        return Err(format!("expected 37 members, found {}", members.len()).into());
    }
    let mut output = String::from(
        "site_id,candidate_id,date,year,doy,vpd_pa,gsi,foliar_activity_fraction,live_foliar_biomass_kg_m2,leaf_on_allocation_kg_m2,leaf_off_litter_kg_m2,mass_closure_residual_kg_m2\n",
    );
    for member in &members {
        for site in ["SH-DB-BEZA", "SH-EN-ALERCE"] {
            let evergreen_fraction = if site == "SH-EN-ALERCE" { 1.0 } else { 0.0 };
            let mut state = ForestCanopyState::new_uninitialized();
            for day in forcing.iter().filter(|day| day.site == site) {
                let vpd = vpd_pa(day)?;
                let result = state.advance(
                    parameters(member, evergreen_fraction),
                    GsiDailyForcing {
                        minimum_temperature_c: day.tmin,
                        vapor_pressure_deficit_pa: vpd,
                        latitude_degrees: day.latitude,
                        date: GsiDate {
                            year: day.year,
                            ordinal_day: day.doy,
                        },
                    },
                )?;
                let canopy = result.canopy;
                let reconstructed = canopy.previous_foliar_biomass_kg_m2
                    + canopy.leaf_on_allocation_kg_m2
                    - canopy.leaf_off_litter_kg_m2;
                let residual = reconstructed - canopy.live_foliar_biomass_kg_m2;
                output.push_str(&format!(
                    "{},{},{},{},{},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17e}\n",
                    site,
                    member.id,
                    day.date,
                    day.year,
                    day.doy,
                    vpd,
                    result.gsi.growing_season_index,
                    canopy.foliar_activity_fraction,
                    canopy.live_foliar_biomass_kg_m2,
                    canopy.leaf_on_allocation_kg_m2,
                    canopy.leaf_off_litter_kg_m2,
                    residual,
                ));
            }
        }
    }
    fs::write(&args[3], output)?;
    Ok(())
}
