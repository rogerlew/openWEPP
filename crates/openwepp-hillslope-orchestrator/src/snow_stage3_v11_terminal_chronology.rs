/// One independently reproduced lane endpoint proposed to the terminal parent
/// chronology. Candidate discovery is outside this type; construction admits
/// only the exact result of a shortened covered solve.
#[derive(Clone, Debug, PartialEq)]
pub struct Stage3V11ActualTerminalCandidateV1 {
    pub lane_id: u32,
    pub tick: ModelTimeNs,
    pub support: TimeSupport,
    pub event: DirectSnowTerminalEventResult,
    pub terminal_state_sha256: Digest32,
    pub shortened_forcing_sha256: Digest32,
    pub shortened_owner_set_sha256: Digest32,
}

impl Stage3V11ActualTerminalCandidateV1 {
    fn validate(
        &self,
        parent: TimeSupport,
        active_lanes: &BTreeSet<u32>,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if !active_lanes.contains(&self.lane_id)
            || self.support.start_ns() != parent.start_ns()
            || self.support.end_ns() != self.tick
            || self.tick < parent.start_ns()
            || self.tick > parent.end_ns()
            || !self.event.event_occurred
            || !self.event.hour_offset_seconds.is_finite()
            || !self.event.evaluated_seconds.is_finite()
            || self.event.evaluated_seconds < 0.0
            || self.event.unevaluated_seconds < 0.0
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "actual terminal candidate identity/support",
            ));
        }
        let evaluated_ns = quantize_seconds_to_tick(
            ModelTimeNs::new(0),
            ModelTimeNs::new(parent.duration_ns()),
            self.event.evaluated_seconds,
        )?;
        if evaluated_ns.get() != self.support.duration_ns()
            || self.event.end_ice_kg_m2.abs() > 1.0e-12
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "actual terminal endpoint reproduction",
            ));
        }
        Ok(())
    }
}

/// Deterministic event group selected at one common earliest tick. Every lane
/// in this group has independently reproduced the same endpoint using its
/// shortened physical support; lanes with later candidates remain active.
#[derive(Clone, Debug, PartialEq)]
pub struct Stage3V11TerminalEventGroupV1 {
    pub tick: ModelTimeNs,
    pub ordinal: u64,
    pub terminating_lanes: BTreeSet<u32>,
    pub pre_active_lanes: BTreeSet<u32>,
    pub post_active_lanes: BTreeSet<u32>,
    pub candidates: Vec<Stage3V11ActualTerminalCandidateV1>,
    pub receipt_sha256: Digest32,
}

fn terminal_event_group_digest(
    parent: TimeSupport,
    tick: ModelTimeNs,
    ordinal: u64,
    pre: &BTreeSet<u32>,
    post: &BTreeSet<u32>,
    candidates: &[Stage3V11ActualTerminalCandidateV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_V11_TERMINAL_EVENT_GROUP_V1\0");
    bytes.extend_from_slice(&parent.start_ns().get().to_be_bytes());
    bytes.extend_from_slice(&parent.end_ns().get().to_be_bytes());
    bytes.extend_from_slice(&tick.get().to_be_bytes());
    bytes.extend_from_slice(&ordinal.to_be_bytes());
    for lanes in [pre, post] {
        let count = u32::try_from(lanes.len()).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("terminal participant count")
        })?;
        bytes.extend_from_slice(&count.to_be_bytes());
        for lane in lanes {
            bytes.extend_from_slice(&lane.to_be_bytes());
        }
    }
    for candidate in candidates {
        bytes.extend_from_slice(&candidate.lane_id.to_be_bytes());
        bytes.extend_from_slice(candidate.terminal_state_sha256.as_bytes());
        bytes.extend_from_slice(candidate.shortened_forcing_sha256.as_bytes());
        bytes.extend_from_slice(candidate.shortened_owner_set_sha256.as_bytes());
    }
    Ok(openwepp_coupled_time::digest_bytes(&bytes))
}

/// Select one coalesced common-earliest group without changing owners. This is
/// deliberately a pure post-solve operation, so it cannot feed a candidate
/// tick or receipt back into any constitutive equation.
pub fn select_common_earliest_actual_terminal_group_v1(
    parent: TimeSupport,
    event_ordinal: u64,
    active_lanes: &BTreeSet<u32>,
    candidates: Vec<Stage3V11ActualTerminalCandidateV1>,
) -> Result<Option<Stage3V11TerminalEventGroupV1>, DirectSnowStage3V11AttachmentError> {
    if active_lanes.is_empty() {
        return Ok(None);
    }
    let mut by_lane = BTreeMap::new();
    for candidate in candidates {
        candidate.validate(parent, active_lanes)?;
        if by_lane.insert(candidate.lane_id, candidate).is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "duplicate terminal lane candidate",
            ));
        }
    }
    let Some(earliest) = by_lane.values().map(|value| value.tick).min() else {
        return Ok(None);
    };
    let selected = by_lane
        .into_values()
        .filter(|value| value.tick == earliest)
        .collect::<Vec<_>>();
    let terminating_lanes = selected
        .iter()
        .map(|value| value.lane_id)
        .collect::<BTreeSet<_>>();
    let post_active_lanes = active_lanes
        .difference(&terminating_lanes)
        .copied()
        .collect::<BTreeSet<_>>();
    if terminating_lanes.is_empty() || !terminating_lanes.is_subset(active_lanes) {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal participant transition",
        ));
    }
    let receipt_sha256 = terminal_event_group_digest(
        parent,
        earliest,
        event_ordinal,
        active_lanes,
        &post_active_lanes,
        &selected,
    )?;
    Ok(Some(Stage3V11TerminalEventGroupV1 {
        tick: earliest,
        ordinal: event_ordinal,
        terminating_lanes,
        pre_active_lanes: active_lanes.clone(),
        post_active_lanes,
        candidates: selected,
        receipt_sha256,
    }))
}
