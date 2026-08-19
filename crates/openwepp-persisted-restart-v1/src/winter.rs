use std::collections::HashSet;

use openwepp_hillslope_orchestrator::{
    DirectFrostFineLayerState, DirectFrostLaneState, DirectFrostLayerShadowState,
    DirectFrostRuntimeCarry, DirectSnowLaneState, DirectSnowLayerState, DirectSnowRuntimeCarry,
    DirectWinterColumnState, SnowAlbedoModel, SnowAlbedoState,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::HexF64;

const ALBEDO_MODEL: &str = "brock2000_temperature_age_v1";
const SNOW_CLOSURE_M: f64 = 1.0e-9;
const SNOW_DENSITY_TOLERANCE: f64 = 1.0e-4;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WinterRestartError {
    #[error("{field} violates its winter domain")]
    Domain { field: &'static str },
    #[error("unsupported snow-albedo model {model}")]
    UnsupportedAlbedoModel { model: String },
    #[error("snow layer aggregates contradict canonical lane state")]
    SnowAggregate,
    #[error("frost layer/fine-layer identities or cardinality are invalid")]
    FrostStructure,
    #[error("winter compatibility carry contradicts canonical winter column")]
    CarryMismatch,
    #[error("winter index is zero or does not fit the runtime platform")]
    Index,
    #[error("active winter state is unsupported by the Child-4 snow-free checkpoint")]
    UnsupportedChild4ActiveWinter,
}

fn finite(field: &'static str, value: &HexF64) -> Result<f64, WinterRestartError> {
    let value = value.to_f64();
    value
        .is_finite()
        .then_some(value)
        .ok_or(WinterRestartError::Domain { field })
}
fn nonnegative(field: &'static str, value: &HexF64) -> Result<f64, WinterRestartError> {
    let value = finite(field, value)?;
    (value >= 0.0)
        .then_some(value)
        .ok_or(WinterRestartError::Domain { field })
}
fn runtime_index(value: u64) -> Result<usize, WinterRestartError> {
    if value > u32::MAX as u64 {
        return Err(WinterRestartError::Index);
    }
    let value = usize::try_from(value).map_err(|_| WinterRestartError::Index)?;
    (value != 0)
        .then_some(value)
        .ok_or(WinterRestartError::Index)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowAlbedoRestartV1 {
    pub model: String,
    pub albedo: HexF64,
    pub accumulated_positive_temperature_c_day: HexF64,
}
impl SnowAlbedoRestartV1 {
    fn project(value: SnowAlbedoState) -> Self {
        let SnowAlbedoState {
            model,
            albedo,
            accumulated_positive_temperature_c_day,
        } = value;
        Self {
            model: model.id().to_owned(),
            albedo: HexF64::from_f64(albedo),
            accumulated_positive_temperature_c_day: HexF64::from_f64(
                accumulated_positive_temperature_c_day,
            ),
        }
    }
    fn restore(&self) -> Result<SnowAlbedoState, WinterRestartError> {
        if self.model != ALBEDO_MODEL {
            return Err(WinterRestartError::UnsupportedAlbedoModel {
                model: self.model.clone(),
            });
        }
        let state = SnowAlbedoState {
            model: SnowAlbedoModel::Brock2000TemperatureAgeV1,
            albedo: finite("snow_albedo.albedo", &self.albedo)?,
            accumulated_positive_temperature_c_day: nonnegative(
                "snow_albedo.accumulated_positive_temperature_c_day",
                &self.accumulated_positive_temperature_c_day,
            )?,
        };
        state.validate().map_err(|_| WinterRestartError::Domain {
            field: "snow_albedo",
        })?;
        Ok(state)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowLayerRestartV1 {
    pub mass_swe_m: HexF64,
    pub thickness_m: HexF64,
    pub density_kg_m3: HexF64,
    pub settle_day_count: HexF64,
    pub temperature_c: HexF64,
    pub liquid_water_m: HexF64,
    pub cold_content_j_m2: HexF64,
    pub refrozen_liquid_m: HexF64,
}
impl SnowLayerRestartV1 {
    fn project(value: &DirectSnowLayerState) -> Self {
        let DirectSnowLayerState {
            mass_swe_m,
            thickness_m,
            density_kg_m3,
            settle_day_count,
            temperature_c,
            liquid_water_m,
            cold_content_j_m2,
            refrozen_liquid_m,
        } = *value;
        Self {
            mass_swe_m: HexF64::from_f64(mass_swe_m),
            thickness_m: HexF64::from_f64(thickness_m),
            density_kg_m3: HexF64::from_f64(density_kg_m3),
            settle_day_count: HexF64::from_f64(settle_day_count),
            temperature_c: HexF64::from_f64(temperature_c),
            liquid_water_m: HexF64::from_f64(liquid_water_m),
            cold_content_j_m2: HexF64::from_f64(cold_content_j_m2),
            refrozen_liquid_m: HexF64::from_f64(refrozen_liquid_m),
        }
    }
    fn restore(&self) -> Result<DirectSnowLayerState, WinterRestartError> {
        let layer = DirectSnowLayerState {
            mass_swe_m: nonnegative("snow.layer.mass_swe_m", &self.mass_swe_m)?,
            thickness_m: nonnegative("snow.layer.thickness_m", &self.thickness_m)?,
            density_kg_m3: nonnegative("snow.layer.density_kg_m3", &self.density_kg_m3)?,
            settle_day_count: nonnegative("snow.layer.settle_day_count", &self.settle_day_count)?,
            temperature_c: finite("snow.layer.temperature_c", &self.temperature_c)?,
            liquid_water_m: nonnegative("snow.layer.liquid_water_m", &self.liquid_water_m)?,
            cold_content_j_m2: nonnegative(
                "snow.layer.cold_content_j_m2",
                &self.cold_content_j_m2,
            )?,
            refrozen_liquid_m: nonnegative(
                "snow.layer.refrozen_liquid_m",
                &self.refrozen_liquid_m,
            )?,
        };
        if layer.density_kg_m3 > 522.0
            || layer.liquid_water_m > layer.mass_swe_m
            || layer.refrozen_liquid_m > layer.mass_swe_m
        {
            return Err(WinterRestartError::Domain {
                field: "snow.layer",
            });
        }
        if layer.mass_swe_m > SNOW_CLOSURE_M && layer.thickness_m > SNOW_CLOSURE_M {
            if (layer.density_kg_m3 - layer.mass_swe_m * 1_000.0 / layer.thickness_m).abs()
                > SNOW_DENSITY_TOLERANCE
            {
                return Err(WinterRestartError::SnowAggregate);
            }
        } else if layer.mass_swe_m > SNOW_CLOSURE_M || layer.thickness_m > SNOW_CLOSURE_M {
            return Err(WinterRestartError::SnowAggregate);
        }
        Ok(layer)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowLaneRestartV1 {
    pub runtime_swe_m: HexF64,
    pub runtime_depth_m: HexF64,
    pub runtime_density_kg_m3: HexF64,
    pub runtime_settle_day_count: HexF64,
    pub coe_boundary_depth_m: HexF64,
    pub coe_boundary_density_kg_m3: HexF64,
    pub coe_boundary_settle_day_count: HexF64,
    pub liquid_water_retained_m: HexF64,
    pub snow_albedo_state: Option<SnowAlbedoRestartV1>,
    pub layers: Vec<SnowLayerRestartV1>,
}
impl SnowLaneRestartV1 {
    fn project(value: &DirectSnowLaneState) -> Self {
        let DirectSnowLaneState {
            runtime_swe_m,
            runtime_depth_m,
            runtime_density_kg_m3,
            runtime_settle_day_count,
            coe_boundary_depth_m,
            coe_boundary_density_kg_m3,
            coe_boundary_settle_day_count,
            liquid_water_retained_m,
            snow_albedo_state,
            layers,
        } = value;
        Self {
            runtime_swe_m: HexF64::from_f64(*runtime_swe_m),
            runtime_depth_m: HexF64::from_f64(*runtime_depth_m),
            runtime_density_kg_m3: HexF64::from_f64(*runtime_density_kg_m3),
            runtime_settle_day_count: HexF64::from_f64(*runtime_settle_day_count),
            coe_boundary_depth_m: HexF64::from_f64(*coe_boundary_depth_m),
            coe_boundary_density_kg_m3: HexF64::from_f64(*coe_boundary_density_kg_m3),
            coe_boundary_settle_day_count: HexF64::from_f64(*coe_boundary_settle_day_count),
            liquid_water_retained_m: HexF64::from_f64(*liquid_water_retained_m),
            snow_albedo_state: snow_albedo_state.map(SnowAlbedoRestartV1::project),
            layers: layers.iter().map(SnowLayerRestartV1::project).collect(),
        }
    }
    fn restore(&self) -> Result<DirectSnowLaneState, WinterRestartError> {
        let state = DirectSnowLaneState {
            runtime_swe_m: nonnegative("snow.runtime_swe_m", &self.runtime_swe_m)?,
            runtime_depth_m: nonnegative("snow.runtime_depth_m", &self.runtime_depth_m)?,
            runtime_density_kg_m3: nonnegative(
                "snow.runtime_density_kg_m3",
                &self.runtime_density_kg_m3,
            )?,
            runtime_settle_day_count: nonnegative(
                "snow.runtime_settle_day_count",
                &self.runtime_settle_day_count,
            )?,
            coe_boundary_depth_m: nonnegative(
                "snow.coe_boundary_depth_m",
                &self.coe_boundary_depth_m,
            )?,
            coe_boundary_density_kg_m3: nonnegative(
                "snow.coe_boundary_density_kg_m3",
                &self.coe_boundary_density_kg_m3,
            )?,
            coe_boundary_settle_day_count: nonnegative(
                "snow.coe_boundary_settle_day_count",
                &self.coe_boundary_settle_day_count,
            )?,
            liquid_water_retained_m: nonnegative(
                "snow.liquid_water_retained_m",
                &self.liquid_water_retained_m,
            )?,
            snow_albedo_state: self
                .snow_albedo_state
                .as_ref()
                .map(SnowAlbedoRestartV1::restore)
                .transpose()?,
            layers: self
                .layers
                .iter()
                .map(SnowLayerRestartV1::restore)
                .collect::<Result<_, _>>()?,
        };
        if state.runtime_density_kg_m3 > 522.0 || state.coe_boundary_density_kg_m3 > 522.0 {
            return Err(WinterRestartError::Domain {
                field: "snow.density",
            });
        }
        if state.snow_albedo_state.is_some()
            && (state.runtime_swe_m <= SNOW_CLOSURE_M || state.runtime_depth_m <= SNOW_CLOSURE_M)
        {
            return Err(WinterRestartError::Domain {
                field: "snow_albedo",
            });
        }
        if !state.layers.is_empty() {
            let swe = state.layers.iter().map(|x| x.mass_swe_m).sum::<f64>();
            let depth = state.layers.iter().map(|x| x.thickness_m).sum::<f64>();
            if (swe - state.runtime_swe_m).abs() > SNOW_CLOSURE_M
                || (depth - state.runtime_depth_m).abs() > SNOW_CLOSURE_M
                || state.runtime_swe_m <= SNOW_CLOSURE_M
                || state.runtime_depth_m <= SNOW_CLOSURE_M
                || (state.runtime_density_kg_m3
                    - state.runtime_swe_m * 1_000.0 / state.runtime_depth_m)
                    .abs()
                    > SNOW_DENSITY_TOLERANCE
            {
                return Err(WinterRestartError::SnowAggregate);
            }
        }
        Ok(state)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrostLayerRestartV1 {
    pub layer_index: u64,
    pub st_m: HexF64,
    pub soil_water_m: HexF64,
    pub frozen_depth_m: HexF64,
    pub frozen_water_m: HexF64,
    pub soilf_m: HexF64,
    pub yst_m: HexF64,
    pub nwfrzz_m: HexF64,
}
impl FrostLayerRestartV1 {
    fn project(value: &DirectFrostLayerShadowState) -> Self {
        let DirectFrostLayerShadowState {
            layer_index,
            st_m,
            soil_water_m,
            frozen_depth_m,
            frozen_water_m,
            soilf_m,
            yst_m,
            nwfrzz_m,
        } = *value;
        Self {
            layer_index: layer_index as u64,
            st_m: HexF64::from_f64(st_m),
            soil_water_m: HexF64::from_f64(soil_water_m),
            frozen_depth_m: HexF64::from_f64(frozen_depth_m),
            frozen_water_m: HexF64::from_f64(frozen_water_m),
            soilf_m: HexF64::from_f64(soilf_m),
            yst_m: HexF64::from_f64(yst_m),
            nwfrzz_m: HexF64::from_f64(nwfrzz_m),
        }
    }
    fn restore(&self) -> Result<DirectFrostLayerShadowState, WinterRestartError> {
        Ok(DirectFrostLayerShadowState {
            layer_index: runtime_index(self.layer_index)?,
            st_m: nonnegative("frost.layer.st_m", &self.st_m)?,
            soil_water_m: nonnegative("frost.layer.soil_water_m", &self.soil_water_m)?,
            frozen_depth_m: nonnegative("frost.layer.frozen_depth_m", &self.frozen_depth_m)?,
            frozen_water_m: nonnegative("frost.layer.frozen_water_m", &self.frozen_water_m)?,
            soilf_m: nonnegative("frost.layer.soilf_m", &self.soilf_m)?,
            yst_m: nonnegative("frost.layer.yst_m", &self.yst_m)?,
            nwfrzz_m: nonnegative("frost.layer.nwfrzz_m", &self.nwfrzz_m)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrostFineRestartV1 {
    pub layer_index: u64,
    pub fine_index: u64,
    pub fgfrst: HexF64,
    pub slfsd_m: HexF64,
    pub slsic_m: HexF64,
    pub slsw_theta: HexF64,
    pub sltime_s: HexF64,
}
impl FrostFineRestartV1 {
    fn project(value: &DirectFrostFineLayerState) -> Self {
        let DirectFrostFineLayerState {
            layer_index,
            fine_index,
            fgfrst,
            slfsd_m,
            slsic_m,
            slsw_theta,
            sltime_s,
        } = *value;
        Self {
            layer_index: layer_index as u64,
            fine_index: fine_index as u64,
            fgfrst: HexF64::from_f64(fgfrst),
            slfsd_m: HexF64::from_f64(slfsd_m),
            slsic_m: HexF64::from_f64(slsic_m),
            slsw_theta: HexF64::from_f64(slsw_theta),
            sltime_s: HexF64::from_f64(sltime_s),
        }
    }
    fn restore(&self) -> Result<DirectFrostFineLayerState, WinterRestartError> {
        Ok(DirectFrostFineLayerState {
            layer_index: runtime_index(self.layer_index)?,
            fine_index: runtime_index(self.fine_index)?,
            fgfrst: nonnegative("frost.fine.fgfrst", &self.fgfrst)?,
            slfsd_m: nonnegative("frost.fine.slfsd_m", &self.slfsd_m)?,
            slsic_m: nonnegative("frost.fine.slsic_m", &self.slsic_m)?,
            slsw_theta: nonnegative("frost.fine.slsw_theta", &self.slsw_theta)?,
            sltime_s: nonnegative("frost.fine.sltime_s", &self.sltime_s)?,
        })
    }
}

macro_rules! frost_lane_dto { ($($field:ident),+ $(,)?) => {
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)] #[serde(deny_unknown_fields)]
pub struct FrostLaneRestartV1 { pub active_frost_coupling:bool,$(pub $field:HexF64,)+pub layer_shadows:Vec<FrostLayerRestartV1>,pub fine_layers:Vec<FrostFineRestartV1> }
impl FrostLaneRestartV1 {
 fn project(value:&DirectFrostLaneState)->Self { let DirectFrostLaneState{active_frost_coupling,$($field,)+layer_shadows,fine_layers}=value;Self{active_frost_coupling:*active_frost_coupling,$($field:HexF64::from_f64(*$field),)+layer_shadows:layer_shadows.iter().map(FrostLayerRestartV1::project).collect(),fine_layers:fine_layers.iter().map(FrostFineRestartV1::project).collect()} }
 fn restore(&self)->Result<DirectFrostLaneState,WinterRestartError>{let state=DirectFrostLaneState{active_frost_coupling:self.active_frost_coupling,$($field:finite(stringify!($field),&self.$field)?,)+layer_shadows:self.layer_shadows.iter().map(FrostLayerRestartV1::restore).collect::<Result<_,_>>()?,fine_layers:self.fine_layers.iter().map(FrostFineRestartV1::restore).collect::<Result<_,_>>()?};validate_frost(&state)?;Ok(state)}
}
};}
frost_lane_dto!(
    dfrost_m,
    dthaw_m,
    nft,
    ws_frz_m,
    infcap_frz_m_s,
    frwatc_soil_water_before_m,
    frwatc_soil_water_after_m,
    frwatc_frozen_water_before_m,
    frwatc_frozen_water_after_m,
    frwatc_freeze_debit_m,
    frwatc_thaw_credit_m,
    frwatc_net_liquid_delta_m,
    frdp_m,
    thdp_m,
    tfrdp_m,
    tthawd_m,
    fgthwd_flag,
    total_fine_layer_count,
    conductivity_tilled_w_m_k,
    conductivity_untilled_w_m_k,
    conductivity_residue_w_m_k,
    shadow_total_water_before_m,
    shadow_total_water_after_m,
    shadow_wb_delta_m,
    shadow_frwatc_residual_m,
    watpdg_m,
    watbtm_m
);

fn validate_frost(state: &DirectFrostLaneState) -> Result<(), WinterRestartError> {
    let signed = [
        state.frwatc_net_liquid_delta_m,
        state.shadow_wb_delta_m,
        state.shadow_frwatc_residual_m,
    ];
    if signed.iter().any(|v| !v.is_finite()) {
        return Err(WinterRestartError::Domain {
            field: "frost.signed",
        });
    }
    let nonnegative = [
        state.dfrost_m,
        state.dthaw_m,
        state.nft,
        state.ws_frz_m,
        state.infcap_frz_m_s,
        state.frwatc_soil_water_before_m,
        state.frwatc_soil_water_after_m,
        state.frwatc_frozen_water_before_m,
        state.frwatc_frozen_water_after_m,
        state.frwatc_freeze_debit_m,
        state.frwatc_thaw_credit_m,
        state.frdp_m,
        state.thdp_m,
        state.tfrdp_m,
        state.tthawd_m,
        state.fgthwd_flag,
        state.total_fine_layer_count,
        state.conductivity_tilled_w_m_k,
        state.conductivity_untilled_w_m_k,
        state.conductivity_residue_w_m_k,
        state.shadow_total_water_before_m,
        state.shadow_total_water_after_m,
        state.watpdg_m,
        state.watbtm_m,
    ];
    if nonnegative.iter().any(|v| !v.is_finite() || *v < 0.0) {
        return Err(WinterRestartError::Domain {
            field: "frost.nonnegative",
        });
    }
    if state.total_fine_layer_count.to_bits() != (state.fine_layers.len() as f64).to_bits()
        || state.layer_shadows.is_empty() != state.fine_layers.is_empty()
    {
        return Err(WinterRestartError::FrostStructure);
    }
    if state
        .layer_shadows
        .windows(2)
        .any(|p| p[0].layer_index >= p[1].layer_index)
    {
        return Err(WinterRestartError::FrostStructure);
    }
    let ids: HashSet<_> = state.layer_shadows.iter().map(|x| x.layer_index).collect();
    let mut prior = None;
    for fine in &state.fine_layers {
        if !ids.contains(&fine.layer_index) {
            return Err(WinterRestartError::FrostStructure);
        }
        let expected = match prior {
            None => 1,
            Some((layer, index)) if layer == fine.layer_index => index + 1,
            Some((layer, _)) if layer < fine.layer_index => 1,
            _ => return Err(WinterRestartError::FrostStructure),
        };
        if fine.fine_index != expected {
            return Err(WinterRestartError::FrostStructure);
        }
        prior = Some((fine.layer_index, fine.fine_index));
    }
    if state.layer_shadows.iter().any(|layer| {
        !state
            .fine_layers
            .iter()
            .any(|fine| fine.layer_index == layer.layer_index)
    }) {
        return Err(WinterRestartError::FrostStructure);
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectWinterColumnRestartV1 {
    pub snow: SnowLaneRestartV1,
    pub frost: FrostLaneRestartV1,
}
#[derive(Debug)]
pub struct RestoredWinterCompatibility {
    pub winter_column: DirectWinterColumnState,
    pub snow_runtime_carry: Option<DirectSnowRuntimeCarry>,
    pub frost_runtime_carry: Option<DirectFrostRuntimeCarry>,
}
impl DirectWinterColumnRestartV1 {
    pub fn project(
        column: &DirectWinterColumnState,
        snow_carry: Option<&DirectSnowRuntimeCarry>,
        frost_carry: Option<&DirectFrostRuntimeCarry>,
    ) -> Result<Self, WinterRestartError> {
        let DirectWinterColumnState { snow, frost } = column;
        let dto = Self {
            snow: SnowLaneRestartV1::project(snow),
            frost: FrostLaneRestartV1::project(frost),
        };
        if let Some(carry) = snow_carry {
            let lane: DirectSnowLaneState = carry.clone().into();
            if SnowLaneRestartV1::project(&lane) != dto.snow {
                return Err(WinterRestartError::CarryMismatch);
            }
        }
        if let Some(carry) = frost_carry {
            let lane: DirectFrostLaneState = carry.clone().into();
            if FrostLaneRestartV1::project(&lane) != dto.frost {
                return Err(WinterRestartError::CarryMismatch);
            }
        }
        Ok(dto)
    }
    pub fn restore(&self) -> Result<RestoredWinterCompatibility, WinterRestartError> {
        let winter_column = DirectWinterColumnState {
            snow: self.snow.restore()?,
            frost: self.frost.restore()?,
        };
        let snow_runtime_carry = winter_column
            .snow
            .has_runtime_state()
            .then(|| DirectSnowRuntimeCarry::from(&winter_column.snow));
        let frost_runtime_carry = winter_column
            .frost
            .has_runtime_state()
            .then(|| DirectFrostRuntimeCarry::from(winter_column.frost.clone()));
        Ok(RestoredWinterCompatibility {
            winter_column,
            snow_runtime_carry,
            frost_runtime_carry,
        })
    }
    pub fn restore_with_compatibility(
        &self,
        snow_carry: Option<&DirectSnowRuntimeCarry>,
        frost_carry: Option<&DirectFrostRuntimeCarry>,
    ) -> Result<RestoredWinterCompatibility, WinterRestartError> {
        let restored = self.restore()?;
        if snow_carry.is_some_and(|carry| {
            SnowLaneRestartV1::project(&DirectSnowLaneState::from(carry.clone())) != self.snow
        }) || frost_carry.is_some_and(|carry| {
            FrostLaneRestartV1::project(&DirectFrostLaneState::from(carry.clone())) != self.frost
        }) {
            return Err(WinterRestartError::CarryMismatch);
        }
        Ok(restored)
    }
    pub fn validate_child4_snow_free(&self) -> Result<(), WinterRestartError> {
        let restored = self.restore()?;
        if restored.winter_column.snow.has_runtime_state()
            || restored.winter_column.snow.liquid_water_retained_m != 0.0
            || restored.winter_column.frost.has_runtime_state()
        {
            return Err(WinterRestartError::UnsupportedChild4ActiveWinter);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snow() -> DirectSnowLaneState {
        DirectSnowLaneState {
            runtime_swe_m: 0.05,
            runtime_depth_m: 0.20,
            runtime_density_kg_m3: 250.0,
            runtime_settle_day_count: 2.0,
            coe_boundary_depth_m: 0.18,
            coe_boundary_density_kg_m3: 240.0,
            coe_boundary_settle_day_count: 1.5,
            liquid_water_retained_m: -0.0,
            snow_albedo_state: Some(SnowAlbedoState {
                model: SnowAlbedoModel::Brock2000TemperatureAgeV1,
                albedo: 0.72,
                accumulated_positive_temperature_c_day: 1.25,
            }),
            layers: vec![
                DirectSnowLayerState {
                    mass_swe_m: 0.02,
                    thickness_m: 0.10,
                    density_kg_m3: 200.0,
                    settle_day_count: 1.0,
                    temperature_c: -4.0,
                    liquid_water_m: -0.0,
                    cold_content_j_m2: 300.0,
                    refrozen_liquid_m: 0.001,
                },
                DirectSnowLayerState {
                    mass_swe_m: 0.03,
                    thickness_m: 0.10,
                    density_kg_m3: 300.0,
                    settle_day_count: 3.0,
                    temperature_c: -1.0,
                    liquid_water_m: 0.002,
                    cold_content_j_m2: 100.0,
                    refrozen_liquid_m: -0.0,
                },
            ],
        }
    }
    fn frost() -> DirectFrostLaneState {
        let mut state = DirectFrostLaneState::zero();
        state.active_frost_coupling = true;
        state.dfrost_m = 0.1;
        state.dthaw_m = 0.02;
        state.nft = 2.0;
        state.ws_frz_m = 0.03;
        state.infcap_frz_m_s = 1.0e-6;
        state.frwatc_soil_water_before_m = 0.2;
        state.frwatc_soil_water_after_m = 0.18;
        state.frwatc_frozen_water_before_m = 0.01;
        state.frwatc_frozen_water_after_m = 0.03;
        state.frwatc_freeze_debit_m = 0.02;
        state.frwatc_thaw_credit_m = -0.0;
        state.frwatc_net_liquid_delta_m = -0.02;
        state.frdp_m = 0.1;
        state.thdp_m = 0.02;
        state.tfrdp_m = 0.1;
        state.tthawd_m = 0.02;
        state.fgthwd_flag = 1.0;
        state.total_fine_layer_count = 3.0;
        state.conductivity_tilled_w_m_k = 0.4;
        state.conductivity_untilled_w_m_k = 0.8;
        state.conductivity_residue_w_m_k = 0.2;
        state.shadow_total_water_before_m = 0.21;
        state.shadow_total_water_after_m = 0.21;
        state.shadow_wb_delta_m = -0.0;
        state.shadow_frwatc_residual_m = 0.0;
        state.watpdg_m = 0.005;
        state.watbtm_m = 0.006;
        state.layer_shadows = vec![
            DirectFrostLayerShadowState {
                layer_index: 1,
                st_m: 0.2,
                soil_water_m: 0.15,
                frozen_depth_m: 0.05,
                frozen_water_m: 0.01,
                soilf_m: 0.02,
                yst_m: 0.03,
                nwfrzz_m: 0.01,
            },
            DirectFrostLayerShadowState {
                layer_index: 2,
                st_m: 0.3,
                soil_water_m: 0.2,
                frozen_depth_m: 0.05,
                frozen_water_m: 0.02,
                soilf_m: 0.03,
                yst_m: 0.04,
                nwfrzz_m: 0.02,
            },
        ];
        state.fine_layers = vec![
            DirectFrostFineLayerState {
                layer_index: 1,
                fine_index: 1,
                fgfrst: 1.0,
                slfsd_m: 0.05,
                slsic_m: 0.01,
                slsw_theta: 0.2,
                sltime_s: 1800.0,
            },
            DirectFrostFineLayerState {
                layer_index: 1,
                fine_index: 2,
                fgfrst: 0.0,
                slfsd_m: 0.05,
                slsic_m: 0.0,
                slsw_theta: 0.25,
                sltime_s: 1800.0,
            },
            DirectFrostFineLayerState {
                layer_index: 2,
                fine_index: 1,
                fgfrst: 1.0,
                slfsd_m: 0.10,
                slsic_m: 0.02,
                slsw_theta: 0.3,
                sltime_s: 1800.0,
            },
        ];
        state
    }
    fn assert_round_trip(column: DirectWinterColumnState) {
        let snow_carry = DirectSnowRuntimeCarry::from(&column.snow);
        let frost_carry = DirectFrostRuntimeCarry::from(column.frost.clone());
        let dto =
            DirectWinterColumnRestartV1::project(&column, Some(&snow_carry), Some(&frost_carry))
                .expect("matching compatibility carries");
        let restored = dto.restore().expect("valid winter DTO");
        assert_eq!(
            DirectWinterColumnRestartV1::project(
                &restored.winter_column,
                restored.snow_runtime_carry.as_ref(),
                restored.frost_runtime_carry.as_ref()
            )
            .expect("reconstructed carries"),
            dto
        );
    }

    #[test]
    fn zero_snow_frost_and_complete_columns_round_trip_bit_exactly() {
        assert_round_trip(DirectWinterColumnState::zero());
        assert_round_trip(DirectWinterColumnState {
            snow: snow(),
            frost: DirectFrostLaneState::zero(),
        });
        assert_round_trip(DirectWinterColumnState {
            snow: DirectSnowLaneState::zero(),
            frost: frost(),
        });
        assert_round_trip(DirectWinterColumnState {
            snow: snow(),
            frost: frost(),
        });
    }
    #[test]
    fn compatibility_carries_are_reconstructed_and_mismatch_rejects() {
        let column = DirectWinterColumnState {
            snow: snow(),
            frost: frost(),
        };
        let dto = DirectWinterColumnRestartV1::project(&column, None, None)
            .expect("canonical-only projection");
        let restored = dto.restore().expect("carry reconstruction");
        assert_eq!(
            SnowLaneRestartV1::project(&DirectSnowLaneState::from(
                restored
                    .snow_runtime_carry
                    .clone()
                    .expect("active snow carry")
            )),
            dto.snow
        );
        let mut bad = restored.snow_runtime_carry.expect("active snow carry");
        bad.runtime_swe_m += 0.001;
        assert_eq!(
            DirectWinterColumnRestartV1::project(&column, Some(&bad), None),
            Err(WinterRestartError::CarryMismatch)
        );
    }
    #[test]
    fn snow_albedo_numeric_and_aggregate_poisons_reject() {
        let mut dto = DirectWinterColumnRestartV1::project(
            &DirectWinterColumnState {
                snow: snow(),
                frost: DirectFrostLaneState::zero(),
            },
            None,
            None,
        )
        .unwrap();
        dto.snow.snow_albedo_state.as_mut().unwrap().model = "unknown".into();
        assert!(matches!(
            dto.restore(),
            Err(WinterRestartError::UnsupportedAlbedoModel { .. })
        ));
        dto.snow.snow_albedo_state.as_mut().unwrap().model = ALBEDO_MODEL.into();
        dto.snow.snow_albedo_state.as_mut().unwrap().albedo = HexF64::from_f64(0.9);
        assert_eq!(
            dto.restore().unwrap_err(),
            WinterRestartError::Domain {
                field: "snow_albedo"
            }
        );
        dto.snow.snow_albedo_state.as_mut().unwrap().albedo = HexF64::from_f64(0.72);
        dto.snow
            .snow_albedo_state
            .as_mut()
            .unwrap()
            .accumulated_positive_temperature_c_day = HexF64::from_f64(-0.1);
        assert_eq!(
            dto.restore().unwrap_err(),
            WinterRestartError::Domain {
                field: "snow_albedo.accumulated_positive_temperature_c_day"
            }
        );
        dto.snow
            .snow_albedo_state
            .as_mut()
            .unwrap()
            .accumulated_positive_temperature_c_day = HexF64::from_f64(1.0);
        let original_carry = DirectSnowRuntimeCarry::from(&snow());
        dto.snow.layers.swap(0, 1);
        assert_eq!(
            dto.restore_with_compatibility(Some(&original_carry), None)
                .unwrap_err(),
            WinterRestartError::CarryMismatch
        );
        dto.snow.layers[0].mass_swe_m = HexF64::from_f64(0.04);
        assert_eq!(
            dto.restore().unwrap_err(),
            WinterRestartError::SnowAggregate
        );
        dto.snow.layers[0].mass_swe_m = HexF64::from_f64(f64::NAN);
        assert!(matches!(
            dto.restore(),
            Err(WinterRestartError::Domain { .. })
        ));
    }
    #[test]
    fn frost_identity_order_cardinality_and_numeric_poisons_reject() {
        let column = DirectWinterColumnState {
            snow: DirectSnowLaneState::zero(),
            frost: frost(),
        };
        let mut dto = DirectWinterColumnRestartV1::project(&column, None, None).unwrap();
        dto.frost.layer_shadows.swap(0, 1);
        assert_eq!(
            dto.restore().unwrap_err(),
            WinterRestartError::FrostStructure
        );
        dto = DirectWinterColumnRestartV1::project(&column, None, None).unwrap();
        dto.frost.layer_shadows[1].layer_index = 1;
        assert_eq!(
            dto.restore().unwrap_err(),
            WinterRestartError::FrostStructure
        );
        dto = DirectWinterColumnRestartV1::project(&column, None, None).unwrap();
        dto.frost.fine_layers[1].fine_index = 1;
        assert_eq!(
            dto.restore().unwrap_err(),
            WinterRestartError::FrostStructure
        );
        dto = DirectWinterColumnRestartV1::project(&column, None, None).unwrap();
        dto.frost.layer_shadows[0].layer_index = 0;
        assert_eq!(dto.restore().unwrap_err(), WinterRestartError::Index);
        dto = DirectWinterColumnRestartV1::project(&column, None, None).unwrap();
        dto.frost.layer_shadows[0].layer_index = u64::from(u32::MAX) + 1;
        assert_eq!(dto.restore().unwrap_err(), WinterRestartError::Index);
        dto = DirectWinterColumnRestartV1::project(&column, None, None).unwrap();
        dto.frost.conductivity_tilled_w_m_k = HexF64::from_f64(f64::INFINITY);
        assert!(matches!(
            dto.restore(),
            Err(WinterRestartError::Domain { .. })
        ));
    }
    #[test]
    fn child4_rejects_active_winter_and_accepts_zero_column() {
        let zero =
            DirectWinterColumnRestartV1::project(&DirectWinterColumnState::zero(), None, None)
                .unwrap();
        assert!(zero.validate_child4_snow_free().is_ok());
        let active = DirectWinterColumnRestartV1::project(
            &DirectWinterColumnState {
                snow: snow(),
                frost: DirectFrostLaneState::zero(),
            },
            None,
            None,
        )
        .unwrap();
        assert_eq!(
            active.validate_child4_snow_free(),
            Err(WinterRestartError::UnsupportedChild4ActiveWinter)
        );
        let mut retained = zero;
        retained.snow.liquid_water_retained_m = HexF64::from_f64(f64::from_bits(1));
        assert_eq!(
            retained.validate_child4_snow_free(),
            Err(WinterRestartError::UnsupportedChild4ActiveWinter)
        );
    }
}
