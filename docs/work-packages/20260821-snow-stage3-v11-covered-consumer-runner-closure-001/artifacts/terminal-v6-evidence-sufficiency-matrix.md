# Terminal V6 evidence-sufficiency matrix

| final claim | exact DTO fields |
|---|---|
| final result is exact BelowCarrierDomain | `RejectedPrefixEvidenceV6.outcome.outer_variant;RejectedPrefixEvidenceV6.outcome.inner_variant` |
| later floor admission makes zero provider calls | `TrialAdmissionV6.decision;TrialAdmissionV6.minimum_carrier_duration;TrialAdmissionV6.provider_calls_before;TrialAdmissionV6.provider_calls_after` |
| pair is independently REJECT_RETRY | `PairDecisionV6.decision;PairDecisionV6.coarse;PairDecisionV6.fine_1;PairDecisionV6.fine_2` |
| pair roles and positions are exact | `PairDecisionV6.coarse.pair_position;PairDecisionV6.coarse.role;PairDecisionV6.fine_1.pair_position;PairDecisionV6.fine_1.role;PairDecisionV6.fine_2.pair_position;PairDecisionV6.fine_2.role` |
| selected receipts and energy are reconstructible | `SelectedTerminalTrialEvidenceV6.beginning_state;SelectedTerminalTrialEvidenceV6.ending_state;SelectedTerminalTrialEvidenceV6.ledger;SelectedTerminalTrialEvidenceV6.selection;SelectedTerminalTrialEvidenceV6.hydrology_complete_ending_joint` |
| terminal liquid has no pre-event ingress | `ZeroTerminalIngressEvidenceV6.hydrology_terminal_liquid_supply;ZeroTerminalIngressEvidenceV6.wb14_terminal_liquid_credit;ZeroTerminalIngressEvidenceV6.surface_liquid_terminal_ingress` |
| capture is noninterfering | `RejectedPrefixEvidenceV6.before;RejectedPrefixEvidenceV6.after;RejectedPrefixEvidenceV6.unchanged` |
