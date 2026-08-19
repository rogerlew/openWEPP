use crate::HexF64;
use openwepp_hillslope_orchestrator::{DirectGroundwaterAuthority, DirectGroundwaterRunState};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum GroundwaterRestartError {
    #[error("{field} must be finite and nonnegative")]
    Domain { field: &'static str },
    #[error("enabled groundwater requires a positive initialized area")]
    Area,
    #[error("disabled groundwater must have zero state and no initialized area")]
    DisabledState,
    #[error("groundwater initialized area does not equal the canonical lane-area sum")]
    TotalAreaJoin,
}
fn nn(field: &'static str, v: &HexF64) -> Result<f64, GroundwaterRestartError> {
    let x = v.to_f64();
    (x.is_finite() && x >= 0.0)
        .then_some(x)
        .ok_or(GroundwaterRestartError::Domain { field })
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(tag = "authority", rename_all = "snake_case", deny_unknown_fields)]
pub enum GroundwaterAuthorityRestartV1 {
    Disabled,
    LinearReservoir {
        initial_storage_depth_m: HexF64,
        baseflow_coeff_per_day: HexF64,
        deep_seepage_coeff_per_day: HexF64,
        baseflow_threshold_area_ha: HexF64,
    },
}
impl GroundwaterAuthorityRestartV1 {
    fn project(v: DirectGroundwaterAuthority) -> Self {
        match v {
            DirectGroundwaterAuthority::Disabled => Self::Disabled,
            DirectGroundwaterAuthority::LinearReservoir {
                initial_storage_depth_m,
                baseflow_coeff_per_day,
                deep_seepage_coeff_per_day,
                baseflow_threshold_area_ha,
            } => Self::LinearReservoir {
                initial_storage_depth_m: HexF64::from_f64(initial_storage_depth_m),
                baseflow_coeff_per_day: HexF64::from_f64(baseflow_coeff_per_day),
                deep_seepage_coeff_per_day: HexF64::from_f64(deep_seepage_coeff_per_day),
                baseflow_threshold_area_ha: HexF64::from_f64(baseflow_threshold_area_ha),
            },
        }
    }
    fn restore(&self) -> Result<DirectGroundwaterAuthority, GroundwaterRestartError> {
        match self {
            Self::Disabled => Ok(DirectGroundwaterAuthority::Disabled),
            Self::LinearReservoir {
                initial_storage_depth_m,
                baseflow_coeff_per_day,
                deep_seepage_coeff_per_day,
                baseflow_threshold_area_ha,
            } => DirectGroundwaterAuthority::linear_reservoir(
                nn(
                    "groundwater.initial_storage_depth_m",
                    initial_storage_depth_m,
                )?,
                nn("groundwater.baseflow_coeff_per_day", baseflow_coeff_per_day)?,
                nn(
                    "groundwater.deep_seepage_coeff_per_day",
                    deep_seepage_coeff_per_day,
                )?,
                nn(
                    "groundwater.baseflow_threshold_area_ha",
                    baseflow_threshold_area_ha,
                )?,
            )
            .map_err(|_| GroundwaterRestartError::Domain {
                field: "groundwater.authority",
            }),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGroundwaterRunStateRestartV1 {
    pub authority: GroundwaterAuthorityRestartV1,
    pub storage_m3: HexF64,
    pub previous_baseflow_m3: HexF64,
    pub previous_deep_seepage_m3: HexF64,
    pub initialized_area_m2: Option<HexF64>,
}
impl DirectGroundwaterRunStateRestartV1 {
    pub fn project(v: &DirectGroundwaterRunState) -> Self {
        let DirectGroundwaterRunState {
            authority,
            storage_m3,
            previous_baseflow_m3,
            previous_deep_seepage_m3,
            initialized_area_m2,
        } = *v;
        Self {
            authority: GroundwaterAuthorityRestartV1::project(authority),
            storage_m3: HexF64::from_f64(storage_m3),
            previous_baseflow_m3: HexF64::from_f64(previous_baseflow_m3),
            previous_deep_seepage_m3: HexF64::from_f64(previous_deep_seepage_m3),
            initialized_area_m2: initialized_area_m2.map(HexF64::from_f64),
        }
    }
    pub fn restore(&self) -> Result<DirectGroundwaterRunState, GroundwaterRestartError> {
        self.restore_for_total_area(None)
    }
    pub fn restore_for_total_area(
        &self,
        expected_total_area_m2: Option<f64>,
    ) -> Result<DirectGroundwaterRunState, GroundwaterRestartError> {
        let authority = self.authority.restore()?;
        let initialized_area_m2 = self
            .initialized_area_m2
            .as_ref()
            .map(|v| nn("groundwater.initialized_area_m2", v))
            .transpose()?;
        let storage_m3 = nn("groundwater.storage_m3", &self.storage_m3)?;
        let previous_baseflow_m3 = nn(
            "groundwater.previous_baseflow_m3",
            &self.previous_baseflow_m3,
        )?;
        let previous_deep_seepage_m3 = nn(
            "groundwater.previous_deep_seepage_m3",
            &self.previous_deep_seepage_m3,
        )?;
        if authority.is_enabled() && !initialized_area_m2.is_some_and(|v| v > 0.0) {
            return Err(GroundwaterRestartError::Area);
        }
        if !authority.is_enabled()
            && (initialized_area_m2.is_some()
                || storage_m3 != 0.0
                || previous_baseflow_m3 != 0.0
                || previous_deep_seepage_m3 != 0.0)
        {
            return Err(GroundwaterRestartError::DisabledState);
        }
        if let Some(expected) = expected_total_area_m2
            && (!expected.is_finite()
                || expected <= 0.0
                || authority.is_enabled() && initialized_area_m2 != Some(expected))
        {
            return Err(GroundwaterRestartError::TotalAreaJoin);
        }
        Ok(DirectGroundwaterRunState {
            authority,
            storage_m3,
            previous_baseflow_m3,
            previous_deep_seepage_m3,
            initialized_area_m2,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn groundwater_round_trip_and_domains() {
        let authority = DirectGroundwaterAuthority::linear_reservoir(0.1, 0.2, 0.05, 1.0).unwrap();
        let state = DirectGroundwaterRunState {
            authority,
            storage_m3: 10.0,
            previous_baseflow_m3: -0.0,
            previous_deep_seepage_m3: 0.2,
            initialized_area_m2: Some(100.0),
        };
        let dto = DirectGroundwaterRunStateRestartV1::project(&state);
        assert_eq!(
            DirectGroundwaterRunStateRestartV1::project(&dto.restore().unwrap()),
            dto
        );
        let mut bad = dto;
        bad.initialized_area_m2 = None;
        assert_eq!(bad.restore(), Err(GroundwaterRestartError::Area));
        let disabled = DirectGroundwaterRunState {
            authority: DirectGroundwaterAuthority::Disabled,
            storage_m3: 1.0,
            previous_baseflow_m3: 0.0,
            previous_deep_seepage_m3: 0.0,
            initialized_area_m2: None,
        };
        assert_eq!(
            DirectGroundwaterRunStateRestartV1::project(&disabled).restore(),
            Err(GroundwaterRestartError::DisabledState)
        );
        let enabled = DirectGroundwaterRunStateRestartV1::project(&state);
        assert_eq!(
            enabled.restore_for_total_area(Some(101.0)),
            Err(GroundwaterRestartError::TotalAreaJoin)
        );
    }
}
