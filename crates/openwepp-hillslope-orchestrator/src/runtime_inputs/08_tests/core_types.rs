type ErrorSurfaceCase = (HillslopeRuntimeInputError, &'static str, &'static str);

fn soil_core_error_surface_cases() -> Vec<ErrorSurfaceCase> {
    vec![
        (
            HillslopeRuntimeInputError::MissingSoilOfe,
            "HS-RUNTIME-E-001",
            "HS-RUNTIME-E-001: soil profile contains no OFE blocks",
        ),
        (
            HillslopeRuntimeInputError::MissingSoilLayer,
            "HS-RUNTIME-E-002",
            "HS-RUNTIME-E-002: primary OFE contains no soil layers",
        ),
        (
            HillslopeRuntimeInputError::MissingThetaResidual,
            "HS-RUNTIME-E-003",
            "HS-RUNTIME-E-003: primary soil layer missing required theta source for thetdr (theta_r_rosetta or wp_measured)",
        ),
        (
            HillslopeRuntimeInputError::MissingThetaFieldCapacity,
            "HS-RUNTIME-E-004",
            "HS-RUNTIME-E-004: primary soil layer missing required theta source for thetfc (fc_rosetta or fc_measured)",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteProfileDepth { value_mm: f64::NAN },
            "HS-RUNTIME-E-005",
            "HS-RUNTIME-E-005: non-finite soil profile depth_mm value NaN",
        ),
        (
            HillslopeRuntimeInputError::NonPositiveProfileDepth { value_mm: 0.0 },
            "HS-RUNTIME-E-006",
            "HS-RUNTIME-E-006: non-positive soil profile depth_mm value 0",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteTopLayerDepth {
                value_mm: f64::INFINITY,
            },
            "HS-RUNTIME-E-007",
            "HS-RUNTIME-E-007: non-finite top-layer depth_mm value inf",
        ),
        (
            HillslopeRuntimeInputError::NonPositiveTopLayerDepth { value_mm: -1.0 },
            "HS-RUNTIME-E-008",
            "HS-RUNTIME-E-008: non-positive top-layer depth_mm value -1",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteThetaResidual { value: f64::NAN },
            "HS-RUNTIME-E-009",
            "HS-RUNTIME-E-009: non-finite thetdr value NaN",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteThetaFieldCapacity {
                value: f64::INFINITY,
            },
            "HS-RUNTIME-E-010",
            "HS-RUNTIME-E-010: non-finite thetfc value inf",
        ),
    ]
}

fn slope_shape_error_surface_cases() -> Vec<ErrorSurfaceCase> {
    vec![
        (
            HillslopeRuntimeInputError::MissingSlopeOfe,
            "HS-RUNTIME-E-011",
            "HS-RUNTIME-E-011: slope profile contains no OFE blocks",
        ),
        (
            HillslopeRuntimeInputError::SlopeOfeCountMismatch {
                declared_ofe_count: 2,
                observed_ofes: 1,
            },
            "HS-RUNTIME-E-012",
            "HS-RUNTIME-E-012: slope ofe_count 2 does not match observed OFE blocks 1",
        ),
        (
            HillslopeRuntimeInputError::SlopeOfeCountOutOfRange { value: 70000 },
            "HS-RUNTIME-E-013",
            "HS-RUNTIME-E-013: slope OFE count 70000 exceeds lossless conversion range",
        ),
        (
            HillslopeRuntimeInputError::SlopePointCountMismatch {
                ofe_index: 1,
                declared_nslpts: 3,
                observed_points: 2,
            },
            "HS-RUNTIME-E-014",
            "HS-RUNTIME-E-014: OFE 1 declares nslpts=3 but contains 2 points",
        ),
        (
            HillslopeRuntimeInputError::SlopePointCountOutOfRange {
                ofe_index: 1,
                value: 70000,
            },
            "HS-RUNTIME-E-015",
            "HS-RUNTIME-E-015: OFE 1 nslpts 70000 exceeds lossless conversion range",
        ),
        (
            HillslopeRuntimeInputError::InsufficientSlopePoints {
                ofe_index: 1,
                observed_points: 1,
            },
            "HS-RUNTIME-E-016",
            "HS-RUNTIME-E-016: OFE 1 requires at least 2 slope points; observed 1",
        ),
    ]
}

fn slope_numeric_error_surface_cases() -> Vec<ErrorSurfaceCase> {
    vec![
        (
            HillslopeRuntimeInputError::NonFiniteSlopeLength {
                ofe_index: 1,
                value_m: f64::NAN,
            },
            "HS-RUNTIME-E-017",
            "HS-RUNTIME-E-017: OFE 1 has non-finite slplen value NaN",
        ),
        (
            HillslopeRuntimeInputError::NonPositiveSlopeLength {
                ofe_index: 1,
                value_m: 0.0,
            },
            "HS-RUNTIME-E-018",
            "HS-RUNTIME-E-018: OFE 1 has non-positive slplen value 0",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteXinput {
                ofe_index: 1,
                point_index: 2,
                value: f64::NAN,
            },
            "HS-RUNTIME-E-019",
            "HS-RUNTIME-E-019: OFE 1 point 2 has non-finite xinput NaN",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteSlpinp {
                ofe_index: 1,
                point_index: 2,
                value: f64::INFINITY,
            },
            "HS-RUNTIME-E-020",
            "HS-RUNTIME-E-020: OFE 1 point 2 has non-finite slpinp inf",
        ),
        (
            HillslopeRuntimeInputError::NonMonotoneXinput {
                ofe_index: 1,
                left_point_index: 1,
                left_value: 2.0,
                right_point_index: 2,
                right_value: 1.0,
            },
            "HS-RUNTIME-E-021",
            "HS-RUNTIME-E-021: OFE 1 xinput must be monotonic non-decreasing (point 1=2 -> point 2=1)",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteDerivedAverageSlope {
                ofe_index: 1,
                value: f64::NAN,
            },
            "HS-RUNTIME-E-022",
            "HS-RUNTIME-E-022: OFE 1 derived avgslp is non-finite (NaN)",
        ),
        (
            HillslopeRuntimeInputError::NonPositiveDerivedAverageSlope {
                ofe_index: 1,
                value: 0.0,
            },
            "HS-RUNTIME-E-023",
            "HS-RUNTIME-E-023: OFE 1 derived avgslp must be > 0, observed 0",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteDerivedSlopeLength {
                ofe_index: 1,
                value_m: f64::NAN,
            },
            "HS-RUNTIME-E-024",
            "HS-RUNTIME-E-024: OFE 1 derived slope length (terminal xinput) is non-finite (NaN)",
        ),
        (
            HillslopeRuntimeInputError::NonPositiveDerivedSlopeLength {
                ofe_index: 1,
                value_m: 0.0,
            },
            "HS-RUNTIME-E-025",
            "HS-RUNTIME-E-025: OFE 1 derived slope length (terminal xinput) must be > 0, observed 0",
        ),
    ]
}

fn soil_layer_error_surface_cases() -> Vec<ErrorSurfaceCase> {
    vec![
        (
            HillslopeRuntimeInputError::SoilOfeCountMismatch {
                declared_ofe_count: 2,
                observed_ofes: 1,
            },
            "HS-RUNTIME-E-026",
            "HS-RUNTIME-E-026: soil ntemp 2 does not match observed OFE blocks 1",
        ),
        (
            HillslopeRuntimeInputError::SoilOfeCountOutOfRange { value: 70000 },
            "HS-RUNTIME-E-027",
            "HS-RUNTIME-E-027: soil OFE count 70000 exceeds lossless conversion range",
        ),
        (
            HillslopeRuntimeInputError::SoilLayerCountMismatch {
                ofe_index: 1,
                declared_nsl: 3,
                observed_layers: 2,
            },
            "HS-RUNTIME-E-028",
            "HS-RUNTIME-E-028: soil OFE 1 declares nsl=3 but contains 2 layer rows",
        ),
        (
            HillslopeRuntimeInputError::SoilLayerCountOutOfRange {
                ofe_index: 1,
                value: 70000,
            },
            "HS-RUNTIME-E-029",
            "HS-RUNTIME-E-029: soil OFE 1 nsl 70000 exceeds lossless conversion range",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteLayerDepth {
                ofe_index: 1,
                layer_index: 2,
                value_mm: f64::NAN,
            },
            "HS-RUNTIME-E-030",
            "HS-RUNTIME-E-030: soil OFE 1 layer 2 has non-finite depth_mm NaN",
        ),
        (
            HillslopeRuntimeInputError::NonPositiveLayerDepth {
                ofe_index: 1,
                layer_index: 2,
                value_mm: 0.0,
            },
            "HS-RUNTIME-E-031",
            "HS-RUNTIME-E-031: soil OFE 1 layer 2 has non-positive depth_mm 0",
        ),
        (
            HillslopeRuntimeInputError::NonMonotoneLayerDepth {
                ofe_index: 1,
                upper_layer_index: 1,
                upper_depth_mm: 200.0,
                lower_layer_index: 2,
                lower_depth_mm: 150.0,
            },
            "HS-RUNTIME-E-032",
            "HS-RUNTIME-E-032: soil OFE 1 layer depth must increase strictly (layer 1=200mm -> layer 2=150mm)",
        ),
        (
            HillslopeRuntimeInputError::MissingSaturatedConductivity {
                ofe_index: 1,
                layer_index: 2,
            },
            "HS-RUNTIME-E-033",
            "HS-RUNTIME-E-033: soil OFE 1 layer 2 missing required ksat (ssc) value",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteSaturatedConductivity {
                ofe_index: 1,
                layer_index: 2,
                value_mm_h: f64::NAN,
            },
            "HS-RUNTIME-E-034",
            "HS-RUNTIME-E-034: soil OFE 1 layer 2 has non-finite ksat_mm_h NaN",
        ),
        (
            HillslopeRuntimeInputError::NonPositiveSaturatedConductivity {
                ofe_index: 1,
                layer_index: 2,
                value_mm_h: 0.0,
            },
            "HS-RUNTIME-E-035",
            "HS-RUNTIME-E-035: soil OFE 1 layer 2 has non-positive ksat_mm_h 0",
        ),
    ]
}

fn management_error_surface_cases() -> Vec<ErrorSurfaceCase> {
    vec![
        (
            HillslopeRuntimeInputError::ManagementTopologyCountMismatch {
                expected_ofes: 2,
                schedule_initial_refs: 1,
            },
            "HS-RUNTIME-E-036",
            "HS-RUNTIME-E-036: management topology count 2 does not match schedule OFE initial-ref count 1",
        ),
        (
            HillslopeRuntimeInputError::ManagementScheduleSlotCountMismatch {
                expected_slots: 2,
                observed_slots: 1,
            },
            "HS-RUNTIME-E-037",
            "HS-RUNTIME-E-037: management schedule slot count mismatch: expected 2, observed 1",
        ),
        (
            HillslopeRuntimeInputError::ManagementScheduleSlotArityMismatch {
                slot_index: 1,
                crop_slots: 2,
                yearly_refs: 1,
            },
            "HS-RUNTIME-E-038",
            "HS-RUNTIME-E-038: management slot 1 crop-slot arity mismatch: crop_slots=2, yearly_refs=1",
        ),
        (
            HillslopeRuntimeInputError::ManagementInitialReferenceOutOfRange {
                ofe_index: 1,
                initial_ref: 3,
                max_initial_ref: 2,
            },
            "HS-RUNTIME-E-039",
            "HS-RUNTIME-E-039: OFE 1 initial reference 3 exceeds max 2",
        ),
        (
            HillslopeRuntimeInputError::ManagementYearlyReferenceOutOfRange {
                slot_index: 1,
                crop_slot_index: 2,
                yearly_ref: 4,
                max_yearly_ref: 3,
            },
            "HS-RUNTIME-E-040",
            "HS-RUNTIME-E-040: slot 1 crop-slot 2 yearly reference 4 exceeds max 3",
        ),
        (
            HillslopeRuntimeInputError::ManagementScheduleOfeIndexOutOfRange {
                slot_index: 1,
                ofe_index: 5,
                max_ofe_index: 4,
            },
            "HS-RUNTIME-E-045",
            "HS-RUNTIME-E-045: management slot 1 OFE index 5 exceeds max 4",
        ),
    ]
}

fn pl_projection_shape_error_surface_cases() -> Vec<ErrorSurfaceCase> {
    vec![
        (
            HillslopeRuntimeInputError::UnsupportedPlLanduse {
                section: "initial",
                value: 99,
            },
            "HS-RUNTIME-E-041",
            "HS-RUNTIME-E-041: unsupported PL landuse 99 in initial",
        ),
        (
            HillslopeRuntimeInputError::UnsupportedPlManagementOption {
                field: "mgtopt",
                value: 9,
                allowed: "1 or 2",
            },
            "HS-RUNTIME-E-042",
            "HS-RUNTIME-E-042: unsupported PL management option mgtopt=9 (allowed 1 or 2)",
        ),
        (
            HillslopeRuntimeInputError::NonFinitePlProjectionField {
                field: "rw",
                slot_index: 1,
                crop_slot_index: 2,
                value: f64::NAN,
            },
            "HS-RUNTIME-E-043",
            "HS-RUNTIME-E-043: non-finite PL projection field rw at slot 1 crop-slot 2 (NaN)",
        ),
        (
            HillslopeRuntimeInputError::PlProjectionCountOutOfRange {
                field: "ncycle",
                value: 70000,
            },
            "HS-RUNTIME-E-044",
            "HS-RUNTIME-E-044: PL projection count ncycle=70000 exceeds lossless conversion range",
        ),
        (
            HillslopeRuntimeInputError::PlProjectionDayOutOfDomain {
                field: "cutday",
                slot_index: 1,
                crop_slot_index: 2,
                value: 367,
                allowed: "1..=366",
            },
            "HS-RUNTIME-E-046",
            "HS-RUNTIME-E-046: PL projection day field cutday at slot 1 crop-slot 2 has invalid value 367 (allowed 1..=366)",
        ),
        (
            HillslopeRuntimeInputError::PlAnnualExtensionMismatch {
                slot_index: 1,
                crop_slot_index: 2,
                resmgt: 1,
                expected: "annual",
                observed: "perennial",
            },
            "HS-RUNTIME-E-047",
            "HS-RUNTIME-E-047: annual extension mismatch at slot 1 crop-slot 2 for resmgt 1 (expected annual, observed perennial)",
        ),
    ]
}

fn pl_projection_payload_error_surface_cases() -> Vec<ErrorSurfaceCase> {
    vec![
        (
            HillslopeRuntimeInputError::PlProjectionCardinalityInvalid {
                field: "ncut",
                slot_index: 1,
                crop_slot_index: 2,
                value: 0,
                expected: ">0",
            },
            "HS-RUNTIME-E-048",
            "HS-RUNTIME-E-048: invalid cardinality for ncut at slot 1 crop-slot 2 (value 0, expected >0)",
        ),
        (
            HillslopeRuntimeInputError::PlGrazingWindowOutOfDomain {
                slot_index: 1,
                crop_slot_index: 2,
                cycle_index: 3,
                gday: 10,
                gend: 5,
            },
            "HS-RUNTIME-E-049",
            "HS-RUNTIME-E-049: invalid grazing window at slot 1 crop-slot 2 cycle 3 (gday 10 must be < gend 5)",
        ),
        (
            HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "oratea",
                slot_index: 1,
                crop_slot_index: 2,
                value: -1.0,
                allowed: ">=0.0",
            },
            "HS-RUNTIME-E-050",
            "HS-RUNTIME-E-050: PL projection field oratea at slot 1 crop-slot 2 is out of domain (-1, allowed >=0.0)",
        ),
        (
            HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                field: "perennial",
                slot_index: 1,
                crop_slot_index: 2,
                reason: "missing branch",
            },
            "HS-RUNTIME-E-051",
            "HS-RUNTIME-E-051: unsupported PL payload combination for perennial at slot 1 crop-slot 2 (missing branch)",
        ),
    ]
}

fn snow_frost_irrigation_error_surface_cases() -> Vec<ErrorSurfaceCase> {
    vec![
        (
            HillslopeRuntimeInputError::NonFiniteSnowControl {
                field: "rst",
                value: f64::NAN,
            },
            "HS-RUNTIME-E-052",
            "HS-RUNTIME-E-052: non-finite snow control rst=NaN",
        ),
        (
            HillslopeRuntimeInputError::SnowControlOutOfDomain {
                field: "rst",
                value: -1.0,
                allowed: ">=0.0",
            },
            "HS-RUNTIME-E-053",
            "HS-RUNTIME-E-053: snow control rst is out of domain (-1, allowed >=0.0)",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteFrostControl {
                field: "frost.enabled",
                value: f64::NAN,
            },
            "HS-RUNTIME-E-054",
            "HS-RUNTIME-E-054: non-finite frost control frost.enabled=NaN",
        ),
        (
            HillslopeRuntimeInputError::FrostControlOutOfDomain {
                field: "frost.enabled",
                value: 2.0,
                allowed: "0 or 1",
            },
            "HS-RUNTIME-E-055",
            "HS-RUNTIME-E-055: frost control frost.enabled is out of domain (2, allowed 0 or 1)",
        ),
        (
            HillslopeRuntimeInputError::MissingIrrigationScheduleField { field: "amount" },
            "HS-RUNTIME-E-056",
            "HS-RUNTIME-E-056: missing required irrigation schedule field amount",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteIrrigationScheduleField {
                field: "amount",
                value: f64::NAN,
            },
            "HS-RUNTIME-E-057",
            "HS-RUNTIME-E-057: non-finite irrigation schedule field amount=NaN",
        ),
        (
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "amount",
                value: -1.0,
                allowed: ">=0.0",
            },
            "HS-RUNTIME-E-058",
            "HS-RUNTIME-E-058: irrigation schedule field amount is out of domain (-1, allowed >=0.0)",
        ),
        (
            HillslopeRuntimeInputError::IrrigationScheduleCountOutOfRange {
                field: "events",
                value: 70000,
            },
            "HS-RUNTIME-E-059",
            "HS-RUNTIME-E-059: irrigation schedule count events=70000 exceeds lossless conversion range",
        ),
    ]
}

fn soil_corrected_error_surface_cases() -> Vec<ErrorSurfaceCase> {
    vec![
        (
            HillslopeRuntimeInputError::MissingCorrectedLayerNormalizationInput {
                ofe_index: 1,
                layer_index: 2,
                field: "bulk_density",
            },
            "HS-RUNTIME-E-060",
            "HS-RUNTIME-E-060: soil OFE 1 layer 2 missing corrected-lineage normalization input field bulk_density",
        ),
        (
            HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable { ofe_index: 1 },
            "HS-RUNTIME-E-061",
            "HS-RUNTIME-E-061: soil OFE 1 cannot derive normalized corrected-layer lineage for authoritative FC/WP projection",
        ),
        (
            HillslopeRuntimeInputError::CorrectedLayerMappingIncomplete {
                ofe_index: 1,
                layer_index: 2,
                layer_top_depth_mm: 100.0,
                layer_bottom_depth_mm: 200.0,
                covered_depth_mm: 50.0,
            },
            "HS-RUNTIME-E-062",
            "HS-RUNTIME-E-062: soil OFE 1 layer 2 corrected-lineage mapping coverage incomplete (100..200 mm, covered 50 mm)",
        ),
        (
            HillslopeRuntimeInputError::NonFiniteProfileFcTailContribution {
                ofe_index: 1,
                value_mm: f64::NAN,
            },
            "HS-RUNTIME-E-063",
            "HS-RUNTIME-E-063: soil OFE 1 produced non-finite ProfileFC tail contribution NaN mm",
        ),
        (
            HillslopeRuntimeInputError::NegativeProfileFcTailContribution {
                ofe_index: 1,
                value_mm: -1.0,
            },
            "HS-RUNTIME-E-064",
            "HS-RUNTIME-E-064: soil OFE 1 produced negative ProfileFC tail contribution -1 mm",
        ),
    ]
}

fn error_surface_cases() -> Vec<ErrorSurfaceCase> {
    let mut cases = Vec::new();
    cases.extend(soil_core_error_surface_cases());
    cases.extend(slope_shape_error_surface_cases());
    cases.extend(slope_numeric_error_surface_cases());
    cases.extend(soil_layer_error_surface_cases());
    cases.extend(management_error_surface_cases());
    cases.extend(pl_projection_shape_error_surface_cases());
    cases.extend(pl_projection_payload_error_surface_cases());
    cases.extend(snow_frost_irrigation_error_surface_cases());
    cases.extend(soil_corrected_error_surface_cases());
    cases
}

#[test]
fn hillslope_runtime_input_error_codes_and_display_are_stable() {
    let cases = error_surface_cases();

    assert_eq!(cases.len(), 64);
    for (error, expected_code, expected_display) in cases {
        assert_eq!(error.code(), expected_code, "{error:?}");
        assert_eq!(error.to_string(), expected_display, "{error:?}");
    }
}
