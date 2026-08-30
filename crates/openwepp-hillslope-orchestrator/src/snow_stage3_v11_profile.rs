//! Non-persisted wall-time attribution for the opt-in Stage-3 parent guard.

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
            _ => return,
        };
        *destination = destination.saturating_add(elapsed);
    }
}
