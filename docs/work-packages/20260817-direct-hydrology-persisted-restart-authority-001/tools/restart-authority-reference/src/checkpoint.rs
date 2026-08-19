use crate::{DirectHydrologyRestartV1, HexU128, Sha256Hex, canonical_sha256};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerKindV1 {
    Gsi,
    Forcing,
    VegetationV10,
    LseV2,
    SoilThermal,
    Biogeochemistry,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OmissionConsequenceV1 {
    PhenologyDivergence,
    ForcingReplay,
    VegetationDivergence,
    EnergyDivergence,
    SoilTemperatureDivergence,
    CarbonNitrogenDivergence,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerPoisonV1 {
    FieldDomain,
    CrossOwnerJoin,
    CanonicalOrder,
    NestedDigest,
    Omission,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PersistedOwnerEnvelopeV1 {
    pub kind: OwnerKindV1,
    pub owner_id: String,
    pub field_domains: Vec<String>,
    pub cross_owner_joins: Vec<String>,
    pub canonical_order_keys: Vec<String>,
    pub last_accepted_transaction_id: Option<HexU128>,
    pub configuration_sha256: Sha256Hex,
    pub payload_hex: String,
    pub nested_sha256: Sha256Hex,
    pub omission_consequence: OmissionConsequenceV1,
    pub executable_poisons: Vec<OwnerPoisonV1>,
}

#[derive(Serialize)]
struct OwnerDigestInput<'a> {
    kind: OwnerKindV1,
    owner_id: &'a str,
    field_domains: &'a [String],
    cross_owner_joins: &'a [String],
    canonical_order_keys: &'a [String],
    last_accepted_transaction_id: &'a Option<HexU128>,
    configuration_sha256: &'a Sha256Hex,
    payload_hex: &'a str,
    omission_consequence: OmissionConsequenceV1,
    executable_poisons: &'a [OwnerPoisonV1],
}
impl PersistedOwnerEnvelopeV1 {
    pub fn compute_digest(&self) -> Result<Sha256Hex, CheckpointError> {
        Sha256Hex::try_new(
            canonical_sha256(&OwnerDigestInput {
                kind: self.kind,
                owner_id: &self.owner_id,
                field_domains: &self.field_domains,
                cross_owner_joins: &self.cross_owner_joins,
                canonical_order_keys: &self.canonical_order_keys,
                last_accepted_transaction_id: &self.last_accepted_transaction_id,
                configuration_sha256: &self.configuration_sha256,
                payload_hex: &self.payload_hex,
                omission_consequence: self.omission_consequence,
                executable_poisons: &self.executable_poisons,
            })
            .map_err(|_| CheckpointError::Canonical)?,
        )
        .map_err(|_| CheckpointError::Digest)
    }
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.owner_id.is_empty()
            || self.field_domains.is_empty()
            || self.cross_owner_joins.is_empty()
            || self.canonical_order_keys.is_empty()
            || self.executable_poisons.len() != 5
            || !self.payload_hex.len().is_multiple_of(2)
            || !self
                .payload_hex
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err(CheckpointError::Owner);
        }
        for values in [
            &self.field_domains,
            &self.cross_owner_joins,
            &self.canonical_order_keys,
        ] {
            if values.windows(2).any(|pair| pair[0] >= pair[1]) {
                return Err(CheckpointError::Ordering);
            }
        }
        if self.compute_digest()? != self.nested_sha256 {
            return Err(CheckpointError::Digest);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerSetV1 {
    pub gsi: PersistedOwnerEnvelopeV1,
    pub forcing: PersistedOwnerEnvelopeV1,
    pub vegetation_v10: PersistedOwnerEnvelopeV1,
    pub lse_v2: PersistedOwnerEnvelopeV1,
    pub soil_thermal: PersistedOwnerEnvelopeV1,
    pub biogeochemistry: PersistedOwnerEnvelopeV1,
}
impl OwnerSetV1 {
    fn ordered(&self) -> [&PersistedOwnerEnvelopeV1; 6] {
        [
            &self.gsi,
            &self.forcing,
            &self.vegetation_v10,
            &self.lse_v2,
            &self.soil_thermal,
            &self.biogeochemistry,
        ]
    }
    pub fn validate(&self, lineage: &Option<HexU128>) -> Result<(), CheckpointError> {
        let expected = [
            OwnerKindV1::Gsi,
            OwnerKindV1::Forcing,
            OwnerKindV1::VegetationV10,
            OwnerKindV1::LseV2,
            OwnerKindV1::SoilThermal,
            OwnerKindV1::Biogeochemistry,
        ];
        let mut ids = BTreeSet::new();
        for (owner, kind) in self.ordered().into_iter().zip(expected) {
            owner.validate()?;
            if owner.kind != kind
                || !ids.insert(&owner.owner_id)
                || &owner.last_accepted_transaction_id != lineage
            {
                return Err(CheckpointError::Join);
            }
        }
        if self.forcing.configuration_sha256 != self.gsi.configuration_sha256 {
            return Err(CheckpointError::Join);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CheckpointPhaseV1 {
    BetweenDays {
        next_day_index: u64,
        accepted_interval_count: u64,
        committed_owners: OwnerSetV1,
    },
    InProgressDay {
        day_index: u64,
        next_interval_index: u8,
        accepted_interval_count: u64,
        day_beginning_owners: OwnerSetV1,
        transactional_owners: Box<OwnerSetV1>,
        forcing_day_receipt_sha256: Sha256Hex,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectV10RealConsumerCheckpointV1 {
    pub schema: String,
    pub version: u16,
    pub run_configuration_sha256: Sha256Hex,
    pub topology_sha256: Sha256Hex,
    pub last_accepted_transaction_id: Option<HexU128>,
    pub direct_hydrology: DirectHydrologyRestartV1,
    pub phase: CheckpointPhaseV1,
    pub payload_sha256: Sha256Hex,
}
#[derive(Serialize)]
struct CheckpointDigestInput<'a> {
    schema: &'a str,
    version: u16,
    run_configuration_sha256: &'a Sha256Hex,
    topology_sha256: &'a Sha256Hex,
    last_accepted_transaction_id: &'a Option<HexU128>,
    direct_hydrology: &'a DirectHydrologyRestartV1,
    phase: &'a CheckpointPhaseV1,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CheckpointError {
    #[error("canonical serialization failed")]
    Canonical,
    #[error("digest mismatch")]
    Digest,
    #[error("owner envelope invalid")]
    Owner,
    #[error("canonical ordering invalid")]
    Ordering,
    #[error("cross-owner join invalid")]
    Join,
    #[error("checkpoint phase/cursor invalid")]
    Cursor,
}
impl DirectV10RealConsumerCheckpointV1 {
    pub fn compute_digest(&self) -> Result<Sha256Hex, CheckpointError> {
        Sha256Hex::try_new(
            canonical_sha256(&CheckpointDigestInput {
                schema: &self.schema,
                version: self.version,
                run_configuration_sha256: &self.run_configuration_sha256,
                topology_sha256: &self.topology_sha256,
                last_accepted_transaction_id: &self.last_accepted_transaction_id,
                direct_hydrology: &self.direct_hydrology,
                phase: &self.phase,
            })
            .map_err(|_| CheckpointError::Canonical)?,
        )
        .map_err(|_| CheckpointError::Digest)
    }
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.schema != "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1" || self.version != 1 {
            return Err(CheckpointError::Join);
        }
        match &self.phase {
            CheckpointPhaseV1::BetweenDays {
                accepted_interval_count,
                committed_owners,
                ..
            } => {
                if accepted_interval_count % 48 != 0 {
                    return Err(CheckpointError::Cursor);
                }
                committed_owners.validate(&self.last_accepted_transaction_id)?;
            }
            CheckpointPhaseV1::InProgressDay {
                next_interval_index,
                accepted_interval_count,
                day_beginning_owners,
                transactional_owners,
                ..
            } => {
                if !(1..=47).contains(next_interval_index)
                    || accepted_interval_count % 48 != u64::from(*next_interval_index)
                {
                    return Err(CheckpointError::Cursor);
                }
                day_beginning_owners.validate(&self.last_accepted_transaction_id)?;
                transactional_owners.validate(&self.last_accepted_transaction_id)?;
            }
        }
        if self.compute_digest()? != self.payload_sha256 {
            return Err(CheckpointError::Digest);
        }
        Ok(())
    }
    pub fn abort_to_day_beginning(&self) -> Result<OwnerSetV1, CheckpointError> {
        self.validate()?;
        match &self.phase {
            CheckpointPhaseV1::InProgressDay {
                day_beginning_owners,
                ..
            } => Ok(day_beginning_owners.clone()),
            CheckpointPhaseV1::BetweenDays {
                committed_owners, ..
            } => Ok(committed_owners.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DirectHydrologyRestartV1;
    use openwepp_hillslope_orchestrator::{
        DirectLaneConstructorInputs, DirectLaneTransferLedger, DirectRunConstructorInputs,
        DirectRunFrame, DirectRunIdentity,
    };

    fn sha(c: char) -> Sha256Hex {
        Sha256Hex::try_new(c.to_string().repeat(64)).unwrap()
    }
    fn owner(
        kind: OwnerKindV1,
        id: &str,
        consequence: OmissionConsequenceV1,
        payload: u8,
        lineage: Option<HexU128>,
    ) -> PersistedOwnerEnvelopeV1 {
        let mut value = PersistedOwnerEnvelopeV1 {
            kind,
            owner_id: id.into(),
            field_domains: vec!["finite".into()],
            cross_owner_joins: vec!["transaction_lineage".into()],
            canonical_order_keys: vec!["owner_id".into()],
            last_accepted_transaction_id: lineage,
            configuration_sha256: sha('a'),
            payload_hex: format!("{payload:02x}"),
            nested_sha256: sha('0'),
            omission_consequence: consequence,
            executable_poisons: vec![
                OwnerPoisonV1::FieldDomain,
                OwnerPoisonV1::CrossOwnerJoin,
                OwnerPoisonV1::CanonicalOrder,
                OwnerPoisonV1::NestedDigest,
                OwnerPoisonV1::Omission,
            ],
        };
        value.nested_sha256 = value.compute_digest().unwrap();
        value
    }
    fn owners(payload: u8, lineage: Option<HexU128>) -> OwnerSetV1 {
        OwnerSetV1 {
            gsi: owner(
                OwnerKindV1::Gsi,
                "gsi",
                OmissionConsequenceV1::PhenologyDivergence,
                payload,
                lineage.clone(),
            ),
            forcing: owner(
                OwnerKindV1::Forcing,
                "forcing",
                OmissionConsequenceV1::ForcingReplay,
                payload,
                lineage.clone(),
            ),
            vegetation_v10: owner(
                OwnerKindV1::VegetationV10,
                "vegetation",
                OmissionConsequenceV1::VegetationDivergence,
                payload,
                lineage.clone(),
            ),
            lse_v2: owner(
                OwnerKindV1::LseV2,
                "lse",
                OmissionConsequenceV1::EnergyDivergence,
                payload,
                lineage.clone(),
            ),
            soil_thermal: owner(
                OwnerKindV1::SoilThermal,
                "soil",
                OmissionConsequenceV1::SoilTemperatureDivergence,
                payload,
                lineage.clone(),
            ),
            biogeochemistry: owner(
                OwnerKindV1::Biogeochemistry,
                "bgc",
                OmissionConsequenceV1::CarbonNitrogenDivergence,
                payload,
                lineage,
            ),
        }
    }
    fn hydrology() -> DirectHydrologyRestartV1 {
        let identity = DirectRunIdentity::new(1, 1, 1, 1).unwrap();
        let mut lane = DirectLaneConstructorInputs::from_topology(0, 1, 1).unwrap();
        lane.area_m2 = 1.0;
        let mut frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
            identity,
            vec![lane],
        ))
        .unwrap();
        frame.lane_transfer_ledger = vec![DirectLaneTransferLedger {
            lane_id: 1,
            upstream_lane_id: 0,
            downstream_lane_id: 0,
            upstream_area_ratio: 1.0,
            area_m2: 1.0,
            outgoing_surface_m: 0.0,
            outgoing_lateral_m: 0.0,
            received_surface_m: 0.0,
            received_lateral_m: 0.0,
            net_transfer_m: 0.0,
        }];
        DirectHydrologyRestartV1::project(&frame, sha('b'), &[sha('c')]).unwrap()
    }
    fn checkpoint(
        phase: CheckpointPhaseV1,
        lineage: Option<HexU128>,
    ) -> DirectV10RealConsumerCheckpointV1 {
        let mut value = DirectV10RealConsumerCheckpointV1 {
            schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
            version: 1,
            run_configuration_sha256: sha('d'),
            topology_sha256: sha('e'),
            last_accepted_transaction_id: lineage,
            direct_hydrology: hydrology(),
            phase,
            payload_sha256: sha('0'),
        };
        value.payload_sha256 = value.compute_digest().unwrap();
        value
    }
    fn advance(mut state: OwnerSetV1, from: u8, through: u8) -> OwnerSetV1 {
        for interval in from..through {
            for owner in [
                &mut state.gsi,
                &mut state.forcing,
                &mut state.vegetation_v10,
                &mut state.lse_v2,
                &mut state.soil_thermal,
                &mut state.biogeochemistry,
            ] {
                let value = u8::from_str_radix(&owner.payload_hex, 16)
                    .unwrap()
                    .wrapping_add(interval)
                    .wrapping_add(1);
                owner.payload_hex = format!("{value:02x}");
                owner.nested_sha256 = owner.compute_digest().unwrap();
            }
        }
        state
    }
    #[test]
    fn interval_24_fresh_object_continuation_and_exact_abort() {
        let lineage = Some(HexU128::from_u128(24));
        let beginning = owners(0, lineage.clone());
        let at_24 = advance(beginning.clone(), 0, 24);
        let continuous = advance(at_24.clone(), 24, 48);
        let cp = checkpoint(
            CheckpointPhaseV1::InProgressDay {
                day_index: 3,
                next_interval_index: 24,
                accepted_interval_count: 24,
                day_beginning_owners: beginning.clone(),
                transactional_owners: Box::new(at_24.clone()),
                forcing_day_receipt_sha256: sha('f'),
            },
            lineage,
        );
        cp.validate().unwrap();
        let serialized = crate::to_canonical_bytes(&cp).unwrap();
        let fresh: DirectV10RealConsumerCheckpointV1 =
            crate::from_canonical_bytes(&serialized).unwrap();
        let resumed = match fresh.phase {
            CheckpointPhaseV1::InProgressDay {
                transactional_owners,
                ..
            } => advance(*transactional_owners, 24, 48),
            _ => unreachable!(),
        };
        assert_eq!(resumed, continuous);
        assert_eq!(cp.abort_to_day_beginning().unwrap(), beginning);
    }
    #[test]
    fn nested_digest_cursor_join_and_omission_poisons_preserve_live_bytes() {
        let lineage = Some(HexU128::from_u128(24));
        let beginning = owners(0, lineage.clone());
        let cp = checkpoint(
            CheckpointPhaseV1::InProgressDay {
                day_index: 3,
                next_interval_index: 24,
                accepted_interval_count: 24,
                day_beginning_owners: beginning.clone(),
                transactional_owners: Box::new(advance(beginning, 0, 24)),
                forcing_day_receipt_sha256: sha('f'),
            },
            lineage,
        );
        let live = crate::to_canonical_bytes(&cp).unwrap();
        let mut poison = cp.clone();
        if let CheckpointPhaseV1::InProgressDay {
            transactional_owners,
            ..
        } = &mut poison.phase
        {
            transactional_owners.gsi.payload_hex = "ff".into();
        }
        poison.payload_sha256 = poison.compute_digest().unwrap();
        assert_eq!(poison.validate(), Err(CheckpointError::Digest));
        assert_eq!(crate::to_canonical_bytes(&cp).unwrap(), live);
        let mut poison = cp.clone();
        if let CheckpointPhaseV1::InProgressDay {
            next_interval_index,
            ..
        } = &mut poison.phase
        {
            *next_interval_index = 0;
        }
        poison.payload_sha256 = poison.compute_digest().unwrap();
        assert_eq!(poison.validate(), Err(CheckpointError::Cursor));
        assert_eq!(crate::to_canonical_bytes(&cp).unwrap(), live);
        let mut poison = cp.clone();
        if let CheckpointPhaseV1::InProgressDay {
            transactional_owners,
            ..
        } = &mut poison.phase
        {
            transactional_owners.gsi.owner_id = transactional_owners.forcing.owner_id.clone();
            transactional_owners.gsi.nested_sha256 =
                transactional_owners.gsi.compute_digest().unwrap();
        }
        poison.payload_sha256 = poison.compute_digest().unwrap();
        assert_eq!(poison.validate(), Err(CheckpointError::Join));
        assert_eq!(crate::to_canonical_bytes(&cp).unwrap(), live);
    }
}
