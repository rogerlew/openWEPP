fn adaptive_owner_dimension_code_v1(
    dimension: crate::v9_real_consumer_shadow::AdaptiveOwnerDimensionV1,
) -> u8 {
    use crate::v9_real_consumer_shadow::AdaptiveOwnerDimensionV1;
    match dimension {
        AdaptiveOwnerDimensionV1::AreaEnergyJM2 => 0,
        AdaptiveOwnerDimensionV1::AreaMassKgM2 => 1,
        AdaptiveOwnerDimensionV1::CarbonDioxidePressurePa => 2,
        AdaptiveOwnerDimensionV1::HydraulicPotentialMm => 3,
        AdaptiveOwnerDimensionV1::SpecificHumidityKgKg => 4,
        AdaptiveOwnerDimensionV1::TemperatureKOrC => 5,
        AdaptiveOwnerDimensionV1::WaterDepthM => 6,
        AdaptiveOwnerDimensionV1::DimensionlessState => 7,
    }
}

fn adaptive_scalar_ordered_identity_set_sha256_v1(
    comparison: &crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut identities = Vec::with_capacity(comparison.scalars.len());
    for (ordinal, scalar) in comparison.scalars.iter().enumerate() {
        let identity = adaptive_framed_sha256(
            "stage3-adaptive-scalar-identity-v1",
            vec![
                ("ordinal", ordinal.to_be_bytes().to_vec()),
                ("owner_id", scalar.owner_id.as_bytes().to_vec()),
                ("path", scalar.path.as_bytes().to_vec()),
                (
                    "contract_id",
                    scalar.tolerance_authority.contract_id.as_bytes().to_vec(),
                ),
                (
                    "tolerance_id",
                    scalar.tolerance_authority.tolerance_id.as_bytes().to_vec(),
                ),
                (
                    "dimension",
                    vec![adaptive_owner_dimension_code_v1(
                        scalar.tolerance_authority.dimension,
                    )],
                ),
                (
                    "absolute_tolerance_bits",
                    scalar.absolute_tolerance.to_bits().to_be_bytes().to_vec(),
                ),
                (
                    "relative_tolerance_bits",
                    scalar.relative_tolerance.to_bits().to_be_bytes().to_vec(),
                ),
            ],
        )?;
        identities.push(("ordered_identity", identity.as_bytes().to_vec()));
    }
    adaptive_framed_sha256("stage3-adaptive-scalar-ordered-identity-set-v1", identities)
}

fn adaptive_pending_parcel_ordered_identity_set_sha256_v1(
    pending_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut identities = Vec::with_capacity(pending_parcels.len());
    for (ordinal, (map_key, parcel)) in pending_parcels.iter().enumerate() {
        let identity = adaptive_framed_sha256(
            "stage3-adaptive-pending-terminal-parcel-identity-v1",
            vec![
                ("ordinal", ordinal.to_be_bytes().to_vec()),
                ("map_key", map_key.as_bytes().to_vec()),
                ("parcel_digest", parcel.parcel_digest.as_bytes().to_vec()),
            ],
        )?;
        identities.push(("ordered_identity", identity.as_bytes().to_vec()));
    }
    adaptive_framed_sha256(
        "stage3-adaptive-pending-terminal-parcel-ordered-identity-set-v1",
        identities,
    )
}

fn adaptive_scalar_and_pending_identity_surfaces_v1(
    comparison: &crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    pending_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
) -> Result<Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1>, DirectSnowStage3V11AttachmentError> {
    Ok(vec![
        Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "complete_owner".to_owned(),
            path: "adaptive_scalars.cardinality".to_owned(),
            kind: "membership".to_owned(),
            exact_value: comparison.scalars.len().to_string(),
        },
        Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "complete_owner".to_owned(),
            path: "adaptive_scalars.ordered_identity_set_sha256".to_owned(),
            kind: "ordering".to_owned(),
            exact_value: format!(
                "{}:{}",
                comparison.scalars.len(),
                digest32_lower_hex(adaptive_scalar_ordered_identity_set_sha256_v1(comparison)?),
            ),
        },
        Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "snow".to_owned(),
            path: "pending_terminal_parcels.cardinality".to_owned(),
            kind: "membership".to_owned(),
            exact_value: pending_parcels.len().to_string(),
        },
        Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "snow".to_owned(),
            path: "pending_terminal_parcels.ordered_identity_set_sha256".to_owned(),
            kind: "ordering".to_owned(),
            exact_value: format!(
                "{}:{}",
                pending_parcels.len(),
                digest32_lower_hex(
                    adaptive_pending_parcel_ordered_identity_set_sha256_v1(pending_parcels)?,
                ),
            ),
        },
    ])
}
