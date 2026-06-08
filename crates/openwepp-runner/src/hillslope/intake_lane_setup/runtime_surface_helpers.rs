#[allow(clippy::wildcard_imports)]
use super::super::*;
pub(crate) fn merge_runtime_surfaces(
    mut base: HillslopeWritebackSurface,
    overlay: HillslopeWritebackSurface,
) -> HillslopeWritebackSurface {
    base.state_surface.extend(overlay.state_surface);
    base.flux_surface.extend(overlay.flux_surface);
    base
}

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

pub(crate) fn build_hillslope_runtime_surface_from_pmetpara(
    management: &ManagementParseOutput,
    pmetpara: &mut PmetparaFile,
    mode: PmetparaParseMode,
) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
    let mut surface = HillslopeWritebackSurface::default();
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.mode.sidecar_present"),
        BoundaryValue::scalar(if pmetpara.sidecar_present { 1.0 } else { 0.0 }),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.mode.iflget"),
        BoundaryValue::scalar(f64::from(pmetpara.iflget)),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.record_count"),
        BoundaryValue::scalar(usize_to_scalar(
            "pmetpara.record_count",
            pmetpara.record_count,
        )?),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.line_count_closed"),
        BoundaryValue::scalar(if pmetpara.line_count_closed { 1.0 } else { 0.0 }),
    );

    if !pmetpara.sidecar_present {
        return Ok(surface);
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

    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.selected.kcb"),
        BoundaryValue::scalar(kcb),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.selected.rawp"),
        BoundaryValue::scalar(rawp),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.selected.line_index"),
        BoundaryValue::scalar(f64::from(line_index)),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("pmetpara.lookup.fallback_first_row_used"),
        BoundaryValue::scalar(if pmetpara.lookup.fallback_first_row_used {
            1.0
        } else {
            0.0
        }),
    );

    Ok(surface)
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
