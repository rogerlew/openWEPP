fn snow_soil_receipt_reseal_roundoff_within_bound_v1(
    residual_j_m2: f64,
    temperature_residual_k: f64,
) -> bool {
    residual_j_m2.is_finite()
        && residual_j_m2 >= 0.0
        && residual_j_m2
            <= crate::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_J_M2
        && temperature_residual_k.is_finite()
        && temperature_residual_k >= 0.0
        && temperature_residual_k
            <= crate::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_TEMPERATURE_K
}

#[cfg(test)]
mod snow_soil_receipt_reseal_roundoff_tests {
    use super::*;

    #[test]
    fn exact_threshold_sides_are_fail_closed() {
        let energy_threshold =
            crate::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_J_M2;
        let temperature_threshold = crate::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_TEMPERATURE_K;
        assert!(snow_soil_receipt_reseal_roundoff_within_bound_v1(0.0, 0.0));
        assert!(snow_soil_receipt_reseal_roundoff_within_bound_v1(
            f64::from_bits(energy_threshold.to_bits() - 1),
            f64::from_bits(temperature_threshold.to_bits() - 1),
        ));
        assert!(snow_soil_receipt_reseal_roundoff_within_bound_v1(
            energy_threshold,
            temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            f64::from_bits(energy_threshold.to_bits() + 1),
            temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            energy_threshold,
            f64::from_bits(temperature_threshold.to_bits() + 1),
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            -energy_threshold,
            temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            energy_threshold,
            -temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            f64::NAN,
            temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            energy_threshold,
            f64::NAN,
        ));
    }
}
