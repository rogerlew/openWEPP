use crate::{HexF64, HexU128, Sha256Hex};
use openwepp_hillslope_orchestrator::{
    DirectGroundIngressMode, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidConfigurationRecord, DirectSurfaceLiquidContinuationState,
    DirectSurfaceLiquidOfeBinding, DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidStateRecord,
    DirectSurfaceLiquidStoreKey,
};
use openwepp_kernel_contract::{ResourceOwnerId, TileId, TransactionId};
use openwepp_land_surface_energy::{OfeId, SourceId, SurfaceClass, SurfaceId, WaterSourceType};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClassWireV1 {
    BareMineralSoil,
    ForestLitter,
}
impl From<SurfaceClass> for SurfaceClassWireV1 {
    fn from(value: SurfaceClass) -> Self {
        match value {
            SurfaceClass::BareMineralSoil => Self::BareMineralSoil,
            SurfaceClass::ForestLitter => Self::ForestLitter,
        }
    }
}
impl From<SurfaceClassWireV1> for SurfaceClass {
    fn from(value: SurfaceClassWireV1) -> Self {
        match value {
            SurfaceClassWireV1::BareMineralSoil => Self::BareMineralSoil,
            SurfaceClassWireV1::ForestLitter => Self::ForestLitter,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WaterSourceWireV1 {
    SurfaceLiquid,
    LitterLiquid,
    SoilLayerLiquid,
}
impl From<WaterSourceType> for WaterSourceWireV1 {
    fn from(value: WaterSourceType) -> Self {
        match value {
            WaterSourceType::SurfaceLiquid => Self::SurfaceLiquid,
            WaterSourceType::LitterLiquid => Self::LitterLiquid,
            WaterSourceType::SoilLayerLiquid => Self::SoilLayerLiquid,
        }
    }
}
impl From<WaterSourceWireV1> for WaterSourceType {
    fn from(value: WaterSourceWireV1) -> Self {
        match value {
            WaterSourceWireV1::SurfaceLiquid => Self::SurfaceLiquid,
            WaterSourceWireV1::LitterLiquid => Self::LitterLiquid,
            WaterSourceWireV1::SoilLayerLiquid => Self::SoilLayerLiquid,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GroundIngressModeWireV1 {
    OpenRawPrecipitation,
    CoveredCanopyRelease,
}
impl From<DirectGroundIngressMode> for GroundIngressModeWireV1 {
    fn from(value: DirectGroundIngressMode) -> Self {
        match value {
            DirectGroundIngressMode::OpenRawPrecipitation => Self::OpenRawPrecipitation,
            DirectGroundIngressMode::CoveredCanopyRelease => Self::CoveredCanopyRelease,
        }
    }
}
impl From<GroundIngressModeWireV1> for DirectGroundIngressMode {
    fn from(value: GroundIngressModeWireV1) -> Self {
        match value {
            GroundIngressModeWireV1::OpenRawPrecipitation => Self::OpenRawPrecipitation,
            GroundIngressModeWireV1::CoveredCanopyRelease => Self::CoveredCanopyRelease,
        }
    }
}
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SurfaceLiquidRestartError {
    #[error("invalid identity or digest")]
    Identity,
    #[error("{field} violates surface-liquid domain")]
    Domain { field: &'static str },
    #[error("records or continuations contain duplicate identities")]
    Ordering,
    #[error("platform-width day index")]
    DayIndex,
}
fn nn(field: &'static str, v: &HexF64) -> Result<f64, SurfaceLiquidRestartError> {
    let x = v.to_f64();
    (x.is_finite() && x >= 0.0)
        .then_some(x)
        .ok_or(SurfaceLiquidRestartError::Domain { field })
}
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceLiquidKeyRestartV1 {
    pub run_id: u64,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub surface_id: SurfaceId,
    pub surface_class: SurfaceClassWireV1,
    pub source_type: WaterSourceWireV1,
    pub source_id: SourceId,
}
impl SurfaceLiquidKeyRestartV1 {
    fn project(v: &DirectSurfaceLiquidStoreKey) -> Self {
        let DirectSurfaceLiquidStoreKey {
            run_id,
            ofe_id,
            tile_id,
            surface_id,
            surface_class,
            source_type,
            source_id,
        } = v;
        Self {
            run_id: *run_id,
            ofe_id: ofe_id.clone(),
            tile_id: tile_id.clone(),
            surface_id: surface_id.clone(),
            surface_class: (*surface_class).into(),
            source_type: (*source_type).into(),
            source_id: source_id.clone(),
        }
    }
    fn restore(&self) -> DirectSurfaceLiquidStoreKey {
        DirectSurfaceLiquidStoreKey {
            run_id: self.run_id,
            ofe_id: self.ofe_id.clone(),
            tile_id: self.tile_id.clone(),
            surface_id: self.surface_id.clone(),
            surface_class: self.surface_class.into(),
            source_type: self.source_type.into(),
            source_id: self.source_id.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceLiquidConfigurationRecordRestartV1 {
    pub key: SurfaceLiquidKeyRestartV1,
    pub tile_fraction: HexF64,
    pub capacity_kg_m2_tile: HexF64,
    pub ofe_area_m2: HexF64,
    pub ground_ingress_mode: GroundIngressModeWireV1,
    pub runon_destination_ofe_id: Option<OfeId>,
    pub runon_destination_tile_id: Option<TileId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidConfigurationRestartV1 {
    pub owner_id: String,
    pub run_id: u64,
    pub configuration_sha256: Sha256Hex,
    pub ofe_topology: Vec<OfeId>,
    pub ofe_bindings: Vec<DirectSurfaceLiquidOfeBinding>,
    pub records: Vec<SurfaceLiquidConfigurationRecordRestartV1>,
}
impl DirectSurfaceLiquidConfigurationRestartV1 {
    pub fn project(
        value: &DirectSurfaceLiquidConfiguration,
    ) -> Result<Self, SurfaceLiquidRestartError> {
        let DirectSurfaceLiquidConfiguration {
            owner_id,
            run_id,
            configuration_sha256,
            ofe_topology,
            ofe_bindings,
            records,
        } = value;
        Ok(Self {
            owner_id: owner_id.as_str().to_owned(),
            run_id: *run_id,
            configuration_sha256: Sha256Hex::try_new(configuration_sha256.clone())
                .map_err(|_| SurfaceLiquidRestartError::Identity)?,
            ofe_topology: ofe_topology.clone(),
            ofe_bindings: ofe_bindings.clone(),
            records: records
                .iter()
                .map(|record| {
                    let DirectSurfaceLiquidConfigurationRecord {
                        key,
                        tile_fraction,
                        capacity_kg_m2_tile,
                        ofe_area_m2,
                        ground_ingress_mode,
                        runon_destination_ofe_id,
                        runon_destination_tile_id,
                    } = record;
                    SurfaceLiquidConfigurationRecordRestartV1 {
                        key: SurfaceLiquidKeyRestartV1::project(key),
                        tile_fraction: HexF64::from_f64(*tile_fraction),
                        capacity_kg_m2_tile: HexF64::from_f64(*capacity_kg_m2_tile),
                        ofe_area_m2: HexF64::from_f64(*ofe_area_m2),
                        ground_ingress_mode: (*ground_ingress_mode).into(),
                        runon_destination_ofe_id: runon_destination_ofe_id.clone(),
                        runon_destination_tile_id: runon_destination_tile_id.clone(),
                    }
                })
                .collect(),
        })
    }
    pub fn restore(&self) -> Result<DirectSurfaceLiquidConfiguration, SurfaceLiquidRestartError> {
        let records = self
            .records
            .iter()
            .map(|record| {
                Ok(DirectSurfaceLiquidConfigurationRecord {
                    key: record.key.restore(),
                    tile_fraction: nn("surface_liquid.tile_fraction", &record.tile_fraction)?,
                    capacity_kg_m2_tile: nn(
                        "surface_liquid.capacity_kg_m2_tile",
                        &record.capacity_kg_m2_tile,
                    )?,
                    ofe_area_m2: nn("surface_liquid.ofe_area_m2", &record.ofe_area_m2)?,
                    ground_ingress_mode: record.ground_ingress_mode.into(),
                    runon_destination_ofe_id: record.runon_destination_ofe_id.clone(),
                    runon_destination_tile_id: record.runon_destination_tile_id.clone(),
                })
            })
            .collect::<Result<Vec<_>, SurfaceLiquidRestartError>>()?;
        let restored = DirectSurfaceLiquidConfiguration::new(
            ResourceOwnerId::try_new(self.owner_id.clone())
                .map_err(|_| SurfaceLiquidRestartError::Identity)?,
            self.run_id,
            self.ofe_topology.clone(),
            self.ofe_bindings.clone(),
            records,
        )
        .map_err(|_| SurfaceLiquidRestartError::Domain {
            field: "surface_liquid.configuration",
        })?;
        if restored.configuration_sha256 != self.configuration_sha256.as_str() {
            return Err(SurfaceLiquidRestartError::Identity);
        }
        Ok(restored)
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceLiquidRecordRestartV1 {
    pub key: SurfaceLiquidKeyRestartV1,
    pub liquid_kg_m2_tile: HexF64,
    pub last_accepted_transaction_id: Option<HexU128>,
}
impl SurfaceLiquidRecordRestartV1 {
    fn project(v: &DirectSurfaceLiquidStateRecord) -> Self {
        let DirectSurfaceLiquidStateRecord {
            key,
            liquid_kg_m2_tile,
            last_accepted_transaction_id,
        } = v;
        Self {
            key: SurfaceLiquidKeyRestartV1::project(key),
            liquid_kg_m2_tile: HexF64::from_f64(*liquid_kg_m2_tile),
            last_accepted_transaction_id: last_accepted_transaction_id
                .map(|value| HexU128::from_u128(value.0)),
        }
    }
    fn restore(&self) -> Result<DirectSurfaceLiquidStateRecord, SurfaceLiquidRestartError> {
        Ok(DirectSurfaceLiquidStateRecord {
            key: self.key.restore(),
            liquid_kg_m2_tile: nn("surface_liquid.liquid_kg_m2_tile", &self.liquid_kg_m2_tile)?,
            last_accepted_transaction_id: self
                .last_accepted_transaction_id
                .as_ref()
                .map(|value| TransactionId(value.to_u128())),
        })
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceLiquidContinuationRestartV1 {
    pub ofe_id: OfeId,
    pub day_index: u64,
    pub next_interval_index: u8,
    pub cumulative_supply_m: HexF64,
    pub cumulative_infiltration_m: HexF64,
    pub last_accepted_transaction_id: Option<HexU128>,
}
impl SurfaceLiquidContinuationRestartV1 {
    fn project(v: &DirectSurfaceLiquidContinuationState) -> Self {
        let DirectSurfaceLiquidContinuationState {
            ofe_id,
            day_index,
            next_interval_index,
            cumulative_supply_m,
            cumulative_infiltration_m,
            last_accepted_transaction_id,
        } = v;
        Self {
            ofe_id: ofe_id.clone(),
            day_index: *day_index as u64,
            next_interval_index: *next_interval_index,
            cumulative_supply_m: HexF64::from_f64(*cumulative_supply_m),
            cumulative_infiltration_m: HexF64::from_f64(*cumulative_infiltration_m),
            last_accepted_transaction_id: last_accepted_transaction_id
                .map(|value| HexU128::from_u128(value.0)),
        }
    }
    fn restore(&self) -> Result<DirectSurfaceLiquidContinuationState, SurfaceLiquidRestartError> {
        if self.next_interval_index > 48 {
            return Err(SurfaceLiquidRestartError::Domain {
                field: "surface_liquid.next_interval_index",
            });
        }
        Ok(DirectSurfaceLiquidContinuationState {
            ofe_id: self.ofe_id.clone(),
            day_index: usize::try_from(self.day_index)
                .map_err(|_| SurfaceLiquidRestartError::DayIndex)?,
            next_interval_index: self.next_interval_index,
            cumulative_supply_m: nn(
                "surface_liquid.cumulative_supply_m",
                &self.cumulative_supply_m,
            )?,
            cumulative_infiltration_m: nn(
                "surface_liquid.cumulative_infiltration_m",
                &self.cumulative_infiltration_m,
            )?,
            last_accepted_transaction_id: self
                .last_accepted_transaction_id
                .as_ref()
                .map(|value| TransactionId(value.to_u128())),
        })
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSurfaceLiquidOwnedStateRestartV1 {
    pub owner_id: String,
    pub configuration_sha256: Sha256Hex,
    pub state_sha256: Sha256Hex,
    pub records: Vec<SurfaceLiquidRecordRestartV1>,
    pub continuations: Vec<SurfaceLiquidContinuationRestartV1>,
}
impl DirectSurfaceLiquidOwnedStateRestartV1 {
    pub fn project(v: &DirectSurfaceLiquidOwnedState) -> Result<Self, SurfaceLiquidRestartError> {
        let DirectSurfaceLiquidOwnedState {
            owner_id,
            configuration_sha256,
            state_sha256,
            records,
            continuations,
        } = v;
        Ok(Self {
            owner_id: owner_id.as_str().to_owned(),
            configuration_sha256: Sha256Hex::try_new(configuration_sha256.clone())
                .map_err(|_| SurfaceLiquidRestartError::Identity)?,
            state_sha256: Sha256Hex::try_new(state_sha256.clone())
                .map_err(|_| SurfaceLiquidRestartError::Identity)?,
            records: records
                .iter()
                .map(SurfaceLiquidRecordRestartV1::project)
                .collect(),
            continuations: continuations
                .iter()
                .map(SurfaceLiquidContinuationRestartV1::project)
                .collect(),
        })
    }
    pub fn restore(&self) -> Result<DirectSurfaceLiquidOwnedState, SurfaceLiquidRestartError> {
        if self
            .records
            .iter()
            .map(|record| &record.key)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != self.records.len()
            || self
                .continuations
                .iter()
                .map(|continuation| &continuation.ofe_id)
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != self.continuations.len()
        {
            return Err(SurfaceLiquidRestartError::Ordering);
        }
        Ok(DirectSurfaceLiquidOwnedState {
            owner_id: ResourceOwnerId::try_new(self.owner_id.clone())
                .map_err(|_| SurfaceLiquidRestartError::Identity)?,
            configuration_sha256: self.configuration_sha256.as_str().to_owned(),
            state_sha256: self.state_sha256.as_str().to_owned(),
            records: self
                .records
                .iter()
                .map(SurfaceLiquidRecordRestartV1::restore)
                .collect::<Result<_, _>>()?,
            continuations: self
                .continuations
                .iter()
                .map(SurfaceLiquidContinuationRestartV1::restore)
                .collect::<Result<_, _>>()?,
        })
    }
    pub fn restore_with_configuration(
        &self,
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> Result<DirectSurfaceLiquidOwnedState, SurfaceLiquidRestartError> {
        if self.configuration_sha256.as_str() != configuration.configuration_sha256 {
            return Err(SurfaceLiquidRestartError::Identity);
        }
        let state = self.restore()?;
        state
            .validate(configuration)
            .map_err(|_| SurfaceLiquidRestartError::Identity)?;
        if state.state_sha256 != self.state_sha256.as_str() {
            return Err(SurfaceLiquidRestartError::Identity);
        }
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(name: &str) -> DirectSurfaceLiquidStoreKey {
        DirectSurfaceLiquidStoreKey {
            run_id: 7,
            ofe_id: OfeId::try_new(name).unwrap(),
            tile_id: TileId::try_new(format!("tile-{name}")).unwrap(),
            surface_id: SurfaceId::try_new(format!("surface-{name}")).unwrap(),
            surface_class: SurfaceClass::BareMineralSoil,
            source_type: WaterSourceType::SurfaceLiquid,
            source_id: SourceId::try_new(format!("source-{name}")).unwrap(),
        }
    }

    fn physical_configuration_and_state(
        ofe_count: usize,
    ) -> (
        DirectSurfaceLiquidConfiguration,
        DirectSurfaceLiquidOwnedState,
    ) {
        let topology = (1..=ofe_count)
            .map(|lane_id| OfeId::try_new(format!("ofe-{lane_id}")).unwrap())
            .collect::<Vec<_>>();
        let bindings = topology
            .iter()
            .enumerate()
            .map(|(lane_index, ofe_id)| {
                let layer = openwepp_kernel_contract::SoilLayerId::try_new(format!(
                    "soil-{}",
                    lane_index + 1
                ))
                .unwrap();
                DirectSurfaceLiquidOfeBinding {
                    ofe_id: ofe_id.clone(),
                    production_lane_index: lane_index,
                    production_lane_id: u32::try_from(lane_index + 1).unwrap(),
                    ordered_soil_layer_ids: vec![layer.clone()],
                    infiltration_soil_thermal_layer_id: layer,
                }
            })
            .collect::<Vec<_>>();
        let records = topology
            .iter()
            .enumerate()
            .map(
                |(lane_index, ofe_id)| DirectSurfaceLiquidConfigurationRecord {
                    key: key(ofe_id.as_str()),
                    tile_fraction: 1.0,
                    capacity_kg_m2_tile: 2.0,
                    ofe_area_m2: 100.0,
                    ground_ingress_mode: DirectGroundIngressMode::OpenRawPrecipitation,
                    runon_destination_ofe_id: topology.get(lane_index + 1).cloned(),
                    runon_destination_tile_id: topology
                        .get(lane_index + 1)
                        .map(|destination| TileId::try_new(format!("tile-{destination}")).unwrap()),
                },
            )
            .collect::<Vec<_>>();
        let configuration = DirectSurfaceLiquidConfiguration::new(
            ResourceOwnerId::try_new("surface-owner").unwrap(),
            7,
            topology,
            bindings,
            records,
        )
        .unwrap();
        let liquid_by_key = configuration
            .records
            .iter()
            .map(|record| (record.key.clone(), 0.25))
            .collect::<std::collections::BTreeMap<_, _>>();
        let state =
            DirectSurfaceLiquidOwnedState::new_initial(&configuration, &liquid_by_key, 0).unwrap();
        (configuration, state)
    }

    fn assert_physical_restore_with_configuration(ofe_count: usize) {
        let (configuration, state) = physical_configuration_and_state(ofe_count);
        let projected = DirectSurfaceLiquidOwnedStateRestartV1::project(&state).unwrap();
        assert_eq!(
            projected
                .restore_with_configuration(&configuration)
                .unwrap(),
            state
        );
    }

    fn assert_restore_with_configuration_rejects_without_mutation(
        configuration: &DirectSurfaceLiquidConfiguration,
        projected: &DirectSurfaceLiquidOwnedStateRestartV1,
    ) {
        let before = projected.clone();
        assert!(projected.restore_with_configuration(configuration).is_err());
        assert_eq!(*projected, before);
    }

    #[test]
    fn surface_liquid_state_restart_accepts_one_ofe_physical_order() {
        assert_physical_restore_with_configuration(1);
    }

    #[test]
    fn surface_liquid_state_restart_accepts_nine_ofe_physical_order() {
        assert_physical_restore_with_configuration(9);
    }

    #[test]
    fn surface_liquid_state_restart_accepts_ten_ofe_physical_order() {
        assert_physical_restore_with_configuration(10);
    }

    #[test]
    fn surface_liquid_state_restart_accepts_nineteen_ofe_physical_order() {
        assert_physical_restore_with_configuration(19);
    }

    #[test]
    fn surface_liquid_state_restart_rejects_duplicate_identities_without_mutation() {
        let (configuration, state) = physical_configuration_and_state(10);
        let projected = DirectSurfaceLiquidOwnedStateRestartV1::project(&state).unwrap();

        let mut duplicate_record = projected.clone();
        duplicate_record.records.push(projected.records[0].clone());
        assert_eq!(
            duplicate_record.restore(),
            Err(SurfaceLiquidRestartError::Ordering)
        );
        assert_restore_with_configuration_rejects_without_mutation(
            &configuration,
            &duplicate_record,
        );

        let mut duplicate_continuation = projected.clone();
        duplicate_continuation
            .continuations
            .push(projected.continuations[0].clone());
        assert_eq!(
            duplicate_continuation.restore(),
            Err(SurfaceLiquidRestartError::Ordering)
        );
        assert_restore_with_configuration_rejects_without_mutation(
            &configuration,
            &duplicate_continuation,
        );
    }

    #[test]
    fn surface_liquid_state_restart_rejects_omission_without_mutation() {
        let (configuration, state) = physical_configuration_and_state(10);
        let mut projected = DirectSurfaceLiquidOwnedStateRestartV1::project(&state).unwrap();
        projected.records.pop().unwrap();
        assert_restore_with_configuration_rejects_without_mutation(&configuration, &projected);
    }

    #[test]
    fn surface_liquid_state_restart_rejects_substitution_without_mutation() {
        let (configuration, state) = physical_configuration_and_state(10);
        let mut projected = DirectSurfaceLiquidOwnedStateRestartV1::project(&state).unwrap();
        projected.records[9].key.ofe_id = OfeId::try_new("ofe-foreign").unwrap();
        assert_restore_with_configuration_rejects_without_mutation(&configuration, &projected);
    }

    #[test]
    fn surface_liquid_state_restart_rejects_reorder_without_mutation() {
        let (configuration, state) = physical_configuration_and_state(10);
        let mut projected = DirectSurfaceLiquidOwnedStateRestartV1::project(&state).unwrap();
        projected.records.swap(8, 9);
        projected.continuations.swap(8, 9);
        assert!(
            projected.restore().is_ok(),
            "bare restore must not impose lexical identifier order"
        );
        assert_restore_with_configuration_rejects_without_mutation(&configuration, &projected);
    }

    #[test]
    fn surface_liquid_state_restart_rejects_stale_digest_without_mutation() {
        let (configuration, state) = physical_configuration_and_state(10);
        let mut projected = DirectSurfaceLiquidOwnedStateRestartV1::project(&state).unwrap();
        projected.state_sha256 = Sha256Hex::try_new("0".repeat(64)).unwrap();
        assert_restore_with_configuration_rejects_without_mutation(&configuration, &projected);
    }

    #[test]
    fn surface_liquid_round_trip_and_order_domain_poisons() {
        let state = DirectSurfaceLiquidOwnedState {
            owner_id: ResourceOwnerId::try_new("surface-owner").unwrap(),
            configuration_sha256: "a".repeat(64),
            state_sha256: "b".repeat(64),
            records: vec![DirectSurfaceLiquidStateRecord {
                key: key("ofe-1"),
                liquid_kg_m2_tile: -0.0,
                last_accepted_transaction_id: Some(TransactionId(7)),
            }],
            continuations: vec![DirectSurfaceLiquidContinuationState {
                ofe_id: OfeId::try_new("ofe-1").unwrap(),
                day_index: 2,
                next_interval_index: 24,
                cumulative_supply_m: 0.02,
                cumulative_infiltration_m: 0.01,
                last_accepted_transaction_id: Some(TransactionId(7)),
            }],
        };
        let dto = DirectSurfaceLiquidOwnedStateRestartV1::project(&state).unwrap();
        assert_eq!(
            DirectSurfaceLiquidOwnedStateRestartV1::project(&dto.restore().unwrap()).unwrap(),
            dto
        );
        let mut duplicate = dto.clone();
        duplicate.records.push(duplicate.records[0].clone());
        assert_eq!(
            duplicate.restore(),
            Err(SurfaceLiquidRestartError::Ordering)
        );
        let mut bad = dto;
        bad.continuations[0].next_interval_index = 49;
        assert_eq!(
            bad.restore(),
            Err(SurfaceLiquidRestartError::Domain {
                field: "surface_liquid.next_interval_index"
            })
        );
    }
}
