// Typed WB12/WB16/WB19 day-zero seed projection cores owned by the direct runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedWb11EfflenAndMProjection {
    pub(crate) efflen_m: f64,
    pub(crate) efflen_was_defaulted: bool,
    pub(crate) exponent_m: f64,
    pub(crate) exponent_was_defaulted: bool,
}

pub(crate) fn project_typed_wb11_efflen_and_m(
    efflen_m: Option<f64>,
    slplen_m: f64,
    exponent_m: Option<f64>,
) -> Result<TypedWb11EfflenAndMProjection, HillslopeCliError> {
    let (efflen_m, efflen_was_defaulted) = if let Some(efflen_m) = efflen_m {
        (efflen_m, false)
    } else {
        if slplen_m <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} slplen must be > 0.0 when seeding efflen, observed {slplen_m}"
                ),
            });
        }
        (slplen_m, true)
    };
    Ok(TypedWb11EfflenAndMProjection {
        efflen_m,
        efflen_was_defaulted,
        exponent_m: exponent_m.unwrap_or(1.5),
        exponent_was_defaulted: exponent_m.is_none(),
    })
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedWb11FrozenDepthLayerInput {
    pub(crate) depth_m: f64,
    pub(crate) fine_frozen_depths_m: Option<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedWb11FrozenDepthRefreshProjection {
    pub(crate) frozen_depths_m: Vec<f64>,
}

pub(crate) fn project_typed_wb11_frozen_depth_refresh(
    scalar_frost_depth_m: Option<f64>,
    layers: &[TypedWb11FrozenDepthLayerInput],
) -> Result<TypedWb11FrozenDepthRefreshProjection, HillslopeCliError> {
    const ZERO_THRESHOLD: f64 = 1.0e-10;

    if layers.is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "wb11_seed",
            detail: format!(
                "{SIMPIPE_GUARD_ID} typed fine frost aggregate refresh requires at least one layer"
            ),
        });
    }
    let mut cumulative_depth_m = 0.0_f64;
    let mut projected_frost_profile_m = Vec::with_capacity(layers.len());
    for (layer_offset, layer) in layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        if !layer.depth_m.is_finite() || layer.depth_m <= ZERO_THRESHOLD {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "wb11_seed",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} layer {layer_index} depth for fine frost aggregate refresh must be finite and > 0.0, observed {}",
                    layer.depth_m
                ),
            });
        }

        let computed_frost_extent_m = if let Some(fine_frozen_depths_m) = &layer.fine_frozen_depths_m {
            if fine_frozen_depths_m.is_empty() {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} layer {layer_index} fine frost state must contain at least one fine layer"
                    ),
                });
            }
            let mut fine_frozen_depth_m = 0.0_f64;
            for (fine_offset, slfsd_m) in fine_frozen_depths_m.iter().copied().enumerate() {
                let fine_index = fine_offset + 1;
                if !slfsd_m.is_finite() || slfsd_m < -ZERO_THRESHOLD {
                    return Err(HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "wb11_seed",
                        detail: format!(
                            "{SIMPIPE_GUARD_ID} layer {layer_index} fine frost depth {fine_index} must be finite and >= 0.0, observed {slfsd_m}"
                        ),
                    });
                }
                fine_frozen_depth_m += slfsd_m.max(0.0);
            }
            if fine_frozen_depth_m > layer.depth_m + ZERO_THRESHOLD {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} fine frost depth sum for layer {layer_index} exceeds layer depth ({} > {})",
                        fine_frozen_depth_m, layer.depth_m
                    ),
                });
            }
            fine_frozen_depth_m.min(layer.depth_m)
        } else if let Some(scalar_frost_depth_m) = scalar_frost_depth_m {
            if !scalar_frost_depth_m.is_finite() || scalar_frost_depth_m < -ZERO_THRESHOLD {
                return Err(HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "wb11_seed",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} scalar frost depth must be finite and >= 0.0 for aggregate refresh, observed {scalar_frost_depth_m}"
                    ),
                });
            }
            (scalar_frost_depth_m - cumulative_depth_m).clamp(0.0, layer.depth_m)
        } else {
            0.0
        };
        projected_frost_profile_m.push(computed_frost_extent_m);
        cumulative_depth_m += layer.depth_m;
    }

    Ok(TypedWb11FrozenDepthRefreshProjection {
        frozen_depths_m: projected_frost_profile_m,
    })
}

#[cfg(test)]
mod cqr_row7_seed_projection_tests {
    use super::*;

    fn frost_layer(
        depth_m: f64,
        fine_frozen_depths_m: Option<Vec<f64>>,
    ) -> TypedWb11FrozenDepthLayerInput {
        TypedWb11FrozenDepthLayerInput {
            depth_m,
            fine_frozen_depths_m,
        }
    }

    #[test]
    fn cqr_row7_wb11_frozen_depth_refresh_covers_fine_scalar_and_guard_paths() {
        let fine_projection = project_typed_wb11_frozen_depth_refresh(
            None,
            &[
                frost_layer(0.10, Some(vec![0.02, 0.03])),
                frost_layer(0.20, Some(vec![0.05])),
            ],
        )
        .expect("fine-layer frost depths should aggregate");
        assert_eq!(fine_projection.frozen_depths_m, vec![0.05, 0.05]);

        let scalar_projection = project_typed_wb11_frozen_depth_refresh(
            Some(0.15),
            &[frost_layer(0.10, None), frost_layer(0.20, None)],
        )
        .expect("scalar frost depth should be distributed by layer depth");
        assert_eq!(scalar_projection.frozen_depths_m[0].to_bits(), 0.10_f64.to_bits());
        assert!((scalar_projection.frozen_depths_m[1] - 0.05).abs() < 1.0e-12);

        assert!(project_typed_wb11_frozen_depth_refresh(Some(0.0), &[]).is_err());
        assert!(project_typed_wb11_frozen_depth_refresh(Some(0.0), &[frost_layer(0.0, None)]).is_err());
        assert!(
            project_typed_wb11_frozen_depth_refresh(None, &[frost_layer(0.10, Some(vec![]))])
                .is_err()
        );
        assert!(
            project_typed_wb11_frozen_depth_refresh(None, &[frost_layer(0.10, Some(vec![-0.01]))])
                .is_err()
        );
        assert!(
            project_typed_wb11_frozen_depth_refresh(
                None,
                &[frost_layer(0.10, Some(vec![0.09, 0.02]))],
            )
            .is_err()
        );
        assert!(
            project_typed_wb11_frozen_depth_refresh(Some(f64::NAN), &[frost_layer(0.10, None)])
                .is_err()
        );
    }
}
