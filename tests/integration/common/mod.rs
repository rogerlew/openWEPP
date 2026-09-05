use openwepp_runner::{
    HillslopeCliError, HillslopeRunReport, HillslopeRunRequest, Stage3TestFixtureSeedBinding,
    Stage3TestFixtureSeedProfile, author_stage3_v11_owner_seed_fixture, execute_hillslope_run,
};

#[allow(dead_code)]
pub(crate) fn bind_complete_stage3_owner_seed(request: &HillslopeRunRequest) {
    bind_frozen_litter_v3_stage3_owner_seed(request, Stage3TestFixtureSeedProfile::CompleteOwner);
}

#[allow(dead_code)]
pub(crate) fn bind_adaptive_stage3_owner_seed(request: &HillslopeRunRequest) {
    // Ordinary CLI contracts exercise the production forest-litter surface.
    // That surface requires its native configured vegetation occupancy; the
    // historical no-strata fixture is a bare/open diagnostic posture and is
    // not a valid forest-litter production owner.
    bind_frozen_litter_v3_stage3_owner_seed(request, Stage3TestFixtureSeedProfile::CompleteOwner);
}

/// Author the retained V1 bootstrap-provenance wire whose checked production
/// bootstrap installs the native `OPENWEPP_SNOW_FREE_LSE_V3`/surface-V2
/// resident before any Stage-3 support executes. The wire is not V1/V2
/// execution or consumer evidence.
fn bind_frozen_litter_v3_stage3_owner_seed(
    request: &HillslopeRunRequest,
    profile: Stage3TestFixtureSeedProfile,
) {
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
