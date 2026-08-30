//! Canonical complete-owner encoding for staged Stage-3 terminal custody.
use crate::{
    hydrology::{DirectSnowStage3PersistentState, Wb11HydrologyKernel},
    snow_stage3_v11_attachment::{
        DirectSnowStage3V11TerminalParcel, DirectSnowStage3V11TerminalParcelPosture,
    },
};
use openwepp_coupled_time::{Digest32, digest_bytes};
use std::collections::BTreeMap;

const OWNER_DOMAIN: &[u8] = b"OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V4\0";
const PARCEL_DOMAIN: &[u8] = b"OPENWEPP_STAGE3_TERMINAL_PARCEL_V1\0";
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CanonicalSnowOwnerV4Error(pub(crate) &'static str);

fn opaque(out: &mut Vec<u8>, value: &[u8]) -> Result<(), CanonicalSnowOwnerV4Error> {
    out.extend_from_slice(
        &u32::try_from(value.len())
            .map_err(|_| CanonicalSnowOwnerV4Error("field length"))?
            .to_be_bytes(),
    );
    out.extend_from_slice(value);
    Ok(())
}

fn parcel_fields(
    value: &DirectSnowStage3V11TerminalParcel,
) -> Result<Vec<u8>, CanonicalSnowOwnerV4Error> {
    let mut topology_bytes = Vec::new();
    let mut fraction_sum = 0.0;
    for (index, destination) in value.receiver_destinations.iter().enumerate() {
        if destination.destination_ofe_id != value.destination_ofe_id
            || !destination.destination_fraction.is_finite()
            || !(0.0..=1.0).contains(&destination.destination_fraction)
            || index > 0
                && (
                    value.receiver_destinations[index - 1]
                        .destination_ofe_id
                        .as_str(),
                    value.receiver_destinations[index - 1]
                        .destination_tile_id
                        .as_str(),
                ) >= (
                    destination.destination_ofe_id.as_str(),
                    destination.destination_tile_id.as_str(),
                )
        {
            return Err(CanonicalSnowOwnerV4Error(
                "pending terminal receiver topology",
            ));
        }
        fraction_sum += destination.destination_fraction;
        topology_bytes.extend_from_slice(destination.destination_ofe_id.as_bytes());
        topology_bytes.push(0);
        topology_bytes.extend_from_slice(destination.destination_tile_id.as_bytes());
        topology_bytes.extend_from_slice(&destination.destination_fraction.to_bits().to_be_bytes());
    }
    if value.posture != DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed
        || [
            value.parent_transaction_id,
            value.terminal_event_proposal_core_id,
            value.event_result_digest,
            value.receiver_topology_sha256,
        ]
        .contains(&Digest32::zero())
        || value.receiver_destinations.is_empty()
        || (fraction_sum - 1.0).abs() > 1.0e-12
        || digest_bytes(&topology_bytes) != value.receiver_topology_sha256
        || !value.mass_kg_m2_tile_ground.is_finite()
        || value.mass_kg_m2_tile_ground < 0.0
        || !value.temperature_k.is_finite()
        || !value.specific_liquid_enthalpy_j_kg.is_finite()
    {
        return Err(CanonicalSnowOwnerV4Error("pending terminal parcel domain"));
    }
    let mut out = Vec::new();
    out.extend_from_slice(&value.source_lane_id.to_be_bytes());
    out.extend_from_slice(value.parent_transaction_id.as_bytes());
    out.extend_from_slice(&value.event_ordinal.to_be_bytes());
    out.extend_from_slice(value.terminal_event_proposal_core_id.as_bytes());
    out.extend_from_slice(value.event_result_digest.as_bytes());
    out.extend_from_slice(value.receiver_topology_sha256.as_bytes());
    out.extend_from_slice(&value.support.start_ns().get().to_be_bytes());
    out.extend_from_slice(&value.support.end_ns().get().to_be_bytes());
    opaque(&mut out, value.destination_ofe_id.as_bytes())?;
    out.extend_from_slice(
        &u32::try_from(value.receiver_destinations.len())
            .map_err(|_| CanonicalSnowOwnerV4Error("receiver destination count"))?
            .to_be_bytes(),
    );
    for destination in &value.receiver_destinations {
        opaque(&mut out, destination.destination_ofe_id.as_bytes())?;
        opaque(&mut out, destination.destination_tile_id.as_bytes())?;
        out.extend_from_slice(&destination.destination_fraction.to_bits().to_be_bytes());
    }
    out.extend_from_slice(&value.mass_kg_m2_tile_ground.to_bits().to_be_bytes());
    out.extend_from_slice(&value.temperature_k.to_bits().to_be_bytes());
    out.extend_from_slice(&value.specific_liquid_enthalpy_j_kg.to_bits().to_be_bytes());
    out.push(0);
    Ok(out)
}

pub(crate) fn canonical_terminal_parcel_digest(
    value: &DirectSnowStage3V11TerminalParcel,
) -> Result<Digest32, CanonicalSnowOwnerV4Error> {
    let fields = parcel_fields(value)?;
    let mut preimage = Vec::with_capacity(PARCEL_DOMAIN.len() + fields.len());
    preimage.extend_from_slice(PARCEL_DOMAIN);
    preimage.extend_from_slice(&fields);
    Ok(digest_bytes(&preimage))
}

pub(crate) fn canonical_stage3_snow_owner_v4_bytes(
    lanes: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    lane_receipts: &BTreeMap<u32, Digest32>,
    tile_receipts: &BTreeMap<(String, String), Digest32>,
) -> Result<Vec<u8>, CanonicalSnowOwnerV4Error> {
    let mut out = OWNER_DOMAIN.to_vec();
    out.extend_from_slice(&4_u32.to_be_bytes());
    out.extend_from_slice(
        &u32::try_from(lanes.len())
            .map_err(|_| CanonicalSnowOwnerV4Error("lane count"))?
            .to_be_bytes(),
    );
    for (id, state) in lanes {
        out.extend_from_slice(&id.to_be_bytes());
        opaque(
            &mut out,
            &Wb11HydrologyKernel::serialize_stage3_persistent_state(state)
                .map_err(|_| CanonicalSnowOwnerV4Error("Stage-3 state"))?,
        )?;
    }
    out.extend_from_slice(
        &u32::try_from(lane_receipts.len())
            .map_err(|_| CanonicalSnowOwnerV4Error("lane receipt count"))?
            .to_be_bytes(),
    );
    for (id, digest) in lane_receipts {
        out.extend_from_slice(&id.to_be_bytes());
        out.extend_from_slice(digest.as_bytes());
    }
    out.extend_from_slice(
        &u32::try_from(tile_receipts.len())
            .map_err(|_| CanonicalSnowOwnerV4Error("tile receipt count"))?
            .to_be_bytes(),
    );
    for ((ofe, tile), digest) in tile_receipts {
        opaque(&mut out, ofe.as_bytes())?;
        opaque(&mut out, tile.as_bytes())?;
        out.extend_from_slice(digest.as_bytes());
    }
    out.extend_from_slice(
        &u32::try_from(parcels.len())
            .map_err(|_| CanonicalSnowOwnerV4Error("parcel count"))?
            .to_be_bytes(),
    );
    for (key, value) in parcels {
        let digest = canonical_terminal_parcel_digest(value)?;
        if key != &digest || value.parcel_digest != digest {
            return Err(CanonicalSnowOwnerV4Error("parcel digest/map key"));
        }
        out.extend_from_slice(digest.as_bytes());
        out.extend_from_slice(&parcel_fields(value)?);
    }
    Ok(out)
}

struct CanonicalSnowOwnerV4Cursor<'a> {
    remaining: &'a [u8],
}

impl<'a> CanonicalSnowOwnerV4Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { remaining: bytes }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], CanonicalSnowOwnerV4Error> {
        let (value, remaining) = self
            .remaining
            .split_at_checked(length)
            .ok_or(CanonicalSnowOwnerV4Error("truncated owner"))?;
        self.remaining = remaining;
        Ok(value)
    }

    fn u32(&mut self) -> Result<u32, CanonicalSnowOwnerV4Error> {
        let bytes = self.take(4)?;
        Ok(u32::from_be_bytes(
            bytes
                .try_into()
                .map_err(|_| CanonicalSnowOwnerV4Error("u32 width"))?,
        ))
    }

    fn opaque(&mut self) -> Result<&'a [u8], CanonicalSnowOwnerV4Error> {
        let length =
            usize::try_from(self.u32()?).map_err(|_| CanonicalSnowOwnerV4Error("field length"))?;
        self.take(length)
    }

    fn digest(&mut self) -> Result<Digest32, CanonicalSnowOwnerV4Error> {
        Ok(Digest32::from_bytes(
            self.take(32)?
                .try_into()
                .map_err(|_| CanonicalSnowOwnerV4Error("digest width"))?,
        ))
    }
}

/// Validate that an installed post-terminal V4 snow owner projects exactly to
/// the supplied persistent lane states before a solid-reappearance event. The
/// V4 boundary receipts remain part of the authenticated beginning owner; the
/// transition may not infer, discard, or substitute a lane state from them.
pub(crate) fn validate_snow_owner_v4_reappearance_projection(
    bytes: &[u8],
    lanes: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<(), CanonicalSnowOwnerV4Error> {
    let mut cursor = CanonicalSnowOwnerV4Cursor::new(bytes);
    if cursor.take(OWNER_DOMAIN.len())? != OWNER_DOMAIN || cursor.u32()? != 4 {
        return Err(CanonicalSnowOwnerV4Error("owner domain or version"));
    }
    if usize::try_from(cursor.u32()?).ok() != Some(lanes.len()) {
        return Err(CanonicalSnowOwnerV4Error("lane count"));
    }
    for (expected_lane, expected_state) in lanes {
        if cursor.u32()? != *expected_lane
            || cursor.opaque()?
                != Wb11HydrologyKernel::serialize_stage3_persistent_state(expected_state)
                    .map_err(|_| CanonicalSnowOwnerV4Error("Stage-3 state"))?
        {
            return Err(CanonicalSnowOwnerV4Error("lane state projection"));
        }
    }

    let lane_receipt_count = cursor.u32()?;
    let mut previous_lane = None;
    for _ in 0..lane_receipt_count {
        let lane = cursor.u32()?;
        let receipt = cursor.digest()?;
        if previous_lane.is_some_and(|previous| previous >= lane)
            || !lanes.contains_key(&lane)
            || receipt == Digest32::zero()
        {
            return Err(CanonicalSnowOwnerV4Error("lane receipt authority"));
        }
        previous_lane = Some(lane);
    }

    let tile_receipt_count = cursor.u32()?;
    let mut previous_tile: Option<(Vec<u8>, Vec<u8>)> = None;
    for _ in 0..tile_receipt_count {
        let ofe = cursor.opaque()?.to_vec();
        let tile = cursor.opaque()?.to_vec();
        let receipt = cursor.digest()?;
        if ofe.is_empty()
            || tile.is_empty()
            || previous_tile
                .as_ref()
                .is_some_and(|previous| previous >= &(ofe.clone(), tile.clone()))
            || receipt == Digest32::zero()
        {
            return Err(CanonicalSnowOwnerV4Error("tile receipt authority"));
        }
        previous_tile = Some((ofe, tile));
    }

    if lane_receipt_count == 0 && tile_receipt_count == 0 {
        return Err(CanonicalSnowOwnerV4Error(
            "reappearance boundary receipt authority",
        ));
    }
    if cursor.u32()? != 0 {
        return Err(CanonicalSnowOwnerV4Error(
            "reappearance pending terminal parcel",
        ));
    }
    if !cursor.remaining.is_empty() {
        return Err(CanonicalSnowOwnerV4Error("trailing owner bytes"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

    fn state(lane_id: u32) -> DirectSnowStage3PersistentState {
        let mut state = DirectSnowStage3PersistentState {
            schema_version: 1,
            terminal_event_model: None,
            fingerprint: 0,
            lane_id,
            next_interval_index: 7,
            layers: Vec::new(),
            detached_retained_liquid_kg_m2: 0.0,
            initial_ice_kg_m2: 0.0,
            initial_retained_liquid_kg_m2: 0.0,
            cumulative_snowfall_kg_m2: 1.0,
            cumulative_external_liquid_kg_m2: 0.0,
            cumulative_deposition_kg_m2: 0.0,
            cumulative_sublimation_kg_m2: 0.0,
            cumulative_melt_kg_m2: 1.0,
            cumulative_unresolved_liquid_kg_m2: 0.0,
            cumulative_complete_energy_j_m2: 0.0,
            cumulative_cold_energy_change_j_m2: 0.0,
            cumulative_terminal_unallocated_energy_j_m2: 0.0,
        };
        state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&state);
        state
    }

    fn parcel() -> DirectSnowStage3V11TerminalParcel {
        let receiver_destinations = vec![
            crate::snow_stage3_v11_attachment::DirectSnowStage3V11TerminalReceiverDestinationV1 {
                destination_ofe_id: "o".into(),
                destination_tile_id: "t".into(),
                destination_fraction: 1.0,
            },
        ];
        let mut topology_bytes = Vec::new();
        for destination in &receiver_destinations {
            topology_bytes.extend_from_slice(destination.destination_ofe_id.as_bytes());
            topology_bytes.push(0);
            topology_bytes.extend_from_slice(destination.destination_tile_id.as_bytes());
            topology_bytes
                .extend_from_slice(&destination.destination_fraction.to_bits().to_be_bytes());
        }
        let mut p = DirectSnowStage3V11TerminalParcel {
            support: TimeSupport::new(ModelTimeNs::new(10), ModelTimeNs::new(20)).unwrap(),
            source_lane_id: 7,
            parent_transaction_id: Digest32::from_bytes([1; 32]),
            event_ordinal: 3,
            terminal_event_proposal_core_id: Digest32::from_bytes([2; 32]),
            event_result_digest: Digest32::from_bytes([3; 32]),
            receiver_topology_sha256: digest_bytes(&topology_bytes),
            destination_ofe_id: "o".into(),
            receiver_destinations,
            mass_kg_m2_tile_ground: 0.25,
            temperature_k: 273.15,
            specific_liquid_enthalpy_j_kg: 0.0,
            posture: DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed,
            parcel_digest: Digest32::zero(),
        };
        p.parcel_digest = canonical_terminal_parcel_digest(&p).unwrap();
        p
    }
    #[test]
    fn exact_v4_is_deterministic() {
        let p = parcel();
        let ps = BTreeMap::from([(p.parcel_digest, p)]);
        let tiles = BTreeMap::from([(("o".into(), "t".into()), Digest32::from_bytes([6; 32]))]);
        let a =
            canonical_stage3_snow_owner_v4_bytes(&BTreeMap::new(), &ps, &BTreeMap::new(), &tiles)
                .unwrap();
        assert_eq!(
            a,
            canonical_stage3_snow_owner_v4_bytes(&BTreeMap::new(), &ps, &BTreeMap::new(), &tiles)
                .unwrap()
        );
        assert!(a.starts_with(OWNER_DOMAIN));
    }
    #[test]
    fn rejects_miskey_and_consumed() {
        let mut p = parcel();
        p.posture = DirectSnowStage3V11TerminalParcelPosture::Consumed;
        assert!(canonical_terminal_parcel_digest(&p).is_err());
        let p = parcel();
        assert!(
            canonical_stage3_snow_owner_v4_bytes(
                &BTreeMap::new(),
                &BTreeMap::from([(Digest32::from_bytes([9; 32]), p)]),
                &BTreeMap::new(),
                &BTreeMap::new()
            )
            .is_err()
        );
    }

    #[test]
    fn reappearance_projection_binds_v4_lanes_and_ordered_boundary_receipts() {
        let lanes = BTreeMap::from([(1, state(1)), (2, state(2))]);
        let lane_receipts = BTreeMap::from([
            (1, Digest32::from_bytes([5; 32])),
            (2, Digest32::from_bytes([6; 32])),
        ]);
        let tile_receipts = BTreeMap::from([
            (("o1".into(), "t1".into()), Digest32::from_bytes([7; 32])),
            (("o2".into(), "t2".into()), Digest32::from_bytes([8; 32])),
        ]);
        let exact = canonical_stage3_snow_owner_v4_bytes(
            &lanes,
            &BTreeMap::new(),
            &lane_receipts,
            &tile_receipts,
        )
        .unwrap();
        validate_snow_owner_v4_reappearance_projection(&exact, &lanes)
            .expect("post-terminal V4 projects to exact reappearance lanes");

        let mut substituted_lanes = lanes.clone();
        substituted_lanes.get_mut(&2).unwrap().next_interval_index += 1;
        assert!(
            validate_snow_owner_v4_reappearance_projection(&exact, &substituted_lanes).is_err()
        );

        let zero_receipt = canonical_stage3_snow_owner_v4_bytes(
            &lanes,
            &BTreeMap::new(),
            &BTreeMap::from([(1, Digest32::zero())]),
            &tile_receipts,
        )
        .unwrap();
        assert!(validate_snow_owner_v4_reappearance_projection(&zero_receipt, &lanes).is_err());

        let no_receipts = canonical_stage3_snow_owner_v4_bytes(
            &lanes,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
        )
        .unwrap();
        assert!(validate_snow_owner_v4_reappearance_projection(&no_receipts, &lanes).is_err());

        let mut trailing = exact.clone();
        trailing.push(0);
        assert!(validate_snow_owner_v4_reappearance_projection(&trailing, &lanes).is_err());

        let p = parcel();
        let pending = canonical_stage3_snow_owner_v4_bytes(
            &lanes,
            &BTreeMap::from([(p.parcel_digest, p)]),
            &lane_receipts,
            &tile_receipts,
        )
        .unwrap();
        assert!(validate_snow_owner_v4_reappearance_projection(&pending, &lanes).is_err());
    }

    #[test]
    fn reappearance_projection_rejects_noncanonical_receipt_order() {
        let lanes = BTreeMap::from([(1, state(1)), (2, state(2))]);
        let exact = canonical_stage3_snow_owner_v4_bytes(
            &lanes,
            &BTreeMap::new(),
            &BTreeMap::from([
                (1, Digest32::from_bytes([5; 32])),
                (2, Digest32::from_bytes([6; 32])),
            ]),
            &BTreeMap::new(),
        )
        .unwrap();
        let first_receipt_lane_offset = OWNER_DOMAIN.len()
            + 4
            + 4
            + lanes
                .values()
                .map(|state| {
                    4 + 4
                        + Wb11HydrologyKernel::serialize_stage3_persistent_state(state)
                            .unwrap()
                            .len()
                })
                .sum::<usize>()
            + 4;
        let mut out_of_order = exact;
        out_of_order[first_receipt_lane_offset..first_receipt_lane_offset + 4]
            .copy_from_slice(&2_u32.to_be_bytes());
        out_of_order[first_receipt_lane_offset + 36..first_receipt_lane_offset + 40]
            .copy_from_slice(&1_u32.to_be_bytes());
        assert!(validate_snow_owner_v4_reappearance_projection(&out_of_order, &lanes).is_err());
    }
}
