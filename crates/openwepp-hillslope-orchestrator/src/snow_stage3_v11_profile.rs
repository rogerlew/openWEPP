//! Non-persisted wall-time attribution for the opt-in Stage-3 parent guard.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use openwepp_coupled_time::TimeSupport;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AdaptiveParentProfileDetailV1 {
    pub physical_topology_elapsed: std::time::Duration,
    pub physical_potential_elapsed: std::time::Duration,
    pub physical_request_elapsed: std::time::Duration,
    pub physical_unified_elapsed: std::time::Duration,
    pub unified_preflight_elapsed: std::time::Duration,
    pub unified_authorization_elapsed: std::time::Duration,
    pub unified_entry_validation_elapsed: std::time::Duration,
    pub unified_protocol_validation_elapsed: std::time::Duration,
    pub unified_candidate_elapsed: std::time::Duration,
    pub candidate_soil_elapsed: std::time::Duration,
    pub candidate_surface_resource_elapsed: std::time::Duration,
    pub candidate_surface_ingress_elapsed: std::time::Duration,
    pub candidate_receivers_elapsed: std::time::Duration,
    pub candidate_validation_elapsed: std::time::Duration,
    pub physical_final_tile_elapsed: std::time::Duration,
    pub physical_protocol_elapsed: std::time::Duration,
    pub physical_ingress_elapsed: std::time::Duration,
    pub physical_post_elapsed: std::time::Duration,
    pub finalization_candidate_elapsed: std::time::Duration,
    pub finalization_sealed_source_elapsed: std::time::Duration,
    pub finalization_install_elapsed: std::time::Duration,
    pub finalization_identity_replay_elapsed: std::time::Duration,
    pub imported_entry_validation_elapsed: std::time::Duration,
    pub imported_physical_candidate_elapsed: std::time::Duration,
    pub imported_physical_setup_elapsed: std::time::Duration,
    pub imported_frozen_evaluation_elapsed: std::time::Duration,
    pub imported_frozen_preparation_elapsed: std::time::Duration,
    pub imported_frozen_execute_accept_elapsed: std::time::Duration,
    pub imported_frozen_execution_setup_elapsed: std::time::Duration,
    pub imported_frozen_runtime_elapsed: std::time::Duration,
    pub imported_frozen_acceptance_elapsed: std::time::Duration,
    pub imported_envelope_construction_elapsed: std::time::Duration,
    pub imported_envelope_validation_elapsed: std::time::Duration,
    pub imported_accepted_candidate_elapsed: std::time::Duration,
    pub imported_owner_publication_elapsed: std::time::Duration,
    pub imported_install_elapsed: std::time::Duration,
    pub imported_reuse_validation_elapsed: std::time::Duration,
    pub imported_reuse_reseal_elapsed: std::time::Duration,
    pub imported_reuse_install_elapsed: std::time::Duration,
    pub terminal_candidate_setup_elapsed: std::time::Duration,
    pub terminal_provider_custody_elapsed: std::time::Duration,
    pub terminal_provider_projection_elapsed: std::time::Duration,
    pub terminal_provider_carrier_elapsed: std::time::Duration,
    pub terminal_provider_retention_elapsed: std::time::Duration,
    pub terminal_result_finalization_elapsed: std::time::Duration,
    pub carrier_physical_phase_elapsed: std::time::Duration,
    pub carrier_complete_phase_elapsed: std::time::Duration,
    pub carrier_physical_setup_elapsed: std::time::Duration,
    pub carrier_physical_evidence_elapsed: std::time::Duration,
    pub carrier_physical_completion_elapsed: std::time::Duration,
    pub carrier_complete_envelope_elapsed: std::time::Duration,
    pub carrier_complete_adoption_elapsed: std::time::Duration,
    pub carrier_complete_projection_elapsed: std::time::Duration,
    pub carrier_complete_owner_elapsed: std::time::Duration,
    pub carrier_owner_vegetation_validation_elapsed: std::time::Duration,
    pub carrier_owner_hydrology_projection_elapsed: std::time::Duration,
    pub carrier_owner_surface_canonical_elapsed: std::time::Duration,
    pub carrier_owner_vegetation_encoding_elapsed: std::time::Duration,
    pub carrier_owner_surface_encoding_elapsed: std::time::Duration,
    pub carrier_owner_soil_encoding_elapsed: std::time::Duration,
    pub carrier_owner_other_encoding_elapsed: std::time::Duration,
    pub carrier_owner_joint_map_elapsed: std::time::Duration,
    pub carrier_owner_joint_seal_elapsed: std::time::Duration,
    pub carrier_owner_soil_custody_elapsed: std::time::Duration,
    pub carrier_owner_candidate_bytes_elapsed: std::time::Duration,
    pub carrier_owner_ephemeral_assembly_elapsed: std::time::Duration,
}

impl AdaptiveParentProfileDetailV1 {
    pub(super) fn record(&mut self, phase: &'static str, elapsed: std::time::Duration) {
        let destination = match phase {
            "physical topology" => &mut self.physical_topology_elapsed,
            "physical potential" => &mut self.physical_potential_elapsed,
            "physical request" => &mut self.physical_request_elapsed,
            "physical unified" => &mut self.physical_unified_elapsed,
            "unified preflight" => &mut self.unified_preflight_elapsed,
            "unified authorization" => &mut self.unified_authorization_elapsed,
            "unified entry validation" => &mut self.unified_entry_validation_elapsed,
            "unified protocol validation" => &mut self.unified_protocol_validation_elapsed,
            "unified candidate" => &mut self.unified_candidate_elapsed,
            "candidate soil" => &mut self.candidate_soil_elapsed,
            "candidate surface resource" => &mut self.candidate_surface_resource_elapsed,
            "candidate surface ingress" => &mut self.candidate_surface_ingress_elapsed,
            "candidate receivers" => &mut self.candidate_receivers_elapsed,
            "candidate validation" => &mut self.candidate_validation_elapsed,
            "physical final tile" => &mut self.physical_final_tile_elapsed,
            "physical protocol" => &mut self.physical_protocol_elapsed,
            "physical ingress" => &mut self.physical_ingress_elapsed,
            "physical post" => &mut self.physical_post_elapsed,
            "finalization candidate" => &mut self.finalization_candidate_elapsed,
            "finalization sealed source" => &mut self.finalization_sealed_source_elapsed,
            "finalization install" => &mut self.finalization_install_elapsed,
            "finalization identity replay" => &mut self.finalization_identity_replay_elapsed,
            "imported entry validation" => &mut self.imported_entry_validation_elapsed,
            "imported physical candidate" => &mut self.imported_physical_candidate_elapsed,
            "imported physical setup" => &mut self.imported_physical_setup_elapsed,
            "imported frozen evaluation" => &mut self.imported_frozen_evaluation_elapsed,
            "imported frozen preparation" => &mut self.imported_frozen_preparation_elapsed,
            "imported frozen execute accept" => &mut self.imported_frozen_execute_accept_elapsed,
            "imported frozen execution setup" => &mut self.imported_frozen_execution_setup_elapsed,
            "imported frozen runtime" => &mut self.imported_frozen_runtime_elapsed,
            "imported frozen acceptance" => &mut self.imported_frozen_acceptance_elapsed,
            "imported envelope construction" => &mut self.imported_envelope_construction_elapsed,
            "imported envelope validation" => &mut self.imported_envelope_validation_elapsed,
            "imported accepted candidate" => &mut self.imported_accepted_candidate_elapsed,
            "imported owner publication" => &mut self.imported_owner_publication_elapsed,
            "imported install" => &mut self.imported_install_elapsed,
            "imported reuse validation" => &mut self.imported_reuse_validation_elapsed,
            "imported reuse reseal" => &mut self.imported_reuse_reseal_elapsed,
            "imported reuse install" => &mut self.imported_reuse_install_elapsed,
            "terminal candidate setup" => &mut self.terminal_candidate_setup_elapsed,
            "terminal provider custody" => &mut self.terminal_provider_custody_elapsed,
            "terminal provider projection" => &mut self.terminal_provider_projection_elapsed,
            "terminal provider carrier" => &mut self.terminal_provider_carrier_elapsed,
            "terminal provider retention" => &mut self.terminal_provider_retention_elapsed,
            "terminal result finalization" => &mut self.terminal_result_finalization_elapsed,
            "carrier physical phase" => &mut self.carrier_physical_phase_elapsed,
            "carrier complete phase" => &mut self.carrier_complete_phase_elapsed,
            "carrier physical setup" => &mut self.carrier_physical_setup_elapsed,
            "carrier physical evidence" => &mut self.carrier_physical_evidence_elapsed,
            "carrier physical completion" => &mut self.carrier_physical_completion_elapsed,
            "carrier complete envelope" => &mut self.carrier_complete_envelope_elapsed,
            "carrier complete adoption" => &mut self.carrier_complete_adoption_elapsed,
            "carrier complete projection" => &mut self.carrier_complete_projection_elapsed,
            "carrier complete owner" => &mut self.carrier_complete_owner_elapsed,
            "carrier owner vegetation validation" => {
                &mut self.carrier_owner_vegetation_validation_elapsed
            }
            "carrier owner hydrology projection" => {
                &mut self.carrier_owner_hydrology_projection_elapsed
            }
            "carrier owner surface canonical" => &mut self.carrier_owner_surface_canonical_elapsed,
            "carrier owner vegetation encoding" => {
                &mut self.carrier_owner_vegetation_encoding_elapsed
            }
            "carrier owner surface encoding" => &mut self.carrier_owner_surface_encoding_elapsed,
            "carrier owner soil encoding" => &mut self.carrier_owner_soil_encoding_elapsed,
            "carrier owner other encoding" => &mut self.carrier_owner_other_encoding_elapsed,
            "carrier owner joint map" => &mut self.carrier_owner_joint_map_elapsed,
            "carrier owner joint seal" => &mut self.carrier_owner_joint_seal_elapsed,
            "carrier owner soil custody" => &mut self.carrier_owner_soil_custody_elapsed,
            "carrier owner candidate bytes" => &mut self.carrier_owner_candidate_bytes_elapsed,
            "carrier owner ephemeral assembly" => {
                &mut self.carrier_owner_ephemeral_assembly_elapsed
            }
            _ => return,
        };
        *destination = destination.saturating_add(elapsed);
    }
}

/// One accepted ordinary-covered support and the authentic physical maps
/// charged while constructing its selected adaptive trial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReleaseQualificationAcceptedCoveredMapV1 {
    pub support: TimeSupport,
    pub physical_map_evaluation_count: u64,
}

/// One successful call into the internal Lane-D per-OFE route operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReleaseQualificationLaneDRouteCallV1 {
    pub call_ordinal: u64,
    pub day_index: usize,
    pub lane_index: usize,
}

/// Exclusive, opt-in timing and counter evidence for the release harness.
///
/// `native_vegetation_et_elapsed` covers the whole snow-free native
/// vegetation/ET successor transaction envelope, so it is a conservative
/// upper bound rather than isolated component time. `stage3_lse_soil_elapsed`
/// covers the remaining Stage-3 day-preparation/LSE/soil envelope after nested
/// native and Lane-D scopes and is likewise a conservative upper bound.
///
/// This state is diagnostic only. It never enters owner state, receipts,
/// restart bytes, publication, or a numerical/controller decision.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReleaseQualificationTelemetryV1 {
    pub total_elapsed: Duration,
    pub native_vegetation_et_elapsed: Duration,
    pub stage3_lse_soil_elapsed: Duration,
    pub lane_d_elapsed: Duration,
    pub remaining_runner_elapsed: Duration,
    pub accepted_covered_maps: Vec<ReleaseQualificationAcceptedCoveredMapV1>,
    pub lane_d_route_calls: Vec<ReleaseQualificationLaneDRouteCallV1>,
    pub stage3_scope_entry_count: u64,
    pub native_vegetation_et_scope_entry_count: u64,
    pub lane_d_scope_entry_count: u64,
    pub scopes_balanced: bool,
    pub counters_complete: bool,
}

pub struct ReleaseQualificationTelemetryGuardV1 {
    _not_send: std::marker::PhantomData<Rc<()>>,
}

pub struct ReleaseQualificationProfileScopeV1 {
    bucket: ReleaseQualificationProfileBucketV1,
    active: bool,
    _not_send: std::marker::PhantomData<Rc<()>>,
}

pub(crate) struct ReleaseQualificationCoveredMapScopeV1 {
    support: TimeSupport,
    active: bool,
    _not_send: std::marker::PhantomData<Rc<()>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReleaseQualificationProfileBucketV1 {
    NativeVegetationEt,
    Stage3LseSoil,
    LaneD,
    RemainingRunner,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ReleaseQualificationCoveredMapCounterV1 {
    support: TimeSupport,
    count: u64,
}

struct ReleaseQualificationTelemetryStateV1 {
    last_transition: Instant,
    current_bucket: ReleaseQualificationProfileBucketV1,
    bucket_stack: Vec<ReleaseQualificationProfileBucketV1>,
    native_vegetation_et_elapsed: Duration,
    stage3_lse_soil_elapsed: Duration,
    lane_d_elapsed: Duration,
    remaining_runner_elapsed: Duration,
    active_covered_map: Option<ReleaseQualificationCoveredMapCounterV1>,
    accepted_covered_maps: Vec<ReleaseQualificationAcceptedCoveredMapV1>,
    lane_d_route_calls: Vec<ReleaseQualificationLaneDRouteCallV1>,
    stage3_scope_entry_count: u64,
    native_vegetation_et_scope_entry_count: u64,
    lane_d_scope_entry_count: u64,
    scopes_balanced: bool,
    counters_complete: bool,
}

impl ReleaseQualificationTelemetryStateV1 {
    fn new() -> Self {
        Self {
            last_transition: Instant::now(),
            current_bucket: ReleaseQualificationProfileBucketV1::RemainingRunner,
            bucket_stack: Vec::new(),
            native_vegetation_et_elapsed: Duration::ZERO,
            stage3_lse_soil_elapsed: Duration::ZERO,
            lane_d_elapsed: Duration::ZERO,
            remaining_runner_elapsed: Duration::ZERO,
            active_covered_map: None,
            accepted_covered_maps: Vec::new(),
            lane_d_route_calls: Vec::new(),
            stage3_scope_entry_count: 0,
            native_vegetation_et_scope_entry_count: 0,
            lane_d_scope_entry_count: 0,
            scopes_balanced: true,
            counters_complete: true,
        }
    }

    fn charge_until(&mut self, now: Instant) {
        let elapsed = now.saturating_duration_since(self.last_transition);
        let destination = match self.current_bucket {
            ReleaseQualificationProfileBucketV1::NativeVegetationEt => {
                &mut self.native_vegetation_et_elapsed
            }
            ReleaseQualificationProfileBucketV1::Stage3LseSoil => &mut self.stage3_lse_soil_elapsed,
            ReleaseQualificationProfileBucketV1::LaneD => &mut self.lane_d_elapsed,
            ReleaseQualificationProfileBucketV1::RemainingRunner => {
                &mut self.remaining_runner_elapsed
            }
        };
        *destination = destination.saturating_add(elapsed);
        self.last_transition = now;
    }
}

thread_local! {
    static RELEASE_QUALIFICATION_TELEMETRY: RefCell<Option<ReleaseQualificationTelemetryStateV1>> = const {
        RefCell::new(None)
    };
}

pub fn begin_release_qualification_telemetry_v1()
-> Result<ReleaseQualificationTelemetryGuardV1, &'static str> {
    RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        if state.is_some() {
            return Err("nested release qualification telemetry");
        }
        *state = Some(ReleaseQualificationTelemetryStateV1::new());
        Ok(ReleaseQualificationTelemetryGuardV1 {
            _not_send: std::marker::PhantomData,
        })
    })
}

pub fn take_release_qualification_telemetry_v1()
-> Result<ReleaseQualificationTelemetryV1, &'static str> {
    RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let mut state = state
            .take()
            .ok_or("release qualification telemetry was not begun")?;
        state.charge_until(Instant::now());
        let scopes_balanced = state.scopes_balanced
            && state.bucket_stack.is_empty()
            && state.current_bucket == ReleaseQualificationProfileBucketV1::RemainingRunner
            && state.active_covered_map.is_none();
        let total_elapsed = state
            .native_vegetation_et_elapsed
            .saturating_add(state.stage3_lse_soil_elapsed)
            .saturating_add(state.lane_d_elapsed)
            .saturating_add(state.remaining_runner_elapsed);
        Ok(ReleaseQualificationTelemetryV1 {
            total_elapsed,
            native_vegetation_et_elapsed: state.native_vegetation_et_elapsed,
            stage3_lse_soil_elapsed: state.stage3_lse_soil_elapsed,
            lane_d_elapsed: state.lane_d_elapsed,
            remaining_runner_elapsed: state.remaining_runner_elapsed,
            accepted_covered_maps: state.accepted_covered_maps,
            lane_d_route_calls: state.lane_d_route_calls,
            stage3_scope_entry_count: state.stage3_scope_entry_count,
            native_vegetation_et_scope_entry_count: state.native_vegetation_et_scope_entry_count,
            lane_d_scope_entry_count: state.lane_d_scope_entry_count,
            scopes_balanced,
            counters_complete: state.counters_complete,
        })
    })
}

fn enter_release_qualification_profile_scope_v1(
    bucket: ReleaseQualificationProfileBucketV1,
) -> ReleaseQualificationProfileScopeV1 {
    let active = RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return false;
        };
        state.charge_until(Instant::now());
        let entry_count = match bucket {
            ReleaseQualificationProfileBucketV1::NativeVegetationEt => {
                &mut state.native_vegetation_et_scope_entry_count
            }
            ReleaseQualificationProfileBucketV1::Stage3LseSoil => {
                &mut state.stage3_scope_entry_count
            }
            ReleaseQualificationProfileBucketV1::LaneD => &mut state.lane_d_scope_entry_count,
            ReleaseQualificationProfileBucketV1::RemainingRunner => {
                state.counters_complete = false;
                return false;
            }
        };
        let Some(next_entry_count) = entry_count.checked_add(1) else {
            state.counters_complete = false;
            return false;
        };
        *entry_count = next_entry_count;
        state.bucket_stack.push(state.current_bucket);
        state.current_bucket = bucket;
        true
    });
    ReleaseQualificationProfileScopeV1 {
        bucket,
        active,
        _not_send: std::marker::PhantomData,
    }
}

pub fn enter_release_qualification_stage3_scope_v1() -> ReleaseQualificationProfileScopeV1 {
    enter_release_qualification_profile_scope_v1(ReleaseQualificationProfileBucketV1::Stage3LseSoil)
}

pub(crate) fn enter_release_qualification_native_vegetation_et_scope_v1()
-> ReleaseQualificationProfileScopeV1 {
    enter_release_qualification_profile_scope_v1(
        ReleaseQualificationProfileBucketV1::NativeVegetationEt,
    )
}

pub(crate) fn enter_release_qualification_lane_d_scope_v1() -> ReleaseQualificationProfileScopeV1 {
    enter_release_qualification_profile_scope_v1(ReleaseQualificationProfileBucketV1::LaneD)
}

impl Drop for ReleaseQualificationProfileScopeV1 {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
            let mut state = state.borrow_mut();
            let Some(state) = state.as_mut() else {
                return;
            };
            state.charge_until(Instant::now());
            let prior = state.bucket_stack.pop();
            if state.current_bucket != self.bucket || prior.is_none() {
                state.scopes_balanced = false;
            }
            state.current_bucket =
                prior.unwrap_or(ReleaseQualificationProfileBucketV1::RemainingRunner);
        });
        self.active = false;
    }
}

pub(crate) fn begin_release_qualification_covered_map_scope_v1(
    support: TimeSupport,
) -> ReleaseQualificationCoveredMapScopeV1 {
    let active = RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return false;
        };
        if state.active_covered_map.is_some() {
            state.scopes_balanced = false;
            return false;
        }
        state.active_covered_map =
            Some(ReleaseQualificationCoveredMapCounterV1 { support, count: 0 });
        true
    });
    ReleaseQualificationCoveredMapScopeV1 {
        support,
        active,
        _not_send: std::marker::PhantomData,
    }
}

pub(crate) fn record_release_qualification_canonical_covered_map_v1() {
    RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return;
        };
        let Some(counter) = state.active_covered_map.as_mut() else {
            state.counters_complete = false;
            state.scopes_balanced = false;
            return;
        };
        if let Some(count) = counter.count.checked_add(1) {
            counter.count = count;
        } else {
            state.counters_complete = false;
        }
    });
}

impl ReleaseQualificationCoveredMapScopeV1 {
    pub(crate) fn finish(mut self) -> Option<ReleaseQualificationAcceptedCoveredMapV1> {
        if !self.active {
            return None;
        }
        let record = RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
            let mut state = state.borrow_mut();
            let state = state.as_mut()?;
            let counter = state.active_covered_map.take()?;
            if counter.support != self.support {
                state.scopes_balanced = false;
                return None;
            }
            if counter.count == 0 {
                state.counters_complete = false;
                return None;
            }
            Some(ReleaseQualificationAcceptedCoveredMapV1 {
                support: counter.support,
                physical_map_evaluation_count: counter.count,
            })
        });
        self.active = false;
        record
    }
}

impl Drop for ReleaseQualificationCoveredMapScopeV1 {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
            if let Some(state) = state.borrow_mut().as_mut() {
                let counter = state.active_covered_map.take();
                if counter.is_none_or(|counter| counter.support != self.support) {
                    state.scopes_balanced = false;
                }
            }
        });
        self.active = false;
    }
}

pub(crate) fn record_release_qualification_accepted_covered_maps_v1(
    records: &[ReleaseQualificationAcceptedCoveredMapV1],
) {
    RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return;
        };
        if records
            .iter()
            .any(|record| record.physical_map_evaluation_count == 0)
        {
            state.counters_complete = false;
            return;
        }
        state.accepted_covered_maps.extend_from_slice(records);
    });
}

pub(crate) fn record_release_qualification_lane_d_route_call_v1(
    day_index: usize,
    lane_index: usize,
) {
    RELEASE_QUALIFICATION_TELEMETRY.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return;
        };
        if state.current_bucket != ReleaseQualificationProfileBucketV1::LaneD {
            state.counters_complete = false;
            state.scopes_balanced = false;
            return;
        }
        let Ok(call_ordinal) = u64::try_from(state.lane_d_route_calls.len()) else {
            state.counters_complete = false;
            return;
        };
        state
            .lane_d_route_calls
            .push(ReleaseQualificationLaneDRouteCallV1 {
                call_ordinal,
                day_index,
                lane_index,
            });
    });
}

#[cfg(test)]
mod release_qualification_tests {
    use super::*;
    use openwepp_coupled_time::ModelTimeNs;

    fn support(start: u128, end: u128) -> TimeSupport {
        TimeSupport::new(ModelTimeNs::new(start), ModelTimeNs::new(end)).expect("test support")
    }

    #[test]
    fn qualification_map_counts_publish_only_when_explicitly_accepted() {
        let guard = begin_release_qualification_telemetry_v1().expect("begin telemetry");
        let rejected = begin_release_qualification_covered_map_scope_v1(support(0, 1));
        record_release_qualification_canonical_covered_map_v1();
        drop(rejected);

        let accepted = begin_release_qualification_covered_map_scope_v1(support(1, 2));
        for _ in 0..3 {
            record_release_qualification_canonical_covered_map_v1();
        }
        let accepted = accepted.finish().expect("accepted map record");
        record_release_qualification_accepted_covered_maps_v1(&[accepted]);

        let telemetry = take_release_qualification_telemetry_v1().expect("take telemetry");
        drop(guard);
        assert_eq!(telemetry.accepted_covered_maps, vec![accepted]);
        assert_eq!(accepted.physical_map_evaluation_count, 3);
        assert!(telemetry.scopes_balanced);
        assert!(telemetry.counters_complete);
    }

    #[test]
    fn qualification_map_counter_fails_closed_when_unscoped_or_zero() {
        let guard = begin_release_qualification_telemetry_v1().expect("begin telemetry");
        record_release_qualification_canonical_covered_map_v1();
        let telemetry = take_release_qualification_telemetry_v1().expect("take telemetry");
        drop(guard);
        assert!(telemetry.accepted_covered_maps.is_empty());
        assert!(!telemetry.counters_complete);
        assert!(!telemetry.scopes_balanced);

        let guard = begin_release_qualification_telemetry_v1().expect("begin telemetry");
        let disconnected = begin_release_qualification_covered_map_scope_v1(support(0, 1));
        assert_eq!(disconnected.finish(), None);
        let telemetry = take_release_qualification_telemetry_v1().expect("take telemetry");
        drop(guard);
        assert!(telemetry.accepted_covered_maps.is_empty());
        assert!(!telemetry.counters_complete);
        assert!(telemetry.scopes_balanced);
    }

    #[test]
    fn qualification_lane_d_event_outside_lane_scope_fails_closed() {
        let guard = begin_release_qualification_telemetry_v1().expect("begin telemetry");
        record_release_qualification_lane_d_route_call_v1(0, 0);
        let telemetry = take_release_qualification_telemetry_v1().expect("take telemetry");
        drop(guard);
        assert!(telemetry.lane_d_route_calls.is_empty());
        assert!(!telemetry.counters_complete);
        assert!(!telemetry.scopes_balanced);
        assert_eq!(telemetry.lane_d_scope_entry_count, 0);
    }

    #[test]
    fn qualification_take_closes_timing_window_before_post_run_work() {
        let guard = begin_release_qualification_telemetry_v1().expect("begin telemetry");
        {
            let _stage3 = enter_release_qualification_stage3_scope_v1();
            std::hint::black_box(1_u64);
        }
        let telemetry = take_release_qualification_telemetry_v1().expect("take telemetry");
        let captured_total = telemetry.total_elapsed;
        std::thread::sleep(Duration::from_millis(2));
        drop(guard);
        assert_eq!(telemetry.total_elapsed, captured_total);
        assert_eq!(
            take_release_qualification_telemetry_v1(),
            Err("release qualification telemetry was not begun")
        );
    }

    #[test]
    fn qualification_profile_is_exclusive_balanced_and_preserves_route_order() {
        let guard = begin_release_qualification_telemetry_v1().expect("begin telemetry");
        {
            let _stage3 = enter_release_qualification_stage3_scope_v1();
            {
                let _native = enter_release_qualification_native_vegetation_et_scope_v1();
                std::hint::black_box(1_u64);
            }
            {
                let _lane_d = enter_release_qualification_lane_d_scope_v1();
                record_release_qualification_lane_d_route_call_v1(0, 0);
                record_release_qualification_lane_d_route_call_v1(0, 1);
            }
        }
        let telemetry = take_release_qualification_telemetry_v1().expect("take telemetry");
        drop(guard);
        assert!(telemetry.scopes_balanced);
        assert!(telemetry.counters_complete);
        assert_eq!(telemetry.stage3_scope_entry_count, 1);
        assert_eq!(telemetry.native_vegetation_et_scope_entry_count, 1);
        assert_eq!(telemetry.lane_d_scope_entry_count, 1);
        assert_eq!(
            telemetry.total_elapsed,
            telemetry
                .native_vegetation_et_elapsed
                .saturating_add(telemetry.stage3_lse_soil_elapsed)
                .saturating_add(telemetry.lane_d_elapsed)
                .saturating_add(telemetry.remaining_runner_elapsed)
        );
        assert_eq!(
            telemetry.lane_d_route_calls,
            vec![
                ReleaseQualificationLaneDRouteCallV1 {
                    call_ordinal: 0,
                    day_index: 0,
                    lane_index: 0,
                },
                ReleaseQualificationLaneDRouteCallV1 {
                    call_ordinal: 1,
                    day_index: 0,
                    lane_index: 1,
                },
            ]
        );
    }
}
