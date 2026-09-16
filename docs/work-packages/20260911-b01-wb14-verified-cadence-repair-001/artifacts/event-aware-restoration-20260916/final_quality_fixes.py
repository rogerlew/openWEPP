from pathlib import Path
R=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916/crates/openwepp-hillslope-orchestrator/src')
p=R/'v9_real_consumer_shadow.rs';s=p.read_text().replace('    RealHydrologyLaneLayerMap, RealHydrologyOfeLaneId, RealHydrologyShadowAdapter,','    RealHydrologyLaneLayerMap, RealHydrologyShadowAdapter,').replace('DirectPublicationDayInput, DirectRunConstructorInputs,','DirectPublicationDayInput,');i=s.index('#[path = "canonical_owner_bytes.rs"]');s=s[:i]+'''#[cfg(test)]
use crate::vegetation_real_hydrology_shadow::RealHydrologyOfeLaneId;
#[cfg(test)]
use crate::DirectRunConstructorInputs;

'''+s[i:]
a=s.index('        let surface_configuration_canonical_json: Vec<u8> =',s.index('pub(crate) fn diagnostic_restore_native_consumer_v1'));b=s.index('        let surface_configuration = pinned_surface_configuration.clone();',a);block=s[a:b];s=s[:a]+'        Self::diagnostic_validate_surface_configuration_v1(captured, pinned_surface_configuration)?;\n'+s[b:]
i=s.index('    #[cfg(all(test, feature = "persisted-restart-v1"))]\n    fn diagnostic_restore_captured_residents_v1')
s=s[:i]+'''    #[cfg(all(test, feature = "persisted-restart-v1"))]
    fn diagnostic_validate_surface_configuration_v1(captured: &serde_json::Value, pinned_surface_configuration: &DirectSurfaceLiquidConfiguration) -> Result<(), DirectV10RealConsumerError> {
'''+block+'        Ok(())\n    }\n\n'+s[i:]
s=s.replace('frozen_litter_residents.ok_or_else(|| {\n            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(\n                "diagnostic native target requires frozen-litter V3/V4 residents",\n            ))\n        })?', 'frozen_litter_residents.ok_or(\n            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(\n                "diagnostic native target requires frozen-litter V3/V4 residents",\n            ))\n        )?');p.write_text(s)
p=R/'snow_stage3_v11_current_context_capture.rs';s=p.read_text();s=s.replace('AcceptedSlabReceiptV1, ConstraintClass, ConstraintReductionReceiptV1, CoupledClockStateV1,','AcceptedSlabReceiptV1, ConstraintReductionReceiptV1, CoupledClockStateV1,').replace('    CoupledSlabCandidateV1, CoupledTimeRestartV2, DiagnosticReductionV1, LedgerEntryV1,','    CoupledTimeRestartV2, DiagnosticReductionV1, LedgerEntryV1,').replace('    StepConstraintV1, TimeSupport, accept_slab, complete_owner_set_digest, digest_bytes,\n    reduce_constraints,','    TimeSupport, digest_bytes,');i=s.index('use openwepp_vegetation::V11ParentTransaction;');s=s[:i]+'''#[cfg(test)]
use openwepp_coupled_time::{ConstraintClass, CoupledSlabCandidateV1, StepConstraintV1, accept_slab, complete_owner_set_digest, reduce_constraints};
'''+s[i:]
# Borrow bytes after removal of unused retained copy; original callers keep ownership.
for name in ['authenticate_archived_day_v1','authenticate_archived_day_with_configuration_v1']:
 a=s.index('fn '+name+'(');b=s.index(') -> Result<',a);s=s[:a]+s[a:b].replace('canonical_record: Vec<u8>','canonical_record: &[u8]')+s[b:]
s=s.replace('digest_bytes(&canonical_record)', 'digest_bytes(canonical_record)').replace('bytes: &canonical_record,','bytes: canonical_record,')
s=s.replace('            canonical_record,\n            context,','            &canonical_record,\n            context,').replace('                bytes,\n                run,','                &bytes,\n                run,').replace('            raw,\n            pin.run_identity,','            &raw,\n            pin.run_identity,')
p.write_text(s)
p=R/'snow_stage3_v11_member_restoration.rs';s=p.read_text().replace('            bytes,\n            pin.run_identity,','            &bytes,\n            pin.run_identity,');p.write_text(s)
# Move the two canonical seal calculations into separate test fixture helpers.
p=R/'v9_real_consumer_shadow_publication_retention.rs';s=p.read_text();a=s.index('        let digest = |name: &str| {',s.index('    fn resealed_event_field_substitution('));b=s.index('        let replacement = |digest: Digest32| {',a);block=s[a:b]
event_start=block.index('        let parent: ParentTransactionId');event_end=block.index('        let begin_clock = digest(');event=block[event_start:event_end]
# The receipt phase restores the same typed common operands, with no altered bytes.
receipt=block[event_end:]
common='''        let digest = |name: &str| serde_json::from_value::<Digest32>(value[name].clone()).expect("selected event digest");
'''
receipt_intro='''        let parent: ParentTransactionId = serde_json::from_value(value["parent_transaction_id"].clone()).expect("selected event parent");
        let tick = value["tick"].as_str().expect("selected event tick").parse::<u128>().expect("selected event tick width");
        let ordinal = u32::try_from(value["ordinal"].as_u64().expect("selected event ordinal")).expect("selected event ordinal width");
        let tick_bytes = tick.to_be_bytes();
        let ordinal_bytes = ordinal.to_be_bytes();
        let context = digest("event_context_digest");
'''
s=s[:a]+'''        let event_id = substituted_event_id(&value);
        let (end_clock, receipt_id) = substituted_event_receipt(&value, event_id);
'''+s[b:]
i=s.index('    fn assert_resealed_event_is_valid(')
s=s[:i]+'''    fn substituted_event_id(value: &serde_json::Value) -> Digest32 {
'''+common+event+'        event_id\n    }\n\n'+'''    fn substituted_event_receipt(value: &serde_json::Value, event_id: Digest32) -> (Digest32, Digest32) {
'''+common+receipt_intro+receipt+'        (end_clock, receipt_id)\n    }\n\n'+s[i:]
p.write_text(s)
