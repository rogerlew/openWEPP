use openwepp_runner::{
    HillslopeCliError, HillslopeRunReport, HillslopeRunRequest, Stage3TestFixtureSeedBinding,
    Stage3TestFixtureSeedProfile, author_stage3_v11_owner_seed_fixture, execute_hillslope_run,
};

#[allow(dead_code)]
pub(crate) fn bind_complete_stage3_owner_seed(request: &HillslopeRunRequest) {
    bind_stage3_owner_seed(request, Stage3TestFixtureSeedProfile::CompleteOwner);
}

#[allow(dead_code)]
pub(crate) fn bind_adaptive_stage3_owner_seed(request: &HillslopeRunRequest) {
    bind_stage3_owner_seed(request, Stage3TestFixtureSeedProfile::AdaptiveNoStrataOwner);
}

fn bind_stage3_owner_seed(request: &HillslopeRunRequest, profile: Stage3TestFixtureSeedProfile) {
    let binding = if request.legacy_sidecar_discovery {
        Stage3TestFixtureSeedBinding::LegacyDiscovery
    } else {
        Stage3TestFixtureSeedBinding::ExplicitRunfile
    };
    author_stage3_v11_owner_seed_fixture(request, profile, binding)
        .expect("author and explicitly bind exact Stage-3 owner-seed fixture");
}

#[allow(dead_code)]
pub(crate) fn execute_with_complete_stage3_owner_seed(
    request: &HillslopeRunRequest,
    argv: &[String],
) -> Result<HillslopeRunReport, HillslopeCliError> {
    bind_complete_stage3_owner_seed(request);
    execute_hillslope_run(request, argv)
}

#[allow(dead_code)]
pub(crate) fn execute_with_adaptive_stage3_owner_seed(
    request: &HillslopeRunRequest,
    argv: &[String],
) -> Result<HillslopeRunReport, HillslopeCliError> {
    bind_adaptive_stage3_owner_seed(request);
    execute_hillslope_run(request, argv)
}

/// Execute an intentionally invalid fixture whose asserted failure occurs
/// while loading the runfile or its typed inputs, before Stage-3 bootstrap can
/// consult owner-seed authority.
#[allow(dead_code)]
pub(crate) fn execute_pre_stage3_validation_failure(
    request: &HillslopeRunRequest,
    argv: &[String],
) -> Result<HillslopeRunReport, HillslopeCliError> {
    execute_hillslope_run(request, argv)
}
