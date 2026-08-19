use crate::HexF64;
use openwepp_hillslope_orchestrator::{
    DirectEvapotranspirationStageState, DirectGrowthStateSurface,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GrowthEtRestartError {
    #[error("{field} must be finite and nonnegative")]
    Nonnegative { field: &'static str },
    #[error("{field} must be within 0..=1")]
    Fraction { field: &'static str },
}
fn nn(field: &'static str, v: &HexF64) -> Result<f64, GrowthEtRestartError> {
    let x = v.to_f64();
    (x.is_finite() && x >= 0.0)
        .then_some(x)
        .ok_or(GrowthEtRestartError::Nonnegative { field })
}
fn frac(field: &'static str, v: &HexF64) -> Result<f64, GrowthEtRestartError> {
    let x = v.to_f64();
    (x.is_finite() && (0.0..=1.0).contains(&x))
        .then_some(x)
        .ok_or(GrowthEtRestartError::Fraction { field })
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectEvapotranspirationStageRestartV1 {
    pub s1_m: HexF64,
    pub s2_m: HexF64,
    pub threshold_m: HexF64,
    pub counter: HexF64,
}
impl DirectEvapotranspirationStageRestartV1 {
    pub fn project(v: &DirectEvapotranspirationStageState) -> Self {
        let DirectEvapotranspirationStageState {
            s1_m,
            s2_m,
            threshold_m,
            counter,
        } = *v;
        Self {
            s1_m: HexF64::from_f64(s1_m),
            s2_m: HexF64::from_f64(s2_m),
            threshold_m: HexF64::from_f64(threshold_m),
            counter: HexF64::from_f64(counter),
        }
    }
    pub fn restore(&self) -> Result<DirectEvapotranspirationStageState, GrowthEtRestartError> {
        Ok(DirectEvapotranspirationStageState {
            s1_m: nn("s1_m", &self.s1_m)?,
            s2_m: nn("s2_m", &self.s2_m)?,
            threshold_m: nn("threshold_m", &self.threshold_m)?,
            counter: nn("counter", &self.counter)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGrowthStateSurfaceRestartV1 {
    pub sumgdd: HexF64,
    pub live_biomass_kg_m2: HexF64,
    pub interception_live_biomass_kg_m2: HexF64,
    pub canopy_height_m: HexF64,
    pub canopy_cover_fraction: HexF64,
    pub leaf_area_index: HexF64,
    pub root_mass_kg_m2: HexF64,
    pub root_depth_m: HexF64,
    pub harvest_index: HexF64,
}
impl DirectGrowthStateSurfaceRestartV1 {
    pub fn project(v: &DirectGrowthStateSurface) -> Self {
        let DirectGrowthStateSurface {
            sumgdd,
            live_biomass_kg_m2,
            interception_live_biomass_kg_m2,
            canopy_height_m,
            canopy_cover_fraction,
            leaf_area_index,
            root_mass_kg_m2,
            root_depth_m,
            harvest_index,
        } = *v;
        Self {
            sumgdd: HexF64::from_f64(sumgdd),
            live_biomass_kg_m2: HexF64::from_f64(live_biomass_kg_m2),
            interception_live_biomass_kg_m2: HexF64::from_f64(interception_live_biomass_kg_m2),
            canopy_height_m: HexF64::from_f64(canopy_height_m),
            canopy_cover_fraction: HexF64::from_f64(canopy_cover_fraction),
            leaf_area_index: HexF64::from_f64(leaf_area_index),
            root_mass_kg_m2: HexF64::from_f64(root_mass_kg_m2),
            root_depth_m: HexF64::from_f64(root_depth_m),
            harvest_index: HexF64::from_f64(harvest_index),
        }
    }
    pub fn restore(&self) -> Result<DirectGrowthStateSurface, GrowthEtRestartError> {
        Ok(DirectGrowthStateSurface {
            sumgdd: nn("sumgdd", &self.sumgdd)?,
            live_biomass_kg_m2: nn("live_biomass_kg_m2", &self.live_biomass_kg_m2)?,
            interception_live_biomass_kg_m2: nn(
                "interception_live_biomass_kg_m2",
                &self.interception_live_biomass_kg_m2,
            )?,
            canopy_height_m: nn("canopy_height_m", &self.canopy_height_m)?,
            canopy_cover_fraction: frac("canopy_cover_fraction", &self.canopy_cover_fraction)?,
            leaf_area_index: nn("leaf_area_index", &self.leaf_area_index)?,
            root_mass_kg_m2: nn("root_mass_kg_m2", &self.root_mass_kg_m2)?,
            root_depth_m: nn("root_depth_m", &self.root_depth_m)?,
            harvest_index: frac("harvest_index", &self.harvest_index)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn field_domains_reject_nonfinite_and_out_of_range_values() {
        let mut growth =
            DirectGrowthStateSurfaceRestartV1::project(&DirectGrowthStateSurface::zero());
        growth.canopy_cover_fraction = HexF64::from_f64(1.01);
        assert_eq!(
            growth.restore(),
            Err(GrowthEtRestartError::Fraction {
                field: "canopy_cover_fraction"
            })
        );
        let mut et =
            DirectEvapotranspirationStageRestartV1::project(&DirectEvapotranspirationStageState {
                s1_m: 0.0,
                s2_m: 0.0,
                threshold_m: 0.0,
                counter: 0.0,
            });
        et.counter = HexF64::from_f64(f64::NAN);
        assert_eq!(
            et.restore(),
            Err(GrowthEtRestartError::Nonnegative { field: "counter" })
        );
    }
}
