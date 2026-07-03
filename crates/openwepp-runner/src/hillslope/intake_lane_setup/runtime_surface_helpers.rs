#[allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn absent_pmetpara_file() -> PmetparaFile {
    PmetparaFile {
        sidecar_present: false,
        iflget: 1,
        record_count: 0,
        line_count_closed: true,
        records: Vec::new(),
        warnings: Vec::new(),
        lookup: PmetLookupState {
            fallback_first_row_used: false,
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedPmetparaSelectedProjection {
    pub(crate) kcb: f64,
    pub(crate) rawp: f64,
    pub(crate) line_index: i32,
    pub(crate) fallback_first_row_used: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TypedPmetparaRuntimeProjection {
    pub(crate) sidecar_present: bool,
    pub(crate) iflget: i32,
    pub(crate) record_count: usize,
    pub(crate) line_count_closed: bool,
    pub(crate) selected: Option<TypedPmetparaSelectedProjection>,
}

pub(crate) fn project_typed_pmetpara_runtime(
    management: &ManagementParseOutput,
    pmetpara: &mut PmetparaFile,
    mode: PmetparaParseMode,
) -> Result<TypedPmetparaRuntimeProjection, HillslopeCliError> {
    if !pmetpara.sidecar_present {
        return Ok(TypedPmetparaRuntimeProjection {
            sidecar_present: pmetpara.sidecar_present,
            iflget: pmetpara.iflget,
            record_count: pmetpara.record_count,
            line_count_closed: pmetpara.line_count_closed,
            selected: None,
        });
    }

    let active_crop_name = active_management_crop_name(management)?;
    // Native forest fails closed on a PMET lookup miss (`LANUSE-AUTH-2`): it must
    // not inherit the compatibility first-row-fallback coefficients of an
    // unrelated crop. Force strict lookup for forest; cropland keeps its
    // configured (compatibility) mode.
    let effective_mode = if active_management_is_native_forest(management) {
        PmetparaParseMode::Strict
    } else {
        mode
    };
    let (kcb, rawp, line_index) = {
        let record = pmetpara
            .lookup_record(active_crop_name, effective_mode)
            .map_err(|error| HillslopeCliError::ParseFailure {
                surface: "pmetpara",
                detail: error.to_string(),
            })?;
        (record.kcb, record.rawp, record.line_index)
    };

    Ok(TypedPmetparaRuntimeProjection {
        sidecar_present: pmetpara.sidecar_present,
        iflget: pmetpara.iflget,
        record_count: pmetpara.record_count,
        line_count_closed: pmetpara.line_count_closed,
        selected: Some(TypedPmetparaSelectedProjection {
            kcb,
            rawp,
            line_index,
            fallback_first_row_used: pmetpara.lookup.fallback_first_row_used,
        }),
    })
}

fn active_yearly_scenario(
    management: &ManagementParseOutput,
) -> Result<&openwepp_input_contract::parsers::management::YearlyScenario, HillslopeCliError> {
    let first_slot = management.schedule.slots.first().ok_or_else(|| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pmetpara",
            detail: format!(
                "{SIMPIPE_GUARD_ID} management schedule has no slot for PMET crop lookup"
            ),
        }
    })?;
    let yearly_ref = first_slot.yearly_refs.first().copied().ok_or_else(|| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pmetpara",
            detail: format!(
                "{SIMPIPE_GUARD_ID} management schedule slot has no yearly ref for PMET crop lookup"
            ),
        }
    })?;
    if yearly_ref == 0 || yearly_ref > management.registries.yearlies.len() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pmetpara",
            detail: format!(
                "{SIMPIPE_GUARD_ID} yearly ref {yearly_ref} out of range for PMET crop lookup"
            ),
        });
    }
    Ok(&management.registries.yearlies[yearly_ref - 1])
}

pub(crate) fn active_management_crop_name(
    management: &ManagementParseOutput,
) -> Result<&str, HillslopeCliError> {
    let yearly = active_yearly_scenario(management)?;
    // The PMET crop name is the referenced plant scenario's name, resolved the
    // same way for cropland and native forest yearly slots.
    let itype = match &yearly.data {
        YearlyScenarioData::Cropland(cropland) => cropland.itype,
        YearlyScenarioData::Forest(forest) => forest.itype,
    };
    if itype == 0 || itype > management.registries.plants.len() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pmetpara",
            detail: format!(
                "{SIMPIPE_GUARD_ID} plant ref {itype} out of range for PMET crop lookup"
            ),
        });
    }

    Ok(management.registries.plants[itype - 1].meta.name.as_str())
}

/// Whether the active management scenario is the openWEPP-native forest
/// `lanuse` mode. Native forest fails closed on a PMET lookup miss
/// (`LANUSE-AUTH-2`), never the compatibility first-row fallback.
pub(crate) fn active_management_is_native_forest(management: &ManagementParseOutput) -> bool {
    matches!(
        active_yearly_scenario(management).map(|yearly| &yearly.data),
        Ok(YearlyScenarioData::Forest(_))
    )
}

#[cfg(test)]
mod forest_pmet_fail_closed_tests {
    use super::*;
    use openwepp_input_contract::parsers::management::{ParseMode, parse_management_from_str};
    use openwepp_input_contract::parsers::pmetpara::{
        PmetparaParseOptions, parse_pmetpara_from_str,
    };

    // A PMET sidecar whose only crop key does not match any forest/cropland plant
    // name used below, so every lookup misses.
    const PMET_SIDECAR_NO_MATCH: &str = "1\nUnrelatedCrop,0.31,0.42,1,loam-crop\n";

    const FOREST_MAN: &str = "ow-lanuse-1
1
1
1
Forest_High_Severity_Fire
d1
d2
d3
3 # Landuse - <Forest>
forest_high_sev_fire
14.0 3.0 0.0 2.0 0.45
17.0 0.2 0.42 0.0 0.5
20.0 0.1 90.0 0.33 0.2
2.0 0.3 1.0 1.0
5.0 0.005
0.0 0.0
-5.0 5.0 0.2 0.1
0.0 0.0 0.0 0.0
0.0 0.0 0.0 0.0
0.02 2.0 8.0 500.0
0
1
Forest_Initial
d1
d2
d3
3 # Landuse - <Forest>
0.4 0.3 0.3 0.06
1
2
0.0 0.0
0.2 0.2
0
0
0
1
Forest_Year
d1
d2
d3
3 # Landuse - <Forest>
1
0
0
0
2
0
0
0
0.0
3
Forest_Management
d1
d2
d3
1
1
1
1
1
1
";

    const CROPLAND_MAN: &str = "98.4
1
1
1
Crop
d1
d2
d3
1 # Landuse - <Cropland>
WeppWillSet
14.0 3.0 0.0 2.0 5.0 5.0 0.0 0.3 1.0 0.005
0.5 1.0 0.45 0.99 17.0 0.0 0.42 0.2
2
0.0 0.0 20.0 0.1 0.5 0.3 0.33 0.2 90 40.0
-40.0 2.0 0.0
0
1
Ini
d1
d2
d3
1 # Landuse - <Cropland>
1.1 0.4 330 1000 0.0 0.3
1
2
400.0 0.06 0.3 0.06 0.0
1
0.0 0.0 0.0 0.0 0.0
0.2 0.2
0
0
0
1
Year
d1
d2
d3
1 # Landuse - <Cropland>
1
0
0
0
2
0
0
0
0.0
3
Manage
d1
d2
d3
1
1
1
1
1
1
";

    fn sidecar() -> PmetparaFile {
        // The runner always parses PMET sidecars in compatibility mode.
        let options = PmetparaParseOptions {
            mode: PmetparaParseMode::Compatibility,
            ..PmetparaParseOptions::default()
        };
        parse_pmetpara_from_str(PMET_SIDECAR_NO_MATCH, options).expect("pmet sidecar should parse")
    }

    #[test]
    fn native_forest_pmet_miss_fails_closed_no_first_row_fallback() {
        let management =
            parse_management_from_str(FOREST_MAN, ParseMode::Strict).expect("forest man parses");
        let mut pmetpara = sidecar();
        // Compatibility mode is what the runner always supplies; forest must
        // still fail closed rather than take the first-row fallback.
        let result = project_typed_pmetpara_runtime(
            &management,
            &mut pmetpara,
            PmetparaParseMode::Compatibility,
        );
        assert!(
            result.is_err(),
            "native forest with a PMET sidecar miss must fail closed, not fall back"
        );
        assert!(
            !pmetpara.lookup.fallback_first_row_used,
            "forest must not trigger the first-row fallback"
        );
    }

    #[test]
    fn cropland_pmet_miss_keeps_compatibility_first_row_fallback() {
        let management = parse_management_from_str(CROPLAND_MAN, ParseMode::Strict)
            .expect("cropland man parses");
        let mut pmetpara = sidecar();
        let projection = project_typed_pmetpara_runtime(
            &management,
            &mut pmetpara,
            PmetparaParseMode::Compatibility,
        )
        .expect("cropland retains the compatibility first-row fallback");
        let selected = projection
            .selected
            .expect("a PMET row is selected for cropland");
        assert!(
            selected.fallback_first_row_used,
            "cropland compatibility behaviour (first-row fallback) is preserved"
        );
    }
}
