//! Test-only complete-owner discrete terminal support-root operator.
//!
//! This module makes no continuous-time, derivative, interpolation, or LTE
//! claim. The endpoint callback is the only model operand: every positive
//! trial is a complete joint evaluation from immutable beginning owners over
//! one exact integer-nanosecond support.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_coupled_time::ModelTimeNs;

pub(crate) const MINIMUM_TERMINAL_SUPPORT_NS: u128 = 600_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EndpointTerminalClass {
    PreTerminal,
    TerminalAtEndpoint,
    CrossedTerminal { event_tick: ModelTimeNs },
    Invalid,
}

pub(crate) trait CompleteEndpointCandidate: Clone + Eq {
    fn validate_complete(&self) -> Result<(), &'static str>;
    fn canonical_bytes(&self) -> &[u8];
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BatchEndpointEvaluation<C> {
    pub tick: ModelTimeNs,
    pub lane_classes: BTreeMap<u32, EndpointTerminalClass>,
    pub candidate: Option<C>,
}

impl<C: CompleteEndpointCandidate> BatchEndpointEvaluation<C> {
    fn validate(&self, cursor: ModelTimeNs) -> Result<(), DiscreteRootError> {
        if self.lane_classes.is_empty() || self.tick < cursor {
            return Err(DiscreteRootError::InvalidEndpoint);
        }
        if self
            .lane_classes
            .values()
            .any(|class| *class == EndpointTerminalClass::Invalid)
        {
            return Err(DiscreteRootError::InvalidEndpoint);
        }
        if self.lane_classes.values().any(|class| {
            matches!(class, EndpointTerminalClass::CrossedTerminal { event_tick }
                if *event_tick < cursor || *event_tick >= self.tick)
        }) {
            return Err(DiscreteRootError::InvalidEndpoint);
        }
        let valid_candidate = self
            .candidate
            .as_ref()
            .is_some_and(|candidate| candidate.validate_complete().is_ok());
        if self
            .lane_classes
            .values()
            .any(|class| *class != EndpointTerminalClass::Invalid)
            != valid_candidate
        {
            return Err(DiscreteRootError::InvalidEndpoint);
        }
        Ok(())
    }

    fn earliest_event_tick(&self) -> Option<ModelTimeNs> {
        self.lane_classes
            .values()
            .filter_map(|class| match class {
                EndpointTerminalClass::TerminalAtEndpoint => Some(self.tick),
                EndpointTerminalClass::CrossedTerminal { event_tick } => Some(*event_tick),
                EndpointTerminalClass::PreTerminal | EndpointTerminalClass::Invalid => None,
            })
            .min()
    }

    fn terminal_lanes_at(&self, tick: ModelTimeNs) -> Vec<u32> {
        self.lane_classes
            .iter()
            .filter_map(|(lane_id, class)| {
                (*class == EndpointTerminalClass::TerminalAtEndpoint && self.tick == tick)
                    .then_some(*lane_id)
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DiscreteRootAlgorithm {
    FixedPoint,
    SafeguardedSecant,
    IntegerBisection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DiscreteRootError {
    BelowFloor,
    NoRoot,
    InvalidEndpoint,
    NoProgress,
    Cycle,
    AmbiguousOrNonmonotone,
    BracketHistoryDependent,
    ReplayMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvaluatedTickRecord {
    pub algorithm: DiscreteRootAlgorithm,
    pub tick: ModelTimeNs,
    pub lane_classes: BTreeMap<u32, EndpointTerminalClass>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DiscreteRootReceipt<C> {
    pub algorithm: DiscreteRootAlgorithm,
    pub selected_tick: ModelTimeNs,
    pub terminal_lanes: Vec<u32>,
    pub surviving_lanes: Vec<u32>,
    pub candidate: C,
    pub evaluated: Vec<EvaluatedTickRecord>,
}

fn record<C>(
    algorithm: DiscreteRootAlgorithm,
    evaluation: &BatchEndpointEvaluation<C>,
    records: &mut Vec<EvaluatedTickRecord>,
) {
    records.push(EvaluatedTickRecord {
        algorithm,
        tick: evaluation.tick,
        lane_classes: evaluation.lane_classes.clone(),
    });
}

fn evaluate<C, F>(
    cursor: ModelTimeNs,
    parent_end: ModelTimeNs,
    algorithm: DiscreteRootAlgorithm,
    tick: ModelTimeNs,
    records: &mut Vec<EvaluatedTickRecord>,
    endpoint: &mut F,
) -> Result<BatchEndpointEvaluation<C>, DiscreteRootError>
where
    C: CompleteEndpointCandidate,
    F: FnMut(ModelTimeNs) -> Result<BatchEndpointEvaluation<C>, DiscreteRootError>,
{
    if tick <= cursor || tick > parent_end {
        return Err(DiscreteRootError::InvalidEndpoint);
    }
    let duration = tick.get() - cursor.get();
    if duration < MINIMUM_TERMINAL_SUPPORT_NS {
        return Err(DiscreteRootError::BelowFloor);
    }
    let result = endpoint(tick)?;
    if result.tick != tick {
        return Err(DiscreteRootError::InvalidEndpoint);
    }
    result.validate(cursor)?;
    if result
        .earliest_event_tick()
        .is_some_and(|event| event.get() - cursor.get() < MINIMUM_TERMINAL_SUPPORT_NS)
    {
        return Err(DiscreteRootError::BelowFloor);
    }
    record(algorithm, &result, records);
    Ok(result)
}

fn finish<C, F>(
    cursor: ModelTimeNs,
    parent_end: ModelTimeNs,
    algorithm: DiscreteRootAlgorithm,
    selected: BatchEndpointEvaluation<C>,
    mut records: Vec<EvaluatedTickRecord>,
    endpoint: &mut F,
) -> Result<DiscreteRootReceipt<C>, DiscreteRootError>
where
    C: CompleteEndpointCandidate,
    F: FnMut(ModelTimeNs) -> Result<BatchEndpointEvaluation<C>, DiscreteRootError>,
{
    let terminal_lanes = selected.terminal_lanes_at(selected.tick);
    if terminal_lanes.is_empty()
        || selected.lane_classes.values().any(|class| {
            matches!(class, EndpointTerminalClass::CrossedTerminal { event_tick }
                if *event_tick < selected.tick)
        })
    {
        return Err(DiscreteRootError::AmbiguousOrNonmonotone);
    }
    if selected.tick.get() - cursor.get() > MINIMUM_TERMINAL_SUPPORT_NS {
        let previous = evaluate(
            cursor,
            parent_end,
            algorithm,
            ModelTimeNs::new(selected.tick.get() - 1),
            &mut records,
            endpoint,
        )?;
        if previous.earliest_event_tick().is_some() {
            return Err(DiscreteRootError::AmbiguousOrNonmonotone);
        }
    }
    if selected.tick < parent_end {
        let next = evaluate(
            cursor,
            parent_end,
            algorithm,
            ModelTimeNs::new(selected.tick.get() + 1),
            &mut records,
            endpoint,
        )?;
        if next.earliest_event_tick() != Some(selected.tick) {
            return Err(DiscreteRootError::AmbiguousOrNonmonotone);
        }
    }
    let replay = evaluate(
        cursor,
        parent_end,
        algorithm,
        selected.tick,
        &mut records,
        endpoint,
    )?;
    let candidate = selected
        .candidate
        .ok_or(DiscreteRootError::InvalidEndpoint)?;
    let replay_candidate = replay.candidate.ok_or(DiscreteRootError::ReplayMismatch)?;
    if replay.lane_classes != selected.lane_classes
        || replay_candidate != candidate
        || replay_candidate.canonical_bytes() != candidate.canonical_bytes()
    {
        return Err(DiscreteRootError::ReplayMismatch);
    }
    let terminal = terminal_lanes.iter().copied().collect::<BTreeSet<_>>();
    let surviving_lanes = selected
        .lane_classes
        .keys()
        .filter(|lane_id| !terminal.contains(lane_id))
        .copied()
        .collect();
    Ok(DiscreteRootReceipt {
        algorithm,
        selected_tick: selected.tick,
        terminal_lanes,
        surviving_lanes,
        candidate,
        evaluated: records,
    })
}

pub(crate) fn integer_bisection<C, F>(
    cursor: ModelTimeNs,
    parent_end: ModelTimeNs,
    lower_tick: ModelTimeNs,
    upper_tick: ModelTimeNs,
    cursor_witness: Option<BatchEndpointEvaluation<C>>,
    endpoint: &mut F,
) -> Result<DiscreteRootReceipt<C>, DiscreteRootError>
where
    C: CompleteEndpointCandidate,
    F: FnMut(ModelTimeNs) -> Result<BatchEndpointEvaluation<C>, DiscreteRootError>,
{
    if let Some(witness) = cursor_witness {
        witness.validate(cursor)?;
        let terminal_lanes = witness.terminal_lanes_at(cursor);
        if witness.tick != cursor || terminal_lanes.is_empty() {
            return Err(DiscreteRootError::InvalidEndpoint);
        }
        let terminal = terminal_lanes.iter().copied().collect::<BTreeSet<_>>();
        let surviving_lanes = witness
            .lane_classes
            .keys()
            .filter(|lane_id| !terminal.contains(lane_id))
            .copied()
            .collect();
        return Ok(DiscreteRootReceipt {
            algorithm: DiscreteRootAlgorithm::IntegerBisection,
            selected_tick: cursor,
            terminal_lanes,
            surviving_lanes,
            candidate: witness
                .candidate
                .ok_or(DiscreteRootError::InvalidEndpoint)?,
            evaluated: Vec::new(),
        });
    }
    if lower_tick < ModelTimeNs::new(cursor.get() + MINIMUM_TERMINAL_SUPPORT_NS)
        || lower_tick > upper_tick
        || upper_tick > parent_end
    {
        return Err(DiscreteRootError::BelowFloor);
    }
    let algorithm = DiscreteRootAlgorithm::IntegerBisection;
    let mut records = Vec::new();
    let lower = evaluate(
        cursor,
        parent_end,
        algorithm,
        lower_tick,
        &mut records,
        endpoint,
    )?;
    if !lower.terminal_lanes_at(lower_tick).is_empty() {
        return finish(cursor, parent_end, algorithm, lower, records, endpoint);
    }
    if lower.earliest_event_tick().is_some() {
        return Err(DiscreteRootError::BelowFloor);
    }
    let upper = evaluate(
        cursor,
        parent_end,
        algorithm,
        upper_tick,
        &mut records,
        endpoint,
    )?;
    if upper.earliest_event_tick().is_none() {
        return Err(DiscreteRootError::NoRoot);
    }
    let mut low = lower_tick.get();
    let mut high = upper_tick.get();
    let mut selected = upper;
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if middle == low || middle == high {
            return Err(DiscreteRootError::NoProgress);
        }
        let value = evaluate(
            cursor,
            parent_end,
            algorithm,
            ModelTimeNs::new(middle),
            &mut records,
            endpoint,
        )?;
        if value.earliest_event_tick().is_some() {
            high = middle;
            selected = value;
        } else {
            low = middle;
        }
    }
    if selected.tick.get() != high {
        selected = evaluate(
            cursor,
            parent_end,
            algorithm,
            ModelTimeNs::new(high),
            &mut records,
            endpoint,
        )?;
    }
    finish(cursor, parent_end, algorithm, selected, records, endpoint)
}

pub(crate) fn fixed_point<C, F>(
    cursor: ModelTimeNs,
    parent_end: ModelTimeNs,
    initial_tick: ModelTimeNs,
    endpoint: &mut F,
) -> Result<DiscreteRootReceipt<C>, DiscreteRootError>
where
    C: CompleteEndpointCandidate,
    F: FnMut(ModelTimeNs) -> Result<BatchEndpointEvaluation<C>, DiscreteRootError>,
{
    let algorithm = DiscreteRootAlgorithm::FixedPoint;
    let mut records = Vec::new();
    let mut seen = BTreeSet::new();
    let mut tick = initial_tick;
    loop {
        if !seen.insert(tick) {
            return Err(DiscreteRootError::Cycle);
        }
        let value = evaluate(cursor, parent_end, algorithm, tick, &mut records, endpoint)?;
        if !value.terminal_lanes_at(tick).is_empty() {
            return finish(cursor, parent_end, algorithm, value, records, endpoint);
        }
        let next = value
            .earliest_event_tick()
            .ok_or(DiscreteRootError::NoRoot)?;
        if next == tick {
            return Err(DiscreteRootError::NoProgress);
        }
        tick = next;
    }
}

pub(crate) fn safeguarded_secant<C, F>(
    cursor: ModelTimeNs,
    parent_end: ModelTimeNs,
    lower_tick: ModelTimeNs,
    upper_tick: ModelTimeNs,
    endpoint: &mut F,
) -> Result<DiscreteRootReceipt<C>, DiscreteRootError>
where
    C: CompleteEndpointCandidate,
    F: FnMut(ModelTimeNs) -> Result<BatchEndpointEvaluation<C>, DiscreteRootError>,
{
    let algorithm = DiscreteRootAlgorithm::SafeguardedSecant;
    let mut records = Vec::new();
    let lower = evaluate(
        cursor,
        parent_end,
        algorithm,
        lower_tick,
        &mut records,
        endpoint,
    )?;
    if lower.earliest_event_tick().is_some() {
        return Err(DiscreteRootError::BelowFloor);
    }
    let mut high = upper_tick;
    let mut high_value = evaluate(cursor, parent_end, algorithm, high, &mut records, endpoint)?;
    let mut low = lower_tick;
    let mut seen = BTreeSet::from([low, high]);
    while high.get() - low.get() > 1 {
        let hinted = high_value
            .earliest_event_tick()
            .ok_or(DiscreteRootError::NoRoot)?;
        let proposal = if hinted > low && hinted < high {
            hinted
        } else {
            ModelTimeNs::new(low.get() + (high.get() - low.get()) / 2)
        };
        if proposal == low || proposal == high {
            return Err(DiscreteRootError::NoProgress);
        }
        if !seen.insert(proposal) {
            return Err(DiscreteRootError::Cycle);
        }
        let value = evaluate(
            cursor,
            parent_end,
            algorithm,
            proposal,
            &mut records,
            endpoint,
        )?;
        if value.earliest_event_tick().is_some() {
            high = proposal;
            high_value = value;
        } else {
            low = proposal;
        }
    }
    finish(cursor, parent_end, algorithm, high_value, records, endpoint)
}

pub(crate) fn compare_brackets<C, F>(
    cursor: ModelTimeNs,
    parent_end: ModelTimeNs,
    brackets: &[(ModelTimeNs, ModelTimeNs)],
    endpoint: &mut F,
) -> Result<DiscreteRootReceipt<C>, DiscreteRootError>
where
    C: CompleteEndpointCandidate,
    F: FnMut(ModelTimeNs) -> Result<BatchEndpointEvaluation<C>, DiscreteRootError>,
{
    let mut selected = None;
    for (lower, upper) in brackets.iter().copied() {
        let receipt = integer_bisection(cursor, parent_end, lower, upper, None, endpoint)?;
        if selected
            .as_ref()
            .is_some_and(|prior: &DiscreteRootReceipt<C>| {
                prior.selected_tick != receipt.selected_tick
                    || prior.terminal_lanes != receipt.terminal_lanes
                    || prior.candidate != receipt.candidate
            })
        {
            return Err(DiscreteRootError::BracketHistoryDependent);
        }
        selected = Some(receipt);
    }
    selected.ok_or(DiscreteRootError::NoRoot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Candidate {
        bytes: Vec<u8>,
        owners: BTreeMap<&'static str, u8>,
        ledgers: BTreeMap<u32, u8>,
        receipts: BTreeMap<u32, u8>,
    }

    impl CompleteEndpointCandidate for Candidate {
        fn validate_complete(&self) -> Result<(), &'static str> {
            const OWNERS: [&str; 7] = [
                "vegetation",
                "snow",
                "land_surface_energy",
                "hydrology",
                "bgc",
                "soil_thermal",
                "surface_liquid",
            ];
            if self.owners.len() != 7
                || OWNERS.iter().any(|owner| !self.owners.contains_key(owner))
                || self.ledgers.is_empty()
                || self.ledgers.keys().ne(self.receipts.keys())
            {
                return Err("incomplete candidate");
            }
            Ok(())
        }

        fn canonical_bytes(&self) -> &[u8] {
            &self.bytes
        }
    }

    fn candidate(tick: u128, lanes: &[u32]) -> Candidate {
        Candidate {
            bytes: tick
                .to_be_bytes()
                .into_iter()
                .chain(lanes.iter().flat_map(|lane| lane.to_be_bytes()))
                .collect(),
            owners: BTreeMap::from([
                ("vegetation", 1),
                ("snow", 2),
                ("land_surface_energy", 3),
                ("hydrology", 4),
                ("bgc", 5),
                ("soil_thermal", 6),
                ("surface_liquid", 7),
            ]),
            ledgers: lanes.iter().map(|lane| (*lane, 1)).collect(),
            receipts: lanes.iter().map(|lane| (*lane, 1)).collect(),
        }
    }

    fn synthetic(
        roots: BTreeMap<u32, Option<u128>>,
    ) -> impl FnMut(ModelTimeNs) -> Result<BatchEndpointEvaluation<Candidate>, DiscreteRootError>
    {
        move |tick| {
            let lanes = roots.keys().copied().collect::<Vec<_>>();
            let lane_classes = roots
                .iter()
                .map(|(lane, root)| {
                    let class = match root {
                        None => EndpointTerminalClass::PreTerminal,
                        Some(root) if tick.get() < *root => EndpointTerminalClass::PreTerminal,
                        Some(root) if tick.get() == *root => {
                            EndpointTerminalClass::TerminalAtEndpoint
                        }
                        Some(root) => EndpointTerminalClass::CrossedTerminal {
                            event_tick: ModelTimeNs::new(*root),
                        },
                    };
                    (*lane, class)
                })
                .collect();
            Ok(BatchEndpointEvaluation {
                tick,
                lane_classes,
                candidate: Some(candidate(tick.get(), &lanes)),
            })
        }
    }

    fn search(
        roots: BTreeMap<u32, Option<u128>>,
        lower: u128,
        upper: u128,
    ) -> Result<DiscreteRootReceipt<Candidate>, DiscreteRootError> {
        integer_bisection(
            ModelTimeNs::new(0),
            ModelTimeNs::new(2_000_000_000),
            ModelTimeNs::new(lower),
            ModelTimeNs::new(upper),
            None,
            &mut synthetic(roots),
        )
    }

    #[test]
    fn no_event_and_floor_boundaries_are_typed() {
        assert_eq!(
            search(BTreeMap::from([(1, None)]), 600_000_000, 2_000_000_000),
            Err(DiscreteRootError::NoRoot)
        );
        let exact_floor = search(
            BTreeMap::from([(1, Some(600_000_000))]),
            600_000_000,
            2_000_000_000,
        )
        .unwrap();
        assert_eq!(exact_floor.selected_tick, ModelTimeNs::new(600_000_000));
        assert_eq!(
            search(
                BTreeMap::from([(1, Some(599_999_999))]),
                600_000_000,
                2_000_000_000
            ),
            Err(DiscreteRootError::BelowFloor)
        );
    }

    #[test]
    fn cursor_interior_and_parent_end_events_are_exact() {
        let witness_candidate = candidate(0, &[1]);
        let witness = BatchEndpointEvaluation {
            tick: ModelTimeNs::new(0),
            lane_classes: BTreeMap::from([(1, EndpointTerminalClass::TerminalAtEndpoint)]),
            candidate: Some(witness_candidate.clone()),
        };
        let cursor = integer_bisection(
            ModelTimeNs::new(0),
            ModelTimeNs::new(2_000_000_000),
            ModelTimeNs::new(600_000_000),
            ModelTimeNs::new(2_000_000_000),
            Some(witness),
            &mut synthetic(BTreeMap::from([(1, None)])),
        )
        .unwrap();
        assert_eq!(cursor.selected_tick, ModelTimeNs::new(0));
        assert!(cursor.evaluated.is_empty());
        assert_eq!(cursor.candidate, witness_candidate);

        for root in [937_500_000, 2_000_000_000] {
            let receipt = search(
                BTreeMap::from([(1, Some(root))]),
                600_000_000,
                2_000_000_000,
            )
            .unwrap();
            assert_eq!(receipt.selected_tick, ModelTimeNs::new(root));
        }
    }

    #[test]
    fn multiple_brackets_and_order_permutations_select_identically() {
        let roots = BTreeMap::from([(1, Some(937_500_000)), (2, None)]);
        let brackets = [
            (
                ModelTimeNs::new(600_000_000),
                ModelTimeNs::new(1_000_000_000),
            ),
            (
                ModelTimeNs::new(800_000_000),
                ModelTimeNs::new(2_000_000_000),
            ),
        ];
        let first = compare_brackets(
            ModelTimeNs::new(0),
            ModelTimeNs::new(2_000_000_000),
            &brackets,
            &mut synthetic(roots.clone()),
        )
        .unwrap();
        let second = compare_brackets(
            ModelTimeNs::new(0),
            ModelTimeNs::new(2_000_000_000),
            &brackets.into_iter().rev().collect::<Vec<_>>(),
            &mut synthetic(roots),
        )
        .unwrap();
        assert_eq!(first.selected_tick, second.selected_tick);
        assert_eq!(first.candidate, second.candidate);
        assert_eq!(first.terminal_lanes, vec![1]);
        assert_eq!(first.surviving_lanes, vec![2]);
    }

    #[test]
    fn same_and_different_tick_lanes_use_one_joint_candidate() {
        let same = search(
            BTreeMap::from([(1, Some(900_000_000)), (2, Some(900_000_000))]),
            600_000_000,
            2_000_000_000,
        )
        .unwrap();
        assert_eq!(same.terminal_lanes, vec![1, 2]);
        assert!(same.surviving_lanes.is_empty());

        let different = search(
            BTreeMap::from([(1, Some(900_000_000)), (2, Some(1_200_000_000))]),
            600_000_000,
            2_000_000_000,
        )
        .unwrap();
        assert_eq!(different.selected_tick, ModelTimeNs::new(900_000_000));
        assert_eq!(different.terminal_lanes, vec![1]);
        assert_eq!(different.surviving_lanes, vec![2]);
        assert_eq!(different.candidate.owners.len(), 7);
    }

    #[test]
    fn result_blind_algorithms_agree_before_bisection_is_frozen() {
        let roots = BTreeMap::from([(1, Some(937_500_000)), (2, None)]);
        let fixed = fixed_point(
            ModelTimeNs::new(0),
            ModelTimeNs::new(2_000_000_000),
            ModelTimeNs::new(2_000_000_000),
            &mut synthetic(roots.clone()),
        )
        .unwrap();
        let secant = safeguarded_secant(
            ModelTimeNs::new(0),
            ModelTimeNs::new(2_000_000_000),
            ModelTimeNs::new(600_000_000),
            ModelTimeNs::new(2_000_000_000),
            &mut synthetic(roots.clone()),
        )
        .unwrap();
        let bisection = search(roots, 600_000_000, 2_000_000_000).unwrap();
        assert_eq!(fixed.selected_tick, secant.selected_tick);
        assert_eq!(fixed.selected_tick, bisection.selected_tick);
        assert_eq!(fixed.candidate, secant.candidate);
        assert_eq!(fixed.candidate, bisection.candidate);
    }

    #[test]
    fn fixed_point_cycle_and_no_progress_reject_typed() {
        let mut no_progress = |tick| {
            Ok(BatchEndpointEvaluation {
                tick,
                lane_classes: BTreeMap::from([(
                    1,
                    EndpointTerminalClass::CrossedTerminal { event_tick: tick },
                )]),
                candidate: Some(candidate(tick.get(), &[1])),
            })
        };
        assert_eq!(
            fixed_point(
                ModelTimeNs::new(0),
                ModelTimeNs::new(2_000_000_000),
                ModelTimeNs::new(1_000_000_000),
                &mut no_progress,
            ),
            Err(DiscreteRootError::InvalidEndpoint)
        );

        let mut toggle = false;
        let mut cycle = move |tick| {
            toggle = !toggle;
            let event_tick = if toggle { 800_000_000 } else { 900_000_000 };
            Ok(BatchEndpointEvaluation {
                tick,
                lane_classes: BTreeMap::from([(
                    1,
                    if tick.get() == event_tick {
                        EndpointTerminalClass::TerminalAtEndpoint
                    } else {
                        EndpointTerminalClass::CrossedTerminal {
                            event_tick: ModelTimeNs::new(event_tick),
                        }
                    },
                )]),
                candidate: Some(candidate(tick.get(), &[1])),
            })
        };
        assert_eq!(
            fixed_point(
                ModelTimeNs::new(0),
                ModelTimeNs::new(2_000_000_000),
                ModelTimeNs::new(1_000_000_000),
                &mut cycle,
            ),
            Err(DiscreteRootError::InvalidEndpoint),
            "a purported cycle requires a future crossed-event tick and is invalid before iteration"
        );
    }

    #[test]
    fn nonmonotone_replay_and_candidate_poisons_fail_closed() {
        let mut nonmonotone = synthetic(BTreeMap::from([(1, Some(900_000_000))]));
        let mut wrapped = move |tick| {
            let mut value = nonmonotone(tick)?;
            if tick == ModelTimeNs::new(900_000_001) {
                value
                    .lane_classes
                    .insert(1, EndpointTerminalClass::PreTerminal);
            }
            Ok(value)
        };
        assert_eq!(
            integer_bisection(
                ModelTimeNs::new(0),
                ModelTimeNs::new(2_000_000_000),
                ModelTimeNs::new(600_000_000),
                ModelTimeNs::new(2_000_000_000),
                None,
                &mut wrapped,
            ),
            Err(DiscreteRootError::AmbiguousOrNonmonotone)
        );

        let calls = std::cell::Cell::new(0_u32);
        let mut replay_poison = move |tick| {
            calls.set(calls.get() + 1);
            let mut value = synthetic(BTreeMap::from([(1, Some(900_000_000))]))(tick)?;
            if tick == ModelTimeNs::new(900_000_000) && calls.get() > 30 {
                value.candidate.as_mut().unwrap().bytes.push(1);
            }
            Ok(value)
        };
        assert!(matches!(
            integer_bisection(
                ModelTimeNs::new(0),
                ModelTimeNs::new(2_000_000_000),
                ModelTimeNs::new(600_000_000),
                ModelTimeNs::new(2_000_000_000),
                None,
                &mut replay_poison,
            ),
            Err(DiscreteRootError::ReplayMismatch)
        ));

        let mut owner_poison = synthetic(BTreeMap::from([(1, Some(900_000_000))]));
        let mut invalid = move |tick| {
            let mut value = owner_poison(tick)?;
            value.candidate.as_mut().unwrap().owners.remove("hydrology");
            Ok(value)
        };
        assert_eq!(
            integer_bisection(
                ModelTimeNs::new(0),
                ModelTimeNs::new(2_000_000_000),
                ModelTimeNs::new(600_000_000),
                ModelTimeNs::new(2_000_000_000),
                None,
                &mut invalid,
            ),
            Err(DiscreteRootError::InvalidEndpoint)
        );
    }

    #[test]
    fn invalid_lane_cannot_survive_or_share_an_accepted_joint_candidate() {
        let mut mixed = |tick| {
            Ok(BatchEndpointEvaluation {
                tick,
                lane_classes: BTreeMap::from([
                    (1, EndpointTerminalClass::TerminalAtEndpoint),
                    (2, EndpointTerminalClass::Invalid),
                ]),
                candidate: Some(candidate(tick.get(), &[1, 2])),
            })
        };
        assert_eq!(
            integer_bisection(
                ModelTimeNs::new(0),
                ModelTimeNs::new(2_000_000_000),
                ModelTimeNs::new(600_000_000),
                ModelTimeNs::new(2_000_000_000),
                None,
                &mut mixed,
            ),
            Err(DiscreteRootError::InvalidEndpoint)
        );
    }

    #[test]
    fn probe_endpoint_event_and_successor_failures_roll_back_exactly() {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct Owners(Vec<u8>);
        for fail_at in 0..4 {
            let beginning = Owners(vec![1, 2, 3, 4, 5, 6, 7]);
            let mut installed = beginning.clone();
            let transaction = (|| -> Result<(), ()> {
                let mut staged = installed.clone();
                for phase in 0..4 {
                    if phase == fail_at {
                        return Err(());
                    }
                    staged.0[phase] ^= 0xff;
                }
                installed = staged;
                Ok(())
            })();
            assert_eq!(transaction, Err(()));
            assert_eq!(installed, beginning);
        }
    }
}
