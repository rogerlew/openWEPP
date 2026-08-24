#[allow(clippy::wildcard_imports)]
use super::*;

/// Canonical owner projections used by the complete default-off V11
/// attachment. These bytes are typed JSON projections of actual successor
/// owners; formatting/debug strings are deliberately excluded.
impl DirectV10RealConsumerShadow {
    pub fn canonical_owner_state_bytes(
        &self,
    ) -> Result<BTreeMap<String, Vec<u8>>, DirectV11RealConsumerError> {
        #[derive(Serialize)]
        struct HydrologyLaneProjection {
            lane_id: u32,
            area_m2: String,
            soil_water_m: String,
            infiltration_m: String,
            runoff_m: String,
            evapotranspiration_m: String,
            drainage_m: String,
            lateral_flow_m: String,
            snow_swe_m: String,
            snow_liquid_m: String,
        }
        #[derive(Serialize)]
        struct HydrologyProjection {
            schema: &'static str,
            run_id: u64,
            hillslope_id: u32,
            lane_count: usize,
            day_count: usize,
            lanes: Vec<HydrologyLaneProjection>,
        }
        let frame = self.inner.hydrology_frame();
        let hydrology = HydrologyProjection {
            schema: "OPENWEPP_DIRECT_HYDROLOGY_OWNER_PROJECTION_V1",
            run_id: frame.identity.run_id,
            hillslope_id: frame.identity.hillslope_id,
            lane_count: frame.identity.lane_count,
            day_count: frame.identity.day_count,
            lanes: frame
                .lanes
                .iter()
                .map(|lane| HydrologyLaneProjection {
                    lane_id: lane.lane_id,
                    area_m2: format!("{:016x}", lane.area_m2.to_bits()),
                    soil_water_m: format!("{:016x}", lane.water.soil_water_m.to_bits()),
                    infiltration_m: format!("{:016x}", lane.water.infiltration_m.to_bits()),
                    runoff_m: format!("{:016x}", lane.water.runoff_m.to_bits()),
                    evapotranspiration_m: format!(
                        "{:016x}",
                        lane.water.evapotranspiration_m.to_bits()
                    ),
                    drainage_m: format!("{:016x}", lane.water.drainage_m.to_bits()),
                    lateral_flow_m: format!("{:016x}", lane.water.lateral_flow_m.to_bits()),
                    snow_swe_m: format!("{:016x}", lane.winter_column.snow.runtime_swe_m.to_bits()),
                    snow_liquid_m: format!(
                        "{:016x}",
                        lane.winter_column.snow.liquid_water_retained_m.to_bits()
                    ),
                })
                .collect(),
        };
        let surface =
            frame
                .surface_liquid_shadow
                .as_ref()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "canonical surface-liquid owner",
                ))?;
        #[derive(Serialize)]
        struct SurfaceLiquidProjection {
            schema: &'static str,
            persistent_owner_bytes: Vec<u8>,
            wb14_parent_working_state_sha256: Option<String>,
        }
        let surface_projection = SurfaceLiquidProjection {
            schema: "OPENWEPP_SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V2",
            persistent_owner_bytes: surface
                .canonical_bytes(&self.inner.surface_configuration)
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                        DirectV9RealConsumerError::Serialization(error.to_string()),
                    ))
                })?,
            wb14_parent_working_state_sha256: self
                .inner
                .wb14_parent_working_state
                .as_ref()
                .map(crate::direct_runtime::DirectWb14ParentWorkingState::canonical_sha256)
                .transpose()
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                        DirectV9RealConsumerError::Serialization(error.to_string()),
                    ))
                })?
                .map(|digest| digest.as_str().to_owned()),
        };
        let mut owners = BTreeMap::new();
        owners.insert(
            "vegetation".to_owned(),
            serde_json::to_vec(&self.vegetation_state)?,
        );
        owners.insert(
            "land_surface_energy".to_owned(),
            serde_json::to_vec(&self.lse_state)?,
        );
        owners.insert(
            "surface_liquid".to_owned(),
            serde_json::to_vec(&surface_projection)?,
        );
        owners.insert("hydrology".to_owned(), serde_json::to_vec(&hydrology)?);
        owners.insert(
            "bgc".to_owned(),
            serde_json::to_vec(&self.inner.biogeochemistry)?,
        );
        owners.insert(
            "soil_thermal".to_owned(),
            serde_json::to_vec(&self.inner.soil_thermal)?,
        );
        Ok(owners)
    }
}
