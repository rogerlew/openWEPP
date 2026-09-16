from pathlib import Path
A=Path(__file__).resolve().parent
R=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916/crates/openwepp-hillslope-orchestrator/src')
p=R/'snow_stage3_v11_current_context_capture.rs'
s=p.read_text()
a=s.index('    let mut positive = parent_receipt.coupled_subslabs.iter()')
b=s.index('    reader.exact(entry.committed_publication_receipt_sha256',a)
block=s[a:b]
s=s[:a]+'    validate_archive_first_positive_join(&parent_receipt, &publication)?;\n'+s[b:]
i=s.index('fn authenticate_archived_day_with_configuration_v1(')
s=s[:i]+'''fn validate_archive_first_positive_join(
    parent_receipt: &crate::DirectSnowStage3V11ParentReceipt,
    publication: &crate::v9_real_consumer_shadow::RestoredStage3PublicationDayEvidenceV1,
) -> Result<(), String> {
'''+block+'    Ok(())\n}\n\n'+s[i:]
i=s.index('    #[test]\n    fn current_phase_members_validate_without_physics()')
s=s[:i]+(A/'additional-controls.rs').read_text()+'\n'+s[i:]
s=s.replace('let archive_ordinals = [27431_u64, 54078, 79222, 107936];','let archive_ordinals = [27_431_u64, 54_078, 79_222, 107_936];').replace('result.err().map(|e| e.to_string())','result.err()')
s=s.replace('#[derive(Clone, Debug)]\nstruct PinnedSeedConfigurationV1','#[cfg(all(test, feature = "persisted-restart-v1"))]\n#[derive(Clone, Debug)]\nstruct PinnedSeedConfigurationV1').replace('fn extract_pinned_seed_configuration_v1(','#[cfg(all(test, feature = "persisted-restart-v1"))]\nfn extract_pinned_seed_configuration_v1(')
p.write_text(s)
p=R/'snow_stage3_v11_member_restoration.rs';s=p.read_text()
a=s.index('    let surface_bytes: Vec<u8> = decode(required(');b=s.index('    let provider = restore_provider_day(',a)
block=s[a:b].replace('&clock_parent.clock','clock').replace('&target','target')
s=s[:a]+'    let surface = restore_pinned_surface(seed, &target, &clock_parent.clock)?;\n'+s[b:]
i=s.index('fn restore_native_context(')
s=s[:i]+'''fn restore_pinned_surface(seed: &Value, target: &Value, clock: &CoupledClockStateV1) -> Result<crate::DirectSurfaceLiquidConfiguration, String> {
'''+block+'    Ok(surface)\n}\n\n'+s[i:]
a=s.index('    let provider = restore_provider_day(',s.index('fn restore_native_context'));b=s.index('    let parent = prepared_day',a)
block=s[a:b].replace('&day','day').replace('&provider_configuration','provider_configuration')
s=s[:a]+'    let (provider, prepared_day) = restore_bound_provider(day, provider_configuration)?;\n'+s[b:]
i=s.index('fn restore_native_context(')
s=s[:i]+'''fn restore_bound_provider(day: &Value, provider_configuration: &crate::runtime_inputs::SnowFreeHalfHourStaticConfiguration) -> Result<(crate::runtime_inputs::PreparedSnowFreeGsiDayV1, crate::ValidatedPreparedStage3V11DayV1), String> {
'''+block+'    Ok((provider, prepared_day))\n}\n\n'+s[i:]
a=s.index('    let parent_support: TimeSupport =',s.index('fn restore_native_context'));b=s.index('    let native = required(&target,',a)
block=s[a:b].replace('&target','target').replace('required(phase,','required(required(target, "phase_diagnostics")?,').replace('clock_parent.clock','clock')
s=s[:a]+'    validate_selected_support(&target, parent, &clock_parent.clock)?;\n'+s[b:]
i=s.index('fn restore_native_context(')
s=s[:i]+'''fn validate_selected_support(target: &Value, parent: &crate::DirectSnowStage3V11PreparedSupport, clock: &CoupledClockStateV1) -> Result<(), String> {
'''+block+'    Ok(())\n}\n\n'+s[i:]
a=s.index('    let forcing = crate::snow_stage3_v11_attachment::canonical_parent_forcing_digest(',s.index('fn restore_native_context'));b=s.index('    let frame = restore_direct_run_frame_dynamic(',a)
block=s[a:b].replace('&target','target').replace('&clock_parent.clock','clock')
s=s[:a]+'    validate_provisional(&target, parent, &provider, &clock_parent.clock, pin)?;\n    restore_native_result(&target, &context, inputs, &surface, position, position_matches, gsi_matches)\n}\n\nfn restore_native_result(target: &Value, context: &NativeRestorationContext<\'_>, inputs: crate::DirectRunConstructorInputs, surface: &crate::DirectSurfaceLiquidConfiguration, position: u128, position_matches: bool, gsi_matches: bool) -> Result<(), String> {\n    let native = required(target, "native_consumer_constructor_operands")?;\n    let pin = context.pin;\n    let day = context.day;\n    let clock_parent = context.clock_parent;\n    let provider_configuration = context.provider_configuration;\n'+s[b:]
i=s.index('fn restore_native_context(')
s=s[:i]+'''fn validate_provisional(target: &Value, parent: &crate::DirectSnowStage3V11PreparedSupport, provider: &crate::runtime_inputs::PreparedSnowFreeGsiDayV1, clock: &CoupledClockStateV1, pin: &super::PinnedSeedConfigurationV1) -> Result<(), String> {
    let support: TimeSupport = decode(required(target, "support")?)?;
'''+block+'    Ok(())\n}\n\n'+s[i:]
a=s.index('    let parcels: BTreeMap<',s.index('pub(super) fn validate_current_phase'));b=s.index('    let expected_clock:',a)
block=s[a:b].replace('&current, &parcels','current, &parcels')
s=s[:a]+'    validate_current_snow_custody(target, phase, clock, &current)?;\n'+s[b:]
i=s.index('#[derive(Serialize)]\n#[serde(rename_all = "snake_case")]')
s=s[:i]+'''fn validate_current_snow_custody(target: &Value, phase: &Value, clock: &CoupledClockStateV1, current: &BTreeMap<u32, crate::DirectSnowStage3PersistentState>) -> Result<(), String> {
'''+block+'    Ok(())\n}\n\n'+s[i:]
s=s.replace('pin_hydrology(&seed, &inputs)?','pin_hydrology(seed, &inputs)?')
p.write_text(s)
p=R/'snow_stage3_v11_attachment.rs';s=p.read_text();a=s.index('    #[cfg(feature = "persisted-restart-v1")]\n    pub(crate) fn diagnostic_snow_free_successor_v1');b=s.index('    fn snow_free_successor(',a)
s=s[:a]+'''    #[cfg(all(test, feature = "persisted-restart-v1"))]
    pub(crate) fn diagnostic_snow_free_successor_v1(
        &self,
        support: TimeSupport,
        ordinal: u32,
        pending: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    ) -> Result<Self, String> {
        let successor = self.coupled_subslab(support, ordinal)
            .and_then(Self::snow_free_successor).map_err(|error| error.to_string())?;
        if pending.is_empty() { Ok(successor) } else {
            successor.with_terminal_receiver_parcels(pending).map_err(|error| error.to_string())
        }
    }

'''+s[b:];p.write_text(s)
