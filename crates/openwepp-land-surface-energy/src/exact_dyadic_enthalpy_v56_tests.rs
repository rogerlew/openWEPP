use super::*;

#[test]
fn exact_product_and_difference_cover_sign_zero_and_frozen_vector() {
    assert_eq!(
        ExactDyadicEnthalpy::exact_product_binary64(&[]).expect("empty exact product"),
        ExactDyadicEnthalpy::from_f64(1.0).expect("exact one")
    );
    assert_eq!(
        ExactDyadicEnthalpy::exact_product_binary64(&[-2.0, -3.0, -4.0])
            .expect("signed exact product"),
        ExactDyadicEnthalpy::from_f64(-24.0).expect("exact negative product")
    );
    for (left, right, expected) in [
        (2.0, 3.0, 6.0),
        (2.0, -3.0, -6.0),
        (-2.0, 3.0, -6.0),
        (-2.0, -3.0, 6.0),
    ] {
        assert_eq!(
            ExactDyadicEnthalpy::exact_product_binary64(&[left, right])
                .expect("complete multiplication sign matrix"),
            ExactDyadicEnthalpy::from_f64(expected).expect("signed product oracle")
        );
    }
    assert_eq!(
        ExactDyadicEnthalpy::exact_product_binary64(&[-0.0, f64::MAX]).expect("zero exact product"),
        ExactDyadicEnthalpy::zero()
    );
    assert_eq!(
        ExactDyadicEnthalpy::exact_difference_binary64(-0.0, 0.0)
            .expect("canonical exact zero difference"),
        ExactDyadicEnthalpy::zero()
    );
    assert_eq!(
        ExactDyadicEnthalpy::exact_difference_binary64(273.15, 263.15)
            .expect("exact temperature deficit"),
        ExactDyadicEnthalpy::from_f64(10.0).expect("exact ten")
    );

    let exact = ExactDyadicEnthalpy::exact_product_binary64(&[-1.0, 2_100.0, 10.0])
        .expect("frozen exact product");
    assert_eq!(
        exact,
        ExactDyadicEnthalpy::try_new(-1, "a41", 3).expect("frozen exact vector")
    );
}

#[test]
fn exact_multiply_rounding_cells_cover_even_odd_ties_and_carry() {
    let one = ExactDyadicEnthalpy::from_f64(1.0).expect("one");
    let even_tie = ExactDyadicEnthalpy::try_new(1, "20000000000001", -53).expect("even tie");
    let exact_even_tie = one
        .exact_multiply(&even_tie)
        .expect("exact tie multiplication");
    assert_eq!(
        exact_even_tie
            .round_nearest_even()
            .expect("named RN-even projection")
            .to_bits(),
        1.0_f64.to_bits()
    );
    let (even_high, even_carry) = exact_even_tie
        .rounded_high_and_remainder()
        .expect("even tie split");
    assert_eq!(even_high.to_bits(), 1.0_f64.to_bits());
    assert_eq!(
        even_carry,
        ExactDyadicEnthalpy::try_new(1, "1", -53).expect("positive half-ulp carry")
    );
    assert!(
        exact_even_tie
            .rounds_to_binary64(1.0)
            .expect("even rounding cell")
    );
    assert!(
        !exact_even_tie
            .rounds_to_binary64(f64::from_bits(1.0_f64.to_bits() + 1))
            .expect("adjacent rounding cell")
    );

    let odd_tie = ExactDyadicEnthalpy::try_new(1, "20000000000003", -53).expect("odd tie");
    let (odd_high, odd_carry) = odd_tie.rounded_high_and_remainder().expect("odd tie split");
    assert_eq!(odd_high.to_bits(), 1.0_f64.to_bits() + 2);
    assert_eq!(
        odd_carry,
        ExactDyadicEnthalpy::try_new(-1, "1", -53).expect("negative half-ulp carry")
    );
    assert!(
        odd_tie
            .rounds_to_binary64(odd_high)
            .expect("odd rounding cell")
    );
}

#[test]
fn checked_exact_multiplication_covers_subnormal_overflow_and_resource_refusals() {
    let minimum = f64::from_bits(1);
    let positive_half_minimum = ExactDyadicEnthalpy::exact_product_binary64(&[minimum, 0.5])
        .expect("positive half-minimum exact product");
    assert_eq!(
        positive_half_minimum,
        ExactDyadicEnthalpy::try_new(1, "1", -1075).expect("half minimum")
    );
    assert_eq!(
        positive_half_minimum
            .round_to_f64()
            .expect("even subnormal underflow")
            .to_bits(),
        0.0_f64.to_bits()
    );
    let negative_half_minimum = ExactDyadicEnthalpy::exact_product_binary64(&[-minimum, 0.5])
        .expect("negative half-minimum exact product");
    assert_eq!(
        negative_half_minimum
            .round_to_f64()
            .expect("signed even subnormal underflow")
            .to_bits(),
        (-0.0_f64).to_bits()
    );

    let exact_overflow = ExactDyadicEnthalpy::exact_product_binary64(&[f64::MAX, 2.0])
        .expect("finite exact product beyond binary64");
    assert_eq!(
        exact_overflow.round_to_f64(),
        Err(ExactDyadicEnthalpyError::Binary64Overflow)
    );
    let maximum_exponent = ExactDyadicEnthalpy::try_new(1, "1", MAX_WIRE_EXPONENT_MAGNITUDE)
        .expect("maximum wire exponent");
    let two = ExactDyadicEnthalpy::from_f64(2.0).expect("two");
    assert_eq!(
        maximum_exponent.exact_multiply(&two),
        Err(ExactDyadicEnthalpyError::ExponentOutOfRange)
    );
    let oversized = ExactDyadicEnthalpy {
        sign: 1,
        coefficient_hex: "1".repeat(MAX_WIRE_HEX_DIGITS + 1),
        exponent2: 0,
    };
    assert_eq!(
        oversized.exact_multiply(&two),
        Err(ExactDyadicEnthalpyError::CoefficientResourceLimit)
    );
    assert_eq!(
        ExactDyadicEnthalpy::exact_product_binary64(&[1.0, f64::INFINITY]),
        Err(ExactDyadicEnthalpyError::NonFiniteBinary64)
    );
    assert_eq!(
        ExactDyadicEnthalpy::from_f64(1.0).and_then(|one| one.rounds_to_binary64(f64::NAN)),
        Err(ExactDyadicEnthalpyError::NonFiniteBinary64)
    );
}

#[test]
fn frozen_snow_enthalpy_frozen_vectors_retain_exact_high_and_carry() {
    let vectors = [
        (
            1.0,
            2_100.0,
            263.15,
            0xc0d4_8200_0000_0000,
            ExactDyadicEnthalpy::zero(),
        ),
        (
            0.357_250_665_748_475_2,
            2_100.0,
            263.204_229_777_162_2,
            0xc0bd_2594_519d_28bf,
            ExactDyadicEnthalpy::try_new(-1, "ff5665170353f", -95).expect("r144 first carry"),
        ),
        (
            0.357_250_665_748_475_2,
            2_100.0,
            263.204_229_777_162_25,
            0xc0bd_2594_519d_2890,
            ExactDyadicEnthalpy::try_new(-1, "f12de434f1b2b", -94).expect("r144 adjacent carry"),
        ),
    ];
    for (water, heat_capacity, temperature, high_bits, expected_carry) in vectors {
        let (high, carry) = frozen_snow_enthalpy_high_and_carry(water, heat_capacity, temperature)
            .expect("strictly frozen exact enthalpy");
        assert_eq!(high.to_bits(), high_bits);
        assert_eq!(carry, expected_carry);

        let exact = ExactDyadicEnthalpy::exact_product_binary64(&[-water, heat_capacity])
            .expect("independent exact frozen magnitude")
            .exact_multiply(
                &ExactDyadicEnthalpy::exact_difference_binary64(273.15, temperature)
                    .expect("independent exact temperature deficit"),
            )
            .expect("independent exact frozen enthalpy");
        assert_eq!(
            ExactDyadicEnthalpy::exact_sum([
                &ExactDyadicEnthalpy::from_f64(high).expect("high wire"),
                &carry,
            ])
            .expect("high plus carry"),
            exact
        );
        assert!(exact.rounds_to_binary64(high).expect("exact frozen cell"));
    }
}

#[test]
fn frozen_snow_enthalpy_rejects_nonfinite_and_nonfrozen_domains() {
    for (water, heat_capacity, temperature, expected) in [
        (0.0, 2_100.0, 263.15, "water"),
        (-1.0, 2_100.0, 263.15, "water"),
        (1.0, 0.0, 263.15, "capacity"),
        (1.0, -2_100.0, 263.15, "capacity"),
        (1.0, 2_100.0, 0.0, "temperature"),
        (1.0, 2_100.0, 273.15, "temperature"),
        (1.0, 2_100.0, 274.0, "temperature"),
    ] {
        let error = frozen_snow_enthalpy_high_and_carry(water, heat_capacity, temperature)
            .expect_err("invalid frozen enthalpy domain");
        assert!(
            matches!(error, ExactDyadicEnthalpyError::Domain(detail) if detail.contains(expected))
        );
    }
    for (water, heat_capacity, temperature) in [
        (f64::NAN, 2_100.0, 263.15),
        (1.0, f64::INFINITY, 263.15),
        (1.0, 2_100.0, f64::NEG_INFINITY),
    ] {
        assert_eq!(
            frozen_snow_enthalpy_high_and_carry(water, heat_capacity, temperature),
            Err(ExactDyadicEnthalpyError::NonFiniteBinary64)
        );
    }
}
