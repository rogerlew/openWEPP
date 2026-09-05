#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DirectV9V8SoilBeginningSourceV44 {
    ResidentAuthenticatedOwner,
    UnpublishedPhysicalCandidate,
    LegacyV1Owner,
}

fn direct_v9_select_v8_soil_beginning_v44<'a>(
    resident: &'a DirectSoilThermalResident,
    unpublished_candidate: Option<&'a DirectSoilThermalCandidate>,
    numerical_fixed_point: bool,
    requested_source: DirectV9V8SoilBeginningSourceV44,
    physical_only_or_v11: bool,
) -> Result<DirectSoilThermalReadView<'a>, DirectV9RealConsumerError> {
    match (
        numerical_fixed_point,
        unpublished_candidate,
        requested_source,
        physical_only_or_v11,
    ) {
        (true, Some(_), DirectV9V8SoilBeginningSourceV44::ResidentAuthenticatedOwner, _) => {
            Ok(resident.read_view())
        }
        (true, _, _, _) => Err(DirectV9RealConsumerError::Identity(
            "V44 projected soil must not enter Stage3-covered V8",
        )),
        (
            false,
            Some(candidate),
            DirectV9V8SoilBeginningSourceV44::UnpublishedPhysicalCandidate,
            _,
        ) => Ok(candidate.read_view()),
        (false, None, DirectV9V8SoilBeginningSourceV44::ResidentAuthenticatedOwner, true) => {
            Ok(resident.read_view())
        }
        (false, None, DirectV9V8SoilBeginningSourceV44::LegacyV1Owner, false) => {
            Ok(DirectSoilThermalReadView::V1(resident.v1()?))
        }
        _ => Err(DirectV9RealConsumerError::Identity(
            "V44 V8 soil beginning posture substitution",
        )),
    }
}

impl DirectV9RealConsumerShadow {
    /// Derive provider identity exclusively from canonical shadow owners and
    /// the live interval template.
    pub fn snow_free_provider_configuration(
        &self,
        template: &DirectV9ShadowDayInput,
    ) -> Result<SnowFreeHalfHourProviderConfiguration, DirectV9RealConsumerError> {
        let first = template
            .intervals
            .first()
            .ok_or(DirectV9RealConsumerError::Identity("shadow day intervals"))?;
        if template.intervals.len() != INTERVALS_PER_DAY {
            return Err(DirectV9RealConsumerError::Identity(
                "shadow day interval cardinality",
            ));
        }
        let mut destinations = Vec::new();
        for ofe in &self.lse_configuration.ofes {
            let wb14 = first
                .wb14_parameters
                .iter()
                .find(|value| value.ofe_id == ofe.ofe_id)
                .ok_or(DirectV9RealConsumerError::Identity(
                    "repository WB14 OFE binding",
                ))?;
            for tile in &ofe.tiles {
                destinations.push(SnowFreeHalfHourDestination {
                    ofe_id: ofe.ofe_id.as_str().to_string(),
                    tile_id: tile.tile_id.as_str().to_string(),
                    wb14_configuration_sha256: wb14_parameter_sha256(wb14),
                });
            }
        }
        Ok(SnowFreeHalfHourProviderConfiguration {
            run_id: self.hydrology_frame.identity.run_id.to_string(),
            co2_pa: first.vegetation_forcing.co2_pa,
            reference_height_m: first.vegetation_forcing.reference_height_m,
            gsi: first.vegetation_forcing.gsi,
            gsi_receipt_sha256: self.provider_gsi_receipt_sha256.clone(),
            destinations,
        })
    }

    /// Project a sealed repository forcing receipt into real Child-4 interval
    /// types while joining run, GSI-owner, and WB14-owner identity.
    pub fn project_repository_forcing_receipts(
        &self,
        provider: &ValidatedSnowFreeHalfHourForcingReceipts,
        template: DirectV9ShadowDayInput,
    ) -> Result<DirectV9ShadowDayInput, DirectV9RealConsumerError> {
        let expected_destinations = self
            .lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles.iter().map(|tile| {
                    (
                        ofe.ofe_id.as_str().to_string(),
                        tile.tile_id.as_str().to_string(),
                    )
                })
            })
            .collect();
        project_repository_forcing_receipts_to_v9_day(
            provider,
            template,
            self.hydrology_frame.identity.run_id,
            &self.provider_gsi_receipt_sha256,
            &expected_destinations,
        )
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub fn try_new(
        vegetation_configuration: VegetationConfiguration,
        vegetation_state: V9CoupledOwnedState,
        vegetation_owner_id: ResourceOwnerId,
        lse_configuration: LandSurfaceEnergyConfiguration,
        lse_state: LandSurfaceEnergyState,
        surface_configuration: DirectSurfaceLiquidConfiguration,
        layer_maps: Vec<RealHydrologyLaneLayerMap>,
        soil_thermal: SoilThermalSnapshot,
        biogeochemistry: BiogeochemistryState,
        hydrology_frame: DirectRunFrame,
        next_day_index: usize,
    ) -> Result<Self, DirectV9RealConsumerError> {
        let provider_gsi_receipt_sha256 = vegetation_state.0.state_sha256.clone();
        Self::try_new_with_authority(
            vegetation_configuration,
            vegetation_state,
            vegetation_owner_id,
            lse_configuration,
            lse_state,
            surface_configuration,
            layer_maps,
            DirectSoilThermalResident::try_new_v1(soil_thermal)?,
            biogeochemistry,
            hydrology_frame,
            next_day_index,
            CoveredColumnAuthority::HistoricalV8,
            provider_gsi_receipt_sha256,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn try_new_with_authority(
        vegetation_configuration: VegetationConfiguration,
        vegetation_state: V9CoupledOwnedState,
        vegetation_owner_id: ResourceOwnerId,
        lse_configuration: LandSurfaceEnergyConfiguration,
        lse_state: LandSurfaceEnergyState,
        surface_configuration: DirectSurfaceLiquidConfiguration,
        layer_maps: Vec<RealHydrologyLaneLayerMap>,
        soil_thermal: DirectSoilThermalResident,
        biogeochemistry: BiogeochemistryState,
        hydrology_frame: DirectRunFrame,
        next_day_index: usize,
        authority: CoveredColumnAuthority,
        provider_gsi_receipt_sha256: String,
    ) -> Result<Self, DirectV9RealConsumerError> {
        vegetation_state.validate(&vegetation_configuration)?;
        let (v8_configuration, v8_state) =
            project_v9_runtime_to_v8(&vegetation_configuration, &vegetation_state)?;
        lse_configuration.validate()?;
        lse_state.validate(&lse_configuration)?;
        soil_thermal.validate()?;
        if lse_configuration
            .vegetation_configuration
            .configuration_sha256
            .as_str()
            != v8_configuration.configuration_sha256
        {
            return Err(DirectV9RealConsumerError::Identity(
                "initial V9/V8/LSE configuration join",
            ));
        }
        if lse_state
            .last_accepted_transaction_id
            .is_some_and(|value| value.0 != v8_state.last_transaction_id)
        {
            return Err(DirectV9RealConsumerError::Identity(
                "initial vegetation/LSE transaction lineage",
            ));
        }
        if next_day_index >= hydrology_frame.identity.day_count
            || surface_configuration.run_id != hydrology_frame.identity.run_id
        {
            return Err(DirectV9RealConsumerError::Identity(
                "initial scheduler/surface owner identity",
            ));
        }
        let value = Self {
            authority,
            provider_gsi_receipt_sha256,
            vegetation_configuration,
            vegetation_state,
            vegetation_owner_id,
            lse_configuration,
            lse_state,
            surface_configuration,
            layer_maps,
            soil_thermal,
            biogeochemistry,
            hydrology_frame,
            next_day_index,
            accepted_interval_count: 0,
            wb14_parent_working_state: None,
            root_zone_hydraulic_configuration: None,
        };
        value.validate_complete_owner_set()?;
        Ok(value)
    }

    #[must_use]
    pub fn checkpoint(&self) -> DirectV9RealConsumerCheckpoint {
        DirectV9RealConsumerCheckpoint {
            shadow: self.clone(),
        }
    }

    pub fn wb14_parent_restart_bytes(&self) -> Result<Option<Vec<u8>>, DirectV9RealConsumerError> {
        self.wb14_parent_working_state
            .as_ref()
            .map(|state| state.restart_bytes(&self.surface_configuration))
            .transpose()
            .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))
    }

    pub fn restore_wb14_parent_restart_bytes(
        &mut self,
        bytes: Option<&[u8]>,
    ) -> Result<(), DirectV9RealConsumerError> {
        let restored = bytes
            .map(|bytes| {
                crate::direct_runtime::DirectWb14ParentWorkingState::from_restart_bytes(
                    &self.surface_configuration,
                    bytes,
                )
            })
            .transpose()
            .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))?;
        if let Some(restored) = &restored {
            let current = self
                .hydrology_frame
                .surface_liquid_shadow
                .as_deref()
                .ok_or(DirectV9RealConsumerError::Identity(
                    "missing restart receiving surface owner",
                ))?;
            restored
                .validate_receiving_owner(current)
                .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))?;
        }
        self.validate_complete_owner_set()?;
        self.wb14_parent_working_state = restored;
        Ok(())
    }

    pub fn restore(
        checkpoint: DirectV9RealConsumerCheckpoint,
    ) -> Result<Self, DirectV9RealConsumerError> {
        checkpoint.shadow.validate_complete_owner_set()?;
        Ok(checkpoint.shadow)
    }

    #[must_use]
    pub const fn next_day_index(&self) -> usize {
        self.next_day_index
    }

    #[must_use]
    pub const fn accepted_interval_count(&self) -> u64 {
        self.accepted_interval_count
    }

    #[must_use]
    pub const fn vegetation_state(&self) -> &V9CoupledOwnedState {
        &self.vegetation_state
    }

    #[must_use]
    pub const fn lse_state(&self) -> &LandSurfaceEnergyState {
        &self.lse_state
    }

    #[must_use]
    pub fn soil_thermal(&self) -> Result<&SoilThermalSnapshot, DirectV9RealConsumerError> {
        self.soil_thermal.v1()
    }

    #[must_use]
    pub const fn hydrology_frame(&self) -> &DirectRunFrame {
        &self.hydrology_frame
    }

    #[must_use]
    pub(crate) const fn surface_configuration(&self) -> &DirectSurfaceLiquidConfiguration {
        &self.surface_configuration
    }

    #[must_use]
    pub const fn biogeochemistry(&self) -> &BiogeochemistryState {
        &self.biogeochemistry
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_surface_configuration(
        &self,
    ) -> &DirectSurfaceLiquidConfiguration {
        &self.surface_configuration
    }

    pub(crate) fn execute_day(
        &mut self,
        production_frame: &DirectRunFrame,
        projected_day_frames: &[DirectDayFrame],
        projected_day_inputs: &[DirectPublicationDayInput],
        input: &DirectV9ShadowDayInput,
    ) -> Result<DirectV9ShadowDayReceipt, DirectV9RealConsumerError> {
        if input.day_index != self.next_day_index
            || input.day_index >= production_frame.identity.day_count
            || production_frame.identity != self.hydrology_frame.identity
        {
            return Err(DirectV9RealConsumerError::Identity(
                "scheduler day or production frame identity",
            ));
        }
        if input.intervals.len() != INTERVALS_PER_DAY {
            return Err(DirectV9RealConsumerError::Unsupported(
                "a shadow day requires exactly 48 intervals",
            ));
        }
        validate_repository_day_projection(
            production_frame,
            projected_day_frames,
            projected_day_inputs,
            input,
            &self.lse_configuration,
            &self.surface_configuration,
        )?;
        let beginning_shadow_diagnostic_fingerprint = self.diagnostic_fingerprint()?;
        let first_transaction_id = input.intervals[0].lse_forcing.transaction_id;
        let last_transaction_id = input.intervals[INTERVALS_PER_DAY - 1]
            .lse_forcing
            .transaction_id;
        let mut candidate = self.clone();
        for (interval_index, interval) in input.intervals.iter().enumerate() {
            candidate.execute_interval(input.day_index, interval_index, interval)?;
        }
        candidate.next_day_index = candidate
            .next_day_index
            .checked_add(1)
            .ok_or(DirectV9RealConsumerError::Identity("shadow day overflow"))?;
        candidate.validate_complete_owner_set()?;
        let ending_shadow_diagnostic_fingerprint = candidate.diagnostic_fingerprint()?;
        *self = candidate;
        Ok(DirectV9ShadowDayReceipt {
            day_index: input.day_index,
            accepted_interval_count: INTERVALS_PER_DAY,
            first_transaction_id,
            last_transaction_id,
            beginning_shadow_diagnostic_fingerprint,
            ending_shadow_diagnostic_fingerprint,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn execute_interval(
        &mut self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
    ) -> Result<(), DirectV9RealConsumerError> {
        let envelope = self.construct_snow_free_interval_envelope_with_duration(
            day_index,
            interval_index,
            input,
            INTERVAL_S,
            None,
        )?;
        self.accept_envelope(envelope.vegetation().transaction_id(), &envelope)
    }

    #[allow(clippy::too_many_lines)]
    fn construct_snow_free_interval_envelope_with_duration(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
        interval_s: f64,
        v11_duration_s_bits: Option<u64>,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
        self.construct_snow_free_parent_child_envelope_with_duration(
            day_index,
            interval_index,
            input,
            interval_s,
            v11_duration_s_bits,
            true,
            None,
        )
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn construct_snow_free_parent_child_envelope_with_duration(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
        interval_s: f64,
        v11_duration_s_bits: Option<u64>,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
        self.construct_canopy_soil_interval_envelope_with_duration(
            day_index,
            interval_index,
            input,
            interval_s,
            v11_duration_s_bits,
            None,
            false,
            None,
            finalize_wb14_parent_interval,
            wb14_coupled_child_binding,
            false,
            None,
            None,
        )
        .and_then(CanopySoilEvaluationV1::into_complete)
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn construct_snow_free_parent_child_envelope_with_duration_and_soil_beginning(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
        interval_s: f64,
        v11_duration_s_bits: Option<u64>,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
        unpublished_soil_candidate: Option<&DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<&DirectSoilThermalUnpublishedContinuationResultV2>,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
        self.construct_canopy_soil_interval_envelope_with_duration(
            day_index,
            interval_index,
            input,
            interval_s,
            v11_duration_s_bits,
            None,
            false,
            None,
            finalize_wb14_parent_interval,
            wb14_coupled_child_binding,
            false,
            unpublished_soil_candidate,
            unpublished_soil_continuation,
        )
        .and_then(CanopySoilEvaluationV1::into_complete)
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn construct_canopy_soil_interval_envelope_with_duration(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
        interval_s: f64,
        v11_duration_s_bits: Option<u64>,
        covered_lower_boundaries: Option<
            &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        >,
        provisional_v11: bool,
        covered_destinations: Option<&BTreeSet<(OfeId, TileId)>>,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
        physical_only: bool,
        unpublished_soil_candidate: Option<&DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<&DirectSoilThermalUnpublishedContinuationResultV2>,
    ) -> Result<CanopySoilEvaluationV1, DirectV9RealConsumerError> {
        let transaction_id = TransactionId(
            self.vegetation_state
                .0
                .last_transaction_id
                .checked_add(1)
                .ok_or(DirectV9RealConsumerError::Identity(
                    "vegetation transaction overflow",
                ))?,
        );
        let interval_index = u8::try_from(interval_index)
            .map_err(|_| DirectV9RealConsumerError::Identity("interval index overflow"))?;
        if input.lse_forcing.transaction_id != transaction_id {
            return Err(DirectV9RealConsumerError::Unsupported(
                "forcing transaction identity",
            ));
        }
        if input.lse_forcing.interval_s.to_bits() != interval_s.to_bits()
            || v11_duration_s_bits.is_some_and(|bits| bits != interval_s.to_bits())
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "forcing cadence identity",
            ));
        }
        if input.lse_forcing.snow_present_at_beginning
            || input.lse_forcing.snow_present_at_end
            || input.lse_forcing.snow_terminal_payload_present
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "forcing snow domain",
            ));
        }
        if !input.lse_forcing.runon_parcels.is_empty() {
            return Err(DirectV9RealConsumerError::Unsupported(
                "runon requires an accepted routing publication owner",
            ));
        }
        input.lse_forcing.validate(transaction_id)?;
        let (v8_configuration, v8_beginning) =
            project_v9_runtime_to_v8(&self.vegetation_configuration, &self.vegetation_state)?;
        if self
            .lse_configuration
            .vegetation_configuration
            .configuration_sha256
            .as_str()
            != v8_configuration.configuration_sha256
        {
            return Err(DirectV9RealConsumerError::Identity(
                "V9/V8/LSE configuration join",
            ));
        }
        let mut effective_hydrology_frame = self.hydrology_frame.clone();
        if let Some(parent) = &self.wb14_parent_working_state {
            effective_hydrology_frame.surface_liquid_shadow =
                Some(Box::new(parent.candidate_state().clone()));
        }
        let hydrology = RealHydrologyShadowAdapter::try_from_day_start(
            &effective_hydrology_frame,
            day_index,
            transaction_id,
            interval_s,
            self.surface_configuration.owner_id.clone(),
            &self.layer_maps,
        )?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
        let fixed_point_history = match (unpublished_soil_candidate, wb14_coupled_child_binding) {
            (Some(candidate), Some(binding)) => {
                self.soil_thermal.validate_unpublished_fixed_point_v2(
                    &self.lse_configuration,
                    candidate,
                    unpublished_soil_continuation,
                    binding.child_support_start_ns,
                    binding.child_support_end_ns,
                )?
            }
            (Some(candidate), None) if candidate.v2().is_ok() => {
                return Err(DirectV9RealConsumerError::Identity(
                    "native V2 unpublished physical support binding",
                ));
            }
            _ => false,
        };
        let unpublished_physical_beginning = if fixed_point_history {
            None
        } else {
            match unpublished_soil_candidate {
                Some(candidate) => {
                    let binding =
                        wb14_coupled_child_binding.ok_or(DirectV9RealConsumerError::Identity(
                            "native V2 unpublished physical support binding",
                        ))?;
                    Some(
                        self.soil_thermal
                            .prepare_unpublished_physical_beginning_v2(
                                &self.lse_configuration,
                                candidate,
                                unpublished_soil_continuation,
                                binding.child_support_start_ns,
                                binding.child_support_end_ns,
                            )?,
                    )
                }
                None if unpublished_soil_continuation.is_some() => {
                    return Err(DirectV9RealConsumerError::Identity(
                        "native V2 unpublished continuation without candidate",
                    ));
                }
                None => None,
            }
        };
        let v8_soil_beginning_source = if fixed_point_history {
            DirectV9V8SoilBeginningSourceV44::ResidentAuthenticatedOwner
        } else if let Some(candidate) = unpublished_soil_candidate {
            let _ = candidate;
            DirectV9V8SoilBeginningSourceV44::UnpublishedPhysicalCandidate
        } else if physical_only || v11_duration_s_bits.is_some() {
            DirectV9V8SoilBeginningSourceV44::ResidentAuthenticatedOwner
        } else {
            DirectV9V8SoilBeginningSourceV44::LegacyV1Owner
        };
        let soil_thermal = direct_v9_select_v8_soil_beginning_v44(
            &self.soil_thermal,
            unpublished_soil_candidate,
            fixed_point_history,
            v8_soil_beginning_source,
            physical_only || v11_duration_s_bits.is_some(),
        )?;
        soil_thermal.validate()?;
        let soil_thermal_snapshot_sha256 = match unpublished_physical_beginning.as_ref() {
            Some(beginning) => beginning
                .predecessor_trial()
                .unpublished_trial_sha256()
                .clone(),
            None => match soil_thermal {
                DirectSoilThermalReadView::V1(beginning) => beginning.snapshot_sha256.clone(),
                DirectSoilThermalReadView::V2(_) => {
                    self.soil_thermal
                        .v2()?
                        .owner()
                        .snapshot()
                        .map_err(|_| {
                            DirectV9RealConsumerError::OwnerClosure("V2 soil snapshot identity")
                        })?
                        .snapshot_sha256
                }
            },
        };
        let hydrology_snapshot = unified_beginning_hydrology_snapshot_sha256(
            &soil_adapter,
            &self.surface_configuration,
        )?;
        let forcing_sha256 = input.lse_forcing.canonical_sha256()?;
        let (vegetation_forcing, root_zone_hydraulics) = project_live_vegetation_forcing(
            &input.vegetation_forcing,
            &hydrology,
            soil_thermal,
            self.root_zone_hydraulic_configuration.as_ref(),
            &self.surface_configuration,
            &self.lse_configuration,
            &self.vegetation_configuration,
            &self.vegetation_state,
            v8_configuration.configuration_sha256.clone(),
            hydrology_snapshot.clone(),
            transaction_id,
            day_index,
            interval_index,
        )?;
        let canopy_forcing = match root_zone_hydraulics {
            Some(receipts) => V8CanopyForcingReceipt::try_new_with_root_zone(
                v8_configuration.configuration_sha256.clone(),
                v8_beginning.state_sha256.clone(),
                self.lse_configuration.configuration_sha256.clone(),
                forcing_sha256,
                hydrology_snapshot,
                soil_thermal_snapshot_sha256,
                transaction_id,
                vegetation_forcing,
                receipts,
            )?,
            None => V8CanopyForcingReceipt::try_new(
                v8_configuration.configuration_sha256.clone(),
                v8_beginning.state_sha256.clone(),
                self.lse_configuration.configuration_sha256.clone(),
                forcing_sha256,
                hydrology_snapshot,
                soil_thermal_snapshot_sha256,
                transaction_id,
                vegetation_forcing,
            )?,
        };
        if physical_only {
            return self.construct_canopy_soil_physical_only_envelope(
                day_index,
                interval_index,
                input,
                v11_duration_s_bits,
                covered_destinations,
                covered_lower_boundaries,
                finalize_wb14_parent_interval,
                wb14_coupled_child_binding,
                &v8_configuration,
                &v8_beginning,
                &hydrology,
                soil_thermal,
                unpublished_physical_beginning.as_ref(),
                &canopy_forcing,
            );
        }
        self.construct_canopy_soil_complete_envelope(
            day_index,
            interval_index,
            input,
            v11_duration_s_bits,
            covered_destinations,
            covered_lower_boundaries,
            provisional_v11,
            finalize_wb14_parent_interval,
            wb14_coupled_child_binding,
            &v8_configuration,
            &v8_beginning,
            &hydrology,
            soil_thermal,
            unpublished_physical_beginning.as_ref(),
            &canopy_forcing,
        )
    }

    #[allow(clippy::too_many_arguments)]
    #[inline(never)]
    fn construct_canopy_soil_physical_only_envelope(
        &self,
        day_index: usize,
        interval_index: u8,
        input: &DirectV9ShadowIntervalInput,
        v11_duration_s_bits: Option<u64>,
        covered_destinations: Option<&BTreeSet<(OfeId, TileId)>>,
        covered_lower_boundaries: Option<
            &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        >,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
        v8_configuration: &VegetationConfiguration,
        v8_beginning: &V8CoupledOwnedState,
        hydrology: &RealHydrologyShadowAdapter,
        soil_thermal: DirectSoilThermalReadView<'_>,
        unpublished_physical_beginning: Option<
            &openwepp_land_surface_energy::SoilThermalUnpublishedPhysicalBeginningV2,
        >,
        canopy_forcing: &V8CanopyForcingReceipt,
    ) -> Result<CanopySoilEvaluationV1, DirectV9RealConsumerError> {
        let nitrogen = BiogeochemistryNitrogenArbiter::try_new(&self.biogeochemistry)?;
        let bits = v11_duration_s_bits.ok_or(DirectV9RealConsumerError::Identity(
            "physical-only endpoint requires V11 duration",
        ))?;
        let destinations = covered_destinations.ok_or(DirectV9RealConsumerError::Identity(
            "physical-only covered endpoint destination set",
        ))?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(hydrology);
        let soil_thermal = match (soil_thermal, unpublished_physical_beginning) {
            (DirectSoilThermalReadView::V2(_), Some(beginning)) => {
                crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::try_from_v2_unpublished(
                    beginning.clone(),
                )?
            }
            (DirectSoilThermalReadView::V1(_), Some(_)) => {
                return Err(DirectV9RealConsumerError::Identity(
                    "unpublished V2 beginning with V1 read view",
                ));
            }
            (DirectSoilThermalReadView::V1(beginning), None) => {
                crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::try_from_v1(
                    beginning,
                )?
            }
            (DirectSoilThermalReadView::V2(_), None) => {
                let binding = wb14_coupled_child_binding.ok_or(
                    DirectV9RealConsumerError::Identity("native V2 physical support binding"),
                )?;
                let prepared = self.soil_thermal.prepare_next_v2_support(
                    binding.child_support_start_ns,
                    binding.child_support_end_ns,
                )?;
                crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::try_from_v2(
                    &prepared,
                )?
            }
        };
        let physical = execute_v8_lse_runtime_shadow_v11_physical_with_carriers(
            v8_configuration,
            v8_beginning,
            &self.vegetation_owner_id,
            canopy_forcing,
            &self.lse_configuration,
            &self.lse_state,
            &input.lse_forcing,
            &soil_adapter,
            &self.surface_configuration,
            day_index,
            interval_index,
            &input.wb14_parameters,
            &soil_thermal,
            &nitrogen,
            &self.biogeochemistry,
            openwepp_land_surface_energy::CoveredColumnAuthority::V11SnowCovered,
            covered_lower_boundaries,
            bits,
            Some(destinations),
            finalize_wb14_parent_interval,
            self.wb14_parent_working_state.as_ref(),
            wb14_coupled_child_binding,
        )?;
        Ok(CanopySoilEvaluationV1::PhysicalOnly(
            ProvisionalCoveredV8PhysicalEvaluationV1::try_new(physical)?,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    #[inline(never)]
    fn construct_canopy_soil_complete_envelope(
        &self,
        day_index: usize,
        interval_index: u8,
        input: &DirectV9ShadowIntervalInput,
        v11_duration_s_bits: Option<u64>,
        covered_destinations: Option<&BTreeSet<(OfeId, TileId)>>,
        covered_lower_boundaries: Option<
            &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        >,
        provisional_v11: bool,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
        v8_configuration: &VegetationConfiguration,
        v8_beginning: &V8CoupledOwnedState,
        hydrology: &RealHydrologyShadowAdapter,
        soil_thermal: DirectSoilThermalReadView<'_>,
        unpublished_physical_beginning: Option<
            &openwepp_land_surface_energy::SoilThermalUnpublishedPhysicalBeginningV2,
        >,
        canopy_forcing: &V8CanopyForcingReceipt,
    ) -> Result<CanopySoilEvaluationV1, DirectV9RealConsumerError> {
        let nitrogen = BiogeochemistryNitrogenArbiter::try_new(&self.biogeochemistry)?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(hydrology);
        if let Some(bits) = v11_duration_s_bits {
            let native_soil_thermal = match (soil_thermal, unpublished_physical_beginning) {
                (DirectSoilThermalReadView::V2(_), Some(beginning)) => {
                    crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::try_from_v2_unpublished(
                        beginning.clone(),
                    )?
                }
                (DirectSoilThermalReadView::V1(_), Some(_)) => {
                    return Err(DirectV9RealConsumerError::Identity(
                        "unpublished V2 beginning with V1 read view",
                    ));
                }
                (DirectSoilThermalReadView::V1(beginning), None) => {
                    crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::try_from_v1(
                        beginning,
                    )?
                }
                (DirectSoilThermalReadView::V2(_), None) => {
                    let binding = wb14_coupled_child_binding.ok_or(
                        DirectV9RealConsumerError::Identity("native V2 V11 support binding"),
                    )?;
                    let prepared = self.soil_thermal.prepare_next_v2_support(
                        binding.child_support_start_ns,
                        binding.child_support_end_ns,
                    )?;
                    crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning::try_from_v2(
                        &prepared,
                    )?
                }
            };
            match covered_destinations {
                Some(destinations) => self.construct_canopy_soil_complete_v11_with_carriers(
                    day_index,
                    interval_index,
                    input,
                    bits,
                    destinations,
                    covered_lower_boundaries,
                    provisional_v11,
                    finalize_wb14_parent_interval,
                    wb14_coupled_child_binding,
                    v8_configuration,
                    v8_beginning,
                    &soil_adapter,
                    &native_soil_thermal,
                    canopy_forcing,
                    &nitrogen,
                ),
                None => self.construct_canopy_soil_complete_v11(
                    day_index,
                    interval_index,
                    input,
                    bits,
                    covered_lower_boundaries,
                    provisional_v11,
                    finalize_wb14_parent_interval,
                    wb14_coupled_child_binding,
                    v8_configuration,
                    v8_beginning,
                    &soil_adapter,
                    &native_soil_thermal,
                    canopy_forcing,
                    &nitrogen,
                ),
            }
        } else {
            let soil_thermal = match soil_thermal {
                DirectSoilThermalReadView::V1(beginning) => beginning,
                DirectSoilThermalReadView::V2(_) => {
                    return Err(DirectV9RealConsumerError::Unsupported(
                        "historical V8 endpoint on V2 soil resident",
                    ));
                }
            };
            self.construct_canopy_soil_complete_v8(
                day_index,
                interval_index,
                input,
                v8_configuration,
                v8_beginning,
                &soil_adapter,
                soil_thermal,
                canopy_forcing,
                &nitrogen,
            )
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[inline(never)]
    fn construct_canopy_soil_complete_v11_with_carriers(
        &self,
        day_index: usize,
        interval_index: u8,
        input: &DirectV9ShadowIntervalInput,
        duration_s_bits: u64,
        destinations: &BTreeSet<(OfeId, TileId)>,
        covered_lower_boundaries: Option<
            &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        >,
        provisional_v11: bool,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
        v8_configuration: &VegetationConfiguration,
        v8_beginning: &V8CoupledOwnedState,
        soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
        soil_thermal: &crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning,
        canopy_forcing: &V8CanopyForcingReceipt,
        nitrogen: &BiogeochemistryNitrogenArbiter,
    ) -> Result<CanopySoilEvaluationV1, DirectV9RealConsumerError> {
        let envelope = execute_v8_lse_runtime_shadow_v11_with_native_soil_beginning(
            v8_configuration,
            v8_beginning,
            &self.vegetation_owner_id,
            canopy_forcing,
            &self.lse_configuration,
            &self.lse_state,
            &input.lse_forcing,
            soil_adapter,
            &self.surface_configuration,
            day_index,
            interval_index,
            &input.wb14_parameters,
            soil_thermal,
            nitrogen,
            &self.biogeochemistry,
            // This entry point is the typed V11 snow-covered carrier path
            // even for unpublished probe envelopes.
            openwepp_land_surface_energy::CoveredColumnAuthority::V11SnowCovered,
            covered_lower_boundaries,
            duration_s_bits,
            !provisional_v11,
            Some(destinations),
            finalize_wb14_parent_interval,
            self.wb14_parent_working_state.as_ref(),
            wb14_coupled_child_binding,
        )?;
        Ok(CanopySoilEvaluationV1::Complete(envelope))
    }

    #[allow(clippy::too_many_arguments)]
    #[inline(never)]
    fn construct_canopy_soil_complete_v11(
        &self,
        day_index: usize,
        interval_index: u8,
        input: &DirectV9ShadowIntervalInput,
        duration_s_bits: u64,
        covered_lower_boundaries: Option<
            &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        >,
        provisional_v11: bool,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
        v8_configuration: &VegetationConfiguration,
        v8_beginning: &V8CoupledOwnedState,
        soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
        soil_thermal: &crate::land_surface_energy_shadow::V8SoilThermalPhysicalBeginning,
        canopy_forcing: &V8CanopyForcingReceipt,
        nitrogen: &BiogeochemistryNitrogenArbiter,
    ) -> Result<CanopySoilEvaluationV1, DirectV9RealConsumerError> {
        let envelope = execute_v8_lse_runtime_shadow_v11_with_native_soil_beginning(
            v8_configuration,
            v8_beginning,
            &self.vegetation_owner_id,
            canopy_forcing,
            &self.lse_configuration,
            &self.lse_state,
            &input.lse_forcing,
            soil_adapter,
            &self.surface_configuration,
            day_index,
            interval_index,
            &input.wb14_parameters,
            soil_thermal,
            nitrogen,
            &self.biogeochemistry,
            self.authority,
            covered_lower_boundaries,
            duration_s_bits,
            !provisional_v11,
            None,
            finalize_wb14_parent_interval,
            self.wb14_parent_working_state.as_ref(),
            wb14_coupled_child_binding,
        )?;
        Ok(CanopySoilEvaluationV1::Complete(envelope))
    }

    #[allow(clippy::too_many_arguments)]
    #[inline(never)]
    fn construct_canopy_soil_complete_v8(
        &self,
        day_index: usize,
        interval_index: u8,
        input: &DirectV9ShadowIntervalInput,
        v8_configuration: &VegetationConfiguration,
        v8_beginning: &V8CoupledOwnedState,
        soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
        soil_thermal: &SoilThermalSnapshot,
        canopy_forcing: &V8CanopyForcingReceipt,
        nitrogen: &BiogeochemistryNitrogenArbiter,
    ) -> Result<CanopySoilEvaluationV1, DirectV9RealConsumerError> {
        let envelope = execute_v8_lse_runtime_shadow_internal(
            v8_configuration,
            v8_beginning,
            &self.vegetation_owner_id,
            canopy_forcing,
            &self.lse_configuration,
            &self.lse_state,
            &input.lse_forcing,
            soil_adapter,
            &self.surface_configuration,
            day_index,
            interval_index,
            &input.wb14_parameters,
            soil_thermal,
            nitrogen,
            &self.biogeochemistry,
            None,
            self.authority,
        )?;
        Ok(CanopySoilEvaluationV1::Complete(envelope))
    }

    /// Construct the V11 canopy/soil envelope for a Child-2C covered slab.
    ///
    /// Snow is not admitted to the snow-free LSE owner. The Stage-3 snow
    /// column and the canopy/snow air carrier are evaluated and sealed by the
    /// covered adopter before this projection. The V8/LSE endpoint here is
    /// consequently a typed canopy/soil continuation with the carrier's
    /// shared air state; it is not the snow-free lower-boundary selector.
    #[allow(clippy::too_many_arguments)]
    fn construct_covered_interval_envelope_with_duration_and_soil_beginning(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV11SnowCoveredSegmentInput,
        interval_s: f64,
        v11_duration_s_bits: u64,
        covered_destinations: &BTreeSet<(OfeId, TileId)>,
        lower_boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        provisional_v11: bool,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        unpublished_soil_candidate: Option<&DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<&DirectSoilThermalUnpublishedContinuationResultV2>,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
        if !input.lse_forcing.snow_present_at_end || input.lse_forcing.snow_terminal_payload_present
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "covered adopter requires persistent snow operands",
            ));
        }
        // The carrier owns the snow surface. The LSE endpoint receives only
        // shared air state by keyed destination; no parent aggregate is used.
        let mut canopy_soil_forcing = input.lse_forcing.clone();
        canopy_soil_forcing.snow_present_at_beginning = false;
        canopy_soil_forcing.snow_present_at_end = false;
        canopy_soil_forcing.forcing_sha256 = canopy_soil_forcing.canonical_sha256()?;
        let covered_vegetation_forcing = input.vegetation_forcing.clone();
        let covered_input = DirectV9ShadowIntervalInput {
            lse_forcing: canopy_soil_forcing,
            vegetation_forcing: covered_vegetation_forcing,
            wb14_parameters: input.wb14_parameters.clone(),
        };
        self.construct_canopy_soil_interval_envelope_with_duration(
            day_index,
            interval_index,
            &covered_input,
            interval_s,
            Some(v11_duration_s_bits),
            Some(lower_boundaries),
            provisional_v11,
            Some(covered_destinations),
            finalize_wb14_parent_interval,
            Some(wb14_coupled_child_binding),
            false,
            unpublished_soil_candidate,
            unpublished_soil_continuation,
        )
        .and_then(CanopySoilEvaluationV1::into_complete)
    }

    fn prepare_covered_canopy_soil_input(
        input: &DirectV11SnowCoveredSegmentInput,
    ) -> Result<PreparedCoveredCanopySoilInputV1, DirectV9RealConsumerError> {
        if !input.lse_forcing.snow_present_at_end || input.lse_forcing.snow_terminal_payload_present
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "covered adopter requires persistent snow operands",
            ));
        }
        let mut canopy_soil_forcing = input.lse_forcing.clone();
        canopy_soil_forcing.snow_present_at_beginning = false;
        canopy_soil_forcing.snow_present_at_end = false;
        canopy_soil_forcing.forcing_sha256 = canopy_soil_forcing.canonical_sha256()?;
        let covered_input = DirectV9ShadowIntervalInput {
            lse_forcing: canopy_soil_forcing,
            vegetation_forcing: input.vegetation_forcing.clone(),
            wb14_parameters: input.wb14_parameters.clone(),
        };
        Ok(PreparedCoveredCanopySoilInputV1(covered_input))
    }

    #[allow(clippy::too_many_arguments)]
    fn construct_prepared_covered_interval_physical_with_duration_and_soil_beginning(
        &self,
        day_index: usize,
        interval_index: usize,
        prepared: &PreparedCoveredCanopySoilInputV1,
        interval_s: f64,
        v11_duration_s_bits: u64,
        covered_destinations: &BTreeSet<(OfeId, TileId)>,
        lower_boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        unpublished_soil_candidate: Option<&DirectSoilThermalCandidate>,
        unpublished_soil_continuation: Option<&DirectSoilThermalUnpublishedContinuationResultV2>,
    ) -> Result<ProvisionalCoveredV8PhysicalEvaluationV1, DirectV9RealConsumerError> {
        self.construct_canopy_soil_interval_envelope_with_duration(
            day_index,
            interval_index,
            &prepared.0,
            interval_s,
            Some(v11_duration_s_bits),
            Some(lower_boundaries),
            true,
            Some(covered_destinations),
            finalize_wb14_parent_interval,
            Some(wb14_coupled_child_binding),
            true,
            unpublished_soil_candidate,
            unpublished_soil_continuation,
        )
        .and_then(CanopySoilEvaluationV1::into_physical)
    }

    fn accept_envelope(
        &mut self,
        transaction_id: TransactionId,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
    ) -> Result<(), DirectV9RealConsumerError> {
        envelope.validate()?;
        let vegetation_state = project_v8_runtime_to_v9(
            envelope.vegetation().ending_state(),
            &self.vegetation_configuration,
        )?;
        let lse_state = build_lse_ending_state(
            &self.lse_state,
            transaction_id,
            envelope.hydrology().ending_lse_tile_states().to_vec(),
        )?;
        let soil_thermal = aggregate_soil_thermal_ending(
            self.soil_thermal.v1()?,
            &self.lse_configuration,
            transaction_id,
            envelope.hydrology().soil_thermal_candidates(),
        )?;
        self.vegetation_state = vegetation_state;
        self.lse_state = lse_state;
        self.soil_thermal = DirectSoilThermalResident::try_new_v1(soil_thermal)?;
        self.biogeochemistry = envelope.biogeochemistry().ending().clone();
        self.hydrology_frame = envelope.hydrology().ending_frame().clone();
        self.wb14_parent_working_state = envelope
            .hydrology()
            .surface_ingress()
            .parent_working_state()
            .cloned();
        if envelope
            .hydrology()
            .surface_ingress()
            .advances_persistent_parent_interval()
        {
            self.accepted_interval_count = self.accepted_interval_count.checked_add(1).ok_or(
                DirectV9RealConsumerError::Identity("accepted parent interval count overflow"),
            )?;
        }
        Ok(())
    }

    /// Stage every accepted snow-free owner except native V2 soil thermal.
    ///
    /// The caller owns an authenticated unpublished sequential soil chain and
    /// installs its single outer accepted resident immediately after this
    /// method returns. This method never projects or replaces that resident;
    /// callers operate on an isolated candidate so any later refusal rolls the
    /// complete owner set back byte-for-byte.
    fn accept_envelope_preserving_native_v2_soil(
        &mut self,
        transaction_id: TransactionId,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
    ) -> Result<(), DirectV9RealConsumerError> {
        envelope.validate()?;
        self.soil_thermal.v2()?.validate()?;
        let vegetation_state = project_v8_runtime_to_v9(
            envelope.vegetation().ending_state(),
            &self.vegetation_configuration,
        )?;
        let lse_state = build_lse_ending_state(
            &self.lse_state,
            transaction_id,
            envelope.hydrology().ending_lse_tile_states().to_vec(),
        )?;
        self.vegetation_state = vegetation_state;
        self.lse_state = lse_state;
        self.biogeochemistry = envelope.biogeochemistry().ending().clone();
        self.hydrology_frame = envelope.hydrology().ending_frame().clone();
        self.wb14_parent_working_state = envelope
            .hydrology()
            .surface_ingress()
            .parent_working_state()
            .cloned();
        if envelope
            .hydrology()
            .surface_ingress()
            .advances_persistent_parent_interval()
        {
            self.accepted_interval_count = self.accepted_interval_count.checked_add(1).ok_or(
                DirectV9RealConsumerError::Identity("accepted parent interval count overflow"),
            )?;
        }
        Ok(())
    }

    fn accept_envelope_with_soil_top_boundary_credits(
        &mut self,
        transaction_id: TransactionId,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
        credits: &[SoilThermalTopBoundaryCreditV1],
    ) -> Result<SoilThermalTopBoundaryCreditSetV1, DirectV9RealConsumerError> {
        for credit in credits {
            if credit.snow_soil_heat_receipt_sha256.as_str().len() != 64 {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "snow-soil receipt digest encoding",
                ));
            }
        }
        let beginning_soil = self.soil_thermal.v1()?.clone();
        self.accept_envelope(transaction_id, envelope)?;
        let accepted = aggregate_soil_thermal_ending_with_top_boundary_credits(
            &beginning_soil,
            &self.lse_configuration,
            transaction_id,
            envelope.hydrology().soil_thermal_candidates(),
            credits,
        )?;
        self.soil_thermal = DirectSoilThermalResident::try_new_v1(accepted.ending.clone())?;
        Ok(accepted)
    }

    fn validate_complete_owner_set(&self) -> Result<(), DirectV9RealConsumerError> {
        self.vegetation_state
            .validate(&self.vegetation_configuration)?;
        self.lse_state.validate(&self.lse_configuration)?;
        self.soil_thermal.validate()?;
        let transaction_id = TransactionId(self.vegetation_state.0.last_transaction_id);
        let lse_transaction_matches = self
            .lse_state
            .last_accepted_transaction_id
            .is_none_or(|value| value == transaction_id);
        let soil_transaction_matches = self
            .soil_thermal
            .last_accepted_transaction_id()
            .is_none_or(|value| value == transaction_id);
        let complete_accepted_lineage = self.accepted_interval_count == 0
            || (self.lse_state.last_accepted_transaction_id == Some(transaction_id)
                && self.soil_thermal.last_accepted_transaction_id() == Some(transaction_id));
        let mapping_matches = self
            .surface_configuration
            .ofe_bindings
            .iter()
            .zip(&self.layer_maps)
            .all(|(binding, map)| {
                binding.production_lane_index == map.ofe_lane.lane_index
                    && binding.production_lane_id == map.ofe_lane.lane_id
                    && binding.ordered_soil_layer_ids == map.layer_ids
            });
        let invalid_coordinate = if self.surface_configuration.ofe_bindings.len()
            != self.hydrology_frame.lanes.len()
        {
            Some("incomplete or mixed complete-owner state: surface/hydrology OFE cardinality")
        } else if self.layer_maps.len() != self.hydrology_frame.lanes.len() {
            Some("incomplete or mixed complete-owner state: soil/hydrology lane cardinality")
        } else if self.biogeochemistry.last_transaction_id
            != self.vegetation_state.0.last_transaction_id
        {
            Some("incomplete or mixed complete-owner state: biogeochemistry/vegetation transaction")
        } else if !lse_transaction_matches {
            Some("incomplete or mixed complete-owner state: land-surface-energy/vegetation transaction")
        } else if !soil_transaction_matches {
            Some("incomplete or mixed complete-owner state: soil-thermal/vegetation transaction")
        } else if !complete_accepted_lineage {
            Some("incomplete or mixed complete-owner state: accepted interval complete lineage")
        } else if !mapping_matches {
            Some("incomplete or mixed complete-owner state: surface/soil lane mapping")
        } else {
            None
        };
        if let Some(coordinate) = invalid_coordinate {
            return Err(DirectV9RealConsumerError::Identity(coordinate));
        }
        Ok(())
    }

    fn diagnostic_fingerprint(&self) -> Result<String, DirectV9RealConsumerError> {
        #[derive(Serialize)]
        struct ShadowBytes<'a> {
            vegetation: &'a V9CoupledOwnedState,
            lse: &'a LandSurfaceEnergyState,
            soil_thermal: &'a DirectSoilThermalResident,
            biogeochemistry: &'a BiogeochemistryState,
            hydrology_debug: String,
            next_day_index: usize,
            accepted_interval_count: u64,
        }
        let bytes = serde_json::to_vec(&ShadowBytes {
            vegetation: &self.vegetation_state,
            lse: &self.lse_state,
            soil_thermal: &self.soil_thermal,
            biogeochemistry: &self.biogeochemistry,
            hydrology_debug: format!("{:?}", self.hydrology_frame),
            next_day_index: self.next_day_index,
            accepted_interval_count: self.accepted_interval_count,
        })
        .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
}
