#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::type_complexity
)]
type AdaptiveSupportCompleteV2 = (
    V11ParentTransaction,
    DirectV10RealConsumerShadow,
    CoupledClockStateV1,
    V11ParentCandidate,
    BTreeMap<u32, DirectSnowStage3PersistentState>,
    Vec<Stage3CoupledSubslabReceiptV1>,
    Vec<Stage3V11TerminalEventGroupV1>,
    Vec<DirectSnowStage3V11TerminalParcel>,
    Stage3AdaptiveSupportReceiptV1,
    Vec<Stage3SnowFreeSuccessorReceiptV1>,
);

enum AdaptiveSupportExecutionOutcomeV2 {
    Complete(AdaptiveSupportCompleteV2),
    Paused(Box<DirectSnowStage3V11InProgressExecutionV2>),
}
