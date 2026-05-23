use openwepp_summary_accumulator::{
    SummaryAccumulatorError, SummaryScalarSurface, WB13_H5_WAT_COLUMNS, Wb13DailyWaterBalanceRow,
    Wb13DailyWaterBalanceSurface,
};

fn seeded_wb13_surface() -> SummaryScalarSurface {
    SummaryScalarSurface::from_pairs([
        ("P", 4.40),
        ("RM", 0.00),
        ("Q", 0.0),
        ("Ep", 0.57),
        ("Es", 0.61),
        ("Er", 0.00),
        ("Dp", 0.02),
        ("UpStrmQ", 0.0),
        ("SubRIn", 0.00),
        ("latqcc", 0.80),
        ("Total-Soil", 71.30),
        ("frozwt", 0.00),
        ("Snow-Water", 4.40),
        ("QOFE", 0.0),
        ("Tile", 0.00),
        ("Irr", 0.00),
        ("Area", 9891.92),
        ("SoilWaterTotal", 71.30),
        ("ProfileDepth", 400.00),
        ("ProfilePorosityCap", 171.48),
        ("ProfileFCStore", 38.75),
        ("ProfileWPStore", 14.38),
    ])
    .expect("valid seeded WB13 scalar surface")
}

#[test]
fn wb13_contract_conformance_emits_canonical_25_column_rows_and_monotonic_order() {
    let row_1 = Wb13DailyWaterBalanceRow::from_surface(1, 1, 2008, &seeded_wb13_surface())
        .expect("first row should build");
    let row_2 = Wb13DailyWaterBalanceRow::from_surface(1, 2, 2008, &seeded_wb13_surface())
        .expect("second row should build");

    let mut surface = Wb13DailyWaterBalanceSurface::new();
    surface
        .append_row(row_1.clone())
        .expect("first row should append");
    surface
        .append_row(row_2.clone())
        .expect("second row should append");

    assert_eq!(
        Wb13DailyWaterBalanceSurface::column_headers(),
        &WB13_H5_WAT_COLUMNS
    );
    let text = surface.render_h5_wat_dat();
    let numeric_rows: Vec<&str> = text
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|token| token.parse::<f64>().is_ok())
        })
        .collect();
    assert_eq!(numeric_rows.len(), 2);
    assert_eq!(numeric_rows[0].split_whitespace().count(), 25);
    assert_eq!(numeric_rows[1].split_whitespace().count(), 25);

    let error = surface
        .append_row(row_1)
        .expect_err("duplicate/non-monotonic row key should fail");
    assert!(matches!(
        error,
        SummaryAccumulatorError::NonMonotonicOutputRow { .. }
    ));
}

#[test]
fn wb13_contract_conformance_rejects_missing_required_profile_symbol() {
    let mut scalars = seeded_wb13_surface().as_map().clone();
    scalars.remove("ProfileDepth");
    let malformed = SummaryScalarSurface::from_map(scalars).expect("map remains non-empty");

    let error = Wb13DailyWaterBalanceRow::from_surface(1, 1, 2008, &malformed)
        .expect_err("missing required profile symbol should fail");
    assert_eq!(
        error,
        SummaryAccumulatorError::MissingRequiredOutputSymbol {
            symbol: "ProfileDepth".to_string(),
        }
    );
}

#[test]
fn wb13_contract_conformance_rejects_non_finite_and_domain_invalid_symbols() {
    let mut nonfinite = seeded_wb13_surface().as_map().clone();
    nonfinite.insert("Area".to_string(), f64::NAN);
    let nonfinite_error =
        SummaryScalarSurface::from_map(nonfinite).expect_err("non-finite map should fail");
    assert!(matches!(
        nonfinite_error,
        SummaryAccumulatorError::NonFiniteInput { .. }
    ));

    let mut domain_invalid = seeded_wb13_surface().as_map().clone();
    domain_invalid.insert("Area".to_string(), -1.0);
    let domain_invalid_surface =
        SummaryScalarSurface::from_map(domain_invalid).expect("map should build");
    let domain_error = Wb13DailyWaterBalanceRow::from_surface(1, 1, 2008, &domain_invalid_surface)
        .expect_err("domain-invalid symbol should fail");
    assert_eq!(
        domain_error,
        SummaryAccumulatorError::OutputSymbolOutOfRange {
            symbol: "Area".to_string(),
            value: -1.0,
            minimum: Some(0.0),
            maximum: None,
        }
    );
}
