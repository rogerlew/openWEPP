/// One explicit default-off invocation of the actual `DirectV10` owner stack.
#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoveredCarrierLiveConsumptionRowV1 {
    pub lane_id: u32,
    pub forcing_sha256: Digest32,
    pub reference_specific_humidity_bits: u64,
    pub snow_specific_humidity_bits: u64,
    pub shared_specific_humidity_bits: u64,
    pub snow_vapor_into_surface_bits: u64,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoveredCarrierCondensationCreditAuditV1 {
    pub transaction_id: u128,
    pub hydrology_owner_id: String,
    pub ofe_id: String,
    pub tile_id: String,
    pub surface_id: String,
    pub amount_bits: u64,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OpenSnowLiveConsumptionRowV1 {
    pub lane_id: u32,
    pub forcing_sha256: Digest32,
    pub reference_specific_humidity_bits: u64,
    pub snow_specific_humidity_bits: u64,
    pub vapor_outward_bits: u64,
}

#[cfg(test)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct CoveredCarrierLiveConsumptionAuditV1 {
    pub carrier_rows: Vec<CoveredCarrierLiveConsumptionRowV1>,
    pub open_snow_rows: Vec<OpenSnowLiveConsumptionRowV1>,
    pub condensation_credits: Vec<CoveredCarrierCondensationCreditAuditV1>,
}

#[cfg(test)]
std::thread_local! {
    static COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT: std::cell::RefCell<Option<CoveredCarrierLiveConsumptionAuditV1>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn begin_covered_carrier_live_consumption_audit() {
    COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT.with(|audit| {
        *audit.borrow_mut() = Some(CoveredCarrierLiveConsumptionAuditV1::default());
    });
}

#[cfg(test)]
pub(crate) fn take_covered_carrier_live_consumption_audit() -> CoveredCarrierLiveConsumptionAuditV1
{
    COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT
        .with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
fn audit_covered_carrier_live_row(row: CoveredCarrierLiveConsumptionRowV1) {
    COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.carrier_rows.push(row);
        }
    });
}

#[cfg(test)]
fn audit_covered_carrier_condensation_credits(
    credits: &[openwepp_land_surface_energy::CondensationCredit],
) {
    COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit
                .condensation_credits
                .extend(
                    credits
                        .iter()
                        .map(|credit| CoveredCarrierCondensationCreditAuditV1 {
                            transaction_id: credit.transaction_id.0,
                            hydrology_owner_id: credit.hydrology_owner_id.as_str().to_owned(),
                            ofe_id: credit.ofe_id.as_str().to_owned(),
                            tile_id: credit.tile_id.as_str().to_owned(),
                            surface_id: credit.surface_id.as_str().to_owned(),
                            amount_bits: credit.amount_kg_m2_stand_ground.to_bits(),
                        }),
                );
        }
    });
}

#[cfg(not(test))]
fn audit_covered_carrier_condensation_credits(
    _: &[openwepp_land_surface_energy::CondensationCredit],
) {
}

fn shared_carrier_specific_humidity_v1(
    surfaces: &[CarrierSurface],
) -> Result<f64, DirectV11RealConsumerError> {
    let mut denominator = 0.0;
    let mut numerator = 0.0;
    let mut common_active_bits = None;
    let mut equal_active_nodes = true;
    for surface in surfaces {
        let conductance = surface.vapor_conductance_m_s;
        if !conductance.is_finite()
            || conductance < 0.0
            || !surface.specific_humidity.is_finite()
            || !(0.0..=1.0).contains(&surface.specific_humidity)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier humidity conductance domain",
            ));
        }
        if conductance.to_bits() == 0.0_f64.to_bits() {
            continue;
        }
        denominator += conductance;
        numerator += conductance * surface.specific_humidity;
        match common_active_bits {
            None => common_active_bits = Some(surface.specific_humidity.to_bits()),
            Some(bits) => equal_active_nodes &= bits == surface.specific_humidity.to_bits(),
        }
    }
    if !denominator.is_finite() || denominator <= 0.0 || !numerator.is_finite() {
        return Err(DirectV11RealConsumerError::Identity(
            "covered carrier humidity denominator",
        ));
    }
    if equal_active_nodes {
        return Ok(f64::from_bits(common_active_bits.ok_or(
            DirectV11RealConsumerError::Identity("covered carrier active humidity set"),
        )?));
    }
    let shared = numerator / denominator;
    if !shared.is_finite() || !(0.0..=1.0).contains(&shared) {
        return Err(DirectV11RealConsumerError::Identity(
            "covered carrier shared humidity domain",
        ));
    }
    Ok(shared)
}

#[cfg(test)]
mod shared_carrier_specific_humidity_tests {
    use super::*;

    fn surface(q: f64, conductance: f64) -> CarrierSurface {
        CarrierSurface {
            temperature_k: 273.15,
            specific_humidity: q,
            heat_conductance_m_s: conductance,
            vapor_conductance_m_s: conductance,
        }
    }

    #[test]
    fn equal_active_nodes_are_exact_and_active_one_bit_poison_uses_weighted_solve() {
        let q = 0.003_757_503_415_507_667_5_f64;
        let inactive_poison_q = f64::from_bits(q.to_bits() + 9);
        let exact = shared_carrier_specific_humidity_v1(&[
            surface(q, 1.0),
            surface(inactive_poison_q, 0.0),
            surface(q, 3.0),
        ])
        .expect("equal active-node humidity");
        assert_eq!(exact.to_bits(), q.to_bits());

        let active_poison_q = f64::from_bits(q.to_bits() + 1);
        let weighted = shared_carrier_specific_humidity_v1(&[
            surface(q, 1.0),
            surface(inactive_poison_q, 0.0),
            surface(active_poison_q, 1.0),
        ])
        .expect("one-bit active-node weighted humidity");
        let expected = (q + active_poison_q) / 2.0;
        assert_eq!(weighted.to_bits(), expected.to_bits());
        assert_ne!(weighted.to_bits(), q.to_bits());

        assert!(shared_carrier_specific_humidity_v1(&[surface(q, 0.0), surface(q, 0.0),]).is_err());
        assert!(shared_carrier_specific_humidity_v1(&[surface(q, -f64::MIN_POSITIVE)]).is_err());
    }
}

#[derive(Clone)]
pub struct DirectV11RealConsumerStack<'a> {
    pub beginning: DirectV10RealConsumerShadow,
    pub interval: &'a DirectV9ShadowIntervalInput,
    pub day_index: usize,
    pub interval_index: usize,
    pub(super) finalize_wb14_parent_interval: bool,
    pub(super) wb14_coupled_child_binding:
        Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
    pub(super) ending: Option<DirectV10RealConsumerShadow>,
    pub(super) last_support_receipt: Option<LseSupportAdmissibilityReceiptV1>,
    #[cfg(test)]
    pub(super) last_hydrology_candidate:
        Option<crate::land_surface_energy_shadow::UnifiedRealHydrologyCandidate>,
    pub(super) ending_snow_owner_bytes: Option<Vec<u8>>,
}

#[derive(Clone)]
pub(crate) struct PrecipitationProducerManifestRowV1 {
    pub destination_topology_index: u32,
    pub source: Stage3PrecipitationSourceV1,
    pub semantic_receipt_ordinal: u32,
    pub mass_kg_m2_tile_ground: f64,
    pub enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1,
    pub source_identity_sha256: Digest32,
    pub producer_beginning_state_sha256: Digest32,
}

pub(crate) fn validate_precipitation_producer_manifest(
    set: &Stage3PrecipitationPhaseParcelSetV1,
    manifest: &[PrecipitationProducerManifestRowV1],
) -> Result<(), DirectV11RealConsumerError> {
    let actual_keys = set
        .parcels
        .iter()
        .map(|parcel| {
            (
                parcel.destination_topology_index,
                parcel.source,
                parcel.semantic_receipt_ordinal,
            )
        })
        .collect::<std::collections::BTreeSet<_>>();
    let positive_manifest_keys = manifest
        .iter()
        .filter(|row| row.mass_kg_m2_tile_ground > 0.0)
        .map(|row| {
            (
                row.destination_topology_index,
                row.source,
                row.semantic_receipt_ordinal,
            )
        })
        .collect::<std::collections::BTreeSet<_>>();
    if actual_keys.len() != set.parcels.len()
        || positive_manifest_keys.len()
            != manifest
                .iter()
                .filter(|row| row.mass_kg_m2_tile_ground > 0.0)
                .count()
        || actual_keys != positive_manifest_keys
    {
        return Err(DirectV11RealConsumerError::Identity(
            "precipitation producer route parcel cardinality",
        ));
    }
    for expected in manifest {
        let matching = set
            .parcels
            .iter()
            .filter(|parcel| {
                parcel.destination_topology_index == expected.destination_topology_index
                    && parcel.source == expected.source
                    && parcel.semantic_receipt_ordinal == expected.semantic_receipt_ordinal
            })
            .collect::<Vec<_>>();
        if expected.mass_kg_m2_tile_ground == 0.0 {
            if !matching.is_empty() {
                return Err(DirectV11RealConsumerError::Identity(
                    "zero precipitation producer route parcel",
                ));
            }
            continue;
        }
        let [parcel] = matching.as_slice() else {
            return Err(DirectV11RealConsumerError::Identity(
                "precipitation producer route parcel cardinality",
            ));
        };
        let provider_matches = parcel.enthalpy_provider == expected.enthalpy_provider;
        if parcel.mass_kg_m2_tile_ground.to_bits() != expected.mass_kg_m2_tile_ground.to_bits()
            || parcel.source_identity_sha256 != expected.source_identity_sha256
            || parcel.producer_beginning_state_sha256 != expected.producer_beginning_state_sha256
            || !provider_matches
        {
            return Err(DirectV11RealConsumerError::Identity(
                "precipitation producer route parcel manifest",
            ));
        }
    }
    Ok(())
}

const STAGE3_SNOW_DENSITY_KG_M3: f64 = 100.0;

struct LanePrecipitationBuildContext<'a, 'stack> {
    stack: &'a DirectV11SnowCoveredRealConsumerStack<'stack>,
    canopy_releases: &'a crate::land_surface_energy_shadow::FixedCapCanopyReleasesByDestination,
    support: openwepp_coupled_time::TimeSupport,
    lane_id: u32,
    ofe_id: OfeId,
    forcing: DirectSnowStage3SupportInput,
    beginning_snow_state_sha256: Digest32,
    beginning_vegetation_state_sha256: Digest32,
    forcing_identity_sha256: Digest32,
    temperature_k: f64,
    solid_specific_heat_j_kg_k: f64,
}

fn seal_precipitation_parcel(
    parcel: Stage3PrecipitationPhaseParcelV1,
) -> Result<Stage3PrecipitationPhaseParcelV1, DirectV11RealConsumerError> {
    parcel
        .seal()
        .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))
}

fn append_precipitation_parcel(
    parcels: &mut Vec<Stage3PrecipitationPhaseParcelV1>,
    parcel: Stage3PrecipitationPhaseParcelV1,
) -> Result<(), DirectV11RealConsumerError> {
    if parcel.mass_kg_m2_tile_ground != 0.0 {
        parcels.push(seal_precipitation_parcel(parcel)?);
    }
    Ok(())
}

fn append_covered_liquid_routes(
    context: &LanePrecipitationBuildContext<'_, '_>,
    destination: &Stage3PrecipitationDestinationV1,
    parcels: &mut Vec<Stage3PrecipitationPhaseParcelV1>,
) -> Result<(), DirectV11RealConsumerError> {
    let key = (destination.ofe_id.clone(), destination.tile_id.clone());
    let (release, source_identity) =
        context
            .canopy_releases
            .get(&key)
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered precipitation release destination",
            ))?;
    for (source, amount) in [
        (
            Stage3PrecipitationSourceV1::VegetationTerminalThroughfall,
            &release.throughfall,
        ),
        (
            Stage3PrecipitationSourceV1::VegetationTerminalInitialDrainage,
            &release.initial_drainage,
        ),
        (
            Stage3PrecipitationSourceV1::VegetationTerminalSecondDrainage,
            &release.second_drainage,
        ),
        (
            Stage3PrecipitationSourceV1::VegetationTerminalStemflow,
            &release.stemflow,
        ),
    ] {
        append_precipitation_parcel(
            parcels,
            Stage3PrecipitationPhaseParcelV1 {
                support: context.support,
                lane_id: context.lane_id,
                destination_topology_index: destination.topology_index,
                destination_ofe_id: destination.ofe_id.clone(),
                destination_tile_id: destination.tile_id.clone(),
                phase: Stage3PrecipitationPhaseV1::Liquid,
                source,
                semantic_receipt_ordinal: 0,
                mass_kg_m2_tile_ground: amount.mass_kg_m2_tile_ground,
                enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
                    specific_enthalpy_j_kg: amount.specific_liquid_enthalpy_j_kg,
                    provider_receipt_sha256: *source_identity,
                },
                source_identity_sha256: *source_identity,
                producer_beginning_state_sha256: context.beginning_vegetation_state_sha256,
                receipt_sha256: Digest32::zero(),
            },
        )?;
    }
    Ok(())
}

fn append_open_raw_rain_routes(
    context: &LanePrecipitationBuildContext<'_, '_>,
    destination: &Stage3PrecipitationDestinationV1,
    parcels: &mut Vec<Stage3PrecipitationPhaseParcelV1>,
) -> Result<(), DirectV11RealConsumerError> {
    for (ordinal, atmospheric) in context
        .stack
        .interval
        .lse_forcing
        .precipitation_parcels
        .iter()
        .filter(|parcel| {
            parcel.destination_ofe_id == destination.ofe_id
                && parcel.destination_tile_id == destination.tile_id
        })
        .enumerate()
    {
        let source_identity = digest_bytes(
            &serde_json::to_vec(atmospheric)
                .map_err(|_| DirectV11RealConsumerError::Identity("open rain source framing"))?,
        );
        append_precipitation_parcel(
            parcels,
            Stage3PrecipitationPhaseParcelV1 {
                support: context.support,
                lane_id: context.lane_id,
                destination_topology_index: destination.topology_index,
                destination_ofe_id: destination.ofe_id.clone(),
                destination_tile_id: destination.tile_id.clone(),
                phase: Stage3PrecipitationPhaseV1::Liquid,
                source: Stage3PrecipitationSourceV1::OpenRawRain,
                semantic_receipt_ordinal: u32::try_from(ordinal).map_err(|_| {
                    DirectV11RealConsumerError::Identity("open rain semantic ordinal")
                })?,
                mass_kg_m2_tile_ground: atmospheric.amount_kg_m2_destination_tile_ground,
                enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
                    specific_enthalpy_j_kg: atmospheric.specific_liquid_enthalpy_j_kg.ok_or(
                        DirectV11RealConsumerError::Identity("open rain enthalpy provider"),
                    )?,
                    provider_receipt_sha256: source_identity,
                },
                source_identity_sha256: source_identity,
                producer_beginning_state_sha256: digest32_from_lower_hex(
                    atmospheric
                        .source_state_sha256
                        .as_ref()
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "open rain source-state identity",
                        ))?
                        .as_str(),
                )?,
                receipt_sha256: Digest32::zero(),
            },
        )?;
    }
    Ok(())
}

fn build_destination_parcels(
    context: &LanePrecipitationBuildContext<'_, '_>,
    destination: &Stage3PrecipitationDestinationV1,
) -> Result<Vec<Stage3PrecipitationPhaseParcelV1>, DirectV11RealConsumerError> {
    let mut parcels = Vec::new();
    append_precipitation_parcel(
        &mut parcels,
        Stage3PrecipitationPhaseParcelV1 {
            support: context.support,
            lane_id: context.lane_id,
            destination_topology_index: destination.topology_index,
            destination_ofe_id: destination.ofe_id.clone(),
            destination_tile_id: destination.tile_id.clone(),
            phase: Stage3PrecipitationPhaseV1::Solid,
            source: Stage3PrecipitationSourceV1::AtmosphericGroundSnow,
            semantic_receipt_ordinal: 0,
            mass_kg_m2_tile_ground: context.forcing.forcing.snowfall_m * STAGE3_SNOW_DENSITY_KG_M3,
            enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1::Temperature {
                temperature_k: context.temperature_k,
                reference_temperature_k: 273.16,
                specific_heat_j_kg_k: context.solid_specific_heat_j_kg_k,
                provider_receipt_sha256: context.forcing_identity_sha256,
            },
            source_identity_sha256: context.forcing_identity_sha256,
            producer_beginning_state_sha256: context.forcing_identity_sha256,
            receipt_sha256: Digest32::zero(),
        },
    )?;
    if destination.canopy_covered {
        append_covered_liquid_routes(context, destination, &mut parcels)?;
    } else {
        append_open_raw_rain_routes(context, destination, &mut parcels)?;
    }
    Ok(parcels)
}

fn append_open_producer_manifest(
    context: &LanePrecipitationBuildContext<'_, '_>,
    destination: &Stage3PrecipitationDestinationV1,
    manifest: &mut Vec<PrecipitationProducerManifestRowV1>,
) -> Result<(), DirectV11RealConsumerError> {
    for (ordinal, atmospheric) in context
        .stack
        .interval
        .lse_forcing
        .precipitation_parcels
        .iter()
        .filter(|parcel| {
            parcel.destination_ofe_id == destination.ofe_id
                && parcel.destination_tile_id == destination.tile_id
        })
        .enumerate()
    {
        let source_identity = digest_bytes(&serde_json::to_vec(atmospheric).map_err(|_| {
            DirectV11RealConsumerError::Identity("open rain producer manifest framing")
        })?);
        manifest.push(PrecipitationProducerManifestRowV1 {
            destination_topology_index: destination.topology_index,
            source: Stage3PrecipitationSourceV1::OpenRawRain,
            semantic_receipt_ordinal: u32::try_from(ordinal).map_err(|_| {
                DirectV11RealConsumerError::Identity("open rain producer manifest ordinal")
            })?,
            mass_kg_m2_tile_ground: atmospheric.amount_kg_m2_destination_tile_ground,
            enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
                specific_enthalpy_j_kg: atmospheric.specific_liquid_enthalpy_j_kg.ok_or(
                    DirectV11RealConsumerError::Identity("open rain producer manifest enthalpy"),
                )?,
                provider_receipt_sha256: source_identity,
            },
            source_identity_sha256: source_identity,
            producer_beginning_state_sha256: digest32_from_lower_hex(
                atmospheric
                    .source_state_sha256
                    .as_ref()
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "open rain producer manifest state",
                    ))?
                    .as_str(),
            )?,
        });
    }
    Ok(())
}

fn append_covered_producer_manifest(
    context: &LanePrecipitationBuildContext<'_, '_>,
    destination: &Stage3PrecipitationDestinationV1,
    manifest: &mut Vec<PrecipitationProducerManifestRowV1>,
) -> Result<(), DirectV11RealConsumerError> {
    let key = (destination.ofe_id.clone(), destination.tile_id.clone());
    let (release, source_identity) =
        context
            .canopy_releases
            .get(&key)
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered precipitation producer manifest destination",
            ))?;
    for (source, amount) in [
        (
            Stage3PrecipitationSourceV1::VegetationTerminalThroughfall,
            &release.throughfall,
        ),
        (
            Stage3PrecipitationSourceV1::VegetationTerminalInitialDrainage,
            &release.initial_drainage,
        ),
        (
            Stage3PrecipitationSourceV1::VegetationTerminalSecondDrainage,
            &release.second_drainage,
        ),
        (
            Stage3PrecipitationSourceV1::VegetationTerminalStemflow,
            &release.stemflow,
        ),
    ] {
        manifest.push(PrecipitationProducerManifestRowV1 {
            destination_topology_index: destination.topology_index,
            source,
            semantic_receipt_ordinal: 0,
            mass_kg_m2_tile_ground: amount.mass_kg_m2_tile_ground,
            enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
                specific_enthalpy_j_kg: amount.specific_liquid_enthalpy_j_kg,
                provider_receipt_sha256: *source_identity,
            },
            source_identity_sha256: *source_identity,
            producer_beginning_state_sha256: context.beginning_vegetation_state_sha256,
        });
    }
    Ok(())
}

fn construct_precipitation_producer_manifest(
    context: &LanePrecipitationBuildContext<'_, '_>,
    destinations: &[Stage3PrecipitationDestinationV1],
) -> Result<Vec<PrecipitationProducerManifestRowV1>, DirectV11RealConsumerError> {
    let mut manifest = Vec::new();
    for destination in destinations {
        manifest.push(PrecipitationProducerManifestRowV1 {
            destination_topology_index: destination.topology_index,
            source: Stage3PrecipitationSourceV1::AtmosphericGroundSnow,
            semantic_receipt_ordinal: 0,
            mass_kg_m2_tile_ground: context.forcing.forcing.snowfall_m * STAGE3_SNOW_DENSITY_KG_M3,
            enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1::Temperature {
                temperature_k: context.temperature_k,
                reference_temperature_k: 273.16,
                specific_heat_j_kg_k: context.solid_specific_heat_j_kg_k,
                provider_receipt_sha256: context.forcing_identity_sha256,
            },
            source_identity_sha256: context.forcing_identity_sha256,
            producer_beginning_state_sha256: context.forcing_identity_sha256,
        });
        if !destination.canopy_covered {
            append_open_producer_manifest(context, destination, &mut manifest)?;
        }
    }
    for destination in destinations.iter().filter(|value| value.canopy_covered) {
        append_covered_producer_manifest(context, destination, &mut manifest)?;
    }
    Ok(manifest)
}

fn build_lane_precipitation_set(
    context: &LanePrecipitationBuildContext<'_, '_>,
) -> Result<Stage3PrecipitationPhaseParcelSetV1, DirectV11RealConsumerError> {
    let mut destinations = Vec::new();
    let mut parcels = Vec::new();
    for record in context
        .stack
        .beginning
        .inner
        .surface_configuration
        .records
        .iter()
        .filter(|record| record.key.ofe_id == context.ofe_id)
    {
        let topology_index = u32::try_from(destinations.len())
            .map_err(|_| DirectV11RealConsumerError::Identity("precipitation topology width"))?;
        let key = (record.key.ofe_id.clone(), record.key.tile_id.clone());
        let destination = Stage3PrecipitationDestinationV1 {
            topology_index,
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            fraction_of_ofe: record.tile_fraction,
            canopy_covered: matches!(
                context.stack.snow_surface_forcing_by_destination.get(&key),
                Some(SealedStage3TileBoundaryForcingV1::V11CanopyCovered(_))
            ),
            destination_identity_sha256: digest_bytes(&serde_json::to_vec(record).map_err(
                |_| DirectV11RealConsumerError::Identity("precipitation topology framing"),
            )?),
        };
        parcels.extend(build_destination_parcels(context, &destination)?);
        destinations.push(destination);
    }
    parcels.sort_by_key(|parcel| {
        (
            parcel.lane_id,
            parcel.destination_topology_index,
            parcel.phase,
            parcel.source,
            parcel.semantic_receipt_ordinal,
        )
    });
    let topology_identity_sha256 = digest_bytes(
        &destinations
            .iter()
            .flat_map(|destination| {
                destination
                    .destination_identity_sha256
                    .as_bytes()
                    .iter()
                    .copied()
            })
            .collect::<Vec<_>>(),
    );
    Stage3PrecipitationPhaseParcelSetV1 {
        schema_version: 1,
        support: context.support,
        lane_id: context.lane_id,
        ofe_id: context.ofe_id.clone(),
        ofe_ground_basis: true,
        beginning_snow_state_sha256: context.beginning_snow_state_sha256,
        topology_identity_sha256,
        destinations,
        parcels,
        receipt_sha256: Digest32::zero(),
    }
    .seal()
    .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))
}

/// Explicit covered lower-boundary adopter for the V11 imported transaction.
///
/// This type is intentionally separate from [`DirectV11RealConsumerStack`].
/// It evaluates the Child-2C carrier and the actual persistent Stage-3
/// transition from the same beginning states and support before it constructs
/// the V11 canopy/soil owner candidate.
#[derive(Clone)]
pub struct DirectV11SnowCoveredRealConsumerStack<'a> {
    pub beginning: DirectV10RealConsumerShadow,
    pub interval: &'a DirectV11SnowCoveredSegmentInput,
    pub stage3_inputs_by_lane: &'a BTreeMap<u32, DirectActiveSnowPartitionInputs>,
    pub stage3_forcing_by_lane: &'a BTreeMap<u32, DirectSnowStage3SupportInput>,
    pub snow_surface_forcing_by_destination:
        &'a BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
    pub stage3_beginning_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub day_index: usize,
    pub interval_index: usize,
    pub finalize_wb14_parent_interval: bool,
    pub wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    terminal_endpoint_mode: bool,
    ending: Option<DirectV10RealConsumerShadow>,
    ending_stage3_by_lane: Option<BTreeMap<u32, DirectSnowStage3PersistentState>>,
    last_support_receipt: Option<LseSupportAdmissibilityReceiptV1>,
    last_final_boundary_receipts:
        Option<BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>>,
    last_lane_boundary_receipts: Option<BTreeMap<u32, LaneStage3BoundaryReceiptV1>>,
    last_component_carrier_receipts:
        Option<BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>>,
    last_snow_soil_heat_receipts: Option<BTreeMap<u32, SnowSoilHeatReceiptV1>>,
    last_terminal_snow_soil_heat_receipts:
        Option<BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1>>,
    last_adaptive_terminal_snow_soil_trial_receipts:
        Option<BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>>,
    last_precipitation_parcel_sets: Option<BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>>,
    last_physical_outcome_ledgers:
        Option<BTreeMap<u32, physical_outcome_ledger::Stage3LanePhysicalOutcomeLedgerV1>>,
    last_terminal_events: Option<BTreeMap<u32, DirectSnowTerminalEventResult>>,
    pending_terminal_parcels:
        BTreeMap<Digest32, crate::snow_stage3_v11_attachment::DirectSnowStage3V11TerminalParcel>,
    precomputed_terminal_accepted: Option<PrecomputedTerminalAcceptedEndpointV1>,
    last_wb14_child_receipt_set_sha256: Option<String>,
    last_wb14_parent_receipt_set_sha256: Option<String>,
    last_wb14_child_replay_bytes: Option<Vec<u8>>,
    last_wb14_parent_replay_bytes: Option<Vec<u8>>,
    ordinary_physical_reuse_seed: Option<CoveredOrdinaryPhysicalReuseSeedV1>,
    terminal_physical_reuse_seed: Option<CoveredTerminalPhysicalReuseSeedV1>,
    terminal_publication_posture: TerminalPublicationPostureV1,
    last_publication_retained: Option<bool>,
}

#[derive(Clone)]
struct CoveredOrdinaryPhysicalReuseSeedV1 {
    physical_authority: CoveredOrdinaryPhysicalAuthorityV1,
    envelope: UncommittedCoveredV8OwnerEnvelope,
}

#[derive(Clone)]
struct CoveredTerminalPhysicalReuseSeedV1 {
    physical_authority_sha256: Digest32,
    envelope: UncommittedCoveredV8OwnerEnvelope,
    compositional_envelopes: Vec<UncommittedCoveredV8OwnerEnvelope>,
    ending_snow_owner_bytes: Vec<u8>,
    soil_top_boundary_credits: Vec<SoilThermalTopBoundaryCreditV1>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum TerminalPublicationPostureV1 {
    #[default]
    RetainFinal,
    DeferProvisional {
        pre_event_authority_sha256: Digest32,
    },
}

/// Fully accepted positive-duration terminal endpoint.  Every field is
/// value evidence produced before this executor is entered; consuming it must
/// not rerun Stage 3, LSE, or probe/root physics.
#[derive(Clone)]
pub(crate) struct PrecomputedTerminalAcceptedEndpointV1 {
    pub carrier_phase: Box<CoveredCarrierPhaseResultV1>,
    /// Ordered exact physical carrier children whose supports compose the
    /// enclosing accepted slab. The final member is `carrier_phase`.
    pub carrier_phase_chain: Vec<CoveredCarrierPhaseResultV1>,
    pub ending_stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub terminal_events: BTreeMap<u32, DirectSnowTerminalEventResult>,
    /// Actual vapor transfer on the final physical child for each event lane.
    /// The enclosing event aggregate remains in `terminal_events`.
    pub final_child_actual_vapor_to_canopy_air_kg_m2_by_lane: BTreeMap<u32, f64>,
    pub terminal_snow_soil_trial_receipts:
        BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>,
    /// Ordered exact physical-child receipts whose supports compose the
    /// enclosing accepted slab. The final member is retained separately in
    /// `terminal_snow_soil_trial_receipts` as the limiting-boundary lineage.
    pub terminal_snow_soil_trial_receipt_chains_by_lane:
        BTreeMap<u32, Vec<physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>>,
    /// Parcels already pending at the positive-support beginning.  Newly
    /// produced event parcels belong to the later zero-duration transition.
    pub beginning_pending_terminal_parcels:
        BTreeMap<Digest32, crate::snow_stage3_v11_attachment::DirectSnowStage3V11TerminalParcel>,
    /// Coupled-time acceptance envelope. This is intentionally distinct from
    /// the final physical child's carrier/trial support.
    pub accepted_envelope_support: openwepp_coupled_time::TimeSupport,
    pub accepted_slab_sha256: Digest32,
    pub beginning_owner_set_sha256: Digest32,
    /// Exact unpublished WB14 trial authorization retained across the later
    /// accepted-slab publication rebind.
    pub wb14_replay_trial_sha256: Digest32,
    pub wb14_replay_beginning_owner_set_sha256: Digest32,
    pub wb14_child_receipt_set_sha256: Digest32,
    pub wb14_parent_receipt_set_sha256: Option<Digest32>,
    pub pre_event_authority_sha256: Digest32,
}

pub(crate) fn precomputed_terminal_pre_event_authority_sha256_v1(
    endpoint: &PrecomputedTerminalAcceptedEndpointV1,
) -> Result<Digest32, DirectV11RealConsumerError> {
    let physical_authority = precomputed_terminal_physical_authority_sha256_v1(endpoint)?;
    let parent_receipt = endpoint
        .wb14_parent_receipt_set_sha256
        .unwrap_or_else(|| digest_bytes(b"no-parent-receipt"));
    framed_sha256(
        "stage3-v11-terminal-pre-event-publication-authority-v2",
        &[
            FramedField {
                tag: "physical_authority",
                value: physical_authority.as_bytes(),
            },
            FramedField {
                tag: "accepted_slab",
                value: endpoint.accepted_slab_sha256.as_bytes(),
            },
            FramedField {
                tag: "wb14_child_receipt_set",
                value: endpoint.wb14_child_receipt_set_sha256.as_bytes(),
            },
            FramedField {
                tag: "wb14_parent_receipt_set",
                value: parent_receipt.as_bytes(),
            },
        ],
    )
    .map_err(|_| {
        DirectV11RealConsumerError::Identity("terminal pre-event publication authority framing")
    })
}

fn precomputed_terminal_physical_authority_sha256_v1(
    endpoint: &PrecomputedTerminalAcceptedEndpointV1,
) -> Result<Digest32, DirectV11RealConsumerError> {
    if endpoint.carrier_phase_chain.is_empty() {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal pre-event empty carrier chain",
        ));
    }
    let mut carrier_chain_evidence = Vec::new();
    carrier_chain_evidence.extend_from_slice(
        &u64::try_from(endpoint.carrier_phase_chain.len())
            .map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "terminal pre-event carrier-chain count width",
                )
            })?
            .to_be_bytes(),
    );
    for phase in &endpoint.carrier_phase_chain {
        let support = phase.transition.boundary.support;
        carrier_chain_evidence.extend_from_slice(&support.start_ns().get().to_be_bytes());
        carrier_chain_evidence.extend_from_slice(&support.end_ns().get().to_be_bytes());
        carrier_chain_evidence.extend_from_slice(
            phase.transition.probe_child_identity.receipt_sha256.as_bytes(),
        );
        carrier_chain_evidence.extend_from_slice(
            phase.ending_candidates.joint().receipt_sha256().as_bytes(),
        );
        for (lane_id, set) in &phase.precipitation_sets {
            carrier_chain_evidence.extend_from_slice(&lane_id.to_be_bytes());
            carrier_chain_evidence.extend_from_slice(set.receipt_sha256.as_bytes());
        }
    }
    let mut lane_evidence = Vec::new();
    for (lane_id, event) in &endpoint.terminal_events {
        let ending = endpoint.ending_stage3_by_lane.get(lane_id).ok_or(
            DirectV11RealConsumerError::Identity("terminal pre-event ending lane"),
        )?;
        let trial = endpoint
            .terminal_snow_soil_trial_receipts
            .get(lane_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "terminal pre-event trial lane",
            ))?;
        lane_evidence.extend_from_slice(&lane_id.to_be_bytes());
        lane_evidence.extend_from_slice(
            digest_bytes(&serde_json::to_vec(event).map_err(|_| {
                DirectV11RealConsumerError::Identity("terminal pre-event event bytes")
            })?)
            .as_bytes(),
        );
        lane_evidence.extend_from_slice(
            digest_bytes(&Wb11HydrologyKernel::serialize_stage3_persistent_state(
                ending,
            )?)
            .as_bytes(),
        );
        lane_evidence.extend_from_slice(trial.receipt_sha256.as_bytes());
        lane_evidence.extend_from_slice(
            &endpoint
                .final_child_actual_vapor_to_canopy_air_kg_m2_by_lane
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "terminal pre-event final-child vapor lane",
                ))?
                .to_bits()
                .to_be_bytes(),
        );
        let chain = endpoint
            .terminal_snow_soil_trial_receipt_chains_by_lane
            .get(lane_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "terminal pre-event trial chain lane",
            ))?;
        lane_evidence.extend_from_slice(&(chain.len() as u64).to_be_bytes());
        for receipt in chain {
            lane_evidence.extend_from_slice(receipt.receipt_sha256.as_bytes());
        }
    }
    let mut pending_evidence = Vec::new();
    for (digest, parcel) in &endpoint.beginning_pending_terminal_parcels {
        pending_evidence.extend_from_slice(digest.as_bytes());
        pending_evidence.extend_from_slice(parcel.parcel_digest.as_bytes());
    }
    let support = endpoint.carrier_phase.transition.boundary.support;
    let accepted_support = endpoint.accepted_envelope_support;
    framed_sha256(
        "stage3-v11-terminal-pre-event-physical-authority-v1",
        &[
            FramedField {
                tag: "support_start",
                value: &support.start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "support_end",
                value: &support.end_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "accepted_support_start",
                value: &accepted_support.start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "accepted_support_end",
                value: &accepted_support.end_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "beginning_owner_set",
                value: endpoint.beginning_owner_set_sha256.as_bytes(),
            },
            FramedField {
                tag: "carrier_ending_joint",
                value: endpoint
                    .carrier_phase
                    .ending_candidates
                    .joint()
                    .receipt_sha256()
                    .as_bytes(),
            },
            FramedField {
                tag: "carrier_child_receipt",
                value: endpoint
                    .carrier_phase
                    .transition
                    .probe_child_identity
                    .receipt_sha256
                    .as_bytes(),
            },
            FramedField {
                tag: "carrier_chain",
                value: &carrier_chain_evidence,
            },
            FramedField {
                tag: "wb14_replay_trial",
                value: endpoint.wb14_replay_trial_sha256.as_bytes(),
            },
            FramedField {
                tag: "wb14_replay_beginning_owner_set",
                value: endpoint.wb14_replay_beginning_owner_set_sha256.as_bytes(),
            },
            FramedField {
                tag: "lanes",
                value: &lane_evidence,
            },
            FramedField {
                tag: "pending_parcels",
                value: &pending_evidence,
            },
        ],
    )
    .map_err(|_| DirectV11RealConsumerError::Identity("terminal pre-event authority seal"))
}

pub struct DirectV11SnowCoveredStackInputs<'a> {
    pub interval: &'a DirectV11SnowCoveredSegmentInput,
    pub stage3_inputs_by_lane: &'a BTreeMap<u32, DirectActiveSnowPartitionInputs>,
    pub stage3_forcing_by_lane: &'a BTreeMap<u32, DirectSnowStage3SupportInput>,
    pub snow_surface_forcing_by_destination:
        &'a BTreeMap<(OfeId, TileId), SealedStage3TileBoundaryForcingV1>,
    pub stage3_beginning_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub pending_terminal_parcels:
        BTreeMap<Digest32, crate::snow_stage3_v11_attachment::DirectSnowStage3V11TerminalParcel>,
    pub day_index: usize,
    pub interval_index: usize,
    pub finalize_wb14_parent_interval: bool,
    pub wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
}

impl<'a> DirectV11SnowCoveredRealConsumerStack<'a> {
    #[must_use]
    pub fn new(
        beginning: &DirectV10RealConsumerShadow,
        inputs: DirectV11SnowCoveredStackInputs<'a>,
    ) -> Self {
        Self {
            beginning: beginning.clone(),
            interval: inputs.interval,
            stage3_inputs_by_lane: inputs.stage3_inputs_by_lane,
            stage3_forcing_by_lane: inputs.stage3_forcing_by_lane,
            snow_surface_forcing_by_destination: inputs.snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: inputs.stage3_beginning_by_lane,
            pending_terminal_parcels: inputs.pending_terminal_parcels,
            precomputed_terminal_accepted: None,
            day_index: inputs.day_index,
            interval_index: inputs.interval_index,
            finalize_wb14_parent_interval: inputs.finalize_wb14_parent_interval,
            wb14_coupled_child_binding: inputs.wb14_coupled_child_binding,
            terminal_endpoint_mode: false,
            ending: None,
            ending_stage3_by_lane: None,
            last_support_receipt: None,
            last_final_boundary_receipts: None,
            last_lane_boundary_receipts: None,
            last_component_carrier_receipts: None,
            last_snow_soil_heat_receipts: None,
            last_terminal_snow_soil_heat_receipts: None,
            last_adaptive_terminal_snow_soil_trial_receipts: None,
            last_precipitation_parcel_sets: None,
            last_physical_outcome_ledgers: None,
            last_terminal_events: None,
            last_wb14_child_receipt_set_sha256: None,
            last_wb14_parent_receipt_set_sha256: None,
            last_wb14_child_replay_bytes: None,
            last_wb14_parent_replay_bytes: None,
            ordinary_physical_reuse_seed: None,
            terminal_physical_reuse_seed: None,
            terminal_publication_posture: TerminalPublicationPostureV1::RetainFinal,
            last_publication_retained: None,
        }
    }

    pub(crate) fn prepare_ordinary_physical_reuse(
        mut self,
        final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let started = crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1()
            .then(std::time::Instant::now);
        let result = if self.terminal_endpoint_mode || self.ordinary_physical_reuse_seed.is_none() {
            Err(DirectV11RealConsumerError::Identity(
                "covered ordinary physical reuse seed",
            ))
        } else {
            self.wb14_coupled_child_binding = final_binding;
            Ok(self)
        };
        if let Some(started) = started {
            crate::snow_stage3_v11_attachment::record_adaptive_parent_reuse_validation_v1(
                started.elapsed(),
                result.is_ok(),
            );
        }
        result
    }

    pub(crate) fn prepare_terminal_physical_reuse(
        mut self,
        final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        endpoint: PrecomputedTerminalAcceptedEndpointV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let started = crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1()
            .then(std::time::Instant::now);
        let result = (|| {
            let seed = self.terminal_physical_reuse_seed.as_ref().ok_or(
                DirectV11RealConsumerError::Identity("covered terminal physical reuse seed"),
            )?;
            if !self.terminal_endpoint_mode
                || self.last_publication_retained != Some(false)
                || endpoint.pre_event_authority_sha256
                    != precomputed_terminal_pre_event_authority_sha256_v1(&endpoint)?
                || seed.physical_authority_sha256
                    != precomputed_terminal_physical_authority_sha256_v1(&endpoint)?
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered terminal physical reuse identity",
                ));
            }
            self.wb14_coupled_child_binding = final_binding;
            self.terminal_publication_posture = TerminalPublicationPostureV1::RetainFinal;
            self.precomputed_terminal_accepted = Some(endpoint);
            Ok(self)
        })();
        if let Some(started) = started {
            crate::snow_stage3_v11_attachment::record_adaptive_parent_reuse_validation_v1(
                started.elapsed(),
                result.is_ok(),
            );
        }
        result
    }

    #[must_use]
    pub(crate) fn with_precomputed_terminal_accepted_endpoint(
        mut self,
        endpoint: PrecomputedTerminalAcceptedEndpointV1,
    ) -> Self {
        self.terminal_endpoint_mode = true;
        self.precomputed_terminal_accepted = Some(endpoint);
        self
    }

    pub(crate) fn with_precomputed_terminal_provisional_endpoint(
        mut self,
        endpoint: PrecomputedTerminalAcceptedEndpointV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        if endpoint.pre_event_authority_sha256 == Digest32::zero() {
            return Err(DirectV11RealConsumerError::Identity(
                "terminal provisional publication authority",
            ));
        }
        self.terminal_endpoint_mode = true;
        self.terminal_publication_posture = TerminalPublicationPostureV1::DeferProvisional {
            pre_event_authority_sha256: endpoint.pre_event_authority_sha256,
        };
        self.precomputed_terminal_accepted = Some(endpoint);
        Ok(self)
    }

    fn precipitation_parcel_sets(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
    ) -> Result<BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>, DirectV11RealConsumerError>
    {
        let interval_s = f64::from_bits(self.interval.lse_forcing.interval_s.to_bits());
        let canopy_releases = envelope
            .fixed_cap_canopy_releases_by_destination(interval_s)
            .map_err(|_| DirectV11RealConsumerError::Identity("fixed-cap precipitation release"))?;
        self.precipitation_parcel_sets_with_releases(support, &canopy_releases)
    }

    fn precipitation_parcel_sets_from_physical(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        physical: &crate::land_surface_energy_shadow::ProvisionalCoveredV8PhysicalEvaluationV1,
    ) -> Result<BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>, DirectV11RealConsumerError>
    {
        let interval_s = f64::from_bits(self.interval.lse_forcing.interval_s.to_bits());
        let canopy_releases = physical
            .fixed_cap_canopy_releases_by_destination(interval_s)
            .map_err(|_| DirectV11RealConsumerError::Identity("fixed-cap precipitation release"))?;
        self.precipitation_parcel_sets_with_releases(support, &canopy_releases)
    }

    fn precipitation_parcel_sets_with_releases(
        &self,
        support: openwepp_coupled_time::TimeSupport,
        canopy_releases: &crate::land_surface_energy_shadow::FixedCapCanopyReleasesByDestination,
    ) -> Result<BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>, DirectV11RealConsumerError>
    {
        let beginning_vegetation_state_sha256 =
            digest32_from_lower_hex(self.beginning.vegetation_state.0.state_sha256.as_str())?;
        let mut sets = BTreeMap::new();
        for (lane_id, ofe_id) in self.covered_lane_to_ofe(&self.stage3_beginning_by_lane)? {
            let forcing = self.stage3_forcing_by_lane.get(&lane_id).copied().ok_or(
                DirectV11RealConsumerError::Identity("precipitation forcing lane"),
            )?;
            let beginning_snow = self.stage3_beginning_by_lane.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("precipitation beginning snow lane"),
            )?;
            let beginning_snow_state_sha256 = digest_bytes(
                &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning_snow).map_err(
                    |_| DirectV11RealConsumerError::Identity("precipitation beginning snow seal"),
                )?,
            );
            let forcing_identity_sha256 =
                digest32_from_lower_hex(stage3_support_forcing_digest(forcing)?.as_str())?;
            let has_precipitation =
                forcing.forcing.rain_m > 0.0 || forcing.forcing.snowfall_m > 0.0;
            let temperature_c = match forcing.forcing.hydrometeor_temperature_c {
                Some(value) => value,
                None if has_precipitation => {
                    return Err(DirectV11RealConsumerError::Identity(
                        "precipitation temperature provider",
                    ));
                }
                None => 0.0,
            };
            let temperature = TemperatureCelsius::try_new(temperature_c).map_err(|_| {
                DirectV11RealConsumerError::Identity("precipitation temperature provider")
            })?;
            let context = LanePrecipitationBuildContext {
                stack: self,
                canopy_releases,
                support,
                lane_id,
                ofe_id,
                forcing,
                beginning_snow_state_sha256,
                beginning_vegetation_state_sha256,
                forcing_identity_sha256,
                temperature_k: celsius_to_kelvin(temperature_c),
                solid_specific_heat_j_kg_k:
                    openwepp_meteorology::surface_energy::specific_heat_ice(temperature)
                        .map_err(|_| DirectV11RealConsumerError::Identity("snow specific heat"))?
                        .as_joules_per_kilogram_kelvin(),
            };
            let set = build_lane_precipitation_set(&context)?;
            let manifest = construct_precipitation_producer_manifest(&context, &set.destinations)?;
            validate_precipitation_producer_manifest(&set, &manifest)?;
            sets.insert(lane_id, set);
        }
        Ok(sets)
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn derive_live_carrier_input(
        &self,
        lane_id: u32,
        stage3_state: &DirectSnowStage3PersistentState,
        vegetation_state: &V8CoupledOwnedState,
        stage3_forcing: DirectSnowStage3SupportInput,
        sealed: &SealedCoveredCarrierForcing,
        tile_override: Option<&TileId>,
        _interval_s: f64,
    ) -> Result<CoveredCarrierInitialGuessV1, DirectV11RealConsumerError> {
        let lane_index = self
            .stage3_beginning_by_lane
            .keys()
            .position(|value| *value == lane_id)
            .ok_or(DirectV11RealConsumerError::Identity("covered lane order"))?;
        let tile = match tile_override {
            Some(tile_id) => self
                .beginning
                .vegetation_configuration
                .topology_tiles
                .iter()
                .find(|tile| tile.tile_id == *tile_id)
                .ok_or(DirectV11RealConsumerError::Identity("covered carrier tile"))?,
            None => self
                .beginning
                .vegetation_configuration
                .topology_tiles
                .get(lane_index % self.beginning.vegetation_configuration.topology_tiles.len())
                .ok_or(DirectV11RealConsumerError::Identity("covered carrier tile"))?,
        };
        let tile_air = vegetation_state.tile_canopy_air.get(&tile.tile_id).ok_or(
            DirectV11RealConsumerError::Identity("committed canopy-air owner"),
        )?;
        let occupancies = self
            .beginning
            .vegetation_configuration
            .strata
            .iter()
            .filter(|stratum| stratum.tile_ids.iter().any(|id| id == &tile.tile_id))
            .filter_map(|stratum| {
                let identity = openwepp_kernel_contract::OccupancyId {
                    stratum_id: stratum.stratum_id.clone(),
                    tile_id: tile.tile_id.clone(),
                };
                vegetation_state
                    .occupancies
                    .get(&identity)
                    .map(|state| (stratum, state))
            })
            .collect::<Vec<_>>();
        if occupancies.is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered canopy owner topology",
            ));
        }
        let count = f64::from(
            u32::try_from(occupancies.len())
                .map_err(|_| DirectV11RealConsumerError::Identity("covered occupancy count"))?,
        );
        let leaf_temperature_k = occupancies
            .iter()
            .map(|(_, state)| {
                state
                    .sun_leaf_temperature_k
                    .midpoint(state.shade_leaf_temperature_k)
            })
            .sum::<f64>()
            / count;
        let stem_temperature_k = occupancies
            .iter()
            .map(|(_, state)| state.dry_stem_temperature_k)
            .sum::<f64>()
            / count;
        let canopy_wind = sealed.exposure.wind_m_s;
        let (canopy_heat, canopy_vapor) = occupancies.iter().try_fold(
            (0.0, 0.0),
            |(heat, vapor), (stratum, _)| -> Result<(f64, f64), DirectV11RealConsumerError> {
                let persistent = vegetation_state.strata.get(&stratum.stratum_id).ok_or(
                    DirectV11RealConsumerError::Identity("covered persistent stratum"),
                )?;
                if persistent.leaf_area.to_bits() == 0.0_f64.to_bits()
                    && persistent.stem_area.to_bits() == 0.0_f64.to_bits()
                    && persistent.root_area.to_bits() == 0.0_f64.to_bits()
                {
                    // SC-VEGETATION-001@29 exact inactive-coordinate class:
                    // the canopy-air coordinate remains live, but an
                    // occupancy with no leaf/stem/root area contributes no
                    // component heat or vapor conductance to the carrier.
                    return Ok((heat, vapor));
                }
                let u_star = canopy_surface_friction_velocity(
                    canopy_wind,
                    self.interval.vegetation_forcing.reference_height_m,
                    stratum.displacement_m,
                    stratum.z0m_m,
                )
                .map_err(|_| DirectV11RealConsumerError::Identity("canopy wind exposure"))?;
                let leaf = leaf_boundary_conductance(u_star, stratum.leaf_dimension_m)
                    .map_err(|_| DirectV11RealConsumerError::Identity("leaf conductance"))?;
                let wet = leaf_boundary_conductance(u_star, stratum.wet_surface_dimension_m)
                    .map_err(|_| DirectV11RealConsumerError::Identity("wet conductance"))?;
                let stem = leaf_boundary_conductance(u_star, stratum.stem_dimension_m)
                    .map_err(|_| DirectV11RealConsumerError::Identity("stem conductance"))?;
                Ok((heat + (leaf + wet + stem) / 3.0, vapor + leaf.midpoint(wet)))
            },
        )?;
        let canopy_heat = canopy_heat / count;
        let canopy_vapor = canopy_vapor / count;
        let reference_resistance = neutral_resistance(
            sealed.exposure.transfer_height_m,
            0.0,
            sealed.exposure.roughness_m,
            sealed.exposure.roughness_m,
            sealed.exposure.wind_m_s,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("reference exposure"))?;
        let snow_resistance = reference_resistance;
        let snow_conductance = 1.0 / snow_resistance;
        let snow_temperature_k =
            if stage3_state.layers.is_empty() && stage3_forcing.forcing.snowfall_m > 0.0 {
                273.15
            } else {
                let surface = if crate::hydrology::stage3_is_terminal_event_domain(stage3_state) {
                    Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(stage3_state)
                } else {
                    Wb11HydrologyKernel::project_stage3_surface_state_v1(stage3_state)
                }
                .map_err(|_| DirectV11RealConsumerError::Identity("snow active-volume surface"))?;
                surface.surface_temperature_k
            };
        let snow_temperature = TemperatureCelsius::try_new(snow_temperature_k - 273.15)
            .map_err(|_| DirectV11RealConsumerError::Identity("snow temperature"))?;
        let saturation_pressure_pa =
            openwepp_meteorology::surface_energy::saturation_vapor_pressure_snobal_pa(
                snow_temperature,
            )
            .map_err(|_| DirectV11RealConsumerError::Identity("snow saturation pressure"))?
            .as_pascals();
        let air_pressure_pa = self.interval.lse_forcing.air_pressure_pa;
        if !air_pressure_pa.is_finite() || air_pressure_pa <= 0.378 * saturation_pressure_pa {
            return Err(DirectV11RealConsumerError::Identity(
                "snow surface humidity pressure",
            ));
        }
        let snow_humidity = (0.622 * saturation_pressure_pa
            / (air_pressure_pa - 0.378 * saturation_pressure_pa))
            .min(1.0);
        let reference_heat = 1.0 / reference_resistance;
        let reference = CarrierSurface {
            temperature_k: sealed.reference_temperature_k,
            specific_humidity: sealed.reference_specific_humidity,
            heat_conductance_m_s: reference_heat,
            vapor_conductance_m_s: reference_heat,
        };
        let canopy = CarrierSurface {
            temperature_k: tile_air.canopy_air_temperature_k,
            specific_humidity: tile_air.canopy_air_specific_humidity_kg_kg,
            heat_conductance_m_s: canopy_heat,
            vapor_conductance_m_s: canopy_vapor,
        };
        let snow = CarrierSurface {
            temperature_k: snow_temperature_k,
            specific_humidity: snow_humidity,
            heat_conductance_m_s: snow_conductance,
            vapor_conductance_m_s: snow_conductance,
        };
        let weight_sum = leaf_temperature_k + stem_temperature_k;
        let components = vec![
            CanopyLongwaveComponent {
                temperature_k: leaf_temperature_k,
                emissive_area_weight: leaf_temperature_k / weight_sum,
            },
            CanopyLongwaveComponent {
                temperature_k: stem_temperature_k,
                emissive_area_weight: stem_temperature_k / weight_sum,
            },
        ];
        let heat_total = reference.heat_conductance_m_s
            + canopy.heat_conductance_m_s
            + snow.heat_conductance_m_s;
        let shared_temperature = (reference.heat_conductance_m_s * reference.temperature_k
            + canopy.heat_conductance_m_s * canopy.temperature_k
            + snow.heat_conductance_m_s * snow.temperature_k)
            / heat_total;
        let shared_humidity = shared_carrier_specific_humidity_v1(&[reference, canopy, snow])?;
        let snow_sensible = -sealed.rho_air_kg_m3
            * sealed.cp_air_j_kg_k
            * snow.heat_conductance_m_s
            * (snow.temperature_k - shared_temperature);
        let snow_vapor = -sealed.rho_air_kg_m3
            * snow.vapor_conductance_m_s
            * (snow.specific_humidity - shared_humidity);
        #[cfg(test)]
        audit_covered_carrier_live_row(CoveredCarrierLiveConsumptionRowV1 {
            lane_id,
            forcing_sha256: sealed.diagnostic_seed_context_digest(),
            reference_specific_humidity_bits: reference.specific_humidity.to_bits(),
            snow_specific_humidity_bits: snow.specific_humidity.to_bits(),
            shared_specific_humidity_bits: shared_humidity.to_bits(),
            snow_vapor_into_surface_bits: (-snow_vapor).to_bits(),
        });
        let sky_view = (1.0 - sealed.effective_canopy_cover).powf(1.6);
        let canopy_longwave = components
            .iter()
            .map(|component| {
                component.emissive_area_weight * 5.670_374_419e-8 * component.temperature_k.powi(4)
            })
            .sum::<f64>();
        let snow_emission = 5.670_374_419e-8 * snow.temperature_k.powi(4);
        let snow_longwave_net_w_m2 = sky_view * sealed.atmospheric_longwave_w_m2
            + (1.0 - sky_view) * canopy_longwave
            - snow_emission;
        let scalar_bytes = [
            shared_temperature,
            shared_humidity,
            snow_temperature_k,
            snow_sensible,
            snow_vapor,
            snow_longwave_net_w_m2,
        ]
        .into_iter()
        .flat_map(|value| value.to_bits().to_be_bytes())
        .collect::<Vec<_>>();
        let diagnostic_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-carrier-initial-guess-v1",
            &[openwepp_coupled_time::FramedField {
                tag: "numerical_guess",
                value: &scalar_bytes,
            }],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier initial guess"))?;
        Ok(CoveredCarrierInitialGuessV1 {
            snow_temperature_k,
            snow_sensible_into_surface_w_m2: -snow_sensible,
            snow_vapor_into_surface_kg_m2_s: -snow_vapor,
            snow_longwave_net_w_m2,
            diagnostic_sha256,
        })
    }

    fn lane_stage3_terms_from_boundaries(
        &self,
        destination_receipts: &BTreeMap<(OfeId, TileId), Digest32>,
        boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        interval_s: f64,
    ) -> Result<BTreeMap<u32, LaneStage3BoundaryTerms>, DirectV11RealConsumerError> {
        let mut lanes = BTreeMap::<u32, LaneStage3BoundaryTerms>::new();
        for (destination, carrier) in destination_receipts {
            let boundary =
                boundaries
                    .get(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane boundary destination",
                    ))?;
            let binding = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane boundary OFE",
                ))?;
            let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
            let entry = lanes.entry(binding.production_lane_id).or_insert_with(|| {
                LaneStage3BoundaryTerms {
                    fractions: 0.0,
                    provisional_carrier_bytes: Vec::new(),
                    provisional_carrier_receipt_sha256: Digest32::zero(),
                    sensible_to_canopy_air_w_m2: 0.0,
                    vapor_to_canopy_air_kg_m2_s: 0.0,
                    latent_energy_to_canopy_air_j_m2: 0.0,
                    snow_absorbed_shortwave_w_m2: 0.0,
                    snow_net_longwave_w_m2: 0.0,
                    snow_temperature_k: 0.0,
                    latent_heat_j_kg: 0.0,
                    common_snow_temperature_k: None,
                    common_latent_heat_j_kg: None,
                }
            });
            if entry
                .common_snow_temperature_k
                .is_some_and(|value| value.to_bits() != boundary.snow_temperature_k.to_bits())
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane common snow temperature",
                ));
            }
            if entry
                .common_latent_heat_j_kg
                .is_some_and(|value| value.to_bits() != boundary.latent_heat_j_kg.to_bits())
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane common latent heat",
                ));
            }
            entry.common_snow_temperature_k = Some(boundary.snow_temperature_k);
            entry.common_latent_heat_j_kg = Some(boundary.latent_heat_j_kg);
            entry.fractions += fraction;
            entry
                .provisional_carrier_bytes
                .extend_from_slice(destination.0.as_str().as_bytes());
            entry.provisional_carrier_bytes.push(0);
            entry
                .provisional_carrier_bytes
                .extend_from_slice(destination.1.as_str().as_bytes());
            entry
                .provisional_carrier_bytes
                .extend_from_slice(&fraction.to_bits().to_le_bytes());
            entry
                .provisional_carrier_bytes
                .extend_from_slice(carrier.as_bytes());
            entry.sensible_to_canopy_air_w_m2 += fraction * boundary.sensible_to_canopy_air_w_m2;
            entry.vapor_to_canopy_air_kg_m2_s += fraction * boundary.vapor_to_canopy_air_kg_m2_s;
            entry.latent_energy_to_canopy_air_j_m2 += fraction
                * boundary.vapor_to_canopy_air_kg_m2_s
                * boundary.latent_heat_j_kg
                * interval_s;
            entry.snow_absorbed_shortwave_w_m2 += fraction * boundary.shortwave_absorbed_w_m2;
            entry.snow_net_longwave_w_m2 += fraction * boundary.net_longwave_w_m2;
            entry.snow_temperature_k += fraction * boundary.snow_temperature_k;
            entry.latent_heat_j_kg += fraction * boundary.latent_heat_j_kg;
        }
        for terms in lanes.values_mut() {
            if !terms.fractions.is_finite()
                || (terms.fractions - 1.0).abs() > STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane is missing a snow-surface contribution",
                ));
            }
            terms.provisional_carrier_receipt_sha256 =
                digest_bytes(&terms.provisional_carrier_bytes);
            terms.snow_temperature_k =
                terms
                    .common_snow_temperature_k
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane snow temperature",
                    ))?;
            terms.latent_heat_j_kg =
                terms
                    .common_latent_heat_j_kg
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane latent heat",
                    ))?;
            terms.latent_energy_to_canopy_air_j_m2 =
                (terms.vapor_to_canopy_air_kg_m2_s * interval_s) * terms.latent_heat_j_kg;
        }
        Ok(lanes)
    }

    fn final_lane_boundary_receipts(
        &self,
        receipt_support: openwepp_coupled_time::TimeSupport,
        final_receipts: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
        precipitation_sets: &BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    ) -> Result<BTreeMap<u32, LaneStage3BoundaryReceiptV1>, DirectV11RealConsumerError> {
        let topology_configuration_sha256 = self.covered_topology_digest();
        let mut grouped =
            BTreeMap::<u32, Vec<((OfeId, TileId), f64, &FinalStage3TileBoundaryReceiptV1)>>::new();
        for (destination, receipt) in final_receipts {
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final lane boundary OFE",
                ))?
                .production_lane_id;
            grouped.entry(lane_id).or_default().push((
                destination.clone(),
                self.covered_destination_fraction(&destination.0, &destination.1)?,
                receipt,
            ));
        }
        grouped
            .into_iter()
            .map(|(lane_id, mut values)| {
                values.sort_by(|left, right| left.0.cmp(&right.0));
                let ofe_id = values.first().map(|value| value.0.0.clone()).ok_or(
                    DirectV11RealConsumerError::Identity("empty covered final lane boundary"),
                )?;
                let fraction_sum = values.iter().map(|value| value.1).sum::<f64>();
                if !fraction_sum.is_finite()
                    || (fraction_sum - 1.0).abs() > STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered final lane boundary is missing a snow-surface contribution",
                    ));
                }
                let mut contributions = Vec::with_capacity(values.len());
                let mut expected_topology = Vec::with_capacity(values.len());
                let mut aggregate = [0.0; 7];
                for (destination, fraction, receipt) in values {
                    if !fraction.is_finite() || fraction <= 0.0 {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered final lane boundary fraction",
                        ));
                    }
                    receipt.validate()?;
                    let (boundary_class, model_definition) = match receipt {
                        FinalStage3TileBoundaryReceiptV1::V11Canopy(_) => (
                            Stage3TileBoundaryClassV1::V11CanopyCovered,
                            digest_bytes(b"OPENWEPP_FINAL_STAGE3_CANOPY_BOUNDARY_V1"),
                        ),
                        FinalStage3TileBoundaryReceiptV1::OpenSnow(_) => (
                            Stage3TileBoundaryClassV1::OpenSnow,
                            digest_bytes(b"OPENWEPP_FINAL_STAGE3_OPEN_SNOW_BOUNDARY_V1"),
                        ),
                    };
                    expected_topology.push(LaneBoundaryTopologyExpectationV1 {
                        tile_id: destination.1.clone(),
                        tile_fraction_bits: fraction.to_bits(),
                        boundary_class,
                        boundary_model_definition_sha256: model_definition,
                    });
                    let sources = receipt.source_digests();
                    let physical = receipt.physical_operands();
                    let contribution = LaneBoundaryContributionV1 {
                        tile_id: destination.1.clone(),
                        tile_fraction: fraction,
                        boundary_class,
                        boundary_model_definition_sha256: model_definition,
                        beginning_stage3_state_sha256: receipt.beginning_stage3_state_sha256(),
                        provisional_carrier_receipt_sha256: sources.0,
                        optical_receipt_sha256: sources.1,
                        reciprocal_longwave_receipt_sha256: sources.2,
                        final_boundary_receipt_sha256: sources.3,
                        sensible_to_canopy_air_w_m2: physical[0],
                        vapor_to_canopy_air_kg_m2_s: physical[1],
                        latent_energy_to_canopy_air_j_m2: physical[2],
                        snow_absorbed_shortwave_w_m2: physical[3],
                        snow_net_longwave_w_m2: physical[4],
                        snow_temperature_k: physical[5],
                        latent_heat_j_kg: physical[6],
                    };
                    for (index, value) in [
                        contribution.sensible_to_canopy_air_w_m2,
                        contribution.vapor_to_canopy_air_kg_m2_s,
                        contribution.latent_energy_to_canopy_air_j_m2,
                        contribution.snow_absorbed_shortwave_w_m2,
                        contribution.snow_net_longwave_w_m2,
                        contribution.snow_temperature_k,
                        contribution.latent_heat_j_kg,
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        aggregate[index] += fraction * value;
                    }
                    contributions.push(contribution);
                }
                let common_snow_temperature_k = contributions[0].snow_temperature_k;
                let common_latent_heat_j_kg = contributions[0].latent_heat_j_kg;
                aggregate[2] = (aggregate[1] * f64::from_bits(receipt_support.duration_s_bits()))
                    * common_latent_heat_j_kg;
                let lane_receipt = LaneStage3BoundaryReceiptV1::try_new(
                    LaneStage3BoundaryReceiptV1 {
                        lane_id,
                        ofe_id,
                        support: receipt_support,
                        area_basis: Stage3LaneAreaBasisV1::OfeGround,
                        topology_configuration_sha256,
                        provisional_carrier_receipt_sha256: Digest32::zero(),
                        optical_receipt_sha256: Digest32::zero(),
                        reciprocal_longwave_receipt_sha256: Digest32::zero(),
                        final_destination_receipt_sha256: Digest32::zero(),
                        precipitation_parcel_set_sha256: precipitation_sets
                            .get(&lane_id)
                            .ok_or(DirectV11RealConsumerError::Identity(
                                "lane precipitation parcel-set receipt",
                            ))?
                            .receipt_sha256,
                        ordered_destinations: contributions,
                        aggregate_sensible_to_canopy_air_w_m2: aggregate[0],
                        aggregate_vapor_to_canopy_air_kg_m2_s: aggregate[1],
                        aggregate_latent_energy_to_canopy_air_j_m2: aggregate[2],
                        aggregate_snow_absorbed_shortwave_w_m2: aggregate[3],
                        aggregate_snow_net_longwave_w_m2: aggregate[4],
                        aggregate_snow_temperature_k: common_snow_temperature_k,
                        aggregate_latent_heat_j_kg: common_latent_heat_j_kg,
                        terminal_bounded_vapor_receipt: None,
                        receipt_sha256: Digest32::zero(),
                    },
                    &expected_topology,
                )?;
                Ok((lane_id, lane_receipt))
            })
            .collect()
    }

    fn covered_topology_digest(&self) -> Digest32 {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"OPENWEPP_COVERED_TILE_TOPOLOGY_V1\0");
        for record in &self.beginning.inner.surface_configuration.records {
            bytes.extend_from_slice(record.key.ofe_id.as_str().as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(record.key.tile_id.as_str().as_bytes());
            bytes.extend_from_slice(&record.tile_fraction.to_bits().to_le_bytes());
        }
        digest_bytes(&bytes)
    }

    fn carrier_receipts_by_destination(
        &self,
        interval_s: f64,
        vegetation_state: &V8CoupledOwnedState,
        stage3_state_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        stage3_forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
    ) -> Result<BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>, DirectV11RealConsumerError>
    {
        let surface = &self.beginning.inner.surface_configuration;
        let lane_to_ofe = self.covered_lane_to_ofe(stage3_state_by_lane)?;
        let expected_destinations = self.covered_expected_destinations();
        let configured_destinations = surface
            .records
            .iter()
            .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
            .filter(|destination| expected_destinations.contains(destination))
            .collect::<BTreeSet<_>>();
        if expected_destinations != configured_destinations {
            return Err(DirectV11RealConsumerError::Identity(
                "covered surface/LSE destination set",
            ));
        }

        let mut receipts = BTreeMap::new();
        for (ofe_id, tile_id) in expected_destinations {
            let binding = surface
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered destination OFE binding",
                ))?;
            if lane_to_ofe.get(&binding.production_lane_id) != Some(&ofe_id) {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered destination lane/OFE binding",
                ));
            }
            let carrier = self.carrier_for_destination(
                interval_s,
                binding.production_lane_id,
                &ofe_id,
                &tile_id,
                vegetation_state,
                stage3_state_by_lane,
                stage3_forcing_by_lane,
            )?;
            if receipts.insert((ofe_id, tile_id), carrier).is_some() {
                return Err(DirectV11RealConsumerError::Identity(
                    "duplicate covered destination carrier receipt",
                ));
            }
        }
        Ok(receipts)
    }

    fn open_snow_boundaries_by_destination(
        &self,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Digest32>,
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), OpenSnowTileBoundaryCandidateV1>,
        ),
        DirectV11RealConsumerError,
    > {
        self.open_snow_boundaries_by_destination_with_beginning(
            stage3_states,
            &self.stage3_beginning_by_lane,
        )
    }

    fn open_snow_boundaries_by_destination_with_beginning(
        &self,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        beginning_stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Digest32>,
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), OpenSnowTileBoundaryCandidateV1>,
        ),
        DirectV11RealConsumerError,
    > {
        let mut diagnostics = BTreeMap::new();
        let mut boundaries = BTreeMap::new();
        let mut candidates = BTreeMap::new();
        for (destination, forcing) in self.snow_surface_forcing_by_destination {
            let SealedStage3TileBoundaryForcingV1::OpenSnow(forcing) = forcing else {
                continue;
            };
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "open-snow destination OFE binding",
                ))?
                .production_lane_id;
            let current =
                stage3_states
                    .get(&lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "open-snow current Stage-3 lane",
                    ))?;
            let beginning = beginning_stage3_states.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("open-snow beginning Stage-3 lane"),
            )?;
            let beginning_digest = if beginning.layers.is_empty() && forcing.snowfall_m > 0.0 {
                digest_bytes(
                    &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning).map_err(
                        |_| {
                            DirectV11RealConsumerError::Identity(
                                "open-snow reappearance beginning state",
                            )
                        },
                    )?,
                )
            } else if crate::hydrology::stage3_is_terminal_event_domain(beginning) {
                Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(beginning)
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "open-snow beginning active-volume surface",
                        )
                    })?
                    .beginning_stage3_state_sha256
            } else {
                Wb11HydrologyKernel::project_stage3_surface_state_v1(beginning)
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "open-snow beginning active-volume surface",
                        )
                    })?
                    .beginning_stage3_state_sha256
            };
            let stage3_inputs = self.stage3_inputs_by_lane.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("open-snow Stage-3 inputs"),
            )?;
            let candidate = evaluate_open_snow_tile_boundary(
                current,
                beginning_digest,
                stage3_inputs,
                forcing,
            )?;
            #[cfg(test)]
            COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT.with(|audit| {
                if let Some(audit) = audit.borrow_mut().as_mut() {
                    let saturation_pressure_pa =
                        openwepp_meteorology::surface_energy::saturation_vapor_pressure_snobal_pa(
                            TemperatureCelsius::try_new(candidate.snow_temperature_k - 273.15)
                                .expect("audited open-snow temperature"),
                        )
                        .expect("audited open-snow saturation pressure")
                        .as_pascals();
                    let snow_q = 0.622 * saturation_pressure_pa
                        / (forcing.air_pressure_pa - 0.378 * saturation_pressure_pa);
                    audit.open_snow_rows.push(OpenSnowLiveConsumptionRowV1 {
                        lane_id,
                        forcing_sha256: forcing.receipt_sha256,
                        reference_specific_humidity_bits: forcing
                            .reference_specific_humidity_kg_kg
                            .to_bits(),
                        snow_specific_humidity_bits: snow_q.to_bits(),
                        vapor_outward_bits: candidate.vapor_outward_kg_m2_s.to_bits(),
                    });
                }
            });
            let carrier_receipt_id = Sha256Digest::try_new(digest32_hex(
                candidate.exposure_receipt_sha256,
            ))
            .map_err(|_| DirectV11RealConsumerError::Identity("open-snow exposure receipt ID"))?;
            let albedo = stage3_inputs
                .snow_albedo_state
                .map_or(STAGE3_DEFAULT_SNOW_ALBEDO, |state| state.albedo);
            let boundary = Stage3SnowCoveredLowerBoundary {
                snow_temperature_k: candidate.snow_temperature_k,
                latent_heat_j_kg: candidate.latent_heat_j_kg,
                sensible_to_canopy_air_w_m2: candidate.sensible_outward_w_m2,
                vapor_to_canopy_air_kg_m2_s: candidate.vapor_outward_kg_m2_s,
                net_longwave_w_m2: candidate.snow_net_longwave_w_m2,
                shortwave_absorbed_w_m2: candidate.snow_absorbed_shortwave_w_m2,
                precipitation_advection_w_m2: 0.0,
                carrier_receipt_id,
                snow_vis_albedo: albedo,
                snow_nir_albedo: albedo,
                stage3_albedo_state_sha256: stage3_albedo_state_digest(stage3_inputs)?,
                forcing_receipt_sha256: Sha256Digest::try_new(digest32_hex(
                    candidate.forcing_receipt_sha256,
                ))
                .map_err(|_| DirectV11RealConsumerError::Identity("open-snow forcing receipt"))?,
                optical_receipt_sha256: Some(
                    Sha256Digest::try_new(digest32_hex(candidate.optical_receipt_sha256)).map_err(
                        |_| DirectV11RealConsumerError::Identity("open-snow optical receipt"),
                    )?,
                ),
                reciprocal_longwave_receipt_sha256: Some(
                    Sha256Digest::try_new(digest32_hex(candidate.longwave_receipt_sha256))
                        .map_err(|_| {
                            DirectV11RealConsumerError::Identity("open-snow longwave receipt")
                        })?,
                ),
                final_canopy_boundary_receipt_sha256: None,
            };
            boundary.validate().map_err(|_| {
                DirectV11RealConsumerError::OpenSnowLowerBoundaryDomain {
                    lane_id,
                    ofe_id: destination.0.as_str().to_owned(),
                    tile_id: destination.1.as_str().to_owned(),
                    start_ns: forcing.support.start_ns().get(),
                    end_ns: forcing.support.end_ns().get(),
                    snow_temperature_k: candidate.snow_temperature_k,
                    latent_heat_j_kg: candidate.latent_heat_j_kg,
                    sensible_outward_w_m2: candidate.sensible_outward_w_m2,
                    vapor_outward_kg_m2_s: candidate.vapor_outward_kg_m2_s,
                    net_longwave_w_m2: candidate.snow_net_longwave_w_m2,
                    shortwave_absorbed_w_m2: candidate.snow_absorbed_shortwave_w_m2,
                    albedo,
                    beginning_stage3: digest32_hex(candidate.beginning_stage3_state_sha256),
                    forcing: digest32_hex(candidate.forcing_receipt_sha256),
                    exposure: digest32_hex(candidate.exposure_receipt_sha256),
                    optical: digest32_hex(candidate.optical_receipt_sha256),
                    longwave: digest32_hex(candidate.longwave_receipt_sha256),
                }
            })?;
            diagnostics.insert(destination.clone(), candidate.exposure_receipt_sha256);
            boundaries.insert(destination.clone(), boundary);
            candidates.insert(destination.clone(), candidate);
        }
        Ok((diagnostics, boundaries, candidates))
    }

    fn seal_final_open_snow_boundaries(
        &self,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        ending_stage3_state_sha256: Digest32,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), FinalStage3OpenSnowBoundaryReceiptV1>,
        ),
        DirectV11RealConsumerError,
    > {
        let (_, mut boundaries, candidates) =
            self.open_snow_boundaries_by_destination(stage3_states)?;
        let receipts = candidates
            .into_iter()
            .map(|(destination, candidate)| {
                let receipt = FinalStage3OpenSnowBoundaryReceiptV1::try_new(
                    candidate,
                    ending_stage3_state_sha256,
                )?;
                let boundary = boundaries.get_mut(&destination).ok_or(
                    DirectV11RealConsumerError::Identity("final open-snow lower boundary"),
                )?;
                boundary.final_canopy_boundary_receipt_sha256 = Some(
                    Sha256Digest::try_new(digest32_hex(receipt.receipt_sha256)).map_err(|_| {
                        DirectV11RealConsumerError::Identity("final open-snow boundary receipt")
                    })?,
                );
                boundary.validate().map_err(|_| {
                    DirectV11RealConsumerError::Identity("sealed final open-snow lower boundary")
                })?;
                Ok((destination, receipt))
            })
            .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()?;
        Ok((boundaries, receipts))
    }

    fn complete_final_boundary_receipts(
        &self,
        covered: BTreeMap<(OfeId, TileId), FinalStage3CanopyBoundaryReceiptV1>,
        open: BTreeMap<(OfeId, TileId), FinalStage3OpenSnowBoundaryReceiptV1>,
    ) -> Result<
        BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
        DirectV11RealConsumerError,
    > {
        let mut complete = covered
            .into_iter()
            .map(|(destination, receipt)| {
                (
                    destination,
                    FinalStage3TileBoundaryReceiptV1::V11Canopy(receipt),
                )
            })
            .collect::<BTreeMap<_, _>>();
        for (destination, receipt) in open {
            if complete
                .insert(
                    destination,
                    FinalStage3TileBoundaryReceiptV1::OpenSnow(receipt),
                )
                .is_some()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered/open final boundary intersection",
                ));
            }
        }
        Ok(complete)
    }

    fn covered_destination_fraction(
        &self,
        ofe_id: &OfeId,
        tile_id: &TileId,
    ) -> Result<f64, DirectV11RealConsumerError> {
        let record = self
            .beginning
            .inner
            .surface_configuration
            .records
            .iter()
            .find(|record| record.key.ofe_id == *ofe_id && record.key.tile_id == *tile_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered destination tile fraction",
            ))?;
        if !record.tile_fraction.is_finite() || record.tile_fraction <= 0.0 {
            return Err(DirectV11RealConsumerError::Identity(
                "covered destination tile fraction domain",
            ));
        }
        Ok(record.tile_fraction)
    }

    fn corrected_covered_boundaries_from_envelope(
        &self,
        base: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<u32, f64>,
            BTreeMap<u32, f64>,
        ),
        DirectV11RealConsumerError,
    > {
        if base.is_empty() {
            return Ok((BTreeMap::new(), BTreeMap::new(), BTreeMap::new()));
        }
        let shortwave = envelope
            .covered_snow_shortwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered optical shortwave set"))?;
        let longwave = envelope
            .covered_snow_longwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered reciprocal longwave set"))?;
        self.corrected_covered_boundaries_from_fluxes(base, shortwave, longwave)
    }

    fn corrected_covered_boundaries_from_physical(
        &self,
        base: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        physical: &crate::land_surface_energy_shadow::ProvisionalCoveredV8PhysicalEvaluationV1,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<u32, f64>,
            BTreeMap<u32, f64>,
        ),
        DirectV11RealConsumerError,
    > {
        if base.is_empty() {
            return Ok((BTreeMap::new(), BTreeMap::new(), BTreeMap::new()));
        }
        let shortwave = physical
            .covered_snow_shortwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered optical shortwave set"))?;
        let longwave = physical
            .covered_snow_longwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered reciprocal longwave set"))?;
        self.corrected_covered_boundaries_from_fluxes(base, shortwave, longwave)
    }

    fn corrected_covered_boundaries_from_fluxes(
        &self,
        base: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        shortwave: BTreeMap<(OfeId, TileId), f64>,
        longwave: BTreeMap<(OfeId, TileId), f64>,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<u32, f64>,
            BTreeMap<u32, f64>,
        ),
        DirectV11RealConsumerError,
    > {
        if shortwave.keys().collect::<BTreeSet<_>>() != base.keys().collect::<BTreeSet<_>>()
            || longwave.keys().collect::<BTreeSet<_>>() != base.keys().collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered corrected boundary destination set",
            ));
        }
        let mut corrected = base.clone();
        let mut shortwave_by_lane = BTreeMap::<u32, (f64, f64)>::new();
        let mut longwave_by_lane = BTreeMap::<u32, (f64, f64)>::new();
        for (destination, value) in shortwave {
            let boundary =
                corrected
                    .get_mut(&destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered corrected shortwave destination",
                    ))?;
            boundary.shortwave_absorbed_w_m2 = value;
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered corrected shortwave OFE binding",
                ))?
                .production_lane_id;
            let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
            let entry = shortwave_by_lane.entry(lane_id).or_default();
            entry.0 += fraction * value;
            entry.1 += fraction;
        }
        for (destination, value) in longwave {
            let boundary =
                corrected
                    .get_mut(&destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered corrected longwave destination",
                    ))?;
            boundary.net_longwave_w_m2 = value;
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered corrected longwave OFE binding",
                ))?
                .production_lane_id;
            let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
            let entry = longwave_by_lane.entry(lane_id).or_default();
            entry.0 += fraction * value;
            entry.1 += fraction;
        }
        Ok((
            corrected,
            shortwave_by_lane
                .into_iter()
                .map(|(lane, (value, weight))| (lane, value / weight))
                .collect(),
            longwave_by_lane
                .into_iter()
                .map(|(lane, (value, weight))| (lane, value / weight))
                .collect(),
        ))
    }

    fn apply_lse_iteration_exchange(
        &self,
        boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        states: &BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    ) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError>
    {
        if boundaries.keys().collect::<BTreeSet<_>>() != states.keys().collect::<BTreeSet<_>>() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered LSE iteration exchange destination set",
            ));
        }
        let mut next = boundaries.clone();
        for (destination, state) in states {
            let boundary =
                next.get_mut(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered LSE iteration exchange destination",
                    ))?;
            boundary.sensible_to_canopy_air_w_m2 = state.snow_sensible_w_m2;
            boundary.vapor_to_canopy_air_kg_m2_s = state.snow_vapor_kg_m2_s;
        }
        Ok(next)
    }

    fn seal_final_covered_boundaries(
        &self,
        receipt_support: openwepp_coupled_time::TimeSupport,
        beginning_v11_state_sha256: Digest32,
        physical_beginning_stage3_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        destination_receipts: &BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
        ending_v8_physical_candidate_sha256: Digest32,
        ending_stage3_state_sha256: Digest32,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), FinalStage3CanopyBoundaryReceiptV1>,
        ),
        DirectV11RealConsumerError,
    > {
        if boundaries.is_empty() {
            if !destination_receipts.is_empty() {
                return Err(DirectV11RealConsumerError::Identity(
                    "open-only carrier receipt set",
                ));
            }
            return Ok((BTreeMap::new(), BTreeMap::new()));
        }
        let optical = envelope
            .covered_snow_optical_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered final optical receipts"))?;
        let longwave = envelope
            .covered_snow_longwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered final longwave receipts"))?;
        let mut final_boundaries = boundaries.clone();
        let mut final_receipts = BTreeMap::new();
        for (destination, boundary) in boundaries {
            let carrier = destination_receipts.get(destination).ok_or(
                DirectV11RealConsumerError::Identity("covered final carrier receipt join"),
            )?;
            let optical = optical
                .get(destination)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final optical receipt join",
                ))?;
            let final_longwave =
                longwave
                    .get(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered final longwave receipt join",
                    ))?;
            if optical.absorbed_w_m2_tile.total().to_bits()
                != boundary.shortwave_absorbed_w_m2.to_bits()
                || final_longwave.to_bits() != boundary.net_longwave_w_m2.to_bits()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered final boundary self-reconstruction",
                ));
            }
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final boundary lane join",
                ))?
                .production_lane_id;
            let beginning_stage3 = physical_beginning_stage3_by_lane.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("covered final beginning Stage-3 state"),
            )?;
            let reappearance = self
                .stage3_forcing_by_lane
                .get(&lane_id)
                .is_some_and(|forcing| forcing.forcing.snowfall_m > 0.0);
            let beginning_stage3_state_sha256 =
                if beginning_stage3.layers.is_empty() && reappearance {
                    digest_bytes(
                        &Wb11HydrologyKernel::serialize_stage3_persistent_state(beginning_stage3)
                            .map_err(|_| {
                            DirectV11RealConsumerError::Identity(
                                "covered final reappearance beginning state",
                            )
                        })?,
                    )
                } else if crate::hydrology::stage3_is_terminal_event_domain(beginning_stage3) {
                    Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(beginning_stage3)
                        .map_err(|_| {
                            DirectV11RealConsumerError::Identity(
                                "covered final beginning active-volume surface",
                            )
                        })?
                        .beginning_stage3_state_sha256
                } else {
                    Wb11HydrologyKernel::project_stage3_surface_state_v1(beginning_stage3)
                        .map_err(|_| {
                            DirectV11RealConsumerError::Identity(
                                "covered final beginning active-volume surface",
                            )
                        })?
                        .beginning_stage3_state_sha256
                };
            let optical_receipt_sha256 = digest32_from_lower_hex(optical.receipt_sha256.as_str())?;
            let reciprocal_longwave_receipt_sha256 =
                reciprocal_longwave_receipt_digest(destination, receipt_support, *final_longwave);
            let final_receipt = FinalStage3CanopyBoundaryReceiptV1::try_new(
                FinalStage3CanopyBoundaryReceiptInputs {
                    support: receipt_support,
                    destination: destination.clone(),
                    beginning_v11_state_sha256,
                    beginning_stage3_state_sha256,
                    ending_v8_physical_candidate_sha256,
                    ending_stage3_state_sha256,
                    provisional_carrier_receipt_sha256: carrier.diagnostic_sha256,
                    optical_receipt_sha256,
                    reciprocal_longwave_receipt_sha256,
                    sensible_to_canopy_air_w_m2: boundary.sensible_to_canopy_air_w_m2,
                    vapor_to_canopy_air_kg_m2_s: boundary.vapor_to_canopy_air_kg_m2_s,
                    latent_energy_to_canopy_air_j_m2: boundary.vapor_to_canopy_air_kg_m2_s
                        * boundary.latent_heat_j_kg
                        * f64::from_bits(receipt_support.duration_s_bits()),
                    snow_temperature_k: boundary.snow_temperature_k,
                    latent_heat_j_kg: boundary.latent_heat_j_kg,
                    snow_absorbed_shortwave_w_m2: optical.absorbed_w_m2_tile.total(),
                    snow_net_longwave_w_m2: *final_longwave,
                },
            )?;
            let final_boundary = final_boundaries.get_mut(destination).ok_or(
                DirectV11RealConsumerError::Identity("covered final boundary storage"),
            )?;
            final_boundary.optical_receipt_sha256 = Some(
                Sha256Digest::try_new(digest32_hex(optical_receipt_sha256)).map_err(|_| {
                    DirectV11RealConsumerError::Identity("covered optical receipt digest")
                })?,
            );
            final_boundary.reciprocal_longwave_receipt_sha256 = Some(
                Sha256Digest::try_new(digest32_hex(reciprocal_longwave_receipt_sha256)).map_err(
                    |_| {
                        DirectV11RealConsumerError::Identity(
                            "covered reciprocal longwave receipt digest",
                        )
                    },
                )?,
            );
            final_boundary.final_canopy_boundary_receipt_sha256 = Some(
                Sha256Digest::try_new(digest32_hex(final_receipt.receipt_sha256)).map_err(
                    |_| {
                        DirectV11RealConsumerError::Identity(
                            "covered final boundary receipt digest",
                        )
                    },
                )?,
            );
            final_receipts.insert(destination.clone(), final_receipt);
        }
        Ok((final_boundaries, final_receipts))
    }

    fn covered_lane_to_ofe(
        &self,
        stage3_beginning_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<BTreeMap<u32, OfeId>, DirectV11RealConsumerError> {
        let mut lane_to_ofe = BTreeMap::new();
        for binding in &self.beginning.inner.surface_configuration.ofe_bindings {
            if lane_to_ofe
                .insert(binding.production_lane_id, binding.ofe_id.clone())
                .is_some()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "duplicate covered lane/OFE binding",
                ));
            }
        }
        if stage3_beginning_by_lane
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != lane_to_ofe.keys().copied().collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier lane/OFE set",
            ));
        }
        Ok(lane_to_ofe)
    }

    fn covered_expected_destinations(&self) -> BTreeSet<(OfeId, TileId)> {
        self.snow_surface_forcing_by_destination
            .iter()
            .filter_map(|(destination, forcing)| {
                matches!(
                    forcing,
                    SealedStage3TileBoundaryForcingV1::V11CanopyCovered(_)
                )
                .then(|| destination.clone())
            })
            .collect()
    }

    fn carrier_for_destination(
        &self,
        interval_s: f64,
        lane_id: u32,
        ofe_id: &OfeId,
        tile_id: &TileId,
        vegetation_state: &V8CoupledOwnedState,
        stage3_state_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        stage3_forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
    ) -> Result<CoveredCarrierInitialGuessV1, DirectV11RealConsumerError> {
        let stage3_state =
            stage3_state_by_lane
                .get(&lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered destination Stage-3 state",
                ))?;
        let stage3_forcing = stage3_forcing_by_lane.get(&lane_id).copied().ok_or(
            DirectV11RealConsumerError::Identity("covered destination Stage-3 forcing"),
        )?;
        let sealed = self
            .snow_surface_forcing_by_destination
            .get(&(ofe_id.clone(), tile_id.clone()))
            .and_then(|forcing| match forcing {
                SealedStage3TileBoundaryForcingV1::V11CanopyCovered(forcing) => Some(forcing),
                SealedStage3TileBoundaryForcingV1::OpenSnow(_) => None,
            })
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered destination carrier forcing",
            ))?;
        let vegetation_tile_id = self
            .beginning
            .inner
            .lse_configuration
            .ofes
            .iter()
            .find(|ofe| ofe.ofe_id == *ofe_id)
            .and_then(|ofe| ofe.tiles.iter().find(|tile| tile.tile_id == *tile_id))
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered destination vegetation tile",
            ))?
            .vegetation_tile_id
            .clone();
        let mut guess = self.derive_live_carrier_input(
            lane_id,
            stage3_state,
            vegetation_state,
            stage3_forcing,
            sealed,
            Some(&vegetation_tile_id),
            interval_s,
        )?;
        let stage3_beginning_sha256 =
            digest_bytes(&canonical_stage3_snow_owner_bytes_v11(&BTreeMap::from([
                (lane_id, stage3_state.clone()),
            ]))?);
        let forcing_sha256 = stage3_support_forcing_digest(stage3_forcing)?;
        let numerical_seed_context_sha256 = sealed.diagnostic_seed_context_digest();
        let duration_bits = interval_s.to_bits().to_be_bytes();
        guess.diagnostic_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-carrier-initial-guess-diagnostic-v1",
            &[
                openwepp_coupled_time::FramedField {
                    tag: "ofe_id",
                    value: ofe_id.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "tile_id",
                    value: tile_id.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "duration_bits",
                    value: &duration_bits,
                },
                openwepp_coupled_time::FramedField {
                    tag: "diagnostic_numerical_seed_context",
                    value: numerical_seed_context_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "forcing_receipt",
                    value: forcing_sha256.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "beginning_v11_state",
                    value: self.beginning.vegetation_state.0.state_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "beginning_stage3_state",
                    value: stage3_beginning_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "guess_values",
                    value: guess.diagnostic_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered initial guess diagnostic"))?;
        Ok(guess)
    }

    fn stage3_lower_boundaries_by_destination(
        &self,
        receipts: &BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
        stage3_inputs_by_lane: &BTreeMap<u32, DirectActiveSnowPartitionInputs>,
        stage3_forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
    ) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError>
    {
        let expected_destinations = self.covered_expected_destinations();
        if receipts.keys().cloned().collect::<BTreeSet<_>>() != expected_destinations {
            return Err(DirectV11RealConsumerError::Identity(
                "covered destination carrier receipt set",
            ));
        }
        let mut boundaries = BTreeMap::new();
        for (destination, receipt) in receipts {
            let carrier_receipt_id = Sha256Digest::try_new(digest32_hex(receipt.diagnostic_sha256))
                .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier receipt ID"))?;
            let binding = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered boundary OFE binding",
                ))?;
            let stage3_input = stage3_inputs_by_lane
                .get(&binding.production_lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered boundary Stage-3 inputs",
                ))?;
            let stage3_forcing = stage3_forcing_by_lane
                .get(&binding.production_lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered boundary Stage-3 forcing",
                ))?;
            let snow_albedo = stage3_input
                .snow_albedo_state
                .map_or(STAGE3_DEFAULT_SNOW_ALBEDO, |state| state.albedo);
            let boundary = Stage3SnowCoveredLowerBoundary {
                snow_temperature_k: receipt.snow_temperature_k,
                latent_heat_j_kg:
                    openwepp_meteorology::surface_energy::latent_heat_for_surface_temperature(
                        TemperatureCelsius::try_new(receipt.snow_temperature_k - 273.15).map_err(
                            |_| DirectV11RealConsumerError::Identity("covered temperature"),
                        )?,
                    )
                    .map_err(|_| DirectV11RealConsumerError::Identity("covered latent heat"))?
                    .as_joules_per_kilogram(),
                sensible_to_canopy_air_w_m2: -receipt.snow_sensible_into_surface_w_m2,
                vapor_to_canopy_air_kg_m2_s: -receipt.snow_vapor_into_surface_kg_m2_s,
                net_longwave_w_m2: receipt.snow_longwave_net_w_m2,
                // The current released carrier receipt does not yet expose a
                // canonical shortwave or precipitation-advection term. Keep
                // those owners explicit and zero only at this default-off
                // seam; the physical covered cutover remains blocked on their
                // Stage-3 projections and ledger reconstruction.
                shortwave_absorbed_w_m2: 0.0,
                precipitation_advection_w_m2: 0.0,
                carrier_receipt_id,
                snow_vis_albedo: snow_albedo,
                snow_nir_albedo: snow_albedo,
                stage3_albedo_state_sha256: stage3_albedo_state_digest(stage3_input)?,
                forcing_receipt_sha256: stage3_support_forcing_digest(*stage3_forcing)?,
                optical_receipt_sha256: None,
                reciprocal_longwave_receipt_sha256: None,
                final_canopy_boundary_receipt_sha256: None,
            };
            boundary.validate().map_err(|_| {
                if covered_boundary_only_cold_temperature_is_refinable_v1(&boundary) {
                    DirectV11RealConsumerError::AdaptiveRefinement(
                        "covered Stage-3 lower-boundary temperature below constitutive domain",
                    )
                } else {
                    DirectV11RealConsumerError::Identity("covered boundary operands")
                }
            })?;
            if boundaries.insert(destination.clone(), boundary).is_some() {
                return Err(DirectV11RealConsumerError::Identity(
                    "duplicate covered destination lower boundary",
                ));
            }
        }
        Ok(boundaries)
    }

    /// Merge the latest persistent Stage-3 state operands into the boundary
    /// whose radiative and turbulent terms came from the preceding LSE solve.
    /// Stage-3 owns snow temperature (and therefore latent heat); LSE owns the
    /// exchanged fluxes.  Neither side may replace the other's operands.
    fn merge_latest_stage3_state_operands(
        &self,
        flux_boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError>
    {
        flux_boundaries
            .iter()
            .map(|(destination, boundary)| {
                let lane_id = self
                    .beginning
                    .inner
                    .surface_configuration
                    .ofe_bindings
                    .iter()
                    .find(|binding| binding.ofe_id == destination.0)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 state boundary OFE",
                    ))?
                    .production_lane_id;
                let state =
                    stage3_states
                        .get(&lane_id)
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "covered Stage-3 state boundary lane",
                        ))?;
                let projected = if crate::hydrology::stage3_is_terminal_event_domain(state) {
                    Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(state)
                } else {
                    Wb11HydrologyKernel::project_stage3_surface_state_v1(state)
                };
                let (snow_temperature_k, latent_heat_j_kg) = match projected {
                    Ok(surface) => (surface.surface_temperature_k, surface.latent_heat_j_kg),
                    // A dormant endpoint has no snow temperature.  The
                    // boundary entering this merge is the last physical
                    // pre-event boundary produced by the covered fixed
                    // point, so retain it as the limiting boundary rather
                    // than inventing a post-event melting-point node.
                    Err(_)
                        if (self.terminal_endpoint_mode
                            || self
                                .stage3_forcing_by_lane
                                .get(&lane_id)
                                .is_some_and(|forcing| forcing.forcing.snowfall_m > 0.0))
                            && state.layers.is_empty() =>
                    {
                        (boundary.snow_temperature_k, boundary.latent_heat_j_kg)
                    }
                    Err(_) => {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered Stage-3 state boundary active-volume surface",
                        ));
                    }
                };
                let mut merged = boundary.clone();
                merged.snow_temperature_k = snow_temperature_k;
                merged.latent_heat_j_kg = latent_heat_j_kg;
                merged.validate().map_err(|_| {
                    DirectV11RealConsumerError::Identity("covered merged Stage-3/LSE boundary")
                })?;
                Ok((destination.clone(), merged))
            })
            .collect()
    }

    pub fn take_staged_ending(&mut self) -> Option<DirectV10RealConsumerShadow> {
        if self.last_publication_retained == Some(false) {
            return None;
        }
        self.ending.take()
    }

    #[must_use]
    pub(crate) const fn last_publication_retained(&self) -> Option<bool> {
        self.last_publication_retained
    }

    pub fn take_staged_stage3(&mut self) -> Option<BTreeMap<u32, DirectSnowStage3PersistentState>> {
        self.ending_stage3_by_lane.take()
    }

    #[must_use]
    pub(crate) fn last_terminal_events(
        &self,
    ) -> Option<&BTreeMap<u32, DirectSnowTerminalEventResult>> {
        self.last_terminal_events.as_ref()
    }

    #[must_use]
    pub fn last_final_boundary_receipts(
        &self,
    ) -> Option<&BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>> {
        self.last_final_boundary_receipts.as_ref()
    }

    #[must_use]
    pub(crate) fn last_lane_boundary_receipts(
        &self,
    ) -> Option<&BTreeMap<u32, LaneStage3BoundaryReceiptV1>> {
        self.last_lane_boundary_receipts.as_ref()
    }

    #[must_use]
    pub(crate) fn last_component_carrier_receipts(
        &self,
    ) -> Option<&BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>> {
        self.last_component_carrier_receipts.as_ref()
    }

    pub(crate) fn last_snow_soil_heat_receipts(
        &self,
    ) -> Option<&BTreeMap<u32, SnowSoilHeatReceiptV1>> {
        self.last_snow_soil_heat_receipts.as_ref()
    }

    pub(crate) fn last_terminal_snow_soil_heat_receipts(
        &self,
    ) -> Option<&BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1>> {
        self.last_terminal_snow_soil_heat_receipts.as_ref()
    }

    pub(crate) fn last_adaptive_terminal_snow_soil_trial_receipts(
        &self,
    ) -> Option<&BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>> {
        self.last_adaptive_terminal_snow_soil_trial_receipts
            .as_ref()
    }

    #[allow(dead_code)]
    pub(crate) fn last_precipitation_parcel_sets(
        &self,
    ) -> Option<&BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>> {
        self.last_precipitation_parcel_sets.as_ref()
    }

    pub(crate) fn last_physical_outcome_ledgers(
        &self,
    ) -> Option<&BTreeMap<u32, physical_outcome_ledger::Stage3LanePhysicalOutcomeLedgerV1>> {
        self.last_physical_outcome_ledgers.as_ref()
    }

    pub(crate) fn last_wb14_receipt_sets(&self) -> Option<(&str, Option<&str>)> {
        self.last_wb14_child_receipt_set_sha256
            .as_deref()
            .map(|child| (child, self.last_wb14_parent_receipt_set_sha256.as_deref()))
    }

    pub(crate) fn last_wb14_replay_bytes(&self) -> Option<(&[u8], Option<&[u8]>)> {
        self.last_wb14_child_replay_bytes
            .as_deref()
            .map(|child| (child, self.last_wb14_parent_replay_bytes.as_deref()))
    }
}

fn covered_boundary_only_cold_temperature_is_refinable_v1(
    boundary: &Stage3SnowCoveredLowerBoundary,
) -> bool {
    (0.0..200.0).contains(&boundary.snow_temperature_k)
        && [
            boundary.latent_heat_j_kg,
            boundary.sensible_to_canopy_air_w_m2,
            boundary.vapor_to_canopy_air_kg_m2_s,
            boundary.net_longwave_w_m2,
            boundary.shortwave_absorbed_w_m2,
            boundary.precipitation_advection_w_m2,
        ]
        .iter()
        .all(|value| value.is_finite())
        && boundary.latent_heat_j_kg > 0.0
        && !boundary.carrier_receipt_id.as_str().is_empty()
        && !boundary.stage3_albedo_state_sha256.as_str().is_empty()
        && !boundary.forcing_receipt_sha256.as_str().is_empty()
        && boundary.snow_vis_albedo.is_finite()
        && boundary.snow_nir_albedo.is_finite()
        && (0.0..=1.0).contains(&boundary.snow_vis_albedo)
        && (0.0..=1.0).contains(&boundary.snow_nir_albedo)
}
