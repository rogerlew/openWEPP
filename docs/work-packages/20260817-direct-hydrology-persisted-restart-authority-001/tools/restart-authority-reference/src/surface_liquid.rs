use crate::{HexF64, Sha256Hex};
use openwepp_hillslope_orchestrator::{
    DirectSurfaceLiquidContinuationState, DirectSurfaceLiquidOwnedState,
    DirectSurfaceLiquidStateRecord, DirectSurfaceLiquidStoreKey,
};
use openwepp_kernel_contract::{ResourceOwnerId, TileId, TransactionId};
use openwepp_land_surface_energy::{OfeId, SourceId, SurfaceClass, SurfaceId, WaterSourceType};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SurfaceLiquidRestartError {
    #[error("invalid identity or digest")]
    Identity,
    #[error("{field} violates surface-liquid domain")]
    Domain { field: &'static str },
    #[error("records or continuations are not strictly ordered and unique")]
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
    pub surface_class: SurfaceClass,
    pub source_type: WaterSourceType,
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
            surface_class: *surface_class,
            source_type: *source_type,
            source_id: source_id.clone(),
        }
    }
    fn restore(&self) -> DirectSurfaceLiquidStoreKey {
        DirectSurfaceLiquidStoreKey {
            run_id: self.run_id,
            ofe_id: self.ofe_id.clone(),
            tile_id: self.tile_id.clone(),
            surface_id: self.surface_id.clone(),
            surface_class: self.surface_class,
            source_type: self.source_type,
            source_id: self.source_id.clone(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceLiquidRecordRestartV1 {
    pub key: SurfaceLiquidKeyRestartV1,
    pub liquid_kg_m2_tile: HexF64,
    pub last_accepted_transaction_id: Option<TransactionId>,
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
            last_accepted_transaction_id: *last_accepted_transaction_id,
        }
    }
    fn restore(&self) -> Result<DirectSurfaceLiquidStateRecord, SurfaceLiquidRestartError> {
        Ok(DirectSurfaceLiquidStateRecord {
            key: self.key.restore(),
            liquid_kg_m2_tile: nn("surface_liquid.liquid_kg_m2_tile", &self.liquid_kg_m2_tile)?,
            last_accepted_transaction_id: self.last_accepted_transaction_id,
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
    pub last_accepted_transaction_id: Option<TransactionId>,
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
            last_accepted_transaction_id: *last_accepted_transaction_id,
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
            last_accepted_transaction_id: self.last_accepted_transaction_id,
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
        if self.records.windows(2).any(|p| p[0].key >= p[1].key)
            || self
                .continuations
                .windows(2)
                .any(|p| p[0].ofe_id >= p[1].ofe_id)
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
