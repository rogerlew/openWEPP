#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectRootZoneLayerConfiguration {
    production_lane_index: usize,
    production_lane_id: u32,
    layer_id: SoilLayerId,
    saturated_matric_potential_mm: f64,
    clapp_hornberger_b: f64,
}

impl DirectRootZoneLayerConfiguration {
    pub fn try_new(
        production_lane_index: usize,
        production_lane_id: u32,
        layer_id: SoilLayerId,
        saturated_matric_potential_mm: f64,
        clapp_hornberger_b: f64,
    ) -> Result<Self, DirectV10RealConsumerError> {
        if !saturated_matric_potential_mm.is_finite()
            || saturated_matric_potential_mm >= 0.0
            || !clapp_hornberger_b.is_finite()
            || clapp_hornberger_b <= 0.0
        {
            return Err(DirectV10RealConsumerError::RootDomain(
                "root-zone configuration layer",
            ));
        }
        Ok(Self {
            production_lane_index,
            production_lane_id,
            layer_id,
            saturated_matric_potential_mm,
            clapp_hornberger_b,
        })
    }

    #[must_use]
    pub fn restart_identity_fields(&self) -> (usize, u32, &SoilLayerId, f64, f64) {
        (
            self.production_lane_index,
            self.production_lane_id,
            &self.layer_id,
            self.saturated_matric_potential_mm,
            self.clapp_hornberger_b,
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectRootZoneStratumGeometry {
    stratum_id: StratumId,
    root_tissue_lateral_path_m: f64,
}

impl DirectRootZoneStratumGeometry {
    pub fn try_new(
        stratum_id: StratumId,
        root_tissue_lateral_path_m: f64,
    ) -> Result<Self, DirectV10RealConsumerError> {
        if !root_tissue_lateral_path_m.is_finite() || root_tissue_lateral_path_m < 0.0 {
            return Err(DirectV10RealConsumerError::RootDomain(
                "root-zone stratum path",
            ));
        }
        Ok(Self {
            stratum_id,
            root_tissue_lateral_path_m,
        })
    }

    #[must_use]
    pub fn restart_identity_fields(&self) -> (&StratumId, f64) {
        (&self.stratum_id, self.root_tissue_lateral_path_m)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectRootZoneHydraulicConfiguration {
    ordered_layers: Vec<DirectRootZoneLayerConfiguration>,
    ordered_strata: Vec<DirectRootZoneStratumGeometry>,
}

impl DirectRootZoneHydraulicConfiguration {
    pub fn try_new(
        ordered_layers: Vec<DirectRootZoneLayerConfiguration>,
        ordered_strata: Vec<DirectRootZoneStratumGeometry>,
    ) -> Result<Self, DirectV10RealConsumerError> {
        let layer_keys = ordered_layers
            .iter()
            .map(|v| {
                (
                    v.production_lane_index,
                    v.production_lane_id,
                    v.layer_id.clone(),
                )
            })
            .collect::<Vec<_>>();
        let stratum_keys = ordered_strata
            .iter()
            .map(|v| v.stratum_id.clone())
            .collect::<Vec<_>>();
        let unique_layers = layer_keys.iter().cloned().collect::<BTreeSet<_>>();
        let unique_strata = stratum_keys.iter().cloned().collect::<BTreeSet<_>>();
        if ordered_layers.is_empty()
            || unique_layers.len() != layer_keys.len()
            || unique_strata.len() != stratum_keys.len()
        {
            return Err(DirectV10RealConsumerError::RootConfigurationIdentity(
                "root-zone configuration order",
            ));
        }
        Ok(Self {
            ordered_layers,
            ordered_strata,
        })
    }

    pub fn restart_identity_sha256(&self) -> Result<String, DirectV10RealConsumerError> {
        #[derive(Serialize)]
        struct Identity<'a> {
            schema: &'static str,
            ordered_layers: Vec<(usize, u32, &'a SoilLayerId, String, String)>,
            ordered_strata: Vec<(&'a StratumId, String)>,
        }
        let identity = Identity {
            schema: "OPENWEPP_ROOT_ZONE_HYDRAULIC_CONFIGURATION_IDENTITY_V1",
            ordered_layers: self
                .ordered_layers
                .iter()
                .map(|layer| {
                    (
                        layer.production_lane_index,
                        layer.production_lane_id,
                        &layer.layer_id,
                        format!("{:016x}", layer.saturated_matric_potential_mm.to_bits()),
                        format!("{:016x}", layer.clapp_hornberger_b.to_bits()),
                    )
                })
                .collect(),
            ordered_strata: self
                .ordered_strata
                .iter()
                .map(|stratum| {
                    (
                        &stratum.stratum_id,
                        format!("{:016x}", stratum.root_tissue_lateral_path_m.to_bits()),
                    )
                })
                .collect(),
        };
        let bytes = serde_json::to_vec(&identity).map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                "root-zone configuration identity serialization",
            ))
        })?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }

    #[must_use]
    pub fn ordered_layers(&self) -> &[DirectRootZoneLayerConfiguration] {
        &self.ordered_layers
    }

    #[must_use]
    pub fn ordered_strata(&self) -> &[DirectRootZoneStratumGeometry] {
        &self.ordered_strata
    }
}
