from pathlib import Path
R=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916/crates/openwepp-hillslope-orchestrator/src')
p=R/'snow_stage3_v11_current_context_capture.rs';s=p.read_text();t=Path('/tmp/openwepp-reader-helper-final/crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_current_context_capture.rs').read_text()
# Integrate only the useful, typed first-function extraction from the proposal.
a=s.index('pub(crate) fn import_and_reconstruct_pre_child_context(');b=s.index('/// File-consumer entrypoint',a);ta=t.index('pub(crate) fn import_and_reconstruct_pre_child_context(');tb=t.index('/// File-consumer entrypoint',ta);s=s[:a]+t[ta:tb]+s[b:]
# Captured-clock/parent restoration is a coherent independent first phase.
a=s.index('fn restore_captured_pre_child_context(');b=s.index('    let beginning_stage3_bytes:',a);start=s.index('    let clock_bytes:',a);block=s[start:b]
s=s[:start]+'    let (clock, parent) = restore_captured_clock_parent(row, context)?;\n'+s[b:]
i=s.index('#[cfg(test)]\nfn restore_captured_pre_child_context(')
s=s[:i]+'''#[cfg(test)]
fn restore_captured_clock_parent(row: &serde_json::Value, context: &DirectSnowStage3V11StaticContext) -> Result<(CoupledClockStateV1, V11ParentTransaction), String> {
    let get = |name: &str| row.get(name).ok_or_else(|| format!("capture missing {name}"));
'''+block+'    Ok((clock, parent))\n}\n\n'+s[i:]
# Partition the existing provider reader at its actual validation phases.
a=s.index('pub(crate) fn restore_provider_bound_context_from_rows(');body=s.index('    let target = rows',a);end=s.index('\n#[cfg(feature = "persisted-restart-v1")]\n#[cfg(test)]\npub(crate) fn restore_provider_bound_context_from_file',body)
old=s[body:end];head=s[a:body]
def section(x,y):return old[old.index(x):old.index(y)]
select=section('    let target = rows','    let provider: ProviderCaptureV1')
prepare=section('    let provider: ProviderCaptureV1','    let beginning_stage3_bytes:')
child=section('    let beginning_stage3_bytes:','    let hydrology_capture:')
native=section('    let hydrology_capture:','    let day_material_residents:')
material=section('    let day_material_residents:','    let archived_next_parent_sequence =')
prefix_read=section('    let archived_next_parent_sequence =','    let archived_rows =')
archives=section('    let archived_rows =','    let committed_clock_restart_bytes:')
clock_join=section('    let committed_clock_restart_bytes:','    let snow_enthalpy_material_resident =')
# The old material DTO was assembled only after all validations, with no work.
material += '''    Ok(crate::snow_stage3_v11_attachment::SnowStage3V11SnowEnthalpyMaterialResidentV1 { current_owner: current_snow_owner, accepted_owner_chronology: snow_chronology })
'''
main='''    let (target, day) = select_provider_capture_rows(rows, context)?;
    let prepared = restore_provider_prepared_parts(target, day, pinned_provider_configuration)?;
    let parent_support = prepared.restored_day.supports().get(B01_INTERVAL_INDEX).ok_or("captured parent support index missing")?;
    let (captured, forcing_receipt) = restore_provider_current_child(target, context, &prepared.provider, &prepared.prepared_support, parent_support, prepared.target_support)?;
    let consumer = restore_provider_native_consumer(target, day, context, &captured, ReaderNativePinsV1 {
        provider: pinned_provider_configuration,
        lse: pinned_lse_configuration,
        frozen_lse: pinned_frozen_litter_lse_configuration,
        hydrology: pinned_hydrology_constructor_inputs,
    })?;
    let snow_enthalpy_material_resident = restore_provider_snow_material(day, &captured)?;
    let (archived_receipt_prefix, authenticated_archived_days) = restore_provider_archive_context(rows, day, context, pinned_lse_configuration)?;
    Ok(RestoredProviderBoundContextV1 {
        prepared_day: prepared.restored_day,
        prepared_support: prepared.prepared_support,
        forcing_receipt,
        consumer,
        archived_receipt_prefix,
        snow_enthalpy_material_resident,
        authenticated_archived_days,
        captured,
    })
}

'''
helpers='''#[cfg(test)]
fn select_provider_capture_rows<'a>(rows: &'a [serde_json::Value], context: &DirectSnowStage3V11StaticContext) -> Result<(&'a serde_json::Value, &'a serde_json::Value), String> {
'''+select+'    Ok((target, day))\n}\n\n'+'''#[cfg(test)]
struct ProviderPreparedPartsV1 {
    provider: crate::runtime_inputs::PreparedSnowFreeGsiDayV1,
    restored_day: crate::ValidatedPreparedStage3V11DayV1,
    prepared_support: crate::DirectSnowStage3V11PreparedSupport,
    target_support: TimeSupport,
}
#[cfg(test)]
fn restore_provider_prepared_parts(target: &serde_json::Value, day: &serde_json::Value, pinned_provider_configuration: &crate::runtime_inputs::SnowFreeHalfHourStaticConfiguration) -> Result<ProviderPreparedPartsV1, String> {
'''+prepare+'    Ok(ProviderPreparedPartsV1 { provider, restored_day, prepared_support, target_support })\n}\n\n'+'''#[cfg(test)]
fn restore_provider_current_child(target: &serde_json::Value, context: &DirectSnowStage3V11StaticContext, provider: &crate::runtime_inputs::PreparedSnowFreeGsiDayV1, prepared_support: &crate::DirectSnowStage3V11PreparedSupport, parent_support: &crate::DirectSnowStage3V11PreparedSupport, target_support: TimeSupport) -> Result<(RestoredCapturedPreChildContextV1, openwepp_coupled_time::Digest32), String> {
'''+child.replace('projected_child != prepared_support', '&projected_child != prepared_support')+'    Ok((captured, forcing_receipt))\n}\n\n'+'''#[cfg(test)]
struct ReaderNativePinsV1<'a> {
    provider: &'a crate::runtime_inputs::SnowFreeHalfHourStaticConfiguration,
    lse: &'a openwepp_land_surface_energy::LandSurfaceEnergyConfiguration,
    frozen_lse: &'a openwepp_land_surface_energy::LandSurfaceEnergyConfiguration,
    hydrology: crate::DirectRunConstructorInputs,
}
#[cfg(test)]
fn restore_provider_native_consumer(target: &serde_json::Value, day: &serde_json::Value, context: &DirectSnowStage3V11StaticContext, captured: &RestoredCapturedPreChildContextV1, pins: ReaderNativePinsV1<'_>) -> Result<DirectV10RealConsumerShadow, String> {
'''+native.replace('pinned_hydrology_constructor_inputs','pins.hydrology').replace('pinned_frozen_litter_lse_configuration','pins.frozen_lse').replace('pinned_lse_configuration','pins.lse').replace('pinned_provider_configuration','pins.provider')+'    Ok(consumer)\n}\n\n'+'''#[cfg(test)]
fn restore_provider_snow_material(day: &serde_json::Value, captured: &RestoredCapturedPreChildContextV1) -> Result<crate::snow_stage3_v11_attachment::SnowStage3V11SnowEnthalpyMaterialResidentV1, String> {
'''+material+'}\n\n'+'''#[cfg(test)]
fn restore_provider_archive_context(rows: &[serde_json::Value], day: &serde_json::Value, context: &DirectSnowStage3V11StaticContext, pinned_lse_configuration: &openwepp_land_surface_energy::LandSurfaceEnergyConfiguration) -> Result<(crate::Stage3ArchivedReceiptPrefixV1, Vec<AuthenticatedArchivedDayV1>), String> {
'''+prefix_read+'    let (archive_manifest, authenticated_archived_days) = restore_provider_archived_rows(rows, context, pinned_lse_configuration)?;\n'+clock_join+'    Ok((archived_receipt_prefix, authenticated_archived_days))\n}\n\n'
a1=archives.index('        if let Some(previous)');b1=archives.index('        archive_manifest',a1);link=archives[a1:b1]
archives=archives[:a1]+'        validate_provider_publication_prefix(authenticated_archived_days.last(), &authenticated)?;\n'+archives[b1:]
helpers+='''#[cfg(test)]
fn restore_provider_archived_rows(rows: &[serde_json::Value], context: &DirectSnowStage3V11StaticContext, pinned_lse_configuration: &openwepp_land_surface_energy::LandSurfaceEnergyConfiguration) -> Result<(crate::Stage3CommittedDayArchiveManifestV1, Vec<AuthenticatedArchivedDayV1>), String> {
'''+archives+'    Ok((archive_manifest, authenticated_archived_days))\n}\n\n'+'''#[cfg(test)]
fn validate_provider_publication_prefix(previous: Option<&AuthenticatedArchivedDayV1>, authenticated: &AuthenticatedArchivedDayV1) -> Result<(), String> {
'''+link.replace('authenticated_archived_days.last()', 'previous')+'    Ok(())\n}\n'
s=s[:a]+head+main+helpers+s[end:]
p.write_text(s)
