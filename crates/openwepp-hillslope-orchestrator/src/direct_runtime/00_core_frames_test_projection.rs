/// Canonical test-only projection of a hydrology candidate's full frame
/// identity. `canonical_beginning_snapshot_sha256` binds every immutable
/// field in the original production frame; the remainder exhaustively frames
/// every field that the soil/surface hydrology candidate may change.
impl DirectRunFrame {
    // The projection is intentionally explicit and exhaustive: splitting it would
    // make omissions across the opaque frame fields harder to audit.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn canonical_hydrology_physical_projection_v1(
        &self,
        canonical_beginning_snapshot_sha256: &str,
    ) -> Result<Vec<u8>, serde_json::Error> {
        fn bytes(out: &mut Vec<u8>, value: &[u8]) {
            out.extend_from_slice(&(value.len() as u64).to_be_bytes());
            out.extend_from_slice(value);
        }
        fn float(out: &mut Vec<u8>, value: f64) {
            out.extend_from_slice(&value.to_bits().to_be_bytes());
        }
        fn optional_float(out: &mut Vec<u8>, value: Option<f64>) {
            if let Some(value) = value {
                out.push(1);
                float(out, value);
            } else {
                out.push(0);
            }
        }
        fn publication(out: &mut Vec<u8>, value: &DirectPublicationFrame) {
            for field in [
                value.runoff_m,
                value.infiltration_m,
                value.evapotranspiration_m,
                value.drainage_m,
                value.lateral_flow_m,
            ] {
                float(out, field);
            }
        }
        fn groundwater(out: &mut Vec<u8>, value: DirectGroundwaterRunState) {
            match value.authority {
                DirectGroundwaterAuthority::Disabled => out.push(0),
                DirectGroundwaterAuthority::LinearReservoir {
                    initial_storage_depth_m,
                    baseflow_coeff_per_day,
                    deep_seepage_coeff_per_day,
                    baseflow_threshold_area_ha,
                } => {
                    out.push(1);
                    for field in [
                        initial_storage_depth_m,
                        baseflow_coeff_per_day,
                        deep_seepage_coeff_per_day,
                        baseflow_threshold_area_ha,
                    ] {
                        float(out, field);
                    }
                }
            }
            for field in [
                value.storage_m3,
                value.previous_baseflow_m3,
                value.previous_deep_seepage_m3,
            ] {
                float(out, field);
            }
            optional_float(out, value.initialized_area_m2);
        }

        let mut out = b"OPENWEPP_DIRECT_HYDROLOGY_PHYSICAL_FRAME_PROJECTION_V1\0".to_vec();
        bytes(&mut out, canonical_beginning_snapshot_sha256.as_bytes());
        out.extend_from_slice(&self.identity.run_id.to_be_bytes());
        out.extend_from_slice(&self.identity.hillslope_id.to_be_bytes());
        out.extend_from_slice(&(self.identity.lane_count as u64).to_be_bytes());
        out.extend_from_slice(&(self.identity.day_count as u64).to_be_bytes());
        out.extend_from_slice(&(self.lanes.len() as u64).to_be_bytes());
        for lane in &self.lanes {
            out.extend_from_slice(&lane.lane_id.to_be_bytes());
            out.extend_from_slice(&lane.upstream_lane_id.to_be_bytes());
            out.extend_from_slice(&lane.downstream_lane_id.to_be_bytes());
            for value in [
                lane.upstream_area_ratio,
                lane.area_m2,
                lane.runoff_publication_q_scale,
                lane.runoff_publication_qofe_scale,
                lane.runoff_publication_efflen_m,
                lane.runoff_publication_cumulative_length_m,
                lane.runoff_publication_ofe_length_m,
                lane.water.soil_water_m,
                lane.water.infiltration_m,
                lane.water.runoff_m,
                lane.water.evapotranspiration_m,
                lane.water.drainage_m,
                lane.water.lateral_flow_m,
            ] {
                float(&mut out, value);
            }
            for values in [
                &lane.transfer.surface_carry_m,
                &lane.transfer.surface_hourly_weights,
                &lane.transfer.lateral_carry_m,
            ] {
                for value in values {
                    float(&mut out, *value);
                }
            }
            float(&mut out, lane.transfer.upstream_flow_m);
            float(&mut out, lane.transfer.subsurface_input_m);
            publication(&mut out, &lane.publication);
            out.extend_from_slice(&(lane.subsurface_layers.len() as u64).to_be_bytes());
            for layer in &lane.subsurface_layers {
                for value in [
                    layer.theta_m,
                    layer.field_capacity_m,
                    layer.upper_limit_m,
                    layer.conductivity_m_s,
                    layer.depth_m,
                    layer.residual_theta,
                    layer.frozen_depth_m,
                    layer.frozen_water_m,
                    layer.porosity,
                    layer.field_capacity_theta,
                    layer.coca,
                    layer.lateral_conductivity_m_s,
                ] {
                    float(&mut out, value);
                }
            }
        }
        for phase in self.phase_plan.phases() {
            out.extend_from_slice(&(*phase as u32).to_be_bytes());
        }
        publication(&mut out, &self.publication);
        out.extend_from_slice(&(self.lane_transfer_ledger.len() as u64).to_be_bytes());
        for lane in &self.lane_transfer_ledger {
            out.extend_from_slice(&lane.lane_id.to_be_bytes());
            out.extend_from_slice(&lane.upstream_lane_id.to_be_bytes());
            out.extend_from_slice(&lane.downstream_lane_id.to_be_bytes());
            for value in [
                lane.upstream_area_ratio,
                lane.area_m2,
                lane.outgoing_surface_m,
                lane.outgoing_lateral_m,
                lane.received_surface_m,
                lane.received_lateral_m,
                lane.net_transfer_m,
            ] {
                float(&mut out, value);
            }
        }
        out.extend_from_slice(&(self.lane_transfer_downstream_operands.lane_count as u64).to_be_bytes());
        out.extend_from_slice(&self.lane_transfer_downstream_operands.outlet_lane_id.to_be_bytes());
        for value in [
            self.lane_transfer_downstream_operands.total_outgoing_surface_m,
            self.lane_transfer_downstream_operands.total_outgoing_lateral_m,
            self.lane_transfer_downstream_operands.total_received_surface_m,
            self.lane_transfer_downstream_operands.total_received_lateral_m,
            self.lane_transfer_downstream_operands.total_net_transfer_m,
        ] {
            float(&mut out, value);
        }
        if let Some(value) = self.lane_transfer_shadow_projection {
            out.push(1);
            out.extend_from_slice(&(value.lane_count as u64).to_be_bytes());
            out.extend_from_slice(&value.outlet_lane_id.to_be_bytes());
            for field in [
                value.total_outgoing_surface_m,
                value.total_outgoing_lateral_m,
                value.total_received_surface_m,
                value.total_received_lateral_m,
                value.total_net_transfer_m,
            ] {
                float(&mut out, field);
            }
        } else {
            out.push(0);
        }
        groundwater(&mut out, self.groundwater);
        match &self.surface_liquid_shadow {
            Some(value) => {
                out.push(1);
                bytes(&mut out, &serde_json::to_vec(value)?);
            }
            None => out.push(0),
        }
        Ok(out)
    }
}

#[test]
fn hydrology_frame_projection_changes_for_physical_lane_field_poison() {
    let identity = DirectRunIdentity::new(41, 7, 1, 1).expect("frame identity");
    let frame = DirectRunFrame::skeleton(identity).expect("frame");
    let mut poisoned = frame.clone();
    poisoned.lanes[0].water.infiltration_m = f64::from_bits(1);
    let base = "01".repeat(32);
    assert_ne!(
        frame
            .canonical_hydrology_physical_projection_v1(&base)
            .expect("beginning projection"),
        poisoned
            .canonical_hydrology_physical_projection_v1(&base)
            .expect("poisoned projection"),
        "a one-ULP physical frame poison must change the private projection"
    );
}
