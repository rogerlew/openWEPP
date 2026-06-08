#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Ws10NodeClass {
    Channel,
    Impoundment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ws10GuardClass {
    MissingRequiredInput,
    NonFinite,
    DomainViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ws11IpeakBranch {
    Rational,
    Creams,
    KinematicWave,
    MuskingumCunge,
    MuskingumCungeVariable,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws11WaveRoutingState {
    q1: f64,
    qin: f64,
    qlat: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws12ImpoundmentCoefficients {
    a: [f64; 15],
    b: [f64; 15],
    c: [f64; 15],
    d: [f64; 15],
    e: [f64; 15],
    ha: [f64; 15],
    a0: f64,
    a1: f64,
    a2: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws15ChannelSedimentControls {
    ishape: f64,
    ctlz: f64,
    chnz: f64,
    chnnbr: f64,
    chntcr: f64,
    chnedm: f64,
    chneds: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Ws18HillslopeSedimentPayload {
    mass_kg: f64,
    fractions: Vec<f64>,
    particle_diameters_m: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
struct Ws19ChannelSedimentPublication {
    qsed: f64,
    tc: f64,
    particle_flow_fractions: Vec<f64>,
    particle_diameters_m: Vec<f64>,
    ws29_widb_points_ft: Option<Vec<f64>>,
    ws31_wida_points_ft: Option<Vec<f64>>,
    ws20_case1_segments: u32,
    ws20_case2_segments: u32,
    ws24_case2_detach_segments: u32,
    ws21_case3_segments: u32,
    ws21_case4_segments: u32,
    ws21_enddet_segments: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws20IncomingPeakPartition {
    hillslope_peak_cms: f64,
    dependency_peak_cms: f64,
    hillslope_volume_m3: f64,
    dependency_volume_m3: f64,
    hillslope_duration_s: f64,
    dependency_duration_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::struct_field_names)]
struct Ws20SegmentRoutingDiagnostics {
    case1_segments: u32,
    case2_segments: u32,
    ws24_case2_detach_segments: u32,
    case3_segments: u32,
    case4_segments: u32,
    enddet_segments: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct Ws20SegmentRoutingResult {
    routed_class_masses_kg: Vec<f64>,
    diagnostics: Ws20SegmentRoutingDiagnostics,
    widb_points_ft: Vec<f64>,
    wida_points_ft: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Ws27EnddetBracketProgress {
    pub(crate) used_xdbig_rebracket: bool,
    pub(crate) used_midpoint_rebracket: bool,
    pub(crate) iteration_count: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Ws26DcapOutcome {
    pub(crate) df_lbs_s_ft2: Vec<f64>,
    pub(crate) depmid_ft: f64,
    pub(crate) werod_ft: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Ws23DetachClosureOutcome {
    next_gstu_lbs_s: Vec<f64>,
    werod_ft: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Ws15ChannelSedimentScaffold {
    chz: f64,
    nbarch: f64,
    crsh: f64,
    depmid: f64,
    depsid: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Ws10GuardError {
    node_class: Ws10NodeClass,
    guard_class: Ws10GuardClass,
}

impl Ws10GuardError {
    #[must_use]
    fn message_id(&self) -> &'static str {
        match (self.node_class, self.guard_class) {
            (Ws10NodeClass::Channel, Ws10GuardClass::MissingRequiredInput) => {
                WS10_CHANNEL_GUARD_MISSING_SYMBOL
            }
            (Ws10NodeClass::Channel, Ws10GuardClass::NonFinite) => WS10_CHANNEL_GUARD_NON_FINITE,
            (Ws10NodeClass::Channel, Ws10GuardClass::DomainViolation) => WS10_CHANNEL_GUARD_DOMAIN,
            (Ws10NodeClass::Impoundment, Ws10GuardClass::MissingRequiredInput) => {
                WS10_IMPOUNDMENT_GUARD_MISSING_SYMBOL
            }
            (Ws10NodeClass::Impoundment, Ws10GuardClass::NonFinite) => {
                WS10_IMPOUNDMENT_GUARD_NON_FINITE
            }
            (Ws10NodeClass::Impoundment, Ws10GuardClass::DomainViolation) => {
                WS10_IMPOUNDMENT_GUARD_DOMAIN
            }
        }
    }

    #[must_use]
    const fn boundary_class(&self) -> BoundaryClass {
        match self.guard_class {
            Ws10GuardClass::MissingRequiredInput => BoundaryClass::MissingRequiredInput,
            Ws10GuardClass::NonFinite => BoundaryClass::NonFinite,
            Ws10GuardClass::DomainViolation => BoundaryClass::DomainViolation,
        }
    }
}

/// WS10 production watershed kernel for channel and impoundment execution.
#[derive(Debug, Default, Clone, Copy)]
pub struct Ws10ChannelImpoundmentKernel;

