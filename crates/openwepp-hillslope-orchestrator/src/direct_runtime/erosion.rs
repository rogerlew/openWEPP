use crate::constants::WB11_ZERO_THRESHOLD;

use super::{
    DIRECT_AUDIT, DIRECT_R7D6_EROSION_PHASE_SPAN_COUNT, DirectDayFrame,
    DirectPublicationErosionOperands, DirectRuntimeError, validate_finite,
    validate_nonnegative_direct_m,
};

const DIRECT_EROSION_CLASS_LIMIT: usize = 5;
const DIRECT_EROD13_CONTINUITY_TOLERANCE: f64 = 1.0e-9;
const DIRECT_EROD13_MIN_TCADJF: f64 = 0.30;
const DIRECT_EROD14_CASE_TOLERANCE: f64 = 1.0e-12;
const DIRECT_EROD14_MAX_PHI: f64 = 100_000.0;
const DIRECT_EROD14_PKRO_ZERO_THRESHOLD: f64 = 1.0e-15;
const DIRECT_EROD14_ATTENUATION_FLOOR: f64 = 1.0e-8;
const DIRECT_EROD14_CLASS_FLOOR: f64 = 1.0e-15;
const DIRECT_EROD14_MAX_REPROPORTION_ITERS: usize = 64;
const DIRECT_EROD14_ENRICHMENT_RATIO_OFFSET: f64 = 0.005;
const DIRECT_EROD14_QOSTAR_DELTA_THRESHOLD: f64 = 1.0e-10;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionInputs {
    pub wave1_enabled: bool,
    pub wave2_enabled: bool,
    pub wave1: DirectErod13Inputs,
    pub wave2: DirectErod14Inputs,
}

impl DirectErosionInputs {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            wave1_enabled: false,
            wave2_enabled: false,
            wave1: DirectErod13Inputs::zero(),
            wave2: DirectErod14Inputs::zero(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErod13Inputs {
    pub ie_m_s: f64,
    pub te_s: f64,
    pub fs: f64,
    pub ft: f64,
    pub taufe_pa: f64,
    pub q_m2_s: f64,
    pub g_kg_s_m: f64,
    pub di_kg_s_m2: f64,
    pub beta: f64,
    pub vf_m_s: f64,
    pub dgdx_kg_s_m2: f64,
    pub cntlen_m: f64,
    pub kr_s_m: f64,
    pub kradjf: f64,
    pub tcadjf: f64,
    pub shrsol_pa: f64,
    pub tcend_kg_s_m: f64,
    pub shcrit_pa: f64,
    pub detinr_kg_s_m2: f64,
    pub effdrr_m: f64,
    pub effdrn_m: f64,
    pub veleff_m_s: f64,
    pub pkro_m3_s: f64,
    pub tc_k: f64,
    pub tc_m: f64,
    pub q_runoff_m: f64,
    pub peakro_m3_s: f64,
    pub watdur_s: f64,
}

impl DirectErod13Inputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            ie_m_s: 0.0,
            te_s: 0.0,
            fs: 0.0,
            ft: 0.0,
            taufe_pa: 0.0,
            q_m2_s: 0.0,
            g_kg_s_m: 0.0,
            di_kg_s_m2: 0.0,
            beta: 0.0,
            vf_m_s: 0.0,
            dgdx_kg_s_m2: 0.0,
            cntlen_m: 0.0,
            kr_s_m: 0.0,
            kradjf: 0.0,
            tcadjf: 0.0,
            shrsol_pa: 0.0,
            tcend_kg_s_m: 0.0,
            shcrit_pa: 0.0,
            detinr_kg_s_m2: 0.0,
            effdrr_m: 0.0,
            effdrn_m: 0.0,
            veleff_m_s: 0.0,
            pkro_m3_s: 0.0,
            tc_k: 0.0,
            tc_m: 0.0,
            q_runoff_m: 0.0,
            peakro_m3_s: 0.0,
            watdur_s: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErod13State {
    pub tau_f_pa: f64,
    pub dc_kg_s_m2: f64,
    pub tc_kg_s_m: f64,
    pub df_kg_s_m2: f64,
    pub eta: f64,
    pub taucn: f64,
    pub theta: f64,
    pub phi: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErod14Inputs {
    pub xtop_m: f64,
    pub xbot_m: f64,
    pub xdetst_m: f64,
    pub ldtop_kg_s_m: f64,
    pub ldbot_kg_s_m: f64,
    pub lddend_kg: f64,
    pub qout_m3_s: f64,
    pub qin_m3_s: f64,
    pub qostar_m: f64,
    pub hbp_sediment_concentration_scale: f64,
    pub slplen_m: f64,
    pub ktrato: f64,
    pub aintc: f64,
    pub bintc: f64,
    pub cintc: f64,
    pub beta: f64,
    pub qj_minus_1_m3_s: f64,
    pub vj_m: f64,
    pub qj_m3_s: f64,
    pub fh_m: f64,
    pub fp_m: f64,
    pub case_value: f64,
    pub peak_runoff_m3_s: f64,
    pub runoff_duration_s: f64,
    pub ssa_soil: f64,
    pub theta: f64,
    pub classes: Vec<DirectErod14ClassInputs>,
}

impl DirectErod14Inputs {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            xtop_m: 0.0,
            xbot_m: 0.0,
            xdetst_m: 0.0,
            ldtop_kg_s_m: 0.0,
            ldbot_kg_s_m: 0.0,
            lddend_kg: 0.0,
            qout_m3_s: 0.0,
            qin_m3_s: 0.0,
            qostar_m: 0.0,
            hbp_sediment_concentration_scale: 1.0,
            slplen_m: 0.0,
            ktrato: 0.0,
            aintc: 0.0,
            bintc: 0.0,
            cintc: 0.0,
            beta: 0.0,
            qj_minus_1_m3_s: 0.0,
            vj_m: 0.0,
            qj_m3_s: 0.0,
            fh_m: 0.0,
            fp_m: 0.0,
            case_value: 0.0,
            peak_runoff_m3_s: 0.0,
            runoff_duration_s: 0.0,
            ssa_soil: 0.0,
            theta: 0.0,
            classes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErod14ClassInputs {
    pub fall_m_s: f64,
    pub frcflw: f64,
    pub frac: f64,
    pub fidel: f64,
    pub tcf1: f64,
    pub ssa_class: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErod14State {
    pub class_count: usize,
    pub sumg_kg: f64,
    pub er: f64,
    pub lddend_kg: f64,
    pub qout_m3_s: f64,
    pub peak_runoff_m3_s: f64,
    pub runoff_duration_s: f64,
    pub classes: Vec<DirectErod14ClassState>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErod14ClassState {
    pub gend_kg: f64,
    pub sedmax_kg: f64,
    pub particle_flow_fraction: f64,
    pub sed_frac: f64,
    pub sediment_concentration_kg_m3: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionState {
    pub wave1: Option<DirectErod13State>,
    pub wave2: Option<DirectErod14State>,
    pub publication_authority: bool,
    pub publication: DirectPublicationErosionOperands,
}

impl DirectErosionState {
    #[must_use]
    pub fn inactive() -> Self {
        Self {
            wave1: None,
            wave2: None,
            publication_authority: false,
            publication: DirectPublicationErosionOperands::zero_authority(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionDownstreamOperands {
    pub publication_authority: bool,
    pub qout_handoff_authority: bool,
    pub publication: DirectPublicationErosionOperands,
    pub qout_m3_s: f64,
    pub particle_flow_fraction: [f64; DIRECT_EROSION_CLASS_LIMIT],
}

impl DirectErosionDownstreamOperands {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            publication_authority: false,
            qout_handoff_authority: false,
            publication: DirectPublicationErosionOperands::zero_authority(),
            qout_m3_s: 0.0,
            particle_flow_fraction: [0.0; DIRECT_EROSION_CLASS_LIMIT],
        }
    }

    fn from_state(state: &DirectErosionState) -> Self {
        let mut particle_flow_fraction = [0.0; DIRECT_EROSION_CLASS_LIMIT];
        let mut qout_m3_s = 0.0;
        if let Some(wave2) = &state.wave2 {
            qout_m3_s = wave2.qout_m3_s;
            for (index, class_state) in wave2.classes.iter().enumerate() {
                if index < DIRECT_EROSION_CLASS_LIMIT {
                    particle_flow_fraction[index] = class_state.particle_flow_fraction;
                }
            }
        }
        Self {
            publication_authority: state.publication_authority,
            qout_handoff_authority: true,
            publication: state.publication,
            qout_m3_s,
            particle_flow_fraction,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub wave1_active: bool,
    pub wave2_active: bool,
    pub publication_authority: bool,
    pub publication: DirectPublicationErosionOperands,
    pub qout_m3_s: f64,
    pub particle_flow_fraction: [f64; DIRECT_EROSION_CLASS_LIMIT],
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectErosionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub erosion_shadow_projection: DirectErosionShadowProjection,
}

impl DirectDayFrame {
    pub fn run_r7d6_erosion_span(&mut self) -> Result<DirectErosionSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R7D6_EROSION_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let erosion = self.compute_r7d6_erosion()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.erosion = erosion;
        DIRECT_AUDIT.record_direct_state_mutation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.erosion_downstream_operands =
            DirectErosionDownstreamOperands::from_state(&self.erosion);
        DIRECT_AUDIT.record_downstream_operand_production();

        let erosion_shadow_projection = DirectErosionShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            wave1_active: self.erosion.wave1.is_some(),
            wave2_active: self.erosion.wave2.is_some(),
            publication_authority: self.erosion_downstream_operands.publication_authority,
            publication: self.erosion_downstream_operands.publication,
            qout_m3_s: self.erosion_downstream_operands.qout_m3_s,
            particle_flow_fraction: self.erosion_downstream_operands.particle_flow_fraction,
        };
        self.erosion_shadow_projection = Some(erosion_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectErosionSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            erosion_shadow_projection,
        })
    }

    fn compute_r7d6_erosion(&self) -> Result<DirectErosionState, DirectRuntimeError> {
        // Check the enable flags before r7d8 clones the inputs (the Wave-2
        // class Vec makes that clone a per-OFE-day allocation).
        if !self.erosion_inputs.wave1_enabled && !self.erosion_inputs.wave2_enabled {
            return Ok(DirectErosionState::inactive());
        }
        let erosion_inputs = self.r7d8_erosion_inputs_with_runoff_authority()?;

        let wave1 = if erosion_inputs.wave1_enabled {
            Some(compute_direct_erod13(&erosion_inputs.wave1)?)
        } else {
            None
        };
        let wave2 = if erosion_inputs.wave2_enabled {
            let theta = wave1
                .as_ref()
                .map_or(erosion_inputs.wave2.theta, |state| state.theta);
            Some(compute_direct_erod14(&erosion_inputs.wave2, theta)?)
        } else {
            None
        };
        let publication = if let Some(wave2) = &wave2 {
            direct_erod15_publication_projection(wave2, &erosion_inputs.wave2)?
        } else {
            DirectPublicationErosionOperands::zero_authority()
        };

        Ok(DirectErosionState {
            wave1,
            wave2,
            publication_authority: erosion_inputs.wave2_enabled,
            publication,
        })
    }

    fn r7d8_erosion_inputs_with_runoff_authority(
        &self,
    ) -> Result<DirectErosionInputs, DirectRuntimeError> {
        let mut inputs = self.erosion_inputs.clone();
        if !inputs.wave1_enabled && !inputs.wave2_enabled {
            return Ok(inputs);
        }
        let peak_runoff = self.peak_runoff_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R7D6 WB16 peak-duration producer",
            },
        )?;
        if inputs.wave1_enabled {
            inputs.wave1.q_runoff_m = peak_runoff.q_runoff_m;
            inputs.wave1.peakro_m3_s = peak_runoff.peak_runoff_m3_s;
            inputs.wave1.watdur_s = peak_runoff.runoff_duration_s;
        }
        if inputs.wave2_enabled {
            inputs.wave2.peak_runoff_m3_s = peak_runoff.peak_runoff_m3_s;
            inputs.wave2.runoff_duration_s = peak_runoff.runoff_duration_s;
            let runoff = self.runoff_shadow_projection.as_ref().ok_or(
                DirectRuntimeError::MissingDirectUpstream {
                    upstream: "R7D8 direct runoff producer for EROD14 qout",
                },
            )?;
            if runoff.q_runoff_m > WB11_ZERO_THRESHOLD {
                inputs.wave2.qout_m3_s = runoff.q_runoff_m;
            }
            inputs.wave2.qin_m3_s = if self.lane_index == 0 {
                0.0
            } else if self
                .upstream_erosion_downstream_operands
                .qout_handoff_authority
            {
                self.upstream_erosion_downstream_operands.qout_m3_s
            } else {
                return Err(DirectRuntimeError::MissingDirectUpstream {
                    upstream: "R7D8 prior-lane erosion qout for EROD14 qin",
                });
            };
            // DC01 (INV-RUNOFFPART-031 / INV-RUNOFFPART-030 hold): with runon
            // re-infiltration, a downslope OFE can absorb upstream inflow, so
            // qout < qin is physically expected (decreasing-flow case). Full
            // decreasing-flow sediment cases remain in the INV-030 hold; the
            // bounded interim behavior clamps qin to qout and counts the
            // occurrence for the manifest.
            if inputs.wave2.qin_m3_s > inputs.wave2.qout_m3_s + WB11_ZERO_THRESHOLD {
                DIRECT_AUDIT.record_erod14_qin_clamped_event();
                inputs.wave2.qin_m3_s = inputs.wave2.qout_m3_s;
            }
            apply_erod14_prior_particle_flow_fraction(
                &mut inputs.wave2,
                &self.prior_erosion_downstream_operands,
            )?;
            inputs.wave2.qostar_m =
                direct_erod14_qostar(inputs.wave2.qout_m3_s, inputs.wave2.qin_m3_s)?;
        }
        Ok(inputs)
    }
}

fn apply_erod14_prior_particle_flow_fraction(
    inputs: &mut DirectErod14Inputs,
    prior_operands: &DirectErosionDownstreamOperands,
) -> Result<(), DirectRuntimeError> {
    let mut fraction_sum = 0.0;
    for (index, class) in inputs.classes.iter_mut().enumerate() {
        let fraction = if prior_operands.publication_authority {
            prior_operands.particle_flow_fraction[index]
        } else {
            0.0
        };
        validate_fraction("erosion.erod14.prior_particle_flow_fraction", fraction)?;
        class.frcflw = fraction;
        fraction_sum += fraction;
    }
    if prior_operands.publication_authority
        && fraction_sum > WB11_ZERO_THRESHOLD
        && (fraction_sum - 1.0).abs() > DIRECT_EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD
    {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod14.prior_particle_flow_fraction_sum",
        });
    }
    Ok(())
}

fn compute_direct_erod13(
    inputs: &DirectErod13Inputs,
) -> Result<DirectErod13State, DirectRuntimeError> {
    validate_erod13_inputs(inputs)?;
    let tau_f_pa = inputs.taufe_pa * (inputs.fs / inputs.ft);
    validate_nonnegative_derived("erosion.erod13.tau_f_pa", tau_f_pa)?;
    let eta =
        (inputs.cntlen_m * inputs.kr_s_m * inputs.kradjf * inputs.shrsol_pa) / inputs.tcend_kg_s_m;
    validate_nonnegative_derived("erosion.erod13.eta", eta)?;
    let taucn = (inputs.tcadjf * inputs.shcrit_pa) / inputs.shrsol_pa;
    validate_nonnegative_derived("erosion.erod13.taucn", taucn)?;
    let theta = ((inputs.cntlen_m * inputs.detinr_kg_s_m2) / inputs.tcend_kg_s_m)
        * (inputs.effdrr_m / inputs.effdrn_m);
    validate_nonnegative_derived("erosion.erod13.theta", theta)?;
    let phi = (inputs.beta * inputs.veleff_m_s) / inputs.pkro_m3_s;
    validate_nonnegative_derived("erosion.erod13.phi", phi)?;
    let tc_kg_s_m = inputs.tcadjf * inputs.tc_k * tau_f_pa.powf(inputs.tc_m);
    validate_nonnegative_derived("erosion.erod13.tc_kg_s_m", tc_kg_s_m)?;

    let (detachment_capacity_kg_s_m2, net_detachment_flux_kg_s_m2) =
        direct_erod13_fluxes(inputs, tau_f_pa, taucn, eta, tc_kg_s_m)?;
    let expected_dgdx = net_detachment_flux_kg_s_m2 + inputs.di_kg_s_m2;
    if (inputs.dgdx_kg_s_m2 - expected_dgdx).abs()
        > DIRECT_EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD
    {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod13.dgdx",
        });
    }

    Ok(DirectErod13State {
        tau_f_pa,
        dc_kg_s_m2: detachment_capacity_kg_s_m2,
        tc_kg_s_m,
        df_kg_s_m2: net_detachment_flux_kg_s_m2,
        eta,
        taucn,
        theta,
        phi,
    })
}

fn validate_erod13_inputs(inputs: &DirectErod13Inputs) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("erosion.erod13.ie_m_s", inputs.ie_m_s)?;
    validate_min("erosion.erod13.te_s", inputs.te_s, WB11_ZERO_THRESHOLD)?;
    validate_nonnegative_direct_m("erosion.erod13.fs", inputs.fs)?;
    validate_min("erosion.erod13.ft", inputs.ft, WB11_ZERO_THRESHOLD)?;
    validate_max("erosion.erod13.fs", inputs.fs, inputs.ft)?;
    validate_nonnegative_direct_m("erosion.erod13.taufe_pa", inputs.taufe_pa)?;
    validate_nonnegative_direct_m("erosion.erod13.q_m2_s", inputs.q_m2_s)?;
    validate_nonnegative_direct_m("erosion.erod13.g_kg_s_m", inputs.g_kg_s_m)?;
    validate_nonnegative_direct_m("erosion.erod13.di_kg_s_m2", inputs.di_kg_s_m2)?;
    validate_nonnegative_direct_m("erosion.erod13.beta", inputs.beta)?;
    validate_nonnegative_direct_m("erosion.erod13.vf_m_s", inputs.vf_m_s)?;
    validate_finite("erosion.erod13.dgdx_kg_s_m2", inputs.dgdx_kg_s_m2)?;
    validate_min(
        "erosion.erod13.cntlen_m",
        inputs.cntlen_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min("erosion.erod13.kr_s_m", inputs.kr_s_m, WB11_ZERO_THRESHOLD)?;
    validate_min("erosion.erod13.kradjf", inputs.kradjf, WB11_ZERO_THRESHOLD)?;
    validate_min(
        "erosion.erod13.tcadjf",
        inputs.tcadjf,
        DIRECT_EROD13_MIN_TCADJF,
    )?;
    validate_min(
        "erosion.erod13.shrsol_pa",
        inputs.shrsol_pa,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod13.tcend_kg_s_m",
        inputs.tcend_kg_s_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_nonnegative_direct_m("erosion.erod13.shcrit_pa", inputs.shcrit_pa)?;
    validate_nonnegative_direct_m("erosion.erod13.detinr_kg_s_m2", inputs.detinr_kg_s_m2)?;
    validate_min(
        "erosion.erod13.effdrr_m",
        inputs.effdrr_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod13.effdrn_m",
        inputs.effdrn_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_nonnegative_direct_m("erosion.erod13.veleff_m_s", inputs.veleff_m_s)?;
    validate_min(
        "erosion.erod13.pkro_m3_s",
        inputs.pkro_m3_s,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min("erosion.erod13.tc_k", inputs.tc_k, WB11_ZERO_THRESHOLD)?;
    validate_min("erosion.erod13.tc_m", inputs.tc_m, WB11_ZERO_THRESHOLD)?;
    validate_min(
        "erosion.erod13.q_runoff_m",
        inputs.q_runoff_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod13.peakro_m3_s",
        inputs.peakro_m3_s,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod13.watdur_s",
        inputs.watdur_s,
        WB11_ZERO_THRESHOLD,
    )?;

    let expected_watdur_s = inputs.q_runoff_m / inputs.peakro_m3_s;
    if (inputs.watdur_s - expected_watdur_s).abs()
        > DIRECT_EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD
    {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod13.watdur_s",
        });
    }
    Ok(())
}

fn direct_erod13_fluxes(
    inputs: &DirectErod13Inputs,
    tau_f_pa: f64,
    taucn: f64,
    eta: f64,
    tc_kg_s_m: f64,
) -> Result<(f64, f64), DirectRuntimeError> {
    if tau_f_pa > taucn && inputs.g_kg_s_m < tc_kg_s_m {
        validate_min("erosion.erod13.tc_kg_s_m", tc_kg_s_m, WB11_ZERO_THRESHOLD)?;
        let dc = eta * (tau_f_pa - taucn);
        validate_nonnegative_derived("erosion.erod13.dc_kg_s_m2", dc)?;
        let df = dc * ((tc_kg_s_m - inputs.g_kg_s_m) / tc_kg_s_m);
        validate_nonnegative_derived("erosion.erod13.df_kg_s_m2", df)?;
        return Ok((dc, df));
    }
    if inputs.g_kg_s_m > tc_kg_s_m {
        validate_min("erosion.erod13.q_m2_s", inputs.q_m2_s, WB11_ZERO_THRESHOLD)?;
        let df = -((inputs.beta * inputs.vf_m_s / inputs.q_m2_s) * (inputs.g_kg_s_m - tc_kg_s_m));
        if !df.is_finite() || df > WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.erod13.df_kg_s_m2",
            });
        }
        return Ok((0.0, df));
    }
    Ok((0.0, 0.0))
}

#[derive(Debug, Clone)]
struct DirectErod14WorkingClassState {
    fall: Vec<f64>,
    frcflw: Vec<f64>,
    tcf1: Vec<f64>,
    ssa_class: Vec<f64>,
    ftheta: Vec<f64>,
    gu: Vec<f64>,
    gend: Vec<f64>,
    sedmax: Vec<f64>,
    sed_frac: Vec<f64>,
}

fn compute_direct_erod14(
    inputs: &DirectErod14Inputs,
    theta: f64,
) -> Result<DirectErod14State, DirectRuntimeError> {
    validate_erod14_inputs(inputs, theta)?;
    let mut classes = load_direct_erod14_class_state(inputs, theta);

    if inputs.qout_m3_s <= WB11_ZERO_THRESHOLD {
        let class_states = (0..inputs.classes.len())
            .map(|_| DirectErod14ClassState {
                gend_kg: 0.0,
                sedmax_kg: 0.0,
                particle_flow_fraction: 0.0,
                sed_frac: 0.0,
                sediment_concentration_kg_m3: 0.0,
            })
            .collect::<Vec<_>>();
        return Ok(DirectErod14State {
            class_count: inputs.classes.len(),
            sumg_kg: 0.0,
            er: 0.0,
            lddend_kg: inputs.lddend_kg.max(0.0),
            qout_m3_s: inputs.qout_m3_s,
            peak_runoff_m3_s: inputs.peak_runoff_m3_s,
            runoff_duration_s: inputs.runoff_duration_s,
            classes: class_states,
        });
    }

    let pkro = direct_erod14_pkro(inputs)?;
    let mut sumg = direct_erod14_project_initial_gend(inputs, &mut classes, pkro)?;
    direct_erod14_reproportion_to_ldbot(inputs, &mut classes, &mut sumg)?;
    let sumg = direct_erod14_validate_sumg_and_caps(&classes)?;
    direct_erod14_update_transport_fractions(&mut classes, sumg)?;
    let er = direct_erod14_enrichment_ratio(inputs, &classes, sumg)?;
    let class_states = direct_erod14_final_class_states(inputs, &classes)?;

    Ok(DirectErod14State {
        class_count: inputs.classes.len(),
        sumg_kg: sumg.max(0.0),
        er,
        lddend_kg: inputs.lddend_kg.max(0.0),
        qout_m3_s: inputs.qout_m3_s,
        peak_runoff_m3_s: inputs.peak_runoff_m3_s,
        runoff_duration_s: inputs.runoff_duration_s,
        classes: class_states,
    })
}

fn validate_erod14_inputs(
    inputs: &DirectErod14Inputs,
    theta: f64,
) -> Result<(), DirectRuntimeError> {
    if inputs.classes.is_empty() || inputs.classes.len() > DIRECT_EROSION_CLASS_LIMIT {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.erod14.class_count",
        });
    }
    validate_nonnegative_direct_m("erosion.erod14.xtop_m", inputs.xtop_m)?;
    validate_min("erosion.erod14.xbot_m", inputs.xbot_m, inputs.xtop_m)?;
    validate_nonnegative_direct_m("erosion.erod14.xdetst_m", inputs.xdetst_m)?;
    validate_max("erosion.erod14.xdetst_m", inputs.xdetst_m, inputs.xtop_m)?;
    validate_nonnegative_direct_m("erosion.erod14.ldtop_kg_s_m", inputs.ldtop_kg_s_m)?;
    validate_nonnegative_direct_m("erosion.erod14.ldbot_kg_s_m", inputs.ldbot_kg_s_m)?;
    validate_nonnegative_direct_m("erosion.erod14.lddend_kg", inputs.lddend_kg)?;
    validate_max(
        "erosion.erod14.lddend_kg",
        inputs.lddend_kg,
        inputs.ldtop_kg_s_m,
    )?;
    validate_nonnegative_direct_m("erosion.erod14.qout_m3_s", inputs.qout_m3_s)?;
    validate_nonnegative_direct_m("erosion.erod14.qin_m3_s", inputs.qin_m3_s)?;
    validate_min(
        "erosion.erod14.hbp_sediment_concentration_scale",
        inputs.hbp_sediment_concentration_scale,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min(
        "erosion.erod14.slplen_m",
        inputs.slplen_m,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_min("erosion.erod14.ktrato", inputs.ktrato, WB11_ZERO_THRESHOLD)?;
    validate_finite("erosion.erod14.aintc", inputs.aintc)?;
    validate_finite("erosion.erod14.bintc", inputs.bintc)?;
    validate_finite("erosion.erod14.cintc", inputs.cintc)?;
    validate_nonnegative_direct_m("erosion.erod14.beta", inputs.beta)?;
    validate_nonnegative_direct_m("erosion.erod14.qj_minus_1_m3_s", inputs.qj_minus_1_m3_s)?;
    validate_nonnegative_direct_m("erosion.erod14.vj_m", inputs.vj_m)?;
    validate_nonnegative_direct_m("erosion.erod14.qj_m3_s", inputs.qj_m3_s)?;
    validate_nonnegative_direct_m("erosion.erod14.fh_m", inputs.fh_m)?;
    validate_nonnegative_direct_m("erosion.erod14.fp_m", inputs.fp_m)?;
    validate_nonnegative_direct_m("erosion.erod14.peak_runoff_m3_s", inputs.peak_runoff_m3_s)?;
    validate_nonnegative_direct_m("erosion.erod14.runoff_duration_s", inputs.runoff_duration_s)?;
    validate_min(
        "erosion.erod14.ssa_soil",
        inputs.ssa_soil,
        WB11_ZERO_THRESHOLD,
    )?;
    validate_nonnegative_direct_m("erosion.erod14.theta", theta)?;
    validate_erod14_case(inputs)?;
    for class in &inputs.classes {
        validate_nonnegative_direct_m("erosion.erod14.class.fall_m_s", class.fall_m_s)?;
        validate_fraction("erosion.erod14.class.frcflw", class.frcflw)?;
        validate_fraction("erosion.erod14.class.frac", class.frac)?;
        validate_fraction("erosion.erod14.class.fidel", class.fidel)?;
        validate_nonnegative_direct_m("erosion.erod14.class.tcf1", class.tcf1)?;
        validate_min(
            "erosion.erod14.class.ssa_class",
            class.ssa_class,
            WB11_ZERO_THRESHOLD,
        )?;
    }
    Ok(())
}

fn validate_erod14_case(inputs: &DirectErod14Inputs) -> Result<(), DirectRuntimeError> {
    let case_rounded = inputs.case_value.round();
    if (inputs.case_value - case_rounded).abs() > WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.erod14.case_value",
        });
    }
    let case_range = 1.0 - WB11_ZERO_THRESHOLD..=4.0 + WB11_ZERO_THRESHOLD;
    if !case_range.contains(&case_rounded) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.erod14.case_value",
        });
    }
    let case_number = erod14_case_number(case_rounded)?;
    let qj_minus_1 = inputs.qj_minus_1_m3_s;
    let vj = inputs.vj_m;
    let qj = inputs.qj_m3_s;
    let fh = inputs.fh_m;
    let fp = inputs.fp_m;
    let case_matches = match case_number {
        1 => erod14_case_is_zero(qj_minus_1) && erod14_case_is_zero(vj) && erod14_case_is_zero(qj),
        2 => {
            qj_minus_1 > DIRECT_EROD14_CASE_TOLERANCE
                && vj > DIRECT_EROD14_CASE_TOLERANCE
                && qj > DIRECT_EROD14_CASE_TOLERANCE
        }
        3 => {
            qj_minus_1 > DIRECT_EROD14_CASE_TOLERANCE
                && erod14_case_is_zero(vj)
                && (fh - fp) > DIRECT_EROD14_CASE_TOLERANCE
                && qj > DIRECT_EROD14_CASE_TOLERANCE
        }
        4 => {
            qj_minus_1 > DIRECT_EROD14_CASE_TOLERANCE
                && erod14_case_is_zero(vj)
                && (fh - fp) <= DIRECT_EROD14_CASE_TOLERANCE
                && erod14_case_is_zero(qj)
        }
        _ => false,
    };
    if case_matches {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.erod14.case_value",
        })
    }
}

fn erod14_case_is_zero(value: f64) -> bool {
    value.abs() <= DIRECT_EROD14_CASE_TOLERANCE
}

fn erod14_case_number(case_rounded: f64) -> Result<i32, DirectRuntimeError> {
    for case_number in 1_i32..=4_i32 {
        if (case_rounded - f64::from(case_number)).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(case_number);
        }
    }
    Err(DirectRuntimeError::DirectDomainViolation {
        field: "erosion.erod14.case_value",
    })
}

fn load_direct_erod14_class_state(
    inputs: &DirectErod14Inputs,
    theta: f64,
) -> DirectErod14WorkingClassState {
    let class_count = inputs.classes.len();
    let mut state = DirectErod14WorkingClassState {
        fall: Vec::with_capacity(class_count),
        frcflw: Vec::with_capacity(class_count),
        tcf1: Vec::with_capacity(class_count),
        ssa_class: Vec::with_capacity(class_count),
        ftheta: Vec::with_capacity(class_count),
        gu: Vec::with_capacity(class_count),
        gend: vec![0.0; class_count],
        sedmax: vec![0.0; class_count],
        sed_frac: vec![0.0; class_count],
    };
    for class in &inputs.classes {
        state.fall.push(class.fall_m_s);
        state.frcflw.push(class.frcflw);
        state.tcf1.push(class.tcf1);
        state.ssa_class.push(class.ssa_class);
        state.ftheta.push(class.fidel * theta);
        state.gu.push(class.frcflw * inputs.ldtop_kg_s_m);
    }
    state
}

fn direct_erod14_pkro(inputs: &DirectErod14Inputs) -> Result<f64, DirectRuntimeError> {
    let pkro = (inputs.qout_m3_s - inputs.qin_m3_s) / inputs.slplen_m;
    validate_finite("erosion.erod14.pkro", pkro)?;
    Ok(pkro)
}

fn direct_erod14_qostar(qout: f64, qin: f64) -> Result<f64, DirectRuntimeError> {
    validate_finite("erosion.erod14.qout_for_qostar", qout)?;
    validate_finite("erosion.erod14.qin_for_qostar", qin)?;
    let del = qout - qin;
    let qostar = if qout <= 0.0 {
        0.0
    } else if del.abs() > DIRECT_EROD14_QOSTAR_DELTA_THRESHOLD {
        if qin <= 0.0 { 0.0 } else { qin / del }
    } else if del >= 0.0 {
        qin / DIRECT_EROD14_QOSTAR_DELTA_THRESHOLD
    } else {
        -qin / DIRECT_EROD14_QOSTAR_DELTA_THRESHOLD
    };
    let qostar = if (qostar + 1.0).abs() <= WB11_ZERO_THRESHOLD {
        -1.001
    } else {
        qostar
    };
    validate_finite("erosion.erod14.qostar", qostar)?;
    Ok(qostar)
}

fn direct_erod14_project_initial_gend(
    inputs: &DirectErod14Inputs,
    classes: &mut DirectErod14WorkingClassState,
    pkro: f64,
) -> Result<f64, DirectRuntimeError> {
    let tmpvr2 = inputs.xbot_m + inputs.qostar_m;
    let tmpvr3 = inputs.xtop_m + inputs.qostar_m;
    validate_min(
        "erosion.erod14.qostar_tmpvr2",
        tmpvr2.abs(),
        WB11_ZERO_THRESHOLD,
    )?;
    let mut sumg = 0.0;
    for index in 0..classes.gend.len() {
        let gend = direct_erod14_project_class_gend(inputs, classes, index, pkro, tmpvr2, tmpvr3)?;
        classes.gend[index] = gend;
        sumg += gend;
    }
    validate_finite("erosion.erod14.sumg", sumg)?;
    Ok(sumg)
}

fn direct_erod14_project_class_gend(
    inputs: &DirectErod14Inputs,
    classes: &DirectErod14WorkingClassState,
    index: usize,
    pkro: f64,
    tmpvr2: f64,
    tmpvr3: f64,
) -> Result<f64, DirectRuntimeError> {
    let tmpvr4 = tmpvr2 * tmpvr2;
    let tmpvr5 = tmpvr3 * tmpvr3;
    let tmpvr1 = inputs.ktrato * classes.tcf1[index];
    let aa = tmpvr1 * inputs.aintc;
    let bb = tmpvr1 * inputs.bintc;
    let cc = tmpvr1 * inputs.cintc;
    let phi = direct_erod14_phi(inputs, classes.fall[index], pkro);
    let attenuation_factor = direct_erod14_attenuation_factor(inputs, phi, tmpvr2, tmpvr3)?;
    let denom_coef1 = phi + 2.0;
    let denom_coef2 = phi + 1.0;
    validate_min(
        "erosion.erod14.denominator",
        denom_coef1.abs().min(denom_coef2.abs()),
        WB11_ZERO_THRESHOLD,
    )?;
    let coef1 = phi * aa / denom_coef1;
    let coef2 = (phi * bb + classes.ftheta[index] - 2.0 * aa * phi * inputs.qostar_m) / denom_coef2;
    let term1 = coef1 * tmpvr4;
    let term2 = coef2 * tmpvr2;
    let term3 = aa * inputs.qostar_m * inputs.qostar_m - bb * inputs.qostar_m + cc;
    let attenuation_tail = classes.gu[index] - coef1 * tmpvr5 - coef2 * tmpvr3 - term3;
    let gend = term1 + term2 + term3 + attenuation_factor * attenuation_tail;
    validate_finite("erosion.erod14.class.gend", gend)?;
    Ok(gend.max(0.0))
}

fn direct_erod14_phi(inputs: &DirectErod14Inputs, fall: f64, pkro: f64) -> f64 {
    let phi = if pkro.abs() > DIRECT_EROD14_PKRO_ZERO_THRESHOLD {
        (inputs.beta * fall) / pkro
    } else if inputs.qostar_m >= 0.0 {
        DIRECT_EROD14_MAX_PHI
    } else {
        -DIRECT_EROD14_MAX_PHI
    };
    phi.clamp(-DIRECT_EROD14_MAX_PHI, DIRECT_EROD14_MAX_PHI)
}

fn direct_erod14_attenuation_factor(
    inputs: &DirectErod14Inputs,
    phi: f64,
    tmpvr2: f64,
    tmpvr3: f64,
) -> Result<f64, DirectRuntimeError> {
    let mut ratio = tmpvr3 / tmpvr2;
    if inputs.qostar_m >= 0.0 && ratio > 1.0 {
        ratio = 1.0;
    }
    validate_nonnegative_direct_m("erosion.erod14.attenuation_ratio", ratio)?;
    let attenuation_factor = ratio.powf(phi);
    validate_finite("erosion.erod14.attenuation_factor", attenuation_factor)?;
    Ok(if attenuation_factor < DIRECT_EROD14_ATTENUATION_FLOOR {
        0.0
    } else {
        attenuation_factor
    })
}

fn direct_erod14_reproportion_to_ldbot(
    inputs: &DirectErod14Inputs,
    classes: &mut DirectErod14WorkingClassState,
    sumg: &mut f64,
) -> Result<(), DirectRuntimeError> {
    if *sumg <= 0.0 {
        return Ok(());
    }
    for index in 0..classes.gend.len() {
        classes.gend[index] = classes.gend[index] * inputs.ldbot_kg_s_m / *sumg;
        classes.sedmax[index] =
            classes.gu[index] + classes.ftheta[index] * (inputs.xbot_m - inputs.xtop_m);
        validate_nonnegative_direct_m("erosion.erod14.class.sedmax", classes.sedmax[index])?;
        if classes.gend[index] < DIRECT_EROD14_CLASS_FLOOR {
            classes.gend[index] = DIRECT_EROD14_CLASS_FLOOR;
        }
    }

    let mut converged = false;
    for _ in 0..DIRECT_EROD14_MAX_REPROPORTION_ITERS {
        if direct_erod14_reproportion_iteration(inputs, classes, sumg)? {
            converged = true;
            break;
        }
    }
    if converged {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.erod14.reproportion",
        })
    }
}

fn direct_erod14_reproportion_iteration(
    inputs: &DirectErod14Inputs,
    classes: &mut DirectErod14WorkingClassState,
    sumg: &mut f64,
) -> Result<bool, DirectRuntimeError> {
    let mut ratbot = 0.0;
    *sumg = 0.0;
    let mut adjusted = false;

    for index in 0..classes.gend.len() {
        if classes.gend[index] > classes.sedmax[index] + WB11_ZERO_THRESHOLD {
            classes.gend[index] = classes.sedmax[index];
            adjusted = true;
        } else if classes.gend[index] < classes.sedmax[index] - WB11_ZERO_THRESHOLD {
            ratbot += classes.gend[index];
        }
        *sumg += classes.gend[index];
    }
    validate_finite("erosion.erod14.sumg", *sumg)?;
    if !adjusted {
        return Ok(true);
    }
    if ratbot <= WB11_ZERO_THRESHOLD {
        return Ok(false);
    }

    let gdeficit = inputs.ldbot_kg_s_m - *sumg;
    for index in 0..classes.gend.len() {
        if classes.gend[index] < classes.sedmax[index] - WB11_ZERO_THRESHOLD {
            let updated = classes.gend[index] + gdeficit * classes.gend[index] / ratbot;
            validate_finite("erosion.erod14.class.gend", updated)?;
            classes.gend[index] = updated;
        }
    }
    Ok(false)
}

fn direct_erod14_validate_sumg_and_caps(
    classes: &DirectErod14WorkingClassState,
) -> Result<f64, DirectRuntimeError> {
    let mut sumg = 0.0;
    for index in 0..classes.gend.len() {
        sumg += classes.gend[index];
        validate_finite("erosion.erod14.sumg", sumg)?;
        if classes.gend[index] > classes.sedmax[index] + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "erosion.erod14.class.sedmax",
            });
        }
    }
    validate_nonnegative_direct_m("erosion.erod14.sumg", sumg)?;
    Ok(sumg)
}

fn direct_erod14_update_transport_fractions(
    classes: &mut DirectErod14WorkingClassState,
    sumg: f64,
) -> Result<(), DirectRuntimeError> {
    if sumg > 0.0 {
        for index in 0..classes.gend.len() {
            classes.frcflw[index] = classes.gend[index] / sumg;
            classes.sed_frac[index] = classes.frcflw[index];
        }
        let sed_frac_sum = classes.sed_frac.iter().copied().sum::<f64>();
        if (sed_frac_sum - 1.0).abs() > DIRECT_EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "erosion.erod14.sed_frac_sum",
            });
        }
    } else {
        for index in 0..classes.gend.len() {
            classes.frcflw[index] = 0.0;
            classes.sed_frac[index] = 0.0;
        }
    }
    Ok(())
}

fn direct_erod14_enrichment_ratio(
    inputs: &DirectErod14Inputs,
    classes: &DirectErod14WorkingClassState,
    sumg: f64,
) -> Result<f64, DirectRuntimeError> {
    let mut sumssa = 0.0;
    for index in 0..classes.sed_frac.len() {
        sumssa += classes.sed_frac[index] * classes.ssa_class[index];
        validate_finite("erosion.erod14.sumssa", sumssa)?;
    }
    let er = if sumg > 0.0 {
        (sumssa / inputs.ssa_soil) + DIRECT_EROD14_ENRICHMENT_RATIO_OFFSET
    } else {
        0.0
    };
    validate_nonnegative_direct_m("erosion.erod14.er", er)?;
    Ok(er)
}

fn direct_erod14_final_class_states(
    inputs: &DirectErod14Inputs,
    classes: &DirectErod14WorkingClassState,
) -> Result<Vec<DirectErod14ClassState>, DirectRuntimeError> {
    let mut states = Vec::with_capacity(classes.gend.len());
    for index in 0..classes.gend.len() {
        let concentration = if inputs.qout_m3_s > WB11_ZERO_THRESHOLD {
            classes.gend[index] / inputs.qout_m3_s
        } else {
            0.0
        };
        validate_nonnegative_direct_m("erosion.erod15.concentration", concentration)?;
        states.push(DirectErod14ClassState {
            gend_kg: classes.gend[index],
            sedmax_kg: classes.sedmax[index],
            particle_flow_fraction: classes.frcflw[index],
            sed_frac: classes.sed_frac[index],
            sediment_concentration_kg_m3: concentration,
        });
    }
    Ok(states)
}

fn direct_erod15_publication_projection(
    wave2: &DirectErod14State,
    inputs: &DirectErod14Inputs,
) -> Result<DirectPublicationErosionOperands, DirectRuntimeError> {
    let mut sediment_concentration_kg_m3 = [0.0; DIRECT_EROSION_CLASS_LIMIT];
    let mut hbp_sediment_concentration_kg_m3 = 0.0;
    for (index, class_state) in wave2.classes.iter().enumerate() {
        if index < DIRECT_EROSION_CLASS_LIMIT {
            sediment_concentration_kg_m3[index] = class_state.sediment_concentration_kg_m3;
        }
        if index == 0 {
            hbp_sediment_concentration_kg_m3 =
                class_state.sediment_concentration_kg_m3 / inputs.hbp_sediment_concentration_scale;
        }
        validate_finite(
            "erosion.erod15.class_concentration",
            class_state.sediment_concentration_kg_m3,
        )?;
    }
    validate_nonnegative_direct_m("erosion.erod15.total_detachment_kg", wave2.sumg_kg)?;
    validate_nonnegative_direct_m("erosion.erod15.total_deposition_kg", wave2.lddend_kg)?;
    validate_nonnegative_direct_m(
        "erosion.erod15.hbp_sediment_concentration_kg_m3",
        hbp_sediment_concentration_kg_m3,
    )?;
    Ok(DirectPublicationErosionOperands {
        peak_runoff_m3_s: Some(wave2.peak_runoff_m3_s),
        runoff_duration_s: Some(wave2.runoff_duration_s),
        total_detachment_kg: Some(wave2.sumg_kg),
        total_deposition_kg: Some(wave2.lddend_kg),
        hbp_total_detachment_kg: Some(wave2.sumg_kg),
        hbp_total_deposition_kg: Some(wave2.lddend_kg),
        hbp_sediment_concentration_kg_m3: Some(hbp_sediment_concentration_kg_m3),
        sediment_concentration_kg_m3: Some(sediment_concentration_kg_m3),
    })
}

fn validate_nonnegative_derived(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value < -WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::NegativeDirectValue { field });
    }
    Ok(())
}

fn validate_min(field: &'static str, value: f64, minimum: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value < minimum - WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    Ok(())
}

fn validate_max(field: &'static str, value: f64, maximum: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value > maximum + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    Ok(())
}

fn validate_fraction(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(field, value)?;
    validate_max(field, value, 1.0)
}
