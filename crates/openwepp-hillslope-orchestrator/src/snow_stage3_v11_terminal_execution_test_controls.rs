thread_local! {
    static TERMINAL_PROVIDER_SUPPORT_AUDIT: std::cell::RefCell<Option<Vec<TimeSupport>>> =
        const { std::cell::RefCell::new(None) };
    static TERMINAL_BATCH_PRODUCTION_AUDIT: std::cell::RefCell<Option<Vec<TerminalBatchProductionAuditV2>>> =
        const { std::cell::RefCell::new(None) };
    static TERMINAL_PRE_EVENT_PACKAGE_AUDIT: std::cell::RefCell<Option<Vec<crate::v9_real_consumer_shadow::PrecomputedTerminalAcceptedEndpointV1>>> =
        const { std::cell::RefCell::new(None) };
    static FORCE_COVERED_PHYSICAL_DOUBLE_EVALUATION: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
    static FORCE_TERMINAL_PROVISIONAL_PUBLICATION_DOUBLE: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
}

pub(crate) fn force_covered_physical_double_evaluation_for_test(force: bool) {
    FORCE_COVERED_PHYSICAL_DOUBLE_EVALUATION.with(|value| value.set(force));
}

pub(crate) fn force_terminal_provisional_publication_double_for_test(force: bool) {
    FORCE_TERMINAL_PROVISIONAL_PUBLICATION_DOUBLE.with(|value| value.set(force));
}

fn begin_terminal_pre_event_package_audit() {
    TERMINAL_PRE_EVENT_PACKAGE_AUDIT.with(|audit| *audit.borrow_mut() = Some(Vec::new()));
}

fn take_terminal_pre_event_package_audit(
) -> Vec<crate::v9_real_consumer_shadow::PrecomputedTerminalAcceptedEndpointV1> {
    TERMINAL_PRE_EVENT_PACKAGE_AUDIT.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

fn terminal_provisional_publication_deferral_enabled() -> bool {
    FORCE_TERMINAL_PROVISIONAL_PUBLICATION_DOUBLE.with(|value| !value.get())
}

fn ordinary_covered_physical_reuse_enabled() -> bool {
    FORCE_COVERED_PHYSICAL_DOUBLE_EVALUATION.with(|value| !value.get())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TerminalBatchProductionAuditV2 {
    pub support: TimeSupport,
    pub lane_ids: Vec<u32>,
    pub event_ticks: Vec<(u32, Option<ModelTimeNs>)>,
    pub ending_terminal_lanes: Vec<u32>,
    pub ending_surviving_lanes: Vec<u32>,
    pub provider_call_count: u32,
    pub join_call_count: u32,
    pub beginning_joint_sha256: Digest32,
    pub ending_joint_sha256: Digest32,
}

pub(crate) fn begin_terminal_batch_production_audit() {
    TERMINAL_BATCH_PRODUCTION_AUDIT.with(|audit| *audit.borrow_mut() = Some(Vec::new()));
}

pub(crate) fn take_terminal_batch_production_audit() -> Vec<TerminalBatchProductionAuditV2> {
    TERMINAL_BATCH_PRODUCTION_AUDIT.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

pub(crate) fn begin_terminal_provider_support_audit() {
    TERMINAL_PROVIDER_SUPPORT_AUDIT.with(|audit| *audit.borrow_mut() = Some(Vec::new()));
}

pub(crate) fn take_terminal_provider_support_audit() -> Vec<TimeSupport> {
    TERMINAL_PROVIDER_SUPPORT_AUDIT.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

fn audit_terminal_provider_support(support: TimeSupport) {
    TERMINAL_PROVIDER_SUPPORT_AUDIT.with(|audit| {
        if let Some(entries) = audit.borrow_mut().as_mut() {
            entries.push(support);
        }
    });
}
