#[derive(Debug, Clone)]
struct Ws20ChannelProfile {
    x_points_ft: Vec<f64>,
    slopes: Vec<f64>,
    depth_a_points_ft: Vec<f64>,
    depth_b_points_ft: Vec<f64>,
    width_a_points_ft: Vec<f64>,
    width_b_points_ft: Vec<f64>,
    eroded_width_a_points_ft: Vec<f64>,
    eroded_width_b_points_ft: Vec<f64>,
}

#[derive(Debug, Clone)]
struct Ws20ClassTransportState {
    gstu_lbs_s: Vec<f64>,
    dlat_lbs_s_ft: Vec<f64>,
    crdia_ft: Vec<f64>,
    crspg: Vec<f64>,
    fall_ft_s: Vec<f64>,
}

#[derive(Debug, Clone)]
struct Ws20SegmentHydraulics {
    segment_index: usize,
    x_upper_ft: f64,
    x_lower_ft: f64,
    dx_ft: f64,
    qu_cfs: f64,
    ql_cfs: f64,
    wfu_ft: f64,
    wfl_ft: f64,
    effshu: f64,
    effshl: f64,
    upper_flagc: i32,
    lower_flagc: i32,
}

struct Ws20SegmentProcessOutcome {
    detached_lbs_s: Vec<f64>,
    max_effective_shear_lb_ft2: f64,
    outlet_transport_capacity_lbs_s: Vec<f64>,
}

#[derive(Debug, Clone)]
struct Ws20TransportSnapshot {
    gsu_lbs_s_ft: Vec<f64>,
    tcu_lbs_s_ft: Vec<f64>,
    potld_lbs_s_ft: Vec<f64>,
    tcl_lbs_s_ft: Vec<f64>,
    dtcdx_lbs_s_ft2: Vec<f64>,
    phi: Vec<f64>,
    excess: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ws20Case12ClassKind {
    Case1,
    Case2,
}

#[derive(Debug, Clone, Copy)]
struct Ws20Case12ClassUpdate {
    next_flux_lbs_s: f64,
    xde_ft: f64,
    gstde_lbs_s: f64,
    case_kind: Ws20Case12ClassKind,
}

#[derive(Debug, Clone, Copy)]
struct Ws20RouteContext<'a> {
    node_class: Ws10NodeClass,
    ws21_case34_enabled: bool,
    event_duration: f64,
    t_exp_s: Option<f64>,
    t_norm_s: f64,
    roughness: f64,
    sediment_controls: Ws15ChannelSedimentControls,
    class_numbers: &'a [usize],
    qu_top_cfs: f64,
    qlat_cfs_per_ft: f64,
    flagct: i32,
    crsh: f64,
    chnk: f64,
    crfrac: Option<&'a [f64]>,
}

#[derive(Debug, Clone, Copy)]
struct Ws18HydchnArgs {
    node_class: Ws10NodeClass,
    flag: i32,
    q_cfs: f64,
    sf: f64,
    c1: f64,
    z: f64,
    wb: f64,
    n_total: f64,
    crsh: f64,
    nbarch: f64,
}

#[derive(Debug, Clone, Copy)]
struct Ws18HydchnGeometry {
    w: f64,
    a: f64,
    nt: f64,
}

#[derive(Debug, Clone, Copy)]
enum Ws18HydchnStep {
    Geometry(Ws18HydchnGeometry),
    Reclassify(i32),
}

#[derive(Debug, Clone)]
struct Ws18TrncapState {
    coef: Vec<f64>,
    p: Vec<f64>,
    dltrat: Vec<f64>,
    ws: Vec<f64>,
    qs_local: Vec<f64>,
}

#[derive(Debug, Clone, Copy)]
struct Ws26DcapInput<'a> {
    node_class: Ws10NodeClass,
    flagm: i32,
    q_cfs: f64,
    sf: f64,
    c1: f64,
    z: f64,
    effsh: f64,
    depsid: f64,
    wflow: f64,
    roughness: f64,
    crsh: f64,
    excess: f64,
    tb: f64,
    flagt: i32,
    chnk: f64,
    nbarch: f64,
    maxe: f64,
    crfrac: &'a [f64],
}

#[derive(Debug, Clone)]
struct Ws26DcapLayerState {
    df: Vec<f64>,
    depmid: f64,
    werod: f64,
    timpot: f64,
    timsh: f64,
    di: f64,
}

#[derive(Debug, Clone)]
enum Ws26DcapLayerStep {
    Complete(Ws26DcapOutcome),
    Continue(Ws26DcapLayerState),
}

#[derive(Debug, Clone, Copy)]
struct Ws23DetachInput<'a> {
    node_class: Ws10NodeClass,
    ql_cfs: f64,
    sfl: f64,
    c1: f64,
    z: f64,
    effshl: f64,
    depsid_ft: f64,
    depmid_ft: f64,
    wfl_ft: f64,
    werod_ft: f64,
    roughness: f64,
    crsh: f64,
    tb_s: f64,
    t_exp_s: Option<f64>,
    flagc: i32,
    chnk: f64,
    nbarch: f64,
    crfrac: &'a [f64],
    gstu_lbs_s: &'a [f64],
    dlat_lbs_s_ft: &'a [f64],
    du_lbs_s_ft: &'a [f64],
    dx_ft: f64,
    crdia_ft: &'a [f64],
    crspg: &'a [f64],
}

#[derive(Debug, Clone)]
struct Ws23DetachWorking {
    dcap_outcome: Ws26DcapOutcome,
    df_lbs_s_ft2: Vec<f64>,
    dl_lbs_s_ft: Vec<f64>,
    potld_lbs_s_ft: Vec<f64>,
    tcl_lbs_s_ft: Vec<f64>,
}

#[derive(Debug, Clone)]
enum Ws23DetachStart {
    Complete(Ws23DetachClosureOutcome),
    Iterate(Ws23DetachWorking),
}

#[derive(Debug, Clone, Copy)]
struct Ws23DetachSums {
    sumtcl: f64,
    sumpld: f64,
    sumdf: f64,
    sumexd: f64,
}

include!("routing/00_ws15_ws18_scaffold_and_hydraulics.rs");
include!("routing/01_ws22_ws23_ws26_detachment.rs");
include!("routing/02_ws20_segment_routing.rs");
