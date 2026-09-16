from pathlib import Path
import re
R=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916/crates/openwepp-hillslope-orchestrator/src')
def remove_function(s,name):
    a=s.index('pub(crate) fn '+name);b=s.index('{',a);depth=1;i=b+1
    while depth:
        depth+=(s[i]=='{')-(s[i]=='}');i+=1
    # remove immediately preceding cfg too, retain truthful generic comments.
    start=s.rfind('\n',0,a)+1
    prev=s.rfind('\n',0,start-1)+1
    if s[prev:start].startswith('#[cfg('):start=prev
    return s[:start]+s[i:].lstrip('\n')
p=R/'snow_stage3_v11_current_context_capture.rs';s=p.read_text()
for n in ['restore_captured_prepared_support_from_provider','restore_snow_free_pre_child_context_from_member_export']:s=remove_function(s,n)
a=s.index('pub(crate) fn restore_coupled_clock(');b=s.index('#[cfg(all(test, feature = "persisted-restart-v1"))]\nmod member_backed_restore_tests',a)
part=s[a:b]
part=re.sub(r'^(?=(?:pub\(crate\) )?(?:fn|struct) |impl<)', '#[cfg(test)]\n',part,flags=re.M)
s=s[:a]+part+s[b:]
s=s.replace('    pub canonical_record: Vec<u8>,\n','').replace('    pub qualification_delta: crate::SnowStage3V11QualificationDayDeltaV1,\n','')
s=s.replace('        canonical_record,\n        parent_receipt,\n        publication,\n        qualification_delta,','        parent_receipt,\n        publication,')
s=s.replace('candidate.append(entries[index].clone())','candidate.append(entries[index].clone()).map_err(|error| error.to_string())')
s=s.replace('assert!(matches!(\n                result,\n                Err(crate::DirectSnowStage3V11AttachmentError::Identity(\n                    "archive manifest append order or prior root"\n                ))\n            ));','assert!(result.as_ref().is_err_and(|error| error.contains("archive manifest append order or prior root")));',1)
# Keep the manifest control under a concise function boundary without changing its fixture.
a=s.index('        for (name, run, topology) in [');b=s.index('    #[test]\n    fn archive_first_positive',a)
block=s[a:s.rfind('    }',a,b)]
s=s[:a]+'        check_foreign_manifest_namespaces(&pin, &entries[0]);\n    }\n\n    fn check_foreign_manifest_namespaces(pin: &PinnedSeedConfigurationV1, entry: &crate::Stage3CommittedDayArchiveEntryV1) {\n'+block.replace('entries[0].clone()', 'entry.clone()')+'    }\n\n'+s[b:]
# Test-only decoder pin poisons; a valid digest of another domain passes JSON typing.
i=s.index('        let mut poisoned = typed.clone();')
s=s[:i]+'''        for (pointer, expected) in [
            ("/checkpoint/phase/committed/scientific/direct_hydrology/phase_plan_sha256", "seed phase plan digest"),
            ("/checkpoint/phase/committed/scientific/direct_hydrology/lanes/0/day_inputs_sha256", "seed lane immutable day inputs digest"),
        ] {
            let mut substituted_seed = seed.clone();
            *substituted_seed.pointer_mut(pointer).expect("pinned digest field") = serde_json::json!(digest_bytes(b"foreign-immutable-operand"));
            assert_eq!(member_restoration::pin_hydrology(&substituted_seed, &typed).as_ref().map_err(String::as_str), Err(expected));
        }
'''+s[i:]
p.write_text(s)
p=R/'snow_stage3_v11_member_restoration.rs';s=p.read_text().replace('            &vegetation,','            vegetation,').replace('context: NativeRestorationContext<\'_>,','context: &NativeRestorationContext<\'_>,').replace('        NativeRestorationContext {','        &NativeRestorationContext {').replace('        &context,','        context,')
a=s.index('fn restore_native_result(');b=s.index('pub(super) fn restore(',a);part=s[a:b].replace('required(&target,','required(target,').replace('            &target,','            target,').replace('            &day,','            day,').replace('        &surface,','        surface,').replace('        &provider_configuration,','        provider_configuration,');s=s[:a]+part+s[b:]
s=s.replace('        .diagnostic_snow_free_successor_v1(support, ordinal, &pending)\n        .map_err(|error| error.to_string())?;','        .diagnostic_snow_free_successor_v1(support, ordinal, &pending)?;')
p.write_text(s)
p=R/'v9_real_consumer_shadow.rs';s=p.read_text()
for n in ['diagnostic_native_field','diagnostic_canonicalize_soil_operand_chain_v1','diagnostic_restore_native_consumer_v1','diagnostic_restore_frozen_litter_residents_v1','diagnostic_restore_native_lane_maps_v1']:
    pattern=r'^( *)(?=(?:pub\(crate\) )?fn '+n+r'\b)';s=re.sub(pattern,lambda m:m[1]+'#[cfg(test)]\n'+m[1],s,flags=re.M)
s=s.replace('captured.get(name).cloned().ok_or_else(|| {\n        DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(\n            "diagnostic native constructor operand",\n        ))\n    })?','captured.get(name).cloned().ok_or(\n        DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(\n            "diagnostic native constructor operand",\n        ))\n    )?')
a=s.index('        let frozen_litter_residents: Option<DiagnosticFrozenLitterResidentsV1> =',s.index('pub(crate) fn diagnostic_restore_native_consumer_v1'));b=s.index('        restored.diagnostic_install_hydrology_constructor_inputs_v1(',a)
block=s[a:b].replace('restored.', 'self.')
s=s[:a]+'''        restored.diagnostic_restore_captured_residents_v1(captured, pinned_frozen_litter_lse_configuration, pinned_surface_configuration)?;
'''+s[b:]
i=s.index('    #[cfg(test)]\n    fn diagnostic_restore_frozen_litter_residents_v1')
s=s[:i]+'''    #[cfg(all(test, feature = "persisted-restart-v1"))]
    fn diagnostic_restore_captured_residents_v1(&mut self, captured: &serde_json::Value, pinned_frozen_litter_lse_configuration: &LandSurfaceEnergyConfiguration, pinned_surface_configuration: &DirectSurfaceLiquidConfiguration) -> Result<(), DirectV10RealConsumerError> {
'''+block+'        Ok(())\n    }\n\n'+s[i:]
p.write_text(s)
p=R/'snow_stage3_v11_restart.rs';s=p.read_text().replace('pub(crate) fn diagnostic_restore_coupled_subslab_receipts_value_v2(', '#[cfg(test)]\npub(crate) fn diagnostic_restore_coupled_subslab_receipts_value_v2(');p.write_text(s)
