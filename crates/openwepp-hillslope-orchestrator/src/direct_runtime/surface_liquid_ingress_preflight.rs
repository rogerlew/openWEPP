fn preflight_surface_liquid_ingress_public_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<(), DirectSurfaceLiquidError> {
    configuration.preflight_schema_and_identity_structure()?;
    resource
        .beginning_state()
        .preflight_schema_and_identity_structure(configuration)?;
    if input.transaction_id != resource.transaction_id() || input.transaction_id.0 == 0 {
        return Err(DirectSurfaceLiquidError::Identity(
            "ingress transaction mismatch",
        ));
    }
    preflight_resource_working_state_identities(configuration, resource)?;
    preflight_surface_liquid_ingress_input_identities(configuration, input)?;
    configuration.preflight_declared_digest()?;
    resource.beginning_state().preflight_declared_digest()?;
    Ok(())
}

pub(crate) fn preflight_surface_liquid_ingress_input_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<(), DirectSurfaceLiquidError> {
    preflight_tile_ingress_identities(configuration, input)?;
    preflight_parameter_identities(configuration, input.transaction_id, &input.wb14_parameters)
}

fn preflight_resource_working_state_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
) -> Result<(), DirectSurfaceLiquidError> {
    if resource.working_state().owner_id != resource.beginning_state().owner_id
        || resource.working_state().configuration_sha256
            != resource.beginning_state().configuration_sha256
        || resource.working_state().records.len() != configuration.records.len()
        || resource.working_state().continuations != resource.beginning_state().continuations
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "resource working-state identity mismatch",
        ));
    }
    for ((working, beginning), configured) in resource
        .working_state()
        .records
        .iter()
        .zip(&resource.beginning_state().records)
        .zip(&configuration.records)
    {
        if working.key != configured.key
            || beginning.key != configured.key
            || working.last_accepted_transaction_id != beginning.last_accepted_transaction_id
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "invalid resource working-state record",
            ));
        }
    }
    Ok(())
}

fn validate_resource_working_state_domains(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
) -> Result<(), DirectSurfaceLiquidError> {
    for (working, configured) in resource
        .working_state()
        .records
        .iter()
        .zip(&configuration.records)
    {
        if !working.liquid_kg_m2_tile.is_finite()
            || working.liquid_kg_m2_tile < 0.0
            || working.liquid_kg_m2_tile > configured.capacity_kg_m2_tile
        {
            return Err(DirectSurfaceLiquidError::Domain(
                "invalid resource working-state liquid",
            ));
        }
    }
    Ok(())
}

fn validate_cadence(
    beginning: &DirectSurfaceLiquidOwnedState,
    input: &DirectSurfaceLiquidIngressInput,
    parent_child_mode: bool,
    finalize_parent_interval: bool,
) -> Result<(), DirectSurfaceLiquidError> {
    if input.interval_s > INTERVAL_S {
        return Err(production_binding_failure(
            input.transaction_id,
            None,
            "surface-liquid interval exceeds the admitted WB14 cadence",
        ));
    }
    let initial = beginning
        .records
        .first()
        .ok_or(DirectSurfaceLiquidError::Schema(
            "empty surface-liquid state",
        ))?
        .last_accepted_transaction_id
        .is_none();
    for continuation in &beginning.continuations {
        if (initial
            && (continuation.next_interval_index != 0
                || continuation.cumulative_supply_m.to_bits() != 0.0_f64.to_bits()
                || continuation.cumulative_infiltration_m.to_bits() != 0.0_f64.to_bits()))
            || (!initial && continuation.next_interval_index == 0 && input.interval_index != 0)
        {
            return Err(production_binding_failure(
                input.transaction_id,
                Some(continuation.ofe_id.clone()),
                "initial or accepted WB14 continuation mismatch",
            ));
        }
        let expected = if continuation.next_interval_index == 48 {
            (continuation.day_index.checked_add(1), 0)
        } else {
            (
                Some(continuation.day_index),
                continuation.next_interval_index,
            )
        };
        let projected_parent_index = input.interval_index.checked_add(1);
        let accepted_parent_local_projection = parent_child_mode
            && continuation.day_index == input.day_index
            && Some(continuation.next_interval_index) == projected_parent_index;
        if (expected.0 != Some(input.day_index) || expected.1 != input.interval_index)
            && !accepted_parent_local_projection
        {
            return Err(production_binding_failure(
                input.transaction_id,
                Some(continuation.ofe_id.clone()),
                "WB14 day or interval continuation mismatch",
            ));
        }
        if !finalize_parent_interval && input.interval_s >= INTERVAL_S {
            return Err(production_binding_failure(
                input.transaction_id,
                Some(continuation.ofe_id.clone()),
                "non-final WB14 child must be shorter than its parent interval",
            ));
        }
    }
    Ok(())
}

fn preflight_parameter_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    transaction_id: TransactionId,
    rows: &[DirectOfeWb14Parameters],
) -> Result<(), DirectSurfaceLiquidError> {
    if let Some(row) = rows
        .iter()
        .find(|row| !configuration.ofe_topology.contains(&row.ofe_id))
    {
        return Err(DirectSurfaceLiquidError::canonical_failure(
            DirectSurfaceLiquidErrorCode::E002,
            DirectSurfaceLiquidPhase::IngressCandidate,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(transaction_id),
                owner_id: Some(configuration.owner_id.clone()),
                ofe_id: Some(row.ofe_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256: None,
                attempted_owner_sha256: None,
            },
            "unknown WB14 parameter OFE identity",
        ));
    }
    Ok(())
}

fn preflight_parameter_domains(
    rows: &[DirectOfeWb14Parameters],
) -> Result<(), DirectSurfaceLiquidError> {
    for row in rows {
        require_positive(row.effective_conductivity_m_s, "effective conductivity")?;
        require_nonnegative(row.matric_potential_m, "matric potential")?;
        require_nonnegative(
            row.infiltration_storage_capacity_m,
            "infiltration storage capacity",
        )?;
    }
    Ok(())
}

fn validate_parameter_cardinality_and_order<'a>(
    configuration: &DirectSurfaceLiquidConfiguration,
    transaction_id: TransactionId,
    rows: &'a [DirectOfeWb14Parameters],
) -> Result<BTreeMap<OfeId, &'a DirectOfeWb14Parameters>, DirectSurfaceLiquidError> {
    if rows.len() != configuration.ofe_topology.len() {
        return Err(DirectSurfaceLiquidError::canonical_failure(
            DirectSurfaceLiquidErrorCode::E005,
            DirectSurfaceLiquidPhase::IngressCandidate,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(transaction_id),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256: None,
                attempted_owner_sha256: None,
            },
            "WB14 parameter cardinality mismatch",
        ));
    }
    let mut result = BTreeMap::new();
    for (row, expected) in rows.iter().zip(&configuration.ofe_topology) {
        if result.insert(row.ofe_id.clone(), row).is_some() {
            return Err(DirectSurfaceLiquidError::canonical_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::IngressCandidate,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(transaction_id),
                    ofe_id: Some(row.ofe_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                    beginning_owner_sha256: None,
                    attempted_owner_sha256: None,
                },
                "duplicate WB14 parameter identity",
            ));
        }
        if &row.ofe_id != expected {
            return Err(production_binding_failure(
                transaction_id,
                Some(row.ofe_id.clone()),
                "WB14 parameter order mismatch",
            ));
        }
    }
    Ok(result)
}

fn preflight_tile_ingress_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<(), DirectSurfaceLiquidError> {
    for ingress in &input.tile_ingress {
        let (ofe_id, tile_id, surface_id) = ingress.identity();
        let configured = configuration
            .records
            .iter()
            .find(|row| {
                &row.key.ofe_id == ofe_id
                    && &row.key.tile_id == tile_id
                    && &row.key.surface_id == surface_id
            })
            .ok_or_else(|| {
                DirectSurfaceLiquidError::canonical_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    DirectSurfaceLiquidPhase::IngressCandidate,
                    DirectSurfaceLiquidErrorContext {
                        transaction_id: Some(input.transaction_id),
                        owner_id: Some(configuration.owner_id.clone()),
                        ofe_id: Some(ofe_id.clone()),
                        tile_id: Some(tile_id.clone()),
                        surface_id: Some(surface_id.clone()),
                        source_id: None,
                        parcel_id: None,
                    },
                    super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                        beginning_owner_sha256: None,
                        attempted_owner_sha256: None,
                    },
                    "unknown tile ground ingress",
                )
            })?;
        if ingress.mode() != configured.ground_ingress_mode {
            return Err(DirectSurfaceLiquidError::canonical_failure(
                DirectSurfaceLiquidErrorCode::E002,
                DirectSurfaceLiquidPhase::IngressCandidate,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ofe_id: Some(ofe_id.clone()),
                    tile_id: Some(tile_id.clone()),
                    surface_id: Some(surface_id.clone()),
                    source_id: Some(configured.key.source_id.clone()),
                    parcel_id: None,
                },
                super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                    beginning_owner_sha256: None,
                    attempted_owner_sha256: None,
                },
                "open/covered ingress mode mismatch",
            ));
        }
    }
    Ok(())
}

fn preflight_tile_ingress_domains(
    configuration: &DirectSurfaceLiquidConfiguration,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<(), DirectSurfaceLiquidError> {
    let invalid_amount = |amount: &DirectIngressAmount, require_full_interval: bool| {
        if !amount.mass_kg_m2_tile_ground.is_finite() || amount.mass_kg_m2_tile_ground < 0.0 {
            Some("ingress mass domain")
        } else if !amount.temperature_k.is_finite()
            || !(200.0..=350.0).contains(&amount.temperature_k)
        {
            Some("liquid temperature domain")
        } else if !amount.specific_liquid_enthalpy_j_kg.is_finite()
            || !amount.start_s.is_finite()
            || !amount.end_s.is_finite()
            || amount.start_s < 0.0
            || amount.end_s <= amount.start_s
            || amount.end_s > input.interval_s
            || (require_full_interval
                && (amount.start_s.to_bits() != 0.0_f64.to_bits()
                    || amount.end_s.to_bits() != input.interval_s.to_bits()))
        {
            Some("invalid ingress amount domain")
        } else {
            None
        }
    };
    for ingress in &input.tile_ingress {
        let detail = match ingress {
            DirectTileGroundIngress::OpenRawPrecipitation {
                raw_precipitation, ..
            } => invalid_amount(raw_precipitation, false),
            DirectTileGroundIngress::OpenLiquidParcels { parcels, .. } => parcels
                .iter()
                .find_map(|parcel| invalid_amount(&parcel.amount, false)),
            DirectTileGroundIngress::CoveredCanopyRelease { release, .. } => {
                invalid_amount(&release.throughfall, true)
                    .or_else(|| invalid_amount(&release.initial_drainage, true))
                    .or_else(|| invalid_amount(&release.second_drainage, true))
                    .or_else(|| invalid_amount(&release.stemflow, true))
            }
            DirectTileGroundIngress::CoveredCanopyReleaseAndRunon {
                release,
                runon_parcels,
                ..
            } => invalid_amount(&release.throughfall, true)
                .or_else(|| invalid_amount(&release.initial_drainage, true))
                .or_else(|| invalid_amount(&release.second_drainage, true))
                .or_else(|| invalid_amount(&release.stemflow, true))
                .or_else(|| {
                    runon_parcels
                        .iter()
                        .find_map(|parcel| invalid_amount(&parcel.amount, false))
                }),
        };
        if let Some(detail) = detail {
            let (ofe_id, tile_id, surface_id) = ingress.identity();
            let source_id = configuration
                .records
                .iter()
                .find(|row| {
                    &row.key.ofe_id == ofe_id
                        && &row.key.tile_id == tile_id
                        && &row.key.surface_id == surface_id
                })
                .map(|row| row.key.source_id.clone());
            return Err(DirectSurfaceLiquidError::canonical_failure(
                DirectSurfaceLiquidErrorCode::E003,
                DirectSurfaceLiquidPhase::IngressCandidate,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ofe_id: Some(ofe_id.clone()),
                    tile_id: Some(tile_id.clone()),
                    surface_id: Some(surface_id.clone()),
                    source_id,
                    parcel_id: None,
                },
                super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                    beginning_owner_sha256: None,
                    attempted_owner_sha256: None,
                },
                detail,
            ));
        }
    }
    Ok(())
}

fn validate_tile_ingress_cardinality(
    configuration: &DirectSurfaceLiquidConfiguration,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<(), DirectSurfaceLiquidError> {
    if input.tile_ingress.len() != configuration.records.len() {
        return Err(DirectSurfaceLiquidError::canonical_failure(
            DirectSurfaceLiquidErrorCode::E005,
            DirectSurfaceLiquidPhase::IngressCandidate,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(input.transaction_id),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256: None,
                attempted_owner_sha256: None,
            },
            "ground-ingress tile cardinality mismatch",
        ));
    }
    let mut seen = BTreeSet::new();
    for ingress in &input.tile_ingress {
        let (ofe_id, tile_id, surface_id) = ingress.identity();
        if !seen.insert((ofe_id.clone(), tile_id.clone(), surface_id.clone())) {
            return Err(DirectSurfaceLiquidError::canonical_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::IngressCandidate,
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(input.transaction_id),
                    owner_id: Some(configuration.owner_id.clone()),
                    ofe_id: Some(ofe_id.clone()),
                    tile_id: Some(tile_id.clone()),
                    surface_id: Some(surface_id.clone()),
                    source_id: None,
                    parcel_id: None,
                },
                super::surface_liquid_owner::DirectSurfaceLiquidRollbackHashes {
                    beginning_owner_sha256: None,
                    attempted_owner_sha256: None,
                },
                "duplicate tile ground ingress",
            ));
        }
    }
    Ok(())
}
