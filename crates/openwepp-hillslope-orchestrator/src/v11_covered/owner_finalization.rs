//! Final owner construction and V11 resource-lineage joins.

use super::*;

fn validate_terminal_custody_lane_sets(
    persistent: &BTreeSet<u32>,
    terminal: &BTreeSet<u32>,
    events: &BTreeSet<u32>,
    ledgers: &BTreeSet<u32>,
) -> Result<(), DirectV11RealConsumerError> {
    if !events.is_subset(terminal)
        || !terminal.is_disjoint(persistent)
        || !terminal.iter().all(|lane_id| ledgers.contains(lane_id))
    {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal snow-soil/event/ledger required-lane set",
        ));
    }
    Ok(())
}

const fn requires_persistent_snow_soil_receipt(
    beginning_resolved: bool,
    ending_resolved: bool,
    ending_terminal: bool,
) -> bool {
    beginning_resolved && (ending_resolved || ending_terminal)
}

const fn requires_adaptive_terminal_snow_soil_receipt(
    beginning_terminal: bool,
    ending_terminal: bool,
) -> bool {
    beginning_terminal && ending_terminal
}

#[cfg(test)]
mod terminal_custody_lane_set_tests {
    use super::*;
    use crate::DirectSnowLayerState;

    #[test]
    fn terminal_receipts_require_exact_event_set_and_ledgers() {
        let persistent = BTreeSet::from([1]);
        let terminal = BTreeSet::from([2]);
        let events = BTreeSet::from([2]);
        let ledgers = BTreeSet::from([1, 2]);
        validate_terminal_custody_lane_sets(&persistent, &terminal, &events, &ledgers)
            .expect("exact terminal custody set");
    }

    #[test]
    fn terminal_to_terminal_uses_only_adaptive_trial_custody() {
        assert!(!requires_persistent_snow_soil_receipt(false, false, true));
        assert!(requires_adaptive_terminal_snow_soil_receipt(true, true));
        for poisoned in [(false, true), (true, false), (false, false)] {
            assert!(!requires_adaptive_terminal_snow_soil_receipt(
                poisoned.0, poisoned.1
            ));
        }
    }

    #[test]
    fn adaptive_terminal_domain_receipt_without_event_is_admitted() {
        validate_terminal_custody_lane_sets(
            &BTreeSet::from([1]),
            &BTreeSet::from([2, 3]),
            &BTreeSet::from([2]),
            &BTreeSet::from([1, 2, 3]),
        )
        .expect("preterminal adaptive receipt remains physical custody");
    }

    fn canonical_adaptive_snow_state() -> DirectSnowStage3PersistentState {
        Wb11HydrologyKernel::initialize_stage3_persistent_state(
            1,
            vec![DirectSnowLayerState::new(0.08, 0.8, 100.0, 12.0)],
        )
        .expect("canonical adaptive snow state")
    }

    #[test]
    fn adaptive_snow_derived_cache_requires_candidate_local_identities() {
        let baseline = BTreeMap::from([(1, canonical_adaptive_snow_state())]);
        canonical_stage3_snow_owner_bytes_v11(&baseline)
            .expect("canonical snow derived identities");

        let inconsistent_density = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            1,
            vec![DirectSnowLayerState::new(0.08, 0.8, 101.0, 12.0)],
        )
        .expect("independently sealed density poison");
        assert!(matches!(
            canonical_stage3_snow_owner_bytes_v11(&BTreeMap::from([(1, inconsistent_density)])),
            Err(DirectV11RealConsumerError::Identity(
                "adaptive snow mass-depth-density derived identity"
            ))
        ));

        let ordering_probe_mass_swe_m: f64 = 0.054_992_284_801_565_73;
        let canonical_thickness_m = ordering_probe_mass_swe_m * 1_000.0 / 100.0;
        let alternate_thickness_m = ordering_probe_mass_swe_m / 100.0 * 1_000.0;
        assert_ne!(
            canonical_thickness_m.to_bits(),
            alternate_thickness_m.to_bits(),
            "ordering probe must distinguish the canonical SWE/density/rho-water projection"
        );
        let canonical_ordering = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            1,
            vec![DirectSnowLayerState::new(
                ordering_probe_mass_swe_m,
                canonical_thickness_m,
                100.0,
                12.0,
            )],
        )
        .expect("canonical density projection ordering");
        canonical_stage3_snow_owner_bytes_v11(&BTreeMap::from([(1, canonical_ordering)]))
            .expect("canonical density projection ordering is admitted");
        let alternate_ordering = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            1,
            vec![DirectSnowLayerState::new(
                ordering_probe_mass_swe_m,
                alternate_thickness_m,
                100.0,
                12.0,
            )],
        )
        .expect("independently sealed alternate-order poison");
        assert!(matches!(
            canonical_stage3_snow_owner_bytes_v11(&BTreeMap::from([(1, alternate_ordering)])),
            Err(DirectV11RealConsumerError::Identity(
                "adaptive snow mass-depth-density derived identity"
            ))
        ));

        let mut stale_settle_bits = canonical_adaptive_snow_state();
        stale_settle_bits.layers[0].settle_day_count =
            f64::from_bits(stale_settle_bits.layers[0].settle_day_count.to_bits() + 1);
        assert!(matches!(
            canonical_stage3_snow_owner_bytes_v11(&BTreeMap::from([(1, stale_settle_bits)])),
            Err(DirectV11RealConsumerError::Identity(
                "adaptive snow derived-cache candidate validation"
            ))
        ));

        let overflowing_weighted_settle = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            1,
            vec![DirectSnowLayerState::new(2.0, 20.0, 100.0, f64::MAX)],
        )
        .expect("independently sealed weighted-settle poison");
        assert!(matches!(
            canonical_stage3_snow_owner_bytes_v11(&BTreeMap::from([(
                1,
                overflowing_weighted_settle
            )])),
            Err(DirectV11RealConsumerError::Identity(
                "adaptive snow weighted-settle derived identity"
            ))
        ));
    }

    #[test]
    fn terminal_lane_without_physical_ledger_is_rejected() {
        assert!(
            validate_terminal_custody_lane_sets(
                &BTreeSet::from([1]),
                &BTreeSet::from([2]),
                &BTreeSet::from([2]),
                &BTreeSet::from([1]),
            )
            .is_err()
        );
    }

    #[test]
    fn solid_reappearance_has_no_persistent_snow_soil_exchange_receipt() {
        assert!(!requires_persistent_snow_soil_receipt(false, true, false));
        assert!(requires_persistent_snow_soil_receipt(true, true, false));
        assert!(requires_persistent_snow_soil_receipt(true, false, true));
    }

    #[test]
    fn v2_finalization_has_one_accepted_receiver_and_one_install() {
        let source = include_str!("owner_finalization.rs");
        let receiver = source
            .rsplit("fn accepted_v2_soil_candidate_for_v11_segment(")
            .next()
            .expect("V2 accepted receiver")
            .split("pub(crate) fn finalize_v11_imported_segment")
            .next()
            .expect("V2 accepted receiver body");
        assert_eq!(
            receiver
                .matches("aggregate_soil_thermal_ending_v2(")
                .count(),
            1
        );
        assert_eq!(
            receiver
                .matches("seal_soil_thermal_accepted_candidate_v2(")
                .count(),
            1
        );
        assert_eq!(
            receiver
                .matches("prepare_next_soil_thermal_support_v2(")
                .count(),
            1
        );
        assert!(!receiver.contains("prepare_soil_thermal_support_v2("));
        let finalization = source
            .rsplit("pub(crate) fn finalize_v11_imported_segment")
            .next()
            .expect("V11 finalization")
            .split("pub(crate) fn digest32_hex")
            .next()
            .expect("V11 finalization body");
        assert_eq!(
            finalization
                .matches("install_soil_thermal_accepted_v2(")
                .count(),
            1
        );
        assert_eq!(
            finalization
                .matches("install_soil_thermal_accepted_v2_from_beginning(")
                .count(),
            1
        );
        assert!(finalization.contains(
            ".install_soil_thermal_accepted_v2_from_beginning(\n                    beginning,"
        ));
        assert!(
            !finalization
                .contains("candidate.inner.soil_thermal = beginning.inner.soil_thermal.clone()")
        );
        assert!(!finalization.contains("if !v2_soil"));
    }

    #[test]
    fn native_v2_accepted_vegetation_lineage_matches_support_transaction() {
        for reused in [false, true] {
            validate_accepted_vegetation_candidate_transaction_lineage_v1(
                true, reused, 40, 41, 41, 41, 41,
            )
            .expect("V2 accepted candidate is the exact support transaction");
        }
    }

    #[test]
    fn native_v2_accepted_vegetation_lineage_rejects_transaction_poisons() {
        for (envelope, outer, inner) in [
            (41, 40, 40), // stale predecessor candidate
            (40, 41, 41), // replayed predecessor support
            (42, 42, 42), // candidate from a crossing support
            (41, 42, 42), // substituted successor candidate
            (41, 41, 40), // split outer/inner lineage
        ] {
            assert!(matches!(
                validate_accepted_vegetation_candidate_transaction_lineage_v1(
                    true, true, 40, 41, envelope, outer, inner,
                ),
                Err(DirectV11RealConsumerError::Identity(
                    "accepted vegetation candidate transaction lineage"
                ))
            ));
        }
    }

    #[test]
    fn v1_accepted_vegetation_lineage_postures_remain_unchanged() {
        validate_accepted_vegetation_candidate_transaction_lineage_v1(
            false, true, 40, 41, 41, 40, 40,
        )
        .expect("V1 reused candidate retains predecessor lineage");
        validate_accepted_vegetation_candidate_transaction_lineage_v1(
            false, false, 40, 41, 41, 41, 41,
        )
        .expect("V1 fresh candidate retains accepted lineage");
    }

    #[test]
    fn native_v2_preinstall_soil_posture_requires_one_exact_owner() {
        validate_native_v2_preinstall_soil_posture_v1(true, false)
            .expect("authenticated beginning resident");
        validate_native_v2_preinstall_soil_posture_v1(false, true)
            .expect("selected unpublished ending resident");
        for poison in [(false, false), (true, true)] {
            assert!(matches!(
                validate_native_v2_preinstall_soil_posture_v1(poison.0, poison.1),
                Err(DirectV11RealConsumerError::Identity(
                    "V2 accepted soil preinstall owner posture"
                ))
            ));
        }
    }
}

pub(crate) fn soil_thermal_owner_with_top_boundary_credit_join_sha256(
    ending_soil_owner_sha256: Digest32,
    accepted_credit_set_sha256: &openwepp_land_surface_energy::Sha256Digest,
) -> Result<Digest32, DirectV11RealConsumerError> {
    if ending_soil_owner_sha256 == Digest32::zero() {
        return Err(DirectV11RealConsumerError::Identity(
            "soil top-boundary owner join",
        ));
    }
    openwepp_coupled_time::framed_sha256(
        "covered-soil-top-boundary-owner-join-v1",
        &[
            openwepp_coupled_time::FramedField {
                tag: "ending_soil_owner",
                value: ending_soil_owner_sha256.as_bytes(),
            },
            openwepp_coupled_time::FramedField {
                tag: "accepted_credit_set",
                value: accepted_credit_set_sha256.as_str().as_bytes(),
            },
        ],
    )
    .map_err(|_| DirectV11RealConsumerError::Identity("soil top-boundary owner join digest"))
}

#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct CoveredParentOwnerJoinReceiptV1 {
    pub run_identity: Digest32,
    pub parent_interval_sha256: Digest32,
    pub parent_transaction_sha256: Digest32,
    pub segment_sha256: Digest32,
    pub accepted_slab_sha256: Digest32,
    pub forcing_receipt_sha256: Digest32,
    pub beginning_complete_owner_set_sha256: Digest32,
    pub ending_complete_owner_set_sha256: Digest32,
    pub support: openwepp_coupled_time::TimeSupport,
    pub final_boundary_receipt_set_sha256: Digest32,
    pub final_lane_boundary_receipt_set_sha256: Digest32,
    pub component_carrier_receipt_set_sha256: Digest32,
    pub snow_soil_heat_receipt_set_sha256: Digest32,
    pub terminal_snow_soil_heat_receipt_set_sha256: Digest32,
    pub physical_outcome_ledger_set_sha256: Digest32,
    pub wb14_child_receipt_set_sha256: Digest32,
    pub wb14_parent_receipt_set_sha256: Option<Digest32>,
    pub stage3_physical_state_sha256: Digest32,
    pub vegetation_owner_sha256: Digest32,
    pub snow_owner_sha256: Digest32,
    pub land_surface_energy_owner_sha256: Digest32,
    pub hydrology_owner_sha256: Digest32,
    pub biogeochemistry_owner_sha256: Digest32,
    pub soil_thermal_owner_sha256: Digest32,
    pub surface_liquid_owner_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

pub(crate) struct CoveredPhysicalCustodyJoinInputs<'a> {
    pub snow_soil_heat_receipts: &'a BTreeMap<u32, SnowSoilHeatReceiptV1>,
    pub adaptive_terminal_snow_soil_heat_receipts: &'a BTreeMap<u32, SnowSoilHeatReceiptV1>,
    pub adaptive_terminal_snow_soil_trial_receipts:
        &'a BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>,
    pub terminal_snow_soil_heat_receipts:
        &'a BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1>,
    pub terminal_events: &'a BTreeMap<u32, DirectSnowTerminalEventResult>,
    pub physical_outcome_ledgers:
        &'a BTreeMap<u32, physical_outcome_ledger::Stage3LanePhysicalOutcomeLedgerV1>,
    pub beginning_stage3_states: &'a BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub ending_stage3_states: &'a BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub pending_terminal_parcels: &'a BTreeMap<
        Digest32,
        crate::snow_stage3_v11_attachment::DirectSnowStage3V11TerminalParcel,
    >,
}

impl CoveredParentOwnerJoinReceiptV1 {
    pub(crate) fn validate_retained_boundary_sets(
        &self,
        final_boundaries: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
        final_lane_boundaries: &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    ) -> Result<(), DirectV11RealConsumerError> {
        for (destination, receipt) in final_boundaries {
            receipt.validate()?;
            if destination != receipt.destination() {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered boundary map key",
                ));
            }
        }
        for (lane_id, receipt) in final_lane_boundaries {
            receipt.validate()?;
            if *lane_id != receipt.lane_id {
                return Err(DirectV11RealConsumerError::Identity("covered lane map key"));
            }
        }
        let boundary_fields = final_boundaries
            .values()
            .map(|receipt| openwepp_coupled_time::FramedField {
                tag: "final_boundary_receipt",
                value: receipt.receipt_sha256().as_bytes(),
            })
            .collect::<Vec<_>>();
        let lane_fields = final_lane_boundaries
            .values()
            .map(|receipt| openwepp_coupled_time::FramedField {
                tag: "final_lane_boundary_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            })
            .collect::<Vec<_>>();
        if openwepp_coupled_time::framed_sha256(
            "covered-stage3-final-boundary-set-v1",
            &boundary_fields,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered boundary receipt set"))?
            != self.final_boundary_receipt_set_sha256
            || openwepp_coupled_time::framed_sha256(
                "covered-stage3-final-lane-boundary-set-v1",
                &lane_fields,
            )
            .map_err(|_| DirectV11RealConsumerError::Identity("covered lane receipt set"))?
                != self.final_lane_boundary_receipt_set_sha256
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered retained boundary receipt set",
            ));
        }
        Ok(())
    }

    pub fn validate_seal(&self) -> Result<(), DirectV11RealConsumerError> {
        if self.reconstructed_digest()? != self.receipt_sha256 {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner join seal",
            ));
        }
        Ok(())
    }

    pub(crate) fn try_new(
        run_identity: Digest32,
        parent_interval_sha256: Digest32,
        parent_transaction_sha256: Digest32,
        segment_sha256: Digest32,
        accepted_slab_sha256: Digest32,
        forcing_receipt_sha256: Digest32,
        beginning_complete_owner_set_sha256: Digest32,
        wb14_child_receipt_set_sha256: Digest32,
        wb14_parent_receipt_set_sha256: Option<Digest32>,
        support: openwepp_coupled_time::TimeSupport,
        final_boundaries: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
        final_lane_boundaries: &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
        component_carriers: &BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>,
        physical_custody: &CoveredPhysicalCustodyJoinInputs<'_>,
        owners: &BTreeMap<String, V11OwnerEnvelope>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        if [
            run_identity,
            parent_interval_sha256,
            parent_transaction_sha256,
            segment_sha256,
            accepted_slab_sha256,
            forcing_receipt_sha256,
            beginning_complete_owner_set_sha256,
            wb14_child_receipt_set_sha256,
        ]
        .contains(&Digest32::zero())
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner transaction lineage",
            ));
        }
        let expected = [
            "vegetation",
            "snow",
            "land_surface_energy",
            "hydrology",
            "bgc",
            "soil_thermal",
            "surface_liquid",
        ];
        if owners.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != expected.into_iter().collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner owner topology",
            ));
        }
        if final_boundaries.is_empty() || final_lane_boundaries.is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner boundary topology",
            ));
        }
        if physical_custody.physical_outcome_ledgers.is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner physical-ledger topology",
            ));
        }
        if component_carriers.keys().collect::<BTreeSet<_>>()
            != final_boundaries
                .iter()
                .filter_map(|(destination, receipt)| {
                    matches!(receipt, FinalStage3TileBoundaryReceiptV1::V11Canopy(_))
                        .then_some(destination)
                })
                .collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner component topology",
            ));
        }
        for owner in owners.values() {
            owner.to_owner_state().map_err(|_| {
                DirectV11RealConsumerError::Identity("covered parent-owner envelope")
            })?;
        }
        let ending_owner_states = owners
            .values()
            .map(V11OwnerEnvelope::to_owner_state)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered ending owner set"))?;
        let ending_complete_owner_set_sha256 =
            openwepp_coupled_time::complete_owner_set_digest(&ending_owner_states).map_err(
                |_| DirectV11RealConsumerError::Identity("covered ending complete-owner digest"),
            )?;
        for (destination, receipt) in final_boundaries {
            receipt.validate()?;
            if destination != receipt.destination() {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered final boundary map key",
                ));
            }
        }
        if !final_lane_boundaries
            .keys()
            .all(|lane_id| physical_custody.ending_stage3_states.contains_key(lane_id))
        {
            return Err(DirectV11RealConsumerError::Identity(
                "active lane receipt outside Stage-3 state set",
            ));
        }
        let mut joined_destinations = BTreeSet::new();
        for (lane_id, receipt) in final_lane_boundaries {
            receipt.validate()?;
            if *lane_id != receipt.lane_id {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered lane receipt map key",
                ));
            }
            for contribution in &receipt.ordered_destinations {
                let destination = (receipt.ofe_id.clone(), contribution.tile_id.clone());
                if !joined_destinations.insert(destination.clone()) {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered lane duplicate destination join",
                    ));
                }
                let boundary = final_boundaries.get(&destination).ok_or(
                    DirectV11RealConsumerError::Identity("covered lane/final boundary destination"),
                )?;
                let sources = boundary.source_digests();
                let physical = boundary.physical_operands();
                let expected_class = match boundary {
                    FinalStage3TileBoundaryReceiptV1::V11Canopy(_) => {
                        Stage3TileBoundaryClassV1::V11CanopyCovered
                    }
                    FinalStage3TileBoundaryReceiptV1::OpenSnow(_) => {
                        Stage3TileBoundaryClassV1::OpenSnow
                    }
                };
                if boundary.destination() != &destination
                    || contribution.boundary_class != expected_class
                    || contribution.final_boundary_receipt_sha256 != sources.3
                    || contribution.provisional_carrier_receipt_sha256 != sources.0
                    || contribution.optical_receipt_sha256 != sources.1
                    || contribution.reciprocal_longwave_receipt_sha256 != sources.2
                    || contribution.beginning_stage3_state_sha256
                        != boundary.beginning_stage3_state_sha256()
                    || contribution.sensible_to_canopy_air_w_m2.to_bits() != physical[0].to_bits()
                    || contribution.vapor_to_canopy_air_kg_m2_s.to_bits() != physical[1].to_bits()
                    || contribution.latent_energy_to_canopy_air_j_m2.to_bits()
                        != physical[2].to_bits()
                    || contribution.snow_absorbed_shortwave_w_m2.to_bits() != physical[3].to_bits()
                    || contribution.snow_net_longwave_w_m2.to_bits() != physical[4].to_bits()
                    || contribution.snow_temperature_k.to_bits() != physical[5].to_bits()
                    || contribution.latent_heat_j_kg.to_bits() != physical[6].to_bits()
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered lane/final boundary semantic join",
                    ));
                }
            }
        }
        if joined_destinations != final_boundaries.keys().cloned().collect::<BTreeSet<_>>() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered lane complete destination join",
            ));
        }
        for (destination, receipt) in component_carriers {
            let boundary =
                final_boundaries
                    .get(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "component/final boundary destination",
                    ))?;
            let FinalStage3TileBoundaryReceiptV1::V11Canopy(boundary) = boundary else {
                return Err(DirectV11RealConsumerError::Identity(
                    "component carrier attached to open-snow boundary",
                ));
            };
            receipt.validate(boundary)?;
        }
        let expected_snow_bytes = canonical_stage3_snow_owner_bytes_v11_with_pending_and_receipts(
            physical_custody.ending_stage3_states,
            physical_custody.pending_terminal_parcels,
            final_lane_boundaries,
            final_boundaries,
        )?;
        let snow_owner = owners
            .get("snow")
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered parent-owner snow envelope",
            ))?;
        validate_exact_snow_owner_bytes(&expected_snow_bytes, snow_owner)?;
        let boundary_fields = final_boundaries
            .values()
            .map(|receipt| openwepp_coupled_time::FramedField {
                tag: "final_boundary_receipt",
                value: receipt.receipt_sha256().as_bytes(),
            })
            .collect::<Vec<_>>();
        let final_boundary_receipt_set_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-stage3-final-boundary-set-v1",
            &boundary_fields,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered boundary receipt set"))?;
        let component_fields = component_carriers
            .values()
            .map(|receipt| openwepp_coupled_time::FramedField {
                tag: "component_carrier_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            })
            .collect::<Vec<_>>();
        let component_carrier_receipt_set_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-component-carrier-set-v1",
            &component_fields,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("component carrier receipt set"))?;
        if physical_custody
            .beginning_stage3_states
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != physical_custody
                .ending_stage3_states
                .keys()
                .copied()
                .collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "snow-soil beginning/ending lane set",
            ));
        }
        let required_snow_soil_lanes = physical_custody
            .beginning_stage3_states
            .iter()
            .filter_map(|(lane_id, beginning)| {
                let ending = physical_custody.ending_stage3_states.get(lane_id)?;
                let beginning_resolved =
                    crate::hydrology::stage3_is_resolved_thermal_domain(beginning);
                let ending_resolved = crate::hydrology::stage3_is_resolved_thermal_domain(ending);
                let ending_terminal = crate::hydrology::stage3_is_terminal_event_domain(ending);
                // Persistent snow--soil custody spans every ordinary covered
                // step whose two endpoints both have a projectable bottom
                // thermal node.  A terminal->terminal adaptive step instead
                // carries its exact trial receipt, and terminal->dormant uses
                // the terminal receipt below.  Terminal->resolved is solid
                // reappearance: SC-SNOWENERGY-001@22 admits no snow--soil
                // exchange across that boundary, and the physical ledger
                // binds the canonical no-exchange receipt instead.
                requires_persistent_snow_soil_receipt(
                    beginning_resolved,
                    ending_resolved,
                    ending_terminal,
                )
                .then_some(*lane_id)
            })
            .collect::<BTreeSet<_>>();
        if physical_custody
            .snow_soil_heat_receipts
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != required_snow_soil_lanes
        {
            return Err(DirectV11RealConsumerError::Identity(
                "snow-soil receipt active-lane set",
            ));
        }
        let required_adaptive_terminal_lanes = physical_custody
            .beginning_stage3_states
            .iter()
            .filter_map(|(lane_id, beginning)| {
                let ending = physical_custody.ending_stage3_states.get(lane_id)?;
                requires_adaptive_terminal_snow_soil_receipt(
                    crate::hydrology::stage3_is_terminal_event_domain(beginning),
                    crate::hydrology::stage3_is_terminal_event_domain(ending),
                )
                .then_some(*lane_id)
            })
            .collect::<BTreeSet<_>>();
        let adaptive_terminal_heat_lanes = physical_custody
            .adaptive_terminal_snow_soil_heat_receipts
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        let adaptive_terminal_trial_lanes = physical_custody
            .adaptive_terminal_snow_soil_trial_receipts
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        let emitted_adaptive_terminal_lanes = adaptive_terminal_heat_lanes
            .union(&adaptive_terminal_trial_lanes)
            .copied()
            .collect::<BTreeSet<_>>();
        if emitted_adaptive_terminal_lanes != required_adaptive_terminal_lanes
            || !adaptive_terminal_heat_lanes.is_disjoint(&adaptive_terminal_trial_lanes)
            || !required_adaptive_terminal_lanes.is_disjoint(&required_snow_soil_lanes)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "adaptive terminal snow-soil receipt active-lane set",
            ));
        }
        let required_terminal_lanes = physical_custody
            .terminal_events
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        if physical_custody
            .terminal_events
            .values()
            .any(|event| !event.event_occurred || event.unevaluated_seconds.abs() > 1.0e-6)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "terminal event exact endpoint custody",
            ));
        }
        let terminal_receipt_lanes = physical_custody
            .terminal_snow_soil_heat_receipts
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        let ledger_lanes = physical_custody
            .physical_outcome_ledgers
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        validate_terminal_custody_lane_sets(
            &required_snow_soil_lanes,
            &terminal_receipt_lanes,
            &required_terminal_lanes,
            &ledger_lanes,
        )?;
        let mut snow_soil_fields =
            Vec::with_capacity(physical_custody.snow_soil_heat_receipts.len());
        for (lane_id, receipt) in physical_custody.snow_soil_heat_receipts {
            crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(receipt).map_err(
                |error| DirectV11RealConsumerError::from_stage3_physical_custody(&error),
            )?;
            if *lane_id != receipt.lane_id {
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-soil parent receipt lane key",
                ));
            }
            snow_soil_fields.push(openwepp_coupled_time::FramedField {
                tag: "snow_soil_heat_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            });
        }
        let snow_soil_heat_receipt_set_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-snow-soil-heat-receipt-set-v1",
            &snow_soil_fields,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("snow-soil receipt set"))?;
        let mut terminal_snow_soil_fields = Vec::with_capacity(
            physical_custody.terminal_snow_soil_heat_receipts.len()
                + physical_custody
                    .adaptive_terminal_snow_soil_heat_receipts
                    .len()
                + physical_custody
                    .adaptive_terminal_snow_soil_trial_receipts
                    .len(),
        );
        for (lane_id, receipt) in physical_custody.adaptive_terminal_snow_soil_heat_receipts {
            crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(receipt).map_err(
                |error| DirectV11RealConsumerError::from_stage3_physical_custody(&error),
            )?;
            if *lane_id != receipt.lane_id {
                return Err(DirectV11RealConsumerError::Identity(
                    "adaptive terminal snow-soil parent receipt lane key",
                ));
            }
            terminal_snow_soil_fields.push(openwepp_coupled_time::FramedField {
                tag: "adaptive_terminal_snow_soil_heat_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            });
        }
        for (lane_id, receipt) in physical_custody.adaptive_terminal_snow_soil_trial_receipts {
            receipt.validate().map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "adaptive terminal snow-soil trial receipt seal",
                )
            })?;
            if *lane_id != receipt.lane_id {
                return Err(DirectV11RealConsumerError::Identity(
                    "adaptive terminal snow-soil parent receipt lane key",
                ));
            }
            terminal_snow_soil_fields.push(openwepp_coupled_time::FramedField {
                tag: "adaptive_terminal_snow_soil_trial_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            });
        }
        for (lane_id, receipt) in physical_custody.terminal_snow_soil_heat_receipts {
            receipt
                .validate()
                .map_err(|_| DirectV11RealConsumerError::Identity("terminal snow-soil receipt"))?;
            if *lane_id != receipt.lane_id {
                return Err(DirectV11RealConsumerError::Identity(
                    "terminal snow-soil parent receipt lane key",
                ));
            }
            terminal_snow_soil_fields.push(openwepp_coupled_time::FramedField {
                tag: "terminal_snow_soil_heat_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            });
        }
        let terminal_snow_soil_heat_receipt_set_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-terminal-snow-soil-heat-receipt-set-v1",
            &terminal_snow_soil_fields,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("terminal snow-soil receipt set"))?;
        let physical_outcome_ledger_set_sha256 =
            physical_outcome_ledger::ledger_set_digest(physical_custody.physical_outcome_ledgers);
        if physical_outcome_ledger_set_sha256 == Digest32::zero() {
            return Err(DirectV11RealConsumerError::Identity(
                "physical outcome ledger set",
            ));
        }
        let lane_fields = final_lane_boundaries
            .values()
            .map(|receipt| openwepp_coupled_time::FramedField {
                tag: "final_lane_boundary_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            })
            .collect::<Vec<_>>();
        let final_lane_boundary_receipt_set_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-stage3-final-lane-boundary-set-v1",
            &lane_fields,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered lane receipt set"))?;
        let stage3_physical_state_sha256 = digest_bytes(&canonical_stage3_snow_owner_bytes_v11(
            physical_custody.ending_stage3_states,
        )?);
        let owner_digest = |name: &'static str| {
            owners.get(name).map(|owner| owner.state_sha256).ok_or(
                DirectV11RealConsumerError::Identity("covered parent-owner join owner"),
            )
        };
        let mut value = Self {
            run_identity,
            parent_interval_sha256,
            parent_transaction_sha256,
            segment_sha256,
            accepted_slab_sha256,
            forcing_receipt_sha256,
            beginning_complete_owner_set_sha256,
            ending_complete_owner_set_sha256,
            support,
            final_boundary_receipt_set_sha256,
            final_lane_boundary_receipt_set_sha256,
            component_carrier_receipt_set_sha256,
            snow_soil_heat_receipt_set_sha256,
            terminal_snow_soil_heat_receipt_set_sha256,
            physical_outcome_ledger_set_sha256,
            wb14_child_receipt_set_sha256,
            wb14_parent_receipt_set_sha256,
            stage3_physical_state_sha256,
            vegetation_owner_sha256: owner_digest("vegetation")?,
            snow_owner_sha256: owner_digest("snow")?,
            land_surface_energy_owner_sha256: owner_digest("land_surface_energy")?,
            hydrology_owner_sha256: owner_digest("hydrology")?,
            biogeochemistry_owner_sha256: owner_digest("bgc")?,
            soil_thermal_owner_sha256: owner_digest("soil_thermal")?,
            surface_liquid_owner_sha256: owner_digest("surface_liquid")?,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        Ok(value)
    }

    pub(crate) fn validate(
        &self,
        final_boundaries: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
        final_lane_boundaries: &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
        component_carriers: &BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>,
        physical_custody: &CoveredPhysicalCustodyJoinInputs<'_>,
        owners: &BTreeMap<String, V11OwnerEnvelope>,
    ) -> Result<(), DirectV11RealConsumerError> {
        let expected = Self::try_new(
            self.run_identity,
            self.parent_interval_sha256,
            self.parent_transaction_sha256,
            self.segment_sha256,
            self.accepted_slab_sha256,
            self.forcing_receipt_sha256,
            self.beginning_complete_owner_set_sha256,
            self.wb14_child_receipt_set_sha256,
            self.wb14_parent_receipt_set_sha256,
            self.support,
            final_boundaries,
            final_lane_boundaries,
            component_carriers,
            physical_custody,
            owners,
        )?;
        if &expected != self {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner join replay",
            ));
        }
        self.validate_seal()?;
        Ok(())
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectV11RealConsumerError> {
        let start = self.support.start_ns().get().to_be_bytes();
        let end = self.support.end_ns().get().to_be_bytes();
        let wb14_parent = self
            .wb14_parent_receipt_set_sha256
            .unwrap_or(Digest32::zero());
        openwepp_coupled_time::framed_sha256(
            "covered-parent-owner-join-v1",
            &[
                digest_field("run_identity", self.run_identity.as_bytes()),
                digest_field("parent_interval", self.parent_interval_sha256.as_bytes()),
                digest_field(
                    "parent_transaction",
                    self.parent_transaction_sha256.as_bytes(),
                ),
                digest_field("segment", self.segment_sha256.as_bytes()),
                digest_field("accepted_slab", self.accepted_slab_sha256.as_bytes()),
                digest_field("forcing_receipt", self.forcing_receipt_sha256.as_bytes()),
                digest_field(
                    "beginning_complete_owner_set",
                    self.beginning_complete_owner_set_sha256.as_bytes(),
                ),
                digest_field(
                    "ending_complete_owner_set",
                    self.ending_complete_owner_set_sha256.as_bytes(),
                ),
                digest_field("support_start_ns", &start),
                digest_field("support_end_ns", &end),
                digest_field(
                    "final_boundary_set",
                    self.final_boundary_receipt_set_sha256.as_bytes(),
                ),
                digest_field(
                    "final_lane_boundary_set",
                    self.final_lane_boundary_receipt_set_sha256.as_bytes(),
                ),
                digest_field(
                    "component_carrier_set",
                    self.component_carrier_receipt_set_sha256.as_bytes(),
                ),
                digest_field(
                    "snow_soil_heat_receipt_set",
                    self.snow_soil_heat_receipt_set_sha256.as_bytes(),
                ),
                digest_field(
                    "terminal_snow_soil_heat_receipt_set",
                    self.terminal_snow_soil_heat_receipt_set_sha256.as_bytes(),
                ),
                digest_field(
                    "physical_outcome_ledger_set",
                    self.physical_outcome_ledger_set_sha256.as_bytes(),
                ),
                digest_field(
                    "wb14_child_receipt_set",
                    self.wb14_child_receipt_set_sha256.as_bytes(),
                ),
                digest_field("wb14_parent_receipt_set", wb14_parent.as_bytes()),
                digest_field(
                    "stage3_physical_state",
                    self.stage3_physical_state_sha256.as_bytes(),
                ),
                digest_field("vegetation_owner", self.vegetation_owner_sha256.as_bytes()),
                digest_field("snow_owner", self.snow_owner_sha256.as_bytes()),
                digest_field(
                    "land_surface_energy_owner",
                    self.land_surface_energy_owner_sha256.as_bytes(),
                ),
                digest_field("hydrology_owner", self.hydrology_owner_sha256.as_bytes()),
                digest_field(
                    "biogeochemistry_owner",
                    self.biogeochemistry_owner_sha256.as_bytes(),
                ),
                digest_field(
                    "soil_thermal_owner",
                    self.soil_thermal_owner_sha256.as_bytes(),
                ),
                digest_field(
                    "surface_liquid_owner",
                    self.surface_liquid_owner_sha256.as_bytes(),
                ),
            ],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered parent-owner join digest"))
    }
}

fn digest_field<'a>(tag: &'static str, value: &'a [u8]) -> openwepp_coupled_time::FramedField<'a> {
    openwepp_coupled_time::FramedField { tag, value }
}

fn validate_exact_snow_owner_bytes(
    expected: &[u8],
    snow_owner: &V11OwnerEnvelope,
) -> Result<(), DirectV11RealConsumerError> {
    snow_owner
        .to_owner_state()
        .map_err(|_| DirectV11RealConsumerError::Identity("covered snow owner envelope"))?;
    if snow_owner.owner_id != "snow" || snow_owner.state_bytes != expected {
        return Err(DirectV11RealConsumerError::Identity(
            "covered parent-owner Stage-3/snow semantic join",
        ));
    }
    Ok(())
}

fn validate_adaptive_snow_derived_integrity_v1(
    states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<(), DirectV11RealConsumerError> {
    for (lane_id, state) in states {
        if state.lane_id != *lane_id {
            return Err(DirectV11RealConsumerError::Identity(
                "adaptive snow derived-cache lane identity",
            ));
        }
        Wb11HydrologyKernel::validate_stage3_persistent_state(state).map_err(|_| {
            DirectV11RealConsumerError::Identity("adaptive snow derived-cache candidate validation")
        })?;
        let mut represented_mass_swe_m = 0.0;
        let mut weighted_settle_day_mass = 0.0;
        for layer in &state.layers {
            // SC-SNOWENERGY-001@22 INV-043: the canonical layer geometry
            // projection applies the water-density conversion before dividing
            // by stored density. The operation ordering is part of the exact
            // derived-cache identity and matches every Stage-3 layer producer.
            let reconstructed_thickness_m = layer.mass_swe_m * 1_000.0 / layer.density_kg_m3;
            if reconstructed_thickness_m.to_bits() != layer.thickness_m.to_bits() {
                return Err(DirectV11RealConsumerError::Identity(
                    "adaptive snow mass-depth-density derived identity",
                ));
            }
            represented_mass_swe_m += layer.mass_swe_m;
            weighted_settle_day_mass += layer.settle_day_count * layer.mass_swe_m;
        }
        if represented_mass_swe_m > 0.0 {
            let weighted_settle_day_count = weighted_settle_day_mass / represented_mass_swe_m;
            if !weighted_settle_day_count.is_finite() || weighted_settle_day_count < 0.0 {
                return Err(DirectV11RealConsumerError::Identity(
                    "adaptive snow weighted-settle derived identity",
                ));
            }
        }
    }
    Ok(())
}

pub(crate) fn canonical_stage3_snow_owner_bytes_v11(
    states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<Vec<u8>, DirectV11RealConsumerError> {
    validate_adaptive_snow_derived_integrity_v1(states)?;
    #[derive(Serialize)]
    struct CanonicalSnowOwner<'a> {
        schema: &'static str,
        lanes: Vec<(&'a u32, &'a DirectSnowStage3PersistentState)>,
    }
    serde_json::to_vec(&CanonicalSnowOwner {
        schema: "OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V1",
        lanes: states.iter().collect(),
    })
    .map_err(|_| DirectV11RealConsumerError::Identity("canonical Stage-3 snow bytes"))
}

pub(crate) fn canonical_stage3_snow_owner_bytes_v11_with_pending_and_receipts(
    states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<
        Digest32,
        crate::snow_stage3_v11_attachment::DirectSnowStage3V11TerminalParcel,
    >,
    lane_receipts: &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    receipts: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
) -> Result<Vec<u8>, DirectV11RealConsumerError> {
    validate_adaptive_snow_derived_integrity_v1(states)?;
    if pending_terminal_parcels.is_empty() {
        #[derive(Serialize)]
        struct CanonicalSnowOwner<'a> {
            schema: &'static str,
            lanes: Vec<(&'a u32, &'a DirectSnowStage3PersistentState)>,
            final_lane_boundary_receipts: BTreeMap<u32, String>,
            final_boundary_receipts: BTreeMap<String, String>,
        }
        return serde_json::to_vec(&CanonicalSnowOwner {
            schema: "OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V3",
            lanes: states.iter().collect(),
            final_lane_boundary_receipts: lane_receipts
                .iter()
                .map(|(lane_id, receipt)| (*lane_id, digest32_hex(receipt.receipt_sha256)))
                .collect(),
            final_boundary_receipts: receipts
                .iter()
                .map(|(destination, receipt)| {
                    (
                        format!("{}\0{}", destination.0.as_str(), destination.1.as_str()),
                        digest32_hex(receipt.source_digests().3),
                    )
                })
                .collect(),
        })
        .map_err(|_| DirectV11RealConsumerError::Identity("canonical Stage-3 snow bytes"));
    }
    let final_boundary_receipts = receipts
        .iter()
        .map(|(destination, receipt)| {
            (
                (
                    destination.0.as_str().to_owned(),
                    destination.1.as_str().to_owned(),
                ),
                receipt.source_digests().3,
            )
        })
        .collect();
    let final_lane_boundary_receipts = lane_receipts
        .iter()
        .map(|(lane_id, receipt)| (*lane_id, receipt.receipt_sha256))
        .collect();
    crate::snow_owner_v4::canonical_stage3_snow_owner_v4_bytes(
        states,
        pending_terminal_parcels,
        &final_lane_boundary_receipts,
        &final_boundary_receipts,
    )
    .map_err(|_| DirectV11RealConsumerError::Identity("canonical Stage-3 snow bytes"))
}

/// Shared V11 post-boundary transaction assembly. Both lower-boundary
/// adopters use this owner/resource/finalization path; only the envelope
/// construction differs between snow-free and covered segments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AcceptedPublicationFinalizationPostureV1 {
    RetainFinal,
    DeferTerminalProvisional {
        pre_event_authority_sha256: Digest32,
    },
}

fn accepted_v2_soil_candidate_for_v11_segment(
    beginning: &DirectV10RealConsumerShadow,
    input: &V11ImportedV10SegmentInput,
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    compositional_envelopes: Option<&[UncommittedCoveredV8OwnerEnvelope]>,
    soil_top_boundary_credits: &[SoilThermalTopBoundaryCreditV1],
) -> Result<
    (
        openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        SoilThermalAcceptedCandidateV2,
        SoilThermalOrchestratorSealsV2,
    ),
    DirectV11RealConsumerError,
> {
    let prepared = beginning
        .prepare_next_soil_thermal_support_v2(
            input.support.start_ns().get(),
            input.support.end_ns().get(),
        )
        .map_err(DirectV11RealConsumerError::Runtime)?;
    let mut child_supports = soil_top_boundary_credits
        .iter()
        .map(|credit| (credit.support_start_ns, credit.support_end_ns))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let envelopes = compositional_envelopes.unwrap_or(std::slice::from_ref(envelope));
    if envelopes.is_empty() {
        return Err(DirectV11RealConsumerError::Identity(
            "V2 accepted soil empty carrier composition",
        ));
    }
    if child_supports.is_empty() && envelopes.len() == 1 {
        child_supports.push((
            i64::try_from(input.support.start_ns().get()).map_err(|_| {
                DirectV11RealConsumerError::Identity("V2 accepted soil support start")
            })?,
            i64::try_from(input.support.end_ns().get()).map_err(|_| {
                DirectV11RealConsumerError::Identity("V2 accepted soil support end")
            })?,
        ));
    }
    if child_supports.len() != envelopes.len()
        || child_supports.first().is_none_or(|support| {
            u128::try_from(support.0).ok() != Some(input.support.start_ns().get())
        })
        || child_supports.last().is_none_or(|support| {
            u128::try_from(support.1).ok() != Some(input.support.end_ns().get())
        })
        || child_supports.windows(2).any(|pair| pair[0].1 != pair[1].0)
    {
        return Err(DirectV11RealConsumerError::Identity(
            "V2 accepted soil child support partition",
        ));
    }

    let mut operands = Vec::new();
    for (child, (start, end)) in envelopes.iter().zip(&child_supports) {
        let start = u128::try_from(*start).map_err(|_| {
            DirectV11RealConsumerError::Identity("V2 accepted soil child support start")
        })?;
        let end = u128::try_from(*end).map_err(|_| {
            DirectV11RealConsumerError::Identity("V2 accepted soil child support end")
        })?;
        operands.extend(
            crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
                child.transaction_id(),
                start,
                end,
                &beginning.inner.lse_configuration.owner_id,
                &beginning.inner.surface_configuration.owner_id,
                child.hydrology().pre_ingress_soil_thermal_candidates(),
                child.hydrology().surface_ingress(),
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::LandSurfaceShadow(error),
                ))
            })?,
        );
    }

    let snow_owner = ResourceOwnerId::try_new("snow")
        .map_err(|_| DirectV11RealConsumerError::Identity("V2 accepted soil snow owner"))?;
    let parent_start = i64::try_from(input.support.start_ns().get()).map_err(|_| {
        DirectV11RealConsumerError::Identity("V2 accepted soil parent support start")
    })?;
    let parent_end = i64::try_from(input.support.end_ns().get())
        .map_err(|_| DirectV11RealConsumerError::Identity("V2 accepted soil parent support end"))?;
    for credit in soil_top_boundary_credits {
        let ofe = prepared
            .beginning_owner()
            .state
            .ofes
            .iter()
            .find(|ofe| ofe.ofe_id == credit.ofe_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "V2 accepted top-boundary OFE",
            ))?;
        let layer = ofe
            .ordered_layers
            .first()
            .ok_or(DirectV11RealConsumerError::Identity(
                "V2 accepted top-boundary layer",
            ))?;
        if credit.beginning_owner_id != prepared.beginning_owner().state.owner_id
            || credit.beginning_configuration_sha256
                != prepared.beginning_owner().state.configuration_sha256
            || credit.beginning_state_sha256 != prepared.beginning_owner().state.state_sha256
            || credit.first_layer_id != layer.layer_id
            || credit.support_start_ns < parent_start
            || credit.support_end_ns > parent_end
            || credit.support_start_ns >= credit.support_end_ns
            || credit.soil_thermal_credit_j_m2_ofe_ground.to_bits()
                != credit.accepted_positive_downward_j_m2_ofe_ground.to_bits()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "V2 accepted top-boundary identity or support",
            ));
        }
        operands.push(
            openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2 {
                ofe_id: credit.ofe_id.clone(),
                layer_id: credit.first_layer_id.clone(),
                source_kind:
                    openwepp_land_surface_energy::SoilThermalEnergyOperandKindV2::TopBoundary,
                source_owner_id: snow_owner.clone(),
                debit_credit_identity_sha256: credit.snow_soil_heat_receipt_sha256.clone(),
                ordinal: credit.lane_id,
                units: "J m^-2 OFE-ground".to_owned(),
                basis: "ofe_ground".to_owned(),
                energy_j_m2_ofe_ground: credit.soil_thermal_credit_j_m2_ofe_ground,
            },
        );
    }
    operands.sort_by(|left, right| {
        (
            &left.ofe_id,
            &left.layer_id,
            left.source_kind,
            left.ordinal,
            &left.debit_credit_identity_sha256,
        )
            .cmp(&(
                &right.ofe_id,
                &right.layer_id,
                right.source_kind,
                right.ordinal,
                &right.debit_credit_identity_sha256,
            ))
    });
    let mut ordinals = BTreeMap::new();
    for operand in &mut operands {
        let ordinal = ordinals
            .entry((
                operand.ofe_id.clone(),
                operand.layer_id.clone(),
                operand.source_kind,
            ))
            .or_insert(0_u32);
        operand.ordinal = *ordinal;
        *ordinal = ordinal
            .checked_add(1)
            .ok_or(DirectV11RealConsumerError::Identity(
                "V2 accepted soil operand ordinal overflow",
            ))?;
    }
    canonicalize_v2_operand_order(prepared.beginning_owner(), &mut operands).map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
    })?;
    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        prepared.beginning_owner(),
        &beginning.inner.lse_configuration,
        operands,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
    })?;
    let accepted = aggregate_soil_thermal_ending_v2(
        prepared.beginning_owner(),
        &beginning.inner.lse_configuration,
        &expected,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
    })?;
    let seals = seal_soil_thermal_accepted_candidate_v2(prepared.beginning_owner(), &accepted)
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
        })?;
    Ok((prepared, accepted, seals))
}

fn validate_accepted_vegetation_candidate_transaction_lineage_v1(
    v2_soil: bool,
    reused_precomputed_physical_ending: bool,
    parent_transaction: u128,
    accepted_transaction: u128,
    envelope_transaction: u128,
    candidate_outer_transaction: u128,
    candidate_inner_transaction: u128,
) -> Result<(), DirectV11RealConsumerError> {
    let expected_candidate_transaction = if v2_soil {
        if envelope_transaction != accepted_transaction {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted vegetation candidate transaction lineage",
            ));
        }
        accepted_transaction
    } else if reused_precomputed_physical_ending {
        parent_transaction
    } else {
        accepted_transaction
    };
    if candidate_outer_transaction != expected_candidate_transaction
        || candidate_inner_transaction != expected_candidate_transaction
    {
        return Err(DirectV11RealConsumerError::Identity(
            "accepted vegetation candidate transaction lineage",
        ));
    }
    Ok(())
}

fn validate_native_v2_preinstall_soil_posture_v1(
    matches_authenticated_beginning: bool,
    matches_selected_ending: bool,
) -> Result<(), DirectV11RealConsumerError> {
    if matches_authenticated_beginning == matches_selected_ending {
        return Err(DirectV11RealConsumerError::Identity(
            "V2 accepted soil preinstall owner posture",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn finalize_v11_imported_segment(
    beginning: &DirectV10RealConsumerShadow,
    input: &V11ImportedV10SegmentInput,
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    compositional_envelopes: Option<&[UncommittedCoveredV8OwnerEnvelope]>,
    precomputed_physical_ending: Option<&DirectV10RealConsumerShadow>,
    ending_snow_owner_bytes: Vec<u8>,
    day_index: usize,
    interval_index: usize,
    publication_interval: &DirectV11SnowCoveredSegmentInput,
    soil_top_boundary_credits: &[SoilThermalTopBoundaryCreditV1],
    physical_outcome_ledgers: &BTreeMap<
        u32,
        physical_outcome_ledger::Stage3LanePhysicalOutcomeLedgerV1,
    >,
    publication_posture: AcceptedPublicationFinalizationPostureV1,
) -> Result<
    (
        V11ImportedV10SegmentOutput,
        DirectV10RealConsumerShadow,
        LseSupportAdmissibilityReceiptV1,
    ),
    DirectV11RealConsumerError,
> {
    let reused_precomputed_physical_ending = precomputed_physical_ending.is_some();
    let support_receipt = LseSupportAdmissibilityReceiptV1::admit(
        &beginning.inner.lse_configuration,
        &beginning.inner.lse_state,
        digest32_hex(input.parent_transaction_id.digest()),
        digest32_hex(input.accepted_slab_receipt.segment_id().digest()),
        digest32_hex(input.accepted_slab_receipt.slab_id().digest()),
        input.accepted_slab_receipt.slab_ordinal(),
        input.support.start_ns().get(),
        input.support.end_ns().get(),
        input.duration_s_bits,
        beginning.inner.soil_thermal.state_sha256().clone(),
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
    })?;
    envelope.validate().map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    let resource_debits = if let Some(envelopes) = compositional_envelopes {
        v11_composed_resource_debits(
            envelopes,
            &beginning.inner.lse_configuration,
            &input.configuration,
            input,
        )?
    } else {
        let mut debits =
            v11_nitrogen_resource_debits(envelope, &beginning.inner.lse_configuration, input)?;
        debits.extend(v11_water_resource_debits(
            envelope,
            &input.configuration,
            input,
        )?);
        debits
    };
    let material_transfers = compositional_envelopes.map_or_else(
        || envelope.vegetation().material_proposals().to_vec(),
        |envelopes| {
            envelopes
                .iter()
                .flat_map(|child| child.vegetation().material_proposals().iter().cloned())
                .collect()
        },
    );
    let mut candidate = if let Some(precomputed) = precomputed_physical_ending {
        let mut candidate = precomputed.clone();
        // A composed physical endpoint may carry trial-local support history
        // from preceding physical children. That cache is neither physical
        // owner state nor accepted publication authority. The enclosing slab
        // publishes once from its authenticated beginning; exact child order
        // remains bound by the terminal snow-soil trial-receipt chain.
        candidate.accepted_publication_history = beginning.accepted_publication_history.clone();
        candidate
    } else {
        let mut candidate = beginning.clone();
        match &beginning.inner.soil_thermal {
            DirectSoilThermalResident::V1(_) => {
                let accepted_soil_credit_set = candidate
                    .inner
                    .accept_envelope_with_soil_top_boundary_credits(
                        envelope.transaction_id(),
                        envelope,
                        soil_top_boundary_credits,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
                let soil = candidate.inner.soil_thermal.v1().map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?;
                let joined_soil_owner = soil_thermal_owner_with_top_boundary_credit_join_sha256(
                    digest32_from_lower_hex(soil.snapshot_sha256.as_str())?,
                    &accepted_soil_credit_set.accepted_credit_set_sha256,
                )?;
                if joined_soil_owner == Digest32::zero() {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered soil top-boundary owner join",
                    ));
                }
            }
            DirectSoilThermalResident::V2(_) => {
                super::super::stage_unpublished_v2_carrier_owners(&mut candidate, envelope)?;
            }
        }
        candidate
    };
    let v2_soil = matches!(
        beginning.inner.soil_thermal,
        DirectSoilThermalResident::V2(_)
    );
    if v2_soil && reused_precomputed_physical_ending {
        candidate.inner.vegetation_state = project_v8_runtime_to_v9(
            envelope.vegetation().ending_state(),
            &candidate.inner.vegetation_configuration,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
        })?;
    }
    candidate.vegetation_state = project_v9_runtime_to_v10(
        candidate.inner.vegetation_state(),
        &candidate.vegetation_configuration,
    )
    .map_err(|error| DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::V10(error)))?;
    candidate.lse_state = project_validated_v1_runtime_to_v2(
        &candidate.inner.lse_configuration,
        candidate.inner.lse_state(),
        &candidate.lse_configuration,
        &openwepp_land_surface_energy::Sha256Digest::try_new(
            candidate
                .vegetation_configuration
                .configuration_sha256
                .clone(),
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
        })?,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LseV2(error))
    })?;

    // Reused V1 carrier trials retain parent-normalized owner candidates.  A
    // native-V2 endpoint instead retains the selected unpublished candidate
    // sealed to the support envelope transaction.  Require that distinction
    // before rebasing the remaining staged owner lineages; no physical value
    // or V2 soil candidate is reconstructed here.
    let parent_transaction = input.beginning.0.last_transaction_id;
    let accepted_transaction =
        parent_transaction
            .checked_add(1)
            .ok_or(DirectV11RealConsumerError::Identity(
                "accepted vegetation transaction overflow",
            ))?;
    validate_accepted_vegetation_candidate_transaction_lineage_v1(
        v2_soil,
        reused_precomputed_physical_ending,
        parent_transaction,
        accepted_transaction,
        envelope.transaction_id().0,
        candidate.vegetation_state.0.last_transaction_id,
        candidate.inner.vegetation_state.0.last_transaction_id,
    )?;
    if v2_soil {
        if reused_precomputed_physical_ending {
            normalize_v11_staged_parent_lineage(&mut candidate, accepted_transaction)?;
        }
        let (prepared, accepted, seals) = accepted_v2_soil_candidate_for_v11_segment(
            beginning,
            input,
            envelope,
            compositional_envelopes,
            soil_top_boundary_credits,
        )?;
        if reused_precomputed_physical_ending {
            let preinstall_soil_owner = candidate
                .soil_thermal_v2()
                .map_err(DirectV11RealConsumerError::Runtime)?
                .owner();
            let authenticated_beginning_soil_owner = beginning
                .soil_thermal_v2()
                .map_err(DirectV11RealConsumerError::Runtime)?
                .owner();
            validate_native_v2_preinstall_soil_posture_v1(
                preinstall_soil_owner == authenticated_beginning_soil_owner,
                preinstall_soil_owner == &accepted.ending_owner,
            )?;
            candidate
                .install_soil_thermal_accepted_v2_from_beginning(
                    beginning,
                    prepared.beginning_owner(),
                    accepted,
                    seals,
                )
                .map_err(DirectV11RealConsumerError::Runtime)?;
        } else {
            candidate
                .install_soil_thermal_accepted_v2(prepared.beginning_owner(), accepted, seals)
                .map_err(DirectV11RealConsumerError::Runtime)?;
        }
    }
    let mut segment_ending = candidate.vegetation_state.clone();
    normalize_v8_parent_lineage(&mut segment_ending.0, accepted_transaction);
    // `segment_ending` is the accepted successor consumed by V11. The shadow
    // retained for the next child must instead equal V11's imported V10 view,
    // whose logical owners are normalized to the parent transaction. For V2,
    // this helper deliberately leaves the installed exact-carry soil resident
    // untouched while rebasing vegetation, LSE, surface, and BGC lineage.
    normalize_v11_staged_parent_lineage(&mut candidate, input.beginning.0.last_transaction_id)?;
    let snow = V11OwnerEnvelope::try_new("snow".to_owned(), ending_snow_owner_bytes)?;
    // WB14 can retain an in-parent working state whose effective surface
    // owner is newer than the frame shadow. Publish the same effective owner
    // that the immediate snow-liquid receiver consumes; otherwise a wider
    // accepted support can expose a stale predecessor byte string even
    // though its physical candidate and ledger are valid.
    let surface = candidate.effective_surface_liquid_state_for_zero_duration_receiver()?;
    let surface_bytes = surface
        .canonical_bytes(&candidate.inner.surface_configuration)
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Serialization(error.to_string()),
            ))
        })?;
    let beginning_hydrology_adapter = RealHydrologyShadowAdapter::try_from_day_start(
        &beginning.inner.hydrology_frame,
        day_index,
        TransactionId(input.beginning.0.last_transaction_id),
        f64::from_bits(input.duration_s_bits),
        candidate.inner.surface_configuration.owner_id.clone(),
        &candidate.inner.layer_maps,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    let hydrology_adapter = RealHydrologyShadowAdapter::try_from_day_start(
        envelope.hydrology().ending_frame(),
        day_index,
        TransactionId(input.beginning.0.last_transaction_id),
        f64::from_bits(input.duration_s_bits),
        candidate.inner.surface_configuration.owner_id.clone(),
        &candidate.inner.layer_maps,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    let ending_resource_owners = [
        ("snow".to_owned(), snow),
        (
            "land_surface_energy".to_owned(),
            v11_owner_envelope("land_surface_energy", &candidate.inner.lse_state)?,
        ),
        (
            "surface_liquid".to_owned(),
            V11OwnerEnvelope::try_new("surface_liquid".to_owned(), surface_bytes)?,
        ),
        (
            "hydrology".to_owned(),
            V11OwnerEnvelope::try_new(
                "hydrology".to_owned(),
                hydrology_adapter.snapshot_bytes().to_vec(),
            )?,
        ),
        (
            "bgc".to_owned(),
            v11_owner_envelope("bgc", &candidate.inner.biogeochemistry)?,
        ),
        (
            "soil_thermal".to_owned(),
            v11_owner_envelope("soil_thermal", &candidate.inner.soil_thermal)?,
        ),
    ]
    .into_iter()
    .collect();
    let shared_resource_transitions = v11_shared_resource_transitions(
        envelope,
        input,
        &resource_debits,
        &ending_resource_owners,
        &beginning_hydrology_adapter,
        &hydrology_adapter,
        &beginning.inner.biogeochemistry,
        &candidate.inner.biogeochemistry,
        compositional_envelopes.is_some(),
    )?;
    let mut accepted_ending_owners = ending_resource_owners.clone();
    accepted_ending_owners.insert(
        "vegetation".to_owned(),
        accepted_v11_vegetation_owner(input, &segment_ending)?,
    );
    let ending_owner_states = accepted_ending_owners
        .values()
        .map(V11OwnerEnvelope::to_owner_state)
        .collect::<Result<Vec<_>, _>>()?;
    let ending_complete_owner_set_sha256 =
        openwepp_coupled_time::complete_owner_set_digest(&ending_owner_states).map_err(|_| {
            DirectV11RealConsumerError::Identity("accepted publication ending complete-owner set")
        })?;
    match publication_posture {
        AcceptedPublicationFinalizationPostureV1::RetainFinal => {
            candidate.retain_accepted_publication_support(
                day_index,
                interval_index,
                input,
                ending_complete_owner_set_sha256,
                support_receipt.clone(),
                publication_interval.lse_forcing.clone(),
                publication_interval.vegetation_forcing.clone(),
                publication_interval.wb14_parameters.clone(),
                resource_debits.clone(),
                material_transfers.clone(),
                envelope.hydrology(),
                Some(physical_outcome_ledgers),
            )?;
        }
        AcceptedPublicationFinalizationPostureV1::DeferTerminalProvisional {
            pre_event_authority_sha256,
        } => {
            if pre_event_authority_sha256 == Digest32::zero() {
                return Err(DirectV11RealConsumerError::Identity(
                    "terminal provisional publication deferral authority",
                ));
            }
        }
    }
    let output = V11ImportedV10SegmentOutput {
        ending: segment_ending,
        lse_support_receipt: V11LseSupportReceiptEnvelope::from_canonical_json(
            serde_json::to_vec(&support_receipt).map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::Serialization(error.to_string()),
                ))
            })?,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("V11 LSE support receipt"))?,
        resource_debits,
        admitted_resource_fluxes: Vec::<V11AdmittedResourceFlux>::new(),
        shared_resource_transitions,
        ending_resource_owners,
        material_transfers,
    };
    Ok((output, candidate, support_receipt))
}

pub(crate) fn digest32_hex(value: Digest32) -> String {
    let mut text = String::with_capacity(64);
    for byte in value.as_bytes() {
        write!(&mut text, "{byte:02x}").expect("writing to String cannot fail");
    }
    text
}

pub(crate) fn digest32_from_lower_hex(value: &str) -> Result<Digest32, DirectV11RealConsumerError> {
    if value.len() != 64
        || value
            .bytes()
            .any(|byte| !byte.is_ascii_hexdigit() || byte.is_ascii_uppercase())
    {
        return Err(DirectV11RealConsumerError::Identity(
            "covered receipt digest encoding",
        ));
    }
    let mut bytes = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let high = match pair[0] {
            byte @ b'0'..=b'9' => byte - b'0',
            byte @ b'a'..=b'f' => byte - b'a' + 10,
            _ => {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered receipt digest encoding",
                ));
            }
        };
        let low = match pair[1] {
            byte @ b'0'..=b'9' => byte - b'0',
            byte @ b'a'..=b'f' => byte - b'a' + 10,
            _ => {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered receipt digest encoding",
                ));
            }
        };
        bytes[index] = (high << 4) | low;
    }
    Ok(Digest32::from_bytes(bytes))
}

pub(crate) fn stage3_albedo_state_digest(
    input: &DirectActiveSnowPartitionInputs,
) -> Result<Sha256Digest, DirectV11RealConsumerError> {
    let mut bytes = Vec::with_capacity(128);
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_SNOW_ALBEDO_STATE_V1\0");
    match input.snow_albedo_state {
        Some(state) => {
            bytes.push(1);
            bytes.extend_from_slice(state.model.id().as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&state.albedo.to_bits().to_le_bytes());
            bytes.extend_from_slice(
                &state
                    .accumulated_positive_temperature_c_day
                    .to_bits()
                    .to_le_bytes(),
            );
        }
        None => bytes.push(0),
    }
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| DirectV11RealConsumerError::Identity("Stage-3 albedo digest"))
}

pub(crate) fn stage3_support_forcing_digest(
    support: DirectSnowStage3SupportInput,
) -> Result<Sha256Digest, DirectV11RealConsumerError> {
    let forcing = support.forcing;
    let mut bytes = Vec::with_capacity(256);
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_SUPPORT_FORCING_V1\0");
    for value in [
        forcing.active_precipitation_m,
        forcing.rain_m,
        forcing.snowfall_m,
        forcing.radiation_mj_m2,
        forcing.air_temperature_c,
        forcing.cloud_fraction,
        forcing.rain_fraction,
        forcing.snow_fraction,
        support.duration_seconds,
    ] {
        bytes.extend_from_slice(&value.to_bits().to_le_bytes());
    }
    bytes.extend_from_slice(forcing.phase_model.id().as_bytes());
    bytes.push(0);
    match forcing.hydrometeor_temperature_c {
        Some(value) => {
            bytes.push(1);
            bytes.extend_from_slice(&value.to_bits().to_le_bytes());
        }
        None => bytes.push(0),
    }
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| DirectV11RealConsumerError::Identity("Stage-3 forcing digest"))
}

pub(crate) fn normalize_v11_staged_parent_lineage(
    staged: &mut DirectV10RealConsumerShadow,
    parent: u128,
) -> Result<(), DirectV11RealConsumerError> {
    normalize_v8_parent_lineage(&mut staged.vegetation_state.0, parent);
    normalize_v8_parent_lineage(&mut staged.inner.vegetation_state.0, parent);
    let transaction = (parent != 0).then_some(TransactionId(parent));
    staged.inner.lse_state.last_accepted_transaction_id = transaction;
    staged.inner.lse_state.state_sha256 =
        staged.inner.lse_state.canonical_sha256().map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
        })?;
    staged.lse_state.0.last_accepted_transaction_id = transaction;
    staged.lse_state.0.state_sha256 = staged.lse_state.0.canonical_sha256().map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
    })?;
    let surface = staged
        .inner
        .hydrology_frame
        .surface_liquid_shadow
        .as_mut()
        .ok_or(DirectV11RealConsumerError::Identity(
            "missing staged surface-liquid owner",
        ))?;
    if staged.inner.wb14_parent_working_state.is_none() {
        for record in &mut surface.records {
            record.last_accepted_transaction_id = transaction;
        }
        for continuation in &mut surface.continuations {
            continuation.last_accepted_transaction_id = transaction;
        }
        surface.state_sha256 = surface.recomputed_sha256().map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Serialization(error.to_string()),
            ))
        })?;
    }
    staged.inner.biogeochemistry.last_transaction_id = parent;
    if let DirectSoilThermalResident::V1(soil) = &mut staged.inner.soil_thermal {
        soil.last_accepted_transaction_id = transaction;
        let soil_transaction = transaction.ok_or(DirectV11RealConsumerError::Identity(
            "zero parent transaction lineage",
        ))?;
        soil.state_sha256 = digest_soil_state(&soil.owner_id, soil_transaction, &soil.ofes)
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })?;
        soil.snapshot_sha256 = digest_soil_snapshot(
            &soil.owner_id,
            &soil.configuration_sha256,
            &soil.state_sha256,
            soil_transaction,
            &soil.ofes,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
        })?;
    }
    Ok(())
}

pub(crate) fn normalize_v8_parent_lineage(
    state: &mut openwepp_vegetation::V8CoupledOwnedState,
    parent: u128,
) {
    state.last_transaction_id = parent;
    for stratum in state.strata.values_mut() {
        stratum.last_transaction_id = parent;
    }
    let accepted = (parent != 0).then_some(parent);
    for occupancy in state.occupancies.values_mut() {
        occupancy.last_accepted_transaction_id = accepted;
    }
    state.state_sha256 = state.canonical_sha256();
}

/// Reconstruct the exact V11 vegetation envelope that the outer accepted
/// segment transaction installs after importing the V10 ending.  The V11
/// model/configuration identity comes from the authenticated beginning owner;
/// the physical ending is the just-accepted imported ending, with the same
/// parent-lineage normalization used by `stage_imported_ending`.
pub(crate) fn accepted_v11_vegetation_owner(
    input: &V11ImportedV10SegmentInput,
    imported_ending: &V10CoupledOwnedState,
) -> Result<V11OwnerEnvelope, DirectV11RealConsumerError> {
    let beginning_envelope = input.staged_resource_owners.get("vegetation").ok_or(
        DirectV11RealConsumerError::Identity("accepted vegetation beginning owner"),
    )?;
    beginning_envelope.to_owner_state()?;
    let beginning: openwepp_vegetation::v11::V11CoupledOwnedState =
        serde_json::from_slice(&beginning_envelope.state_bytes).map_err(|_| {
            DirectV11RealConsumerError::Identity("accepted vegetation beginning state")
        })?;
    let parent = input.beginning.0.last_transaction_id;
    if beginning.last_parent_transaction_id != parent {
        return Err(DirectV11RealConsumerError::Identity(
            "accepted vegetation parent lineage",
        ));
    }

    let mut physical = imported_ending.0.clone();
    physical
        .model_definition_sha256
        .clone_from(&beginning.model_definition_sha256);
    physical
        .configuration_sha256
        .clone_from(&beginning.configuration_sha256);
    normalize_v8_parent_lineage(&mut physical, parent);
    let mut ending = openwepp_vegetation::v11::V11CoupledOwnedState {
        model_definition_sha256: beginning.model_definition_sha256,
        configuration_sha256: beginning.configuration_sha256,
        state_sha256: String::new(),
        physical,
        last_parent_transaction_id: parent,
    };
    ending.state_sha256 = ending.canonical_sha256()?;
    Ok(openwepp_vegetation::v11::v11_vegetation_owner_envelope(
        &ending,
    )?)
}

fn v11_composed_resource_debits(
    envelopes: &[UncommittedCoveredV8OwnerEnvelope],
    lse_configuration: &openwepp_land_surface_energy::LandSurfaceEnergyConfiguration,
    vegetation_configuration: &VegetationConfiguration,
    input: &V11ImportedV10SegmentInput,
) -> Result<Vec<V11ResourceDebit>, DirectV11RealConsumerError> {
    if envelopes.is_empty() {
        return Err(DirectV11RealConsumerError::Identity(
            "V11 empty compositional resource custody",
        ));
    }
    type DebitKey = (
        String,
        V11ResourceKey,
        String,
        String,
        String,
        String,
        String,
        String,
    );
    let mut composed = BTreeMap::<DebitKey, V11ResourceDebit>::new();
    for envelope in envelopes {
        let mut child = v11_nitrogen_resource_debits(envelope, lse_configuration, input)?;
        child.extend(v11_water_resource_debits(
            envelope,
            vegetation_configuration,
            input,
        )?);
        for debit in child {
            let key = (
                debit.owner_id.clone(),
                debit.resource_key.clone(),
                debit.ofe_id.clone(),
                debit.tile_id.clone(),
                debit.occupancy_id.clone(),
                debit.layer_id.clone(),
                debit.source_id.clone(),
                debit.amount_basis.clone(),
            );
            if let Some(aggregate) = composed.get_mut(&key) {
                aggregate.request += debit.request;
                aggregate.authorization += debit.authorization;
                aggregate.final_use += debit.final_use;
                if !aggregate.request.is_finite()
                    || !aggregate.authorization.is_finite()
                    || !aggregate.final_use.is_finite()
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "V11 compositional resource debit finite sum",
                    ));
                }
            } else {
                composed.insert(key, debit);
            }
        }
    }
    let mut debits = composed
        .into_values()
        .map(|mut debit| {
            debit.receipt_id = Digest32::zero();
            V11ResourceDebit::new(debit).map_err(|_| {
                DirectV11RealConsumerError::Identity("V11 compositional resource debit")
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    debits.sort_by(|left, right| {
        left.owner_id
            .cmp(&right.owner_id)
            .then_with(|| left.occupancy_id.cmp(&right.occupancy_id))
            .then_with(|| left.layer_id.cmp(&right.layer_id))
            .then_with(|| left.resource_key.cmp(&right.resource_key))
            .then_with(|| left.ofe_id.cmp(&right.ofe_id))
            .then_with(|| left.tile_id.cmp(&right.tile_id))
            .then_with(|| left.source_id.cmp(&right.source_id))
            .then_with(|| left.amount_basis.cmp(&right.amount_basis))
    });
    Ok(debits)
}

pub(crate) fn v11_nitrogen_resource_debits(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    lse_configuration: &openwepp_land_surface_energy::LandSurfaceEnergyConfiguration,
    input: &V11ImportedV10SegmentInput,
) -> Result<Vec<V11ResourceDebit>, DirectV11RealConsumerError> {
    let (requests, authorizations, uses) = envelope.vegetation().nitrogen_protocol();
    if validate_v11_nitrogen_protocol_cardinality(requests.len(), authorizations.len(), uses.len())?
    {
        return Ok(Vec::new());
    }
    let occupancies = input.configuration.expected_occupancies();
    let bgc_ofe_id = v11_bgc_bearing_ofe(&occupancies, lse_configuration)?;
    let mut ordered_uses = uses.iter().collect::<Vec<_>>();
    ordered_uses.sort_by(|left, right| {
        left.owner_id
            .cmp(&right.owner_id)
            .then_with(|| left.key.cmp(&right.key))
    });
    ordered_uses
        .into_iter()
        .map(|used| {
            let request = requests
                .iter()
                .find(|row| row.owner_id == used.owner_id && row.key == used.key)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 nitrogen request binding",
                ))?;
            let authorization = authorizations
                .iter()
                .find(|row| row.owner_id == used.owner_id && row.key == used.key)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 nitrogen authorization binding",
                ))?;
            if !occupancies
                .iter()
                .any(|id| id.stratum_id.as_str() == used.owner_id.as_str())
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "V11 nitrogen stratum binding",
                ));
            }
            V11ResourceDebit::new(V11ResourceDebit {
                receipt_id: Digest32::zero(),
                parent_transaction_id: input.parent_transaction_id,
                segment_id: input.accepted_slab_receipt.segment_id(),
                accepted_slab_id: input.accepted_slab_receipt.slab_id(),
                support: input.support,
                owner_id: "bgc".into(),
                resource_key: V11ResourceKey::MineralNitrogen(used.key.clone()),
                ofe_id: bgc_ofe_id.clone(),
                tile_id: "stratum_scoped".into(),
                occupancy_id: used.owner_id.as_str().to_owned(),
                layer_id: used.key.layer_id.as_str().to_owned(),
                source_id: match used.key.species {
                    MineralNitrogenSpecies::Ammonium => "nh4",
                    MineralNitrogenSpecies::Nitrate => "no3",
                }
                .into(),
                amount_basis: "kg_n_m2".into(),
                request: request.amount,
                authorization: authorization.amount,
                final_use: used.amount,
            })
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 nitrogen debit"))
        })
        .collect()
}

fn v11_bgc_bearing_ofe(
    occupancies: &BTreeSet<openwepp_kernel_contract::OccupancyId>,
    lse_configuration: &openwepp_land_surface_energy::LandSurfaceEnergyConfiguration,
) -> Result<String, DirectV11RealConsumerError> {
    let mut resolved = BTreeSet::new();
    for occupancy in occupancies {
        let matching = lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles
                    .iter()
                    .filter(|tile| tile.vegetation_tile_id == occupancy.tile_id)
                    .map(move |_| ofe.ofe_id.as_str())
            })
            .collect::<BTreeSet<_>>();
        let ofe = matching
            .iter()
            .next()
            .filter(|_| matching.len() == 1)
            .ok_or(DirectV11RealConsumerError::Identity(
                "V11 BGC vegetation-tile/OFE binding",
            ))?;
        resolved.insert((*ofe).to_owned());
    }
    let resolved_count = resolved.len();
    resolved
        .into_iter()
        .next()
        .filter(|_| resolved_count == 1)
        .ok_or(DirectV11RealConsumerError::Identity(
            "V11 exact-one BGC-bearing OFE",
        ))
}

pub(crate) fn v11_bgc_debit_scope(
    vegetation_configuration: &VegetationConfiguration,
    lse_configuration: &openwepp_land_surface_energy::LandSurfaceEnergyConfiguration,
) -> Result<V11BgcDebitScope, DirectV11RealConsumerError> {
    let occupancies = vegetation_configuration.expected_occupancies();
    let mut stratum_ofe_ids = BTreeMap::new();
    let mut ambiguous_strata = BTreeSet::new();
    for occupancy in occupancies {
        let matching = lse_configuration
            .ofes
            .iter()
            .filter(|ofe| {
                ofe.tiles
                    .iter()
                    .any(|tile| tile.vegetation_tile_id == occupancy.tile_id)
            })
            .map(|ofe| ofe.ofe_id.as_str())
            .collect::<BTreeSet<_>>();
        if matching.len() != 1 {
            ambiguous_strata.insert(occupancy.stratum_id.as_str().to_owned());
            continue;
        }
        let ofe = (*matching
            .iter()
            .next()
            .ok_or(DirectV11RealConsumerError::Identity(
                "V11 BGC stratum/OFE scope",
            ))?)
        .to_owned();
        if stratum_ofe_ids
            .insert(occupancy.stratum_id.as_str().to_owned(), ofe.clone())
            .is_some_and(|prior| prior != ofe)
        {
            ambiguous_strata.insert(occupancy.stratum_id.as_str().to_owned());
        }
    }
    for stratum in ambiguous_strata {
        stratum_ofe_ids.remove(&stratum);
    }
    V11BgcDebitScope::try_new(stratum_ofe_ids)
        .map_err(|_| DirectV11RealConsumerError::Identity("V11 BGC stratum/OFE scope"))
}

fn validate_v11_nitrogen_protocol_cardinality(
    request_count: usize,
    authorization_count: usize,
    use_count: usize,
) -> Result<bool, DirectV11RealConsumerError> {
    if request_count == 0 && authorization_count == 0 && use_count == 0 {
        return Ok(true);
    }
    if request_count == 0 || authorization_count == 0 || use_count == 0 {
        return Err(DirectV11RealConsumerError::Identity(
            "V11 nitrogen protocol cardinality",
        ));
    }
    Ok(false)
}

#[cfg(test)]
mod nitrogen_protocol_cardinality_tests {
    use super::*;
    use crate::land_surface_energy_shadow::strict_v8_endpoint::endpoint_rollback_tests::{
        endpoint_fixture, two_ofe_routed_endpoint_fixture,
    };

    #[test]
    fn empty_protocol_is_admissible_and_every_partial_empty_protocol_rejects() {
        assert!(validate_v11_nitrogen_protocol_cardinality(0, 0, 0).expect("empty protocol"));
        for counts in [
            (0, 0, 1),
            (0, 1, 0),
            (1, 0, 0),
            (0, 1, 1),
            (1, 0, 1),
            (1, 1, 0),
        ] {
            assert!(
                validate_v11_nitrogen_protocol_cardinality(counts.0, counts.1, counts.2).is_err(),
                "partial-empty poison {counts:?}",
            );
        }
        assert!(!validate_v11_nitrogen_protocol_cardinality(1, 1, 1).expect("nonempty protocol"));
    }

    #[test]
    fn bgc_ofe_resolution_uses_explicit_vegetation_tile_mapping() {
        let mut fixture = two_ofe_routed_endpoint_fixture();
        for tile in &mut fixture.lse_configuration.ofes[0].tiles {
            tile.vegetation_tile_id = openwepp_kernel_contract::TileId::try_new(format!(
                "upper-open-{}",
                tile.tile_id.as_str()
            ))
            .expect("upper open vegetation tile");
        }
        let lower_forest = fixture.lse_configuration.ofes[1]
            .tiles
            .iter_mut()
            .find(|tile| tile.tile_id.as_str() == "lower-forest")
            .expect("lower forest tile");
        assert_ne!(lower_forest.tile_id, lower_forest.vegetation_tile_id);
        assert_eq!(
            v11_bgc_bearing_ofe(
                &fixture.vegetation_configuration.expected_occupancies(),
                &fixture.lse_configuration,
            )
            .expect("open first OFE and vegetated second OFE"),
            "ofe-2"
        );
    }

    #[test]
    fn bgc_ofe_resolution_admits_multi_tile_stratum_within_one_ofe() {
        let mut fixture = endpoint_fixture();
        let second_tile = openwepp_kernel_contract::TileId::try_new("open").expect("tile");
        fixture.vegetation_configuration.strata[0]
            .tile_ids
            .push(second_tile);
        assert_eq!(
            v11_bgc_bearing_ofe(
                &fixture.vegetation_configuration.expected_occupancies(),
                &fixture.lse_configuration,
            )
            .expect("one stratum on multiple vegetation tiles"),
            "ofe-1"
        );
    }

    #[test]
    fn bgc_ofe_resolution_rejects_two_covered_vegetated_ofes() {
        let fixture = two_ofe_routed_endpoint_fixture();
        assert!(
            v11_bgc_bearing_ofe(
                &fixture.vegetation_configuration.expected_occupancies(),
                &fixture.lse_configuration,
            )
            .is_err()
        );
    }

    #[test]
    fn repeated_local_lse_tile_ids_do_not_replace_vegetation_mapping() {
        let mut fixture = two_ofe_routed_endpoint_fixture();
        for tile in &mut fixture.lse_configuration.ofes[0].tiles {
            tile.vegetation_tile_id = openwepp_kernel_contract::TileId::try_new(format!(
                "upper-open-{}",
                tile.tile_id.as_str()
            ))
            .expect("upper open vegetation tile");
        }
        let repeated = fixture.lse_configuration.ofes[0].tiles[0].tile_id.clone();
        fixture.lse_configuration.ofes[1].tiles[0].tile_id = repeated;
        assert_eq!(
            v11_bgc_bearing_ofe(
                &fixture.vegetation_configuration.expected_occupancies(),
                &fixture.lse_configuration,
            )
            .expect("repeated local LSE IDs with unique vegetation mapping"),
            "ofe-2"
        );
    }

    #[test]
    fn bgc_linkage_uses_pre_hash_three_stratum_nonassociative_order() {
        use openwepp_coupled_time::{
            AcceptedSlabId, ModelTimeNs, ParentTransactionId, SegmentId, TimeSupport,
        };
        use openwepp_kernel_contract::{MineralNitrogenKey, SoilLayerId};

        let key = MineralNitrogenKey {
            layer_id: SoilLayerId::try_new("layer-1").expect("layer"),
            species: MineralNitrogenSpecies::Ammonium,
        };
        let shared = V11SharedResourceKey {
            resource: V11SharedResourceKind::Ammonium,
            owner_id: "bgc".into(),
            ofe_id: "ofe-2".into(),
            layer_id: "layer-1".into(),
            source_id: "nh4".into(),
            amount_basis: "kg_n_m2".into(),
        };
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1)).expect("support");
        let values = [
            0.001_626_199_161_107_315_3,
            0.000_000_038_444_775_879_237_09,
            0.000_000_016_590_450_830_746_63,
        ];
        let make = |ordinal: u8, stratum: &str, amount: f64| V11ResourceDebit {
            receipt_id: Digest32::from_bytes([ordinal; 32]),
            parent_transaction_id: ParentTransactionId::from_digest(Digest32::from_bytes([9; 32])),
            segment_id: SegmentId::from_digest(Digest32::from_bytes([8; 32])),
            accepted_slab_id: AcceptedSlabId::from_digest(Digest32::from_bytes([7; 32])),
            support,
            owner_id: "bgc".into(),
            resource_key: V11ResourceKey::MineralNitrogen(key.clone()),
            ofe_id: "ofe-2".into(),
            tile_id: "stratum_scoped".into(),
            occupancy_id: stratum.into(),
            layer_id: "layer-1".into(),
            source_id: "nh4".into(),
            amount_basis: "kg_n_m2".into(),
            request: amount,
            authorization: amount,
            final_use: amount,
        };
        let debits = vec![
            make(2, "stratum-b", values[1]),
            make(1, "stratum-c", values[2]),
            make(3, "stratum-a", values[0]),
        ];
        let ids = v11_linked_debit_ids(&debits, &shared, true);
        assert_eq!(
            ids,
            vec![
                Digest32::from_bytes([3; 32]),
                Digest32::from_bytes([2; 32]),
                Digest32::from_bytes([1; 32])
            ]
        );
        let semantic = ids.iter().fold(0.0_f64, |sum, id| {
            sum + debits
                .iter()
                .find(|debit| debit.receipt_id == *id)
                .expect("linked")
                .final_use
        });
        let alternate_permutation = debits
            .iter()
            .map(|debit| debit.final_use)
            .fold(0.0_f64, |sum, value| sum + value);
        assert_eq!(
            semantic.to_bits(),
            0.001_626_254_196_334_025_4_f64.to_bits()
        );
        assert_eq!(
            alternate_permutation.to_bits(),
            0.001_626_254_196_334_025_1_f64.to_bits()
        );
        assert_ne!(semantic.to_bits(), alternate_permutation.to_bits());
    }
}

pub(crate) fn v11_shared_resource_transitions(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
    debits: &[V11ResourceDebit],
    owners: &BTreeMap<String, V11OwnerEnvelope>,
    beginning_hydrology: &RealHydrologyShadowAdapter,
    ending_hydrology: &RealHydrologyShadowAdapter,
    beginning_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
    ending_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
    compositional: bool,
) -> Result<Vec<V11SharedResourceOwnerTransition>, DirectV11RealConsumerError> {
    let hydrology_digest = owners
        .get("hydrology")
        .ok_or(DirectV11RealConsumerError::Identity(
            "V11 hydrology candidate",
        ))?
        .state_sha256;
    let mut rows = v11_water_owner_transitions(
        envelope,
        input,
        debits,
        beginning_hydrology,
        ending_hydrology,
        hydrology_digest,
    )?;
    let bgc_digest = owners
        .get("bgc")
        .ok_or(DirectV11RealConsumerError::Identity("V11 BGC candidate"))?
        .state_sha256;
    let bgc_ofes = debits
        .iter()
        .filter(|debit| matches!(&debit.resource_key, V11ResourceKey::MineralNitrogen(_)))
        .map(|debit| debit.ofe_id.as_str())
        .collect::<BTreeSet<_>>();
    let ofe_id = bgc_ofes
        .iter()
        .next()
        .filter(|_| bgc_ofes.len() == 1)
        .copied();
    if envelope
        .biogeochemistry()
        .mineral_operands()
        .iter()
        .any(|operand| operand.finalized_use_kg_n_m2 > 0.0)
        && ofe_id.is_none()
    {
        return Err(DirectV11RealConsumerError::Identity(
            "V11 exact-one BGC transition OFE",
        ));
    }
    if let Some(ofe_id) = ofe_id {
        rows.extend(v11_bgc_owner_transitions(
            envelope,
            input,
            debits,
            beginning_bgc,
            ending_bgc,
            ofe_id,
            bgc_digest,
            compositional,
        )?);
    }
    Ok(rows)
}

pub(crate) fn v11_water_owner_transitions(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
    debits: &[V11ResourceDebit],
    beginning: &RealHydrologyShadowAdapter,
    ending: &RealHydrologyShadowAdapter,
    owner_digest: Digest32,
) -> Result<Vec<V11SharedResourceOwnerTransition>, DirectV11RealConsumerError> {
    let mut rows = Vec::new();
    for (ofe_index, ofe) in envelope
        .hydrology()
        .receiver_closure_operands()
        .production_soil
        .iter()
        .enumerate()
    {
        for layer in &ofe.ordered_layers {
            let key = V11SharedResourceKey {
                resource: V11SharedResourceKind::Water,
                owner_id: "hydrology".into(),
                ofe_id: ofe.ofe_id.as_str().to_owned(),
                layer_id: layer.layer_id.as_str().to_owned(),
                source_id: "soil_water".into(),
                amount_basis: "kg_m2_stand_ground".into(),
            };
            let ids = v11_linked_debit_ids(debits, &key, true);
            if ids.is_empty() {
                continue;
            }
            let amount = |owner: &RealHydrologyShadowAdapter, message| {
                owner
                    .layer_facts()
                    .values()
                    .find(|fact| {
                        fact.source.ofe_lane.lane_index == ofe_index
                            && fact.source.layer_id == layer.layer_id
                    })
                    .map(|fact| fact.liquid_supply_kg_m2)
                    .ok_or(DirectV11RealConsumerError::Identity(message))
            };
            rows.push(v11_shared_transition(
                input,
                key,
                amount(beginning, "V11 beginning hydrology layer binding")?,
                amount(ending, "V11 ending hydrology layer binding")?,
                ids,
                owner_digest,
            )?);
        }
    }
    Ok(rows)
}

pub(crate) fn v11_bgc_owner_transitions(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
    debits: &[V11ResourceDebit],
    beginning_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
    ending_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
    ofe_id: &str,
    owner_digest: Digest32,
    compositional: bool,
) -> Result<Vec<V11SharedResourceOwnerTransition>, DirectV11RealConsumerError> {
    let mut rows = Vec::new();
    for operand in envelope.biogeochemistry().mineral_operands() {
        let source_id = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => "nh4",
            MineralNitrogenSpecies::Nitrate => "no3",
        };
        let resource = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => V11SharedResourceKind::Ammonium,
            MineralNitrogenSpecies::Nitrate => V11SharedResourceKind::Nitrate,
        };
        let key = V11SharedResourceKey {
            resource,
            owner_id: "bgc".into(),
            ofe_id: ofe_id.to_owned(),
            layer_id: operand.key.layer_id.as_str().to_owned(),
            source_id: source_id.into(),
            amount_basis: "kg_n_m2".into(),
        };
        let ids = v11_linked_debit_ids(debits, &key, true);
        if ids.is_empty() {
            if operand.finalized_use_kg_n_m2 > 0.0 {
                return Err(DirectV11RealConsumerError::Identity(
                    "V11 BGC debit omission",
                ));
            }
            continue;
        }
        let beginning_layer = beginning_bgc
            .layers
            .get(operand.key.layer_id.as_str())
            .ok_or(DirectV11RealConsumerError::Identity(
                "V11 beginning BGC layer binding",
            ))?;
        let beginning_amount = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => beginning_layer.ammonium_n,
            MineralNitrogenSpecies::Nitrate => beginning_layer.nitrate_n,
        };
        let ending_layer = ending_bgc.layers.get(operand.key.layer_id.as_str()).ok_or(
            DirectV11RealConsumerError::Identity("V11 ending BGC layer binding"),
        )?;
        let ending_amount = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => ending_layer.ammonium_n,
            MineralNitrogenSpecies::Nitrate => ending_layer.nitrate_n,
        };
        let linked_use = ids
            .iter()
            .map(|id| {
                debits.iter().find(|debit| debit.receipt_id == *id).ok_or(
                    DirectV11RealConsumerError::Identity("V11 BGC linked debit identity"),
                )
            })
            .try_fold(0.0_f64, |sum, debit| {
                let next = sum + debit?.final_use;
                next.is_finite()
                    .then_some(next)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "V11 BGC finalized-use sum",
                    ))
            })?;
        let reconstructed_ending = beginning_amount - linked_use;
        if (!compositional
            && (linked_use.to_bits() != operand.finalized_use_kg_n_m2.to_bits()
                || ending_amount.to_bits() != operand.ending_kg_n_m2.to_bits()))
            || reconstructed_ending.to_bits() != ending_amount.to_bits()
                && (!compositional
                    || !v11_compositional_pool_roundoff_within_one_ulp(
                        reconstructed_ending,
                        ending_amount,
                    ))
        {
            return Err(DirectV11RealConsumerError::Identity(
                "V11 BGC mineral-pool delta",
            ));
        }
        rows.push(v11_shared_transition(
            input,
            key,
            beginning_amount,
            ending_amount,
            ids,
            owner_digest,
        )?);
    }
    Ok(rows)
}

fn v11_compositional_pool_roundoff_within_one_ulp(reconstructed: f64, installed: f64) -> bool {
    reconstructed.is_finite()
        && installed.is_finite()
        && reconstructed >= 0.0
        && installed >= 0.0
        && reconstructed.to_bits().abs_diff(installed.to_bits()) <= 1
}

pub(crate) fn v11_linked_debit_ids(
    debits: &[V11ResourceDebit],
    key: &V11SharedResourceKey,
    bind_amount_basis: bool,
) -> Vec<Digest32> {
    let mut linked = debits
        .iter()
        .filter(|debit| {
            debit.owner_id == key.owner_id
                && debit.ofe_id == key.ofe_id
                && debit.layer_id == key.layer_id
                && debit.source_id == key.source_id
                && (!bind_amount_basis || debit.amount_basis == key.amount_basis)
        })
        .collect::<Vec<_>>();
    if key.owner_id == "bgc"
        && matches!(
            key.resource,
            V11SharedResourceKind::Ammonium | V11SharedResourceKind::Nitrate
        )
    {
        linked.sort_by(|left, right| {
            left.occupancy_id
                .cmp(&right.occupancy_id)
                .then_with(|| left.layer_id.cmp(&right.layer_id))
                .then_with(|| left.resource_key.cmp(&right.resource_key))
        });
    } else {
        linked.sort_by_key(|debit| debit.receipt_id);
    }
    linked.into_iter().map(|debit| debit.receipt_id).collect()
}

pub(crate) fn v11_shared_transition(
    input: &V11ImportedV10SegmentInput,
    key: V11SharedResourceKey,
    beginning_amount: f64,
    ending_amount: f64,
    debit_receipt_ids: Vec<Digest32>,
    owner_candidate_sha256: Digest32,
) -> Result<V11SharedResourceOwnerTransition, DirectV11RealConsumerError> {
    Ok(V11SharedResourceOwnerTransition::new(
        V11SharedResourceOwnerTransition {
            transition_id: Digest32::zero(),
            parent_transaction_id: input.parent_transaction_id,
            segment_id: input.accepted_slab_receipt.segment_id(),
            accepted_slab_id: input.accepted_slab_receipt.slab_id(),
            support: input.support,
            shared_resource_key: key,
            beginning_amount,
            ending_amount,
            debit_receipt_ids,
            admitted_flux_receipt_ids: Vec::new(),
            owner_candidate_sha256,
        },
    )?)
}

pub(crate) fn v11_water_resource_debits(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    configuration: &VegetationConfiguration,
    input: &V11ImportedV10SegmentInput,
) -> Result<Vec<V11ResourceDebit>, DirectV11RealConsumerError> {
    let occupancies = configuration
        .expected_occupancies()
        .into_iter()
        .map(|id| {
            (
                format!("{}::{}", id.stratum_id.as_str(), id.tile_id.as_str()),
                id,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let requests = &envelope.hydrology().arbitration().requests;
    let authorizations = &envelope.hydrology().arbitration().authorizations;
    envelope
        .hydrology()
        .finalized_uses()
        .iter()
        .filter_map(|value| {
            let component = value.key.occupancy_id.as_ref()?;
            let layer = value.key.soil_layer_id.as_ref()?;
            Some((value, component.as_str(), layer))
        })
        .map(|(value, component, layer)| {
            let occupancy =
                occupancies
                    .get(component)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "V11 water occupancy binding",
                    ))?;
            let request = requests.iter().find(|row| row.key == value.key).ok_or(
                DirectV11RealConsumerError::Identity("V11 water request binding"),
            )?;
            let authorization = authorizations
                .iter()
                .find(|row| row.key == value.key)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 water authorization binding",
                ))?;
            V11ResourceDebit::new(V11ResourceDebit {
                receipt_id: Digest32::zero(),
                parent_transaction_id: input.parent_transaction_id,
                segment_id: input.accepted_slab_receipt.segment_id(),
                accepted_slab_id: input.accepted_slab_receipt.slab_id(),
                support: input.support,
                owner_id: "hydrology".to_owned(),
                resource_key: V11ResourceKey::Water(openwepp_kernel_contract::WaterResourceKey {
                    occupancy_id: occupancy.clone(),
                    layer_id: layer.clone(),
                }),
                ofe_id: value.key.ofe_id.as_str().to_owned(),
                tile_id: occupancy.tile_id.as_str().to_owned(),
                occupancy_id: component.to_owned(),
                layer_id: layer.as_str().to_owned(),
                source_id: "soil_water".to_owned(),
                amount_basis: "kg_m2_stand_ground".to_owned(),
                request: request.amount_kg_m2_stand_ground,
                authorization: authorization.amount_kg_m2_stand_ground,
                final_use: value.amount_kg_m2_stand_ground,
            })
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 water debit"))
        })
        .collect()
}

#[cfg(test)]
mod owner_join_tests {
    use super::*;
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

    #[test]
    fn compositional_pool_roundoff_accepts_only_one_ulp() {
        let value = 1.0_f64;
        assert!(v11_compositional_pool_roundoff_within_one_ulp(value, value));
        assert!(v11_compositional_pool_roundoff_within_one_ulp(
            value,
            f64::from_bits(value.to_bits() + 1),
        ));
        assert!(!v11_compositional_pool_roundoff_within_one_ulp(
            value,
            f64::from_bits(value.to_bits() + 2),
        ));
        assert!(!v11_compositional_pool_roundoff_within_one_ulp(
            f64::NAN,
            value,
        ));
        assert!(!v11_compositional_pool_roundoff_within_one_ulp(
            -value, value,
        ));
    }

    #[test]
    fn final_owner_join_seal_rejects_each_owner_digest_substitution() {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("support");
        let mut receipt = CoveredParentOwnerJoinReceiptV1 {
            run_identity: Digest32::from_bytes([21; 32]),
            parent_interval_sha256: Digest32::from_bytes([20; 32]),
            parent_transaction_sha256: Digest32::from_bytes([22; 32]),
            segment_sha256: Digest32::from_bytes([23; 32]),
            accepted_slab_sha256: Digest32::from_bytes([24; 32]),
            forcing_receipt_sha256: Digest32::from_bytes([25; 32]),
            beginning_complete_owner_set_sha256: Digest32::from_bytes([26; 32]),
            ending_complete_owner_set_sha256: Digest32::from_bytes([27; 32]),
            support,
            final_boundary_receipt_set_sha256: Digest32::from_bytes([1; 32]),
            final_lane_boundary_receipt_set_sha256: Digest32::from_bytes([18; 32]),
            component_carrier_receipt_set_sha256: Digest32::from_bytes([2; 32]),
            snow_soil_heat_receipt_set_sha256: Digest32::from_bytes([19; 32]),
            terminal_snow_soil_heat_receipt_set_sha256: Digest32::from_bytes([30; 32]),
            physical_outcome_ledger_set_sha256: Digest32::from_bytes([31; 32]),
            wb14_child_receipt_set_sha256: Digest32::from_bytes([29; 32]),
            wb14_parent_receipt_set_sha256: None,
            stage3_physical_state_sha256: Digest32::from_bytes([3; 32]),
            vegetation_owner_sha256: Digest32::from_bytes([4; 32]),
            snow_owner_sha256: Digest32::from_bytes([5; 32]),
            land_surface_energy_owner_sha256: Digest32::from_bytes([6; 32]),
            hydrology_owner_sha256: Digest32::from_bytes([7; 32]),
            biogeochemistry_owner_sha256: Digest32::from_bytes([8; 32]),
            soil_thermal_owner_sha256: Digest32::from_bytes([9; 32]),
            surface_liquid_owner_sha256: Digest32::from_bytes([10; 32]),
            receipt_sha256: Digest32::zero(),
        };
        receipt.receipt_sha256 = receipt.reconstructed_digest().expect("join digest");
        receipt.validate_seal().expect("valid join seal");
        for mutate in [
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.ending_complete_owner_set_sha256 = Digest32::from_bytes([28; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.final_lane_boundary_receipt_set_sha256 = Digest32::from_bytes([19; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.vegetation_owner_sha256 = Digest32::from_bytes([11; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.snow_owner_sha256 = Digest32::from_bytes([12; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.land_surface_energy_owner_sha256 = Digest32::from_bytes([13; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.hydrology_owner_sha256 = Digest32::from_bytes([14; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.biogeochemistry_owner_sha256 = Digest32::from_bytes([15; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.soil_thermal_owner_sha256 = Digest32::from_bytes([16; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.surface_liquid_owner_sha256 = Digest32::from_bytes([17; 32]);
            },
        ] {
            let mut poisoned = receipt.clone();
            mutate(&mut poisoned);
            assert!(poisoned.validate_seal().is_err());
        }
    }

    #[test]
    fn valid_alternate_snow_owner_rejects_against_unchanged_physical_bytes() {
        let expected = b"canonical-stage3-and-boundaries";
        let expected_owner =
            V11OwnerEnvelope::try_new("snow".into(), expected.to_vec()).expect("snow owner");
        validate_exact_snow_owner_bytes(expected, &expected_owner).expect("exact snow join");

        let alternate = V11OwnerEnvelope::try_new(
            "snow".into(),
            b"different-valid-canonical-snow-owner".to_vec(),
        )
        .expect("alternate valid snow owner");
        assert!(validate_exact_snow_owner_bytes(expected, &alternate).is_err());
    }
}
