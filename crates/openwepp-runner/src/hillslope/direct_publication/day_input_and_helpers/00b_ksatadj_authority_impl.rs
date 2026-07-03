#[derive(Clone)]
struct DirectProductionKsatadjPolicy {
    solwpv: f64,
    ksatfac_mm_h: Option<f64>,
    ksatrec_per_day: Option<f64>,
    lkeff_mm_h: Option<f64>,
    layers: Vec<DirectProductionKsatadjLayerPolicy>,
}

#[derive(Clone)]
struct DirectProductionKsatadjLayerPolicy {
    cpm: f64,
}

fn direct_production_ksatadj_effective_conductivity(
    policy: &DirectProductionKsatadjPolicy,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<f64, HillslopeCliError> {
    let soil_conductivity_m_s = layers
        .first()
        .map(|layer| layer.conductivity_m_s)
        .ok_or_else(|| {
            direct_production_executor_blocked(
                "direct production WB14 ksatadj requires at least one layer conductivity",
            )
        })?;
    let ksatadj_layers = layers
        .iter()
        .enumerate()
        .map(|(offset, layer)| {
            let policy_layer = policy.layers.get(offset).ok_or_else(|| {
                direct_production_executor_blocked(format!(
                    "direct production WB14 ksatadj missing static cpm for layer {}",
                    offset + 1
                ))
            })?;
            Ok(DirectKsatadjLayerInputs {
                theta_m: layer.theta_m,
                field_capacity_m: layer.field_capacity_m,
                upper_limit_m: layer.upper_limit_m,
                depth_m: layer.depth_m,
                porosity: layer.porosity,
                cpm: policy_layer.cpm,
                field_capacity_theta: layer.field_capacity_theta,
                residual_theta: layer.residual_theta,
            })
        })
        .collect::<Result<Vec<_>, HillslopeCliError>>()?;
    let outcome = Wb11HydrologyKernel::compute_direct_ksatadj_effective_conductivity(
        &DirectKsatadjEffectiveConductivityInputs {
            ksatadj: true,
            solwpv: policy.solwpv,
            soil_conductivity_m_s,
            ksatfac_mm_h: policy.ksatfac_mm_h,
            ksatrec_per_day: policy.ksatrec_per_day,
            lkeff_mm_h: policy.lkeff_mm_h,
            layers: ksatadj_layers,
        },
    )
    .map_err(|source| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!(
            "{SIMOUT_GUARD_ID} direct production WB14 ksatadj effective conductivity failed: {source}"
        ),
    })?;
    let outcome = outcome.ok_or_else(|| {
        direct_production_executor_blocked(
            "direct production WB14 ksatadj policy evaluated inactive branch",
        )
    })?;
    openwepp_hillslope_orchestrator::record_direct_runtime_ksatadj_effective_conductivity_evaluation();
    Ok(outcome.effective_conductivity_m_s)
}

#[cfg(test)]
mod direct_production_ksatadj_tests {
    use super::*;

    fn assert_close(observed: f64, expected: f64) {
        assert!(
            (observed - expected).abs() <= 1.0e-12,
            "observed {observed}, expected {expected}"
        );
    }

    fn ksatadj_policy(solwpv: f64) -> DirectProductionKsatadjPolicy {
        DirectProductionKsatadjPolicy {
            solwpv,
            ksatfac_mm_h: Some(2.5),
            ksatrec_per_day: Some(0.75),
            lkeff_mm_h: Some(0.1),
            layers: vec![
                DirectProductionKsatadjLayerPolicy { cpm: 1.0 },
                DirectProductionKsatadjLayerPolicy { cpm: 1.0 },
            ],
        }
    }

    fn ksatadj_layers() -> Vec<DirectSubsurfaceLayerState> {
        vec![
            DirectSubsurfaceLayerState {
                theta_m: 0.02,
                field_capacity_m: 0.03,
                upper_limit_m: 0.20,
                conductivity_m_s: 2.5e-6,
                depth_m: 0.10,
                residual_theta: 0.10,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 0.55,
                field_capacity_theta: 0.40,
                coca: 0.0,
                lateral_conductivity_m_s: 2.5e-6,
            },
            DirectSubsurfaceLayerState {
                theta_m: 0.04,
                field_capacity_m: 0.033,
                upper_limit_m: 0.20,
                conductivity_m_s: 2.5e-6,
                depth_m: 0.10,
                residual_theta: 0.12,
                frozen_depth_m: 0.0,
                frozen_water_m: 0.0,
                porosity: 0.55,
                field_capacity_theta: 0.45,
                coca: 0.0,
                lateral_conductivity_m_s: 2.5e-6,
            },
        ]
    }

    #[test]
    fn active_ksatadj_supplies_wb14_effective_conductivity_before_base_fallback() {
        let authority = DirectProductionInfiltrationAuthority {
            effective_conductivity_m_s: Some(9.0e-4),
            ksatadj_policy: Some(ksatadj_policy(9002.0)),
            matric_potential_m: None,
            depression_storage_capacity_m: 0.0,
        };
        let inputs = authority
            .inputs(0, &ksatadj_layers(), Vec::new(), Some(8.0e-4))
            .expect("active ksatadj should build WB14 inputs");
        let producer = inputs
            .producer_inputs
            .expect("WB14 producer inputs should be populated");
        let sat_frac = 0.41_f64 / 0.55_f64;
        let psi = (1500.0_f64.ln() - 33.0_f64.ln()) / (0.425_f64.ln() - 0.11_f64.ln());
        let exponent = (2.0 / psi) + 3.0;
        let expected_m_s = (2.5e-6 * 3.6e6 * sat_frac.powf(exponent)) / 3.6e6;

        assert_close(producer.effective_conductivity_m_s, expected_m_s);
        assert!(
            (producer.effective_conductivity_m_s - 8.0e-4).abs() > 1.0e-6,
            "active ksatadj must not inherit the frost fallback conductivity"
        );
    }

    #[test]
    fn active_ksatadj_final_wb14_conductivity_remains_frost_limited() {
        let authority = DirectProductionInfiltrationAuthority {
            effective_conductivity_m_s: Some(9.0e-4),
            ksatadj_policy: Some(ksatadj_policy(9002.0)),
            matric_potential_m: None,
            depression_storage_capacity_m: 0.0,
        };
        let inputs = authority
            .inputs(0, &ksatadj_layers(), Vec::new(), Some(1.0e-9))
            .expect("active ksatadj should build WB14 inputs under frost cap");
        let producer = inputs
            .producer_inputs
            .expect("WB14 producer inputs should be populated");

        assert_close(producer.effective_conductivity_m_s, 1.0e-9);
    }
}
