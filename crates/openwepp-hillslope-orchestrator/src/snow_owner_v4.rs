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
    if value.posture != DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed
        || [
            value.parent_transaction_id,
            value.terminal_event_proposal_core_id,
            value.event_result_digest,
            value.receiver_topology_sha256,
        ]
        .contains(&Digest32::zero())
        || !value.destination_fraction.is_finite()
        || !(0.0..=1.0).contains(&value.destination_fraction)
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
    opaque(&mut out, value.destination_tile_id.as_bytes())?;
    out.extend_from_slice(&value.destination_fraction.to_bits().to_be_bytes());
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

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};
    fn parcel() -> DirectSnowStage3V11TerminalParcel {
        let mut p = DirectSnowStage3V11TerminalParcel {
            support: TimeSupport::new(ModelTimeNs::new(10), ModelTimeNs::new(20)).unwrap(),
            source_lane_id: 7,
            parent_transaction_id: Digest32::from_bytes([1; 32]),
            event_ordinal: 3,
            terminal_event_proposal_core_id: Digest32::from_bytes([2; 32]),
            event_result_digest: Digest32::from_bytes([3; 32]),
            receiver_topology_sha256: Digest32::from_bytes([4; 32]),
            destination_ofe_id: "o".into(),
            destination_tile_id: "t".into(),
            destination_fraction: 1.0,
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
}
