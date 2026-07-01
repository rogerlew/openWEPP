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
    let (kcb, rawp, line_index) = {
        let record = pmetpara
            .lookup_record(active_crop_name, mode)
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

pub(crate) fn active_management_crop_name(
    management: &ManagementParseOutput,
) -> Result<&str, HillslopeCliError> {
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

    let yearly = &management.registries.yearlies[yearly_ref - 1];
    let YearlyScenarioData::Cropland(cropland) = &yearly.data;
    if cropland.itype == 0 || cropland.itype > management.registries.plants.len() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pmetpara",
            detail: format!(
                "{SIMPIPE_GUARD_ID} plant ref {} out of range for PMET crop lookup",
                cropland.itype
            ),
        });
    }

    Ok(management.registries.plants[cropland.itype - 1]
        .meta
        .name
        .as_str())
}
