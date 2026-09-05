use std::cell::RefCell;
use std::collections::BTreeMap;

use openwepp_coupled_time::TimeSupport;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RunnerStage3V11QualificationAuditV1 {
    pub(crate) support_chronology_by_day: BTreeMap<usize, Vec<TimeSupport>>,
    pub(crate) committed_snapshot:
        Option<openwepp_hillslope_orchestrator::SnowStage3V11ProductionQualificationSnapshotV1>,
    pub(crate) attachment_adoption:
        openwepp_hillslope_orchestrator::SnowStage3V11AttachmentAdoptionAuditV1,
}

#[derive(Default)]
struct AuditState {
    enabled: bool,
    audit: RunnerStage3V11QualificationAuditV1,
}

thread_local! {
    static STATE: RefCell<AuditState> = RefCell::new(AuditState::default());
}

pub(crate) fn begin() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        assert!(!state.enabled, "nested Stage-3 qualification audit");
        state.enabled = true;
        state.audit = RunnerStage3V11QualificationAuditV1::default();
        openwepp_hillslope_orchestrator::begin_snow_stage3_v11_attachment_adoption_audit_v1();
    });
}

pub(crate) fn record_support(day_index: usize, support_index: usize, support: TimeSupport) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if !state.enabled {
            return;
        }
        let supports = state
            .audit
            .support_chronology_by_day
            .entry(day_index)
            .or_default();
        assert_eq!(supports.len(), support_index, "JIT support ordinal");
        supports.push(support);
    });
}

pub(crate) fn is_enabled() -> bool {
    STATE.with(|state| state.borrow().enabled)
}

pub(crate) fn record_committed_snapshot(
    snapshot: openwepp_hillslope_orchestrator::SnowStage3V11ProductionQualificationSnapshotV1,
) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        assert!(state.enabled, "Stage-3 qualification audit was not begun");
        assert!(
            state.audit.committed_snapshot.replace(snapshot).is_none(),
            "Stage-3 qualification snapshot must be recorded exactly once"
        );
    });
}

pub(crate) fn take() -> RunnerStage3V11QualificationAuditV1 {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        assert!(state.enabled, "Stage-3 qualification audit was not begun");
        state.enabled = false;
        state.audit.attachment_adoption =
            openwepp_hillslope_orchestrator::take_snow_stage3_v11_attachment_adoption_audit_v1();
        std::mem::take(&mut state.audit)
    })
}
