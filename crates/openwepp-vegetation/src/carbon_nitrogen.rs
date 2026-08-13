//! E16--E22 persistent carbon, nitrogen, phenology, and material transfer.
use crate::VegetationError;
use crate::photosynthesis::peaked_response;
use crate::transaction::PhenologyPhase;
use openwepp_kernel_contract::{MaterialDonorClass, ResourceOwnerId, SoilLayerId, TransactionId};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementPool {
    pub carbon: f64,
    pub nitrogen: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TissuePool {
    pub display: ElementPool,
    pub storage: ElementPool,
    pub transfer: ElementPool,
}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Tissue {
    Leaf,
    FineRoot,
    LiveStem,
    DeadStem,
    LiveCoarseRoot,
    DeadCoarseRoot,
}

pub use openwepp_kernel_contract::MaterialReceiverClass as ReceiverClass;
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MaterialTransfer {
    pub transaction_id: u128,
    pub owner_id: String,
    pub proposal_id: u64,
    pub donor: MaterialDonorClass,
    pub receiver: ReceiverClass,
    pub carbon: f64,
    pub nitrogen: f64,
    pub dry_matter: f64,
}

/// Constitutive material amounts before transaction identity is assigned.
///
/// Turnover and phenology calculate these amounts without manufacturing a
/// transaction, owner, or proposal identity. The transaction orchestrator must
/// bind each amount exactly once before it can become a [`MaterialTransfer`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialTransferAmounts {
    donor: MaterialDonorClass,
    receiver: ReceiverClass,
    carbon: f64,
    nitrogen: f64,
    dry_matter: f64,
}

impl MaterialTransferAmounts {
    #[must_use]
    pub const fn donor(&self) -> MaterialDonorClass {
        self.donor
    }

    #[must_use]
    pub const fn receiver(&self) -> ReceiverClass {
        self.receiver
    }

    #[must_use]
    pub const fn carbon(&self) -> f64 {
        self.carbon
    }

    #[must_use]
    pub const fn nitrogen(&self) -> f64 {
        self.nitrogen
    }

    #[must_use]
    pub const fn dry_matter(&self) -> f64 {
        self.dry_matter
    }

    pub fn bind(
        self,
        transaction_id: TransactionId,
        owner_id: &ResourceOwnerId,
        proposal_id: u64,
    ) -> Result<MaterialTransfer, VegetationError> {
        if transaction_id.0 == 0
            || proposal_id == 0
            || [self.carbon, self.nitrogen, self.dry_matter]
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(VegetationError::Domain("material transfer identity"));
        }
        Ok(MaterialTransfer {
            transaction_id: transaction_id.0,
            owner_id: owner_id.as_str().to_owned(),
            proposal_id,
            donor: self.donor,
            receiver: self.receiver,
            carbon: self.carbon,
            nitrogen: self.nitrogen,
            dry_matter: self.dry_matter,
        })
    }
}

/// One typed fine-root maintenance-respiration operand.
#[derive(Clone, Debug, PartialEq)]
pub struct RootRespirationOperand {
    pub layer_id: SoilLayerId,
    pub temperature_k: f64,
    pub nitrogen_fraction: f64,
}

pub fn gpp_kg_c(
    dt_s: f64,
    tile_fraction: f64,
    ag_sun: f64,
    lai_sun: f64,
    ag_shade: f64,
    lai_shade: f64,
) -> Result<f64, VegetationError> {
    let v = [dt_s, tile_fraction, ag_sun, lai_sun, ag_shade, lai_shade];
    if v.iter().any(|x| !x.is_finite())
        || dt_s <= 0.0
        || tile_fraction < 0.0
        || lai_sun < 0.0
        || lai_shade < 0.0
    {
        return Err(VegetationError::Domain("GPP"));
    }
    Ok(0.012_011e-6 * dt_s * tile_fraction * (ag_sun * lai_sun + ag_shade * lai_shade))
}
pub fn update_t10(t10: f64, tair: f64, dt: f64) -> Result<f64, VegetationError> {
    if [t10, tair, dt].iter().any(|v| !v.is_finite()) || t10 <= 0.0 || tair <= 0.0 || dt <= 0.0 {
        return Err(VegetationError::Domain("T10"));
    }
    Ok(tair + (t10 - tair) * (-dt / 864_000.0).exp())
}

#[allow(clippy::too_many_arguments)]
pub fn maintenance_respiration(
    tissues: &std::collections::BTreeMap<Tissue, TissuePool>,
    accepted_leaf_respiration_kg_c: f64,
    air_temperature_k: f64,
    fine_root_layers: &[RootRespirationOperand],
    base_rate: f64,
    q10: f64,
    dt_s: f64,
) -> Result<f64, VegetationError> {
    if ![
        accepted_leaf_respiration_kg_c,
        air_temperature_k,
        base_rate,
        q10,
        dt_s,
    ]
    .iter()
    .all(|value| value.is_finite())
        || accepted_leaf_respiration_kg_c < 0.0
        || air_temperature_k <= 0.0
        || base_rate < 0.0
        || q10 <= 0.0
        || dt_s <= 0.0
        || fine_root_layers.is_empty()
        || fine_root_layers.iter().any(|layer| {
            !layer.temperature_k.is_finite()
                || layer.temperature_k <= 0.0
                || !layer.nitrogen_fraction.is_finite()
                || layer.nitrogen_fraction < 0.0
        })
        || (fine_root_layers
            .iter()
            .map(|layer| layer.nitrogen_fraction)
            .sum::<f64>()
            - 1.0)
            .abs()
            > 1e-12
    {
        return Err(VegetationError::Domain("maintenance respiration"));
    }
    let mut layer_ids = BTreeSet::new();
    if fine_root_layers
        .iter()
        .any(|layer| !layer_ids.insert(&layer.layer_id))
    {
        return Err(VegetationError::Domain(
            "duplicate fine-root respiration layer",
        ));
    }
    let nitrogen = |tissue: Tissue| -> Result<f64, VegetationError> {
        let pool = tissues
            .get(&tissue)
            .ok_or(VegetationError::Domain("missing tissue pool"))?;
        Ok(pool.display.nitrogen + pool.storage.nitrogen + pool.transfer.nitrogen)
    };
    let livewood_n = nitrogen(Tissue::LiveStem)? + nitrogen(Tissue::LiveCoarseRoot)?;
    let livewood = livewood_n * base_rate * q10.powf((air_temperature_k - 293.15) / 10.0) * dt_s;
    let fine_root_n = nitrogen(Tissue::FineRoot)?;
    let fine_root = fine_root_layers
        .iter()
        .map(|layer| {
            fine_root_n
                * layer.nitrogen_fraction
                * base_rate
                * q10.powf((layer.temperature_k - 293.15) / 10.0)
                * dt_s
        })
        .sum::<f64>();
    let total = accepted_leaf_respiration_kg_c + livewood + fine_root;
    if total.is_finite() {
        Ok(total)
    } else {
        Err(VegetationError::Domain("maintenance response"))
    }
}

/// V3 Atkin leaf dark respiration at 25 C in umol CO2 m-2 leaf s-1.
pub fn atkin_rd25(
    leaf_nitrogen_kg_n: f64,
    leaf_area_m2: f64,
    t10_k: f64,
    intercept_umol_co2_m2_leaf_s: f64,
) -> Result<f64, VegetationError> {
    if [
        leaf_nitrogen_kg_n,
        leaf_area_m2,
        t10_k,
        intercept_umol_co2_m2_leaf_s,
    ]
    .iter()
    .all(|value| value.is_finite())
        && leaf_nitrogen_kg_n >= 0.0
        && leaf_area_m2 >= 0.0
        && t10_k > 0.0
    {
        if leaf_area_m2 == 0.0 {
            return Ok(0.0);
        }
        let rd25 = intercept_umol_co2_m2_leaf_s
            + 0.2061 * (1_000.0 * leaf_nitrogen_kg_n / leaf_area_m2)
            - 0.0402 * (t10_k - 273.15);
        if rd25.is_finite() && rd25 > 0.0 {
            return Ok(rd25);
        }
    }
    Err(VegetationError::Domain("nonpositive Atkin Rd25"))
}

/// Applies the admitted Rd-specific peaked temperature response.
pub fn leaf_rd_at_temperature(rd25: f64, temperature_k: f64) -> Result<f64, VegetationError> {
    if !rd25.is_finite() || rd25 < 0.0 {
        return Err(VegetationError::Domain("Rd25"));
    }
    Ok(rd25 * peaked_response(temperature_k, 46_390.0, 150_650.0, 490.0)?)
}

/// Integrates one accepted class-resolved Rd exactly once into kg C.
pub fn leaf_rd_carbon_debit(
    rd_umol_co2_m2_leaf_s: f64,
    leaf_area_m2_m2_tile: f64,
    dt_s: f64,
    tile_fraction: f64,
) -> Result<f64, VegetationError> {
    if [
        rd_umol_co2_m2_leaf_s,
        leaf_area_m2_m2_tile,
        dt_s,
        tile_fraction,
    ]
    .iter()
    .all(|value| value.is_finite())
        && rd_umol_co2_m2_leaf_s >= 0.0
        && leaf_area_m2_m2_tile >= 0.0
        && dt_s > 0.0
        && tile_fraction > 0.0
        && tile_fraction <= 1.0
    {
        return Ok(rd_umol_co2_m2_leaf_s
            * 1.0e-6
            * 0.012_011
            * leaf_area_m2_m2_tile
            * dt_s
            * tile_fraction);
    }
    Err(VegetationError::Domain("leaf respiration carbon debit"))
}

pub fn bounded_turnover(pool: f64, dt: f64, lifetime: f64) -> Result<f64, VegetationError> {
    if !pool.is_finite()
        || !dt.is_finite()
        || !lifetime.is_finite()
        || pool < 0.0
        || dt < 0.0
        || lifetime <= 0.0
    {
        return Err(VegetationError::Domain("turnover"));
    }
    Ok(-(-dt / lifetime).exp_m1() * pool)
}
pub fn material_transfer(
    tissue: Tissue,
    receiver: ReceiverClass,
    carbon: f64,
    nitrogen: f64,
    carbon_fraction: f64,
) -> Result<MaterialTransferAmounts, VegetationError> {
    if !carbon.is_finite()
        || !nitrogen.is_finite()
        || carbon < 0.0
        || nitrogen < 0.0
        || !carbon_fraction.is_finite()
        || !(0.0..=1.0).contains(&carbon_fraction)
        || carbon_fraction == 0.0
    {
        return Err(VegetationError::Domain("material transfer"));
    }
    Ok(MaterialTransferAmounts {
        donor: match tissue {
            Tissue::Leaf => MaterialDonorClass::Leaf,
            Tissue::FineRoot => MaterialDonorClass::FineRoot,
            Tissue::LiveStem => MaterialDonorClass::LiveStem,
            Tissue::DeadStem => MaterialDonorClass::DeadStem,
            Tissue::LiveCoarseRoot => MaterialDonorClass::LiveCoarseRoot,
            Tissue::DeadCoarseRoot => MaterialDonorClass::DeadCoarseRoot,
        },
        receiver,
        carbon,
        nitrogen,
        dry_matter: carbon / carbon_fraction,
    })
}

#[derive(Clone, Debug)]
pub struct CnParameters {
    pub growth_respiration_ratio: f64,
    pub a1_froot_leaf: f64,
    pub a2_croot_stem: f64,
    pub a3_stem_leaf: f64,
    pub a4_livewood_fraction: f64,
    pub current_growth_fraction: f64,
    pub cn_leaf: f64,
    pub cn_leaf_litter: f64,
    pub cn_froot: f64,
    pub cn_livewood: f64,
    pub cn_deadwood: f64,
    pub drymatter_carbon_fraction: f64,
    pub xs_recovery_days: f64,
    pub leaf_lifetime_s: f64,
    pub froot_lifetime_s: f64,
    pub livewood_turnover_s: f64,
    pub mortality_rate_s1: f64,
    pub leaf_litter_fractions: [f64; 3],
    pub froot_litter_fractions: [f64; 3],
}

#[derive(Clone, Debug, PartialEq)]
pub struct CarbonOffer {
    pub maintenance_from_gpp: f64,
    pub reserve_recovery: f64,
    pub offer: f64,
    pub xs_next: f64,
}

pub fn carbon_offer(
    gpp: f64,
    maintenance: f64,
    xs0: f64,
    nsc0: f64,
    dt_s: f64,
    xs_recovery_days: f64,
) -> Result<CarbonOffer, VegetationError> {
    if ![gpp, maintenance, xs0, nsc0, dt_s, xs_recovery_days]
        .iter()
        .all(|v| v.is_finite())
        || gpp < 0.0
        || maintenance < 0.0
        || nsc0 < 0.0
        || dt_s <= 0.0
        || xs_recovery_days <= 0.0
    {
        return Err(VegetationError::Domain("carbon offer"));
    }
    let maintenance_from_gpp = gpp.min(maintenance);
    let xs_mr = maintenance - maintenance_from_gpp;
    let reserve_recovery = (-xs0 / (86_400.0 * xs_recovery_days))
        .max(0.0)
        .min(gpp - maintenance_from_gpp);
    Ok(CarbonOffer {
        maintenance_from_gpp,
        reserve_recovery,
        offer: gpp - maintenance_from_gpp - reserve_recovery + nsc0,
        xs_next: xs0 - xs_mr + reserve_recovery,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct NitrogenDemand {
    pub demand: f64,
    pub external_shortfall: f64,
    pub callom: f64,
    pub nallom: f64,
}

pub fn nitrogen_demand(
    offer: f64,
    internal_offer: f64,
    p: &CnParameters,
) -> Result<NitrogenDemand, VegetationError> {
    validate_parameters(p)?;
    if !offer.is_finite() || !internal_offer.is_finite() || offer < 0.0 || internal_offer < 0.0 {
        return Err(VegetationError::Domain("nitrogen demand"));
    }
    let callom = (1.0 + p.growth_respiration_ratio)
        * (1.0 + p.a1_froot_leaf + p.a3_stem_leaf * (1.0 + p.a2_croot_stem));
    let nallom = 1.0 / p.cn_leaf
        + p.a1_froot_leaf / p.cn_froot
        + p.a3_stem_leaf * p.a4_livewood_fraction * (1.0 + p.a2_croot_stem) / p.cn_livewood
        + p.a3_stem_leaf * (1.0 - p.a4_livewood_fraction) * (1.0 + p.a2_croot_stem) / p.cn_deadwood;
    let demand = offer * nallom / callom;
    Ok(NitrogenDemand {
        demand,
        external_shortfall: (demand - internal_offer).max(0.0),
        callom,
        nallom,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrowthFinalization {
    pub tissue_carbon: [f64; 6],
    pub tissue_nitrogen: [f64; 6],
    pub growth_respiration: f64,
    pub n_internal_use: f64,
    pub n_external_use: f64,
    pub eta: f64,
    pub nsc_next: f64,
    pub xs_next: f64,
    pub carbon_residual: f64,
    pub nitrogen_residual: f64,
}

pub fn finalize_growth(
    tissues: &mut std::collections::BTreeMap<Tissue, TissuePool>,
    final_offer: &CarbonOffer,
    internal_n: &mut f64,
    external_authorized: f64,
    p: &CnParameters,
) -> Result<GrowthFinalization, VegetationError> {
    let demand = nitrogen_demand(final_offer.offer, *internal_n, p)?;
    if !external_authorized.is_finite() || external_authorized < 0.0 {
        return Err(VegetationError::Domain("nitrogen authorization"));
    }
    let external_use = demand.external_shortfall.min(external_authorized);
    let internal_use = (*internal_n).min(demand.demand);
    let nused = internal_use + external_use;
    let eta = if demand.demand == 0.0 {
        1.0
    } else {
        (nused / demand.demand).min(1.0)
    };
    let leaf = eta * final_offer.offer / demand.callom;
    let coefficients = [
        1.0,
        p.a1_froot_leaf,
        p.a3_stem_leaf * p.a4_livewood_fraction,
        p.a3_stem_leaf * (1.0 - p.a4_livewood_fraction),
        p.a2_croot_stem * p.a3_stem_leaf * p.a4_livewood_fraction,
        p.a2_croot_stem * p.a3_stem_leaf * (1.0 - p.a4_livewood_fraction),
    ];
    let tissue_carbon = coefficients.map(|coefficient| coefficient * leaf);
    let cns = [
        p.cn_leaf,
        p.cn_froot,
        p.cn_livewood,
        p.cn_deadwood,
        p.cn_livewood,
        p.cn_deadwood,
    ];
    let tissue_nitrogen = std::array::from_fn(|index| tissue_carbon[index] / cns[index]);
    let identities = [
        Tissue::Leaf,
        Tissue::FineRoot,
        Tissue::LiveStem,
        Tissue::DeadStem,
        Tissue::LiveCoarseRoot,
        Tissue::DeadCoarseRoot,
    ];
    for index in 0..6 {
        let pool = tissues
            .get_mut(&identities[index])
            .ok_or(VegetationError::Domain("missing tissue pool"))?;
        let display = tissue_carbon[index] * p.current_growth_fraction;
        let storage = tissue_carbon[index] - display;
        pool.display.carbon += display;
        pool.storage.carbon += storage;
        pool.display.nitrogen += tissue_nitrogen[index] * p.current_growth_fraction;
        pool.storage.nitrogen += tissue_nitrogen[index] * (1.0 - p.current_growth_fraction);
    }
    *internal_n -= internal_use;
    let growth_respiration = p.growth_respiration_ratio * tissue_carbon.iter().sum::<f64>();
    let nsc = (1.0 - eta) * final_offer.offer;
    let carbon_residual =
        final_offer.offer - tissue_carbon.iter().sum::<f64>() - growth_respiration - nsc;
    let nitrogen_residual = nused - tissue_nitrogen.iter().sum::<f64>();
    if carbon_residual.abs() > 1e-12 || nitrogen_residual.abs() > 1e-12 {
        return Err(VegetationError::Closure {
            ledger: "C/N allocation",
            residual: carbon_residual.abs().max(nitrogen_residual.abs()),
        });
    }
    Ok(GrowthFinalization {
        tissue_carbon,
        tissue_nitrogen,
        growth_respiration,
        n_internal_use: internal_use,
        n_external_use: external_use,
        eta,
        nsc_next: nsc,
        xs_next: final_offer.xs_next,
        carbon_residual,
        nitrogen_residual,
    })
}

pub fn advance_turnover(
    tissues: &mut std::collections::BTreeMap<Tissue, TissuePool>,
    dt_s: f64,
    p: &CnParameters,
) -> Result<Vec<MaterialTransferAmounts>, VegetationError> {
    validate_parameters(p)?;
    if !dt_s.is_finite() || dt_s <= 0.0 {
        return Err(VegetationError::Domain("turnover interval"));
    }
    let mut transfers = Vec::new();
    route_fraction(
        tissues,
        Tissue::FineRoot,
        bounded_fraction(dt_s, p.froot_lifetime_s)?,
        &p.froot_litter_fractions,
        p,
        &mut transfers,
    )?;
    internal_wood_turnover(
        tissues,
        Tissue::LiveStem,
        Tissue::DeadStem,
        bounded_fraction(dt_s, p.livewood_turnover_s)?,
    )?;
    internal_wood_turnover(
        tissues,
        Tissue::LiveCoarseRoot,
        Tissue::DeadCoarseRoot,
        bounded_fraction(dt_s, p.livewood_turnover_s)?,
    )?;
    let mortality = -(-p.mortality_rate_s1 * dt_s).exp_m1();
    for tissue in [
        Tissue::Leaf,
        Tissue::FineRoot,
        Tissue::LiveStem,
        Tissue::DeadStem,
        Tissue::LiveCoarseRoot,
        Tissue::DeadCoarseRoot,
    ] {
        if matches!(tissue, Tissue::Leaf | Tissue::FineRoot) {
            let fractions = if tissue == Tissue::Leaf {
                &p.leaf_litter_fractions
            } else {
                &p.froot_litter_fractions
            };
            route_fraction(tissues, tissue, mortality, fractions, p, &mut transfers)?;
        } else {
            route_cwd(tissues, tissue, mortality, p, &mut transfers)?;
        }
    }
    Ok(transfers)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhenologyMode {
    Evergreen,
    SeasonalDeciduous,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhenologyUpdate {
    pub phase: PhenologyPhase,
    pub onset_remaining_s: f64,
    pub offset_remaining_s: f64,
    pub previous_gsi: f64,
    pub transfers: Vec<MaterialTransferAmounts>,
    pub retranslocated_n: f64,
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn advance_phenology(
    tissues: &mut std::collections::BTreeMap<Tissue, TissuePool>,
    mode: PhenologyMode,
    mut phase: PhenologyPhase,
    mut onset_remaining_s: f64,
    mut offset_remaining_s: f64,
    previous_gsi: f64,
    gsi: f64,
    dt_s: f64,
    on_threshold: f64,
    off_threshold: f64,
    onset_duration_s: f64,
    offset_duration_s: f64,
    p: &CnParameters,
) -> Result<PhenologyUpdate, VegetationError> {
    validate_parameters(p)?;
    if [
        onset_remaining_s,
        offset_remaining_s,
        previous_gsi,
        gsi,
        dt_s,
        on_threshold,
        off_threshold,
        onset_duration_s,
        offset_duration_s,
    ]
    .iter()
    .any(|value| !value.is_finite())
        || onset_remaining_s < 0.0
        || offset_remaining_s < 0.0
        || !(0.0..=1.0).contains(&previous_gsi)
        || !(0.0..=1.0).contains(&gsi)
        || dt_s <= 0.0
        || off_threshold >= on_threshold
        || onset_duration_s <= 0.0
        || offset_duration_s <= 0.0
    {
        return Err(VegetationError::Domain("phenology"));
    }
    let mut transfers = Vec::new();
    let mut retranslocated_n = 0.0;
    if mode == PhenologyMode::Evergreen {
        route_fraction(
            tissues,
            Tissue::Leaf,
            bounded_fraction(dt_s, p.leaf_lifetime_s)?,
            &p.leaf_litter_fractions,
            p,
            &mut transfers,
        )?;
        phase = PhenologyPhase::Active;
    } else {
        if phase == PhenologyPhase::Dormant && previous_gsi < on_threshold && gsi > on_threshold {
            phase = PhenologyPhase::Onset;
            onset_remaining_s = onset_duration_s;
        }
        if phase == PhenologyPhase::Active && previous_gsi > off_threshold && gsi < off_threshold {
            phase = PhenologyPhase::Offset;
            offset_remaining_s = offset_duration_s;
        }
        if phase == PhenologyPhase::Onset {
            let pool = tissues
                .get_mut(&Tissue::Leaf)
                .ok_or(VegetationError::Domain("missing leaf pool"))?;
            let fraction = if onset_remaining_s <= dt_s {
                1.0
            } else {
                (2.0 * dt_s / onset_remaining_s).min(1.0)
            };
            let moved_c = pool.transfer.carbon * fraction;
            let moved_n = pool.transfer.nitrogen * fraction;
            pool.transfer.carbon -= moved_c;
            pool.transfer.nitrogen -= moved_n;
            pool.display.carbon += moved_c;
            pool.display.nitrogen += moved_n;
            onset_remaining_s = (onset_remaining_s - dt_s).max(0.0);
            if pool.transfer.carbon <= 1e-15 && pool.transfer.nitrogen <= 1e-15 {
                pool.transfer = ElementPool::default();
                phase = PhenologyPhase::Active;
            }
        } else if phase == PhenologyPhase::Offset {
            let fraction = if offset_remaining_s <= dt_s {
                1.0
            } else {
                (2.0 * dt_s / offset_remaining_s).min(1.0)
            };
            let pool = tissues
                .get_mut(&Tissue::Leaf)
                .ok_or(VegetationError::Domain("missing leaf pool"))?;
            let fallen_c = pool.display.carbon * fraction;
            let leaf_n_debit = fallen_c / p.cn_leaf;
            let litter_n = fallen_c / p.cn_leaf_litter;
            let retranslocated = leaf_n_debit - litter_n;
            if pool.display.carbon < fallen_c || pool.display.nitrogen < leaf_n_debit {
                return Err(VegetationError::Domain(
                    "leaf offset donor cannot cover prescribed C/N debit",
                ));
            }
            pool.display.carbon -= fallen_c;
            pool.display.nitrogen -= leaf_n_debit;
            retranslocated_n = retranslocated;
            for (receiver, share) in [
                ReceiverClass::Metabolic,
                ReceiverClass::Cellulose,
                ReceiverClass::Lignin,
            ]
            .into_iter()
            .zip(p.leaf_litter_fractions)
            {
                transfers.push(material_transfer(
                    Tissue::Leaf,
                    receiver,
                    fallen_c * share,
                    litter_n * share,
                    p.drymatter_carbon_fraction,
                )?);
            }
            offset_remaining_s = (offset_remaining_s - dt_s).max(0.0);
            if pool.display.carbon <= 1e-15 && pool.display.nitrogen <= 1e-15 {
                pool.display = ElementPool::default();
                phase = PhenologyPhase::Dormant;
            }
        }
    }
    Ok(PhenologyUpdate {
        phase,
        onset_remaining_s,
        offset_remaining_s,
        previous_gsi: gsi,
        transfers,
        retranslocated_n,
    })
}

fn bounded_fraction(dt: f64, lifetime: f64) -> Result<f64, VegetationError> {
    if !lifetime.is_finite() || lifetime <= 0.0 {
        return Err(VegetationError::Domain("turnover lifetime"));
    }
    Ok(-(-dt / lifetime).exp_m1())
}
fn debit_fraction(pool: &mut TissuePool, fraction: f64) -> ElementPool {
    let mut loss = ElementPool::default();
    for subpool in [&mut pool.display, &mut pool.storage, &mut pool.transfer] {
        let c = subpool.carbon * fraction;
        let n = subpool.nitrogen * fraction;
        subpool.carbon -= c;
        subpool.nitrogen -= n;
        loss.carbon += c;
        loss.nitrogen += n;
    }
    loss
}
fn internal_wood_turnover(
    tissues: &mut std::collections::BTreeMap<Tissue, TissuePool>,
    donor: Tissue,
    receiver: Tissue,
    fraction: f64,
) -> Result<(), VegetationError> {
    let source = tissues
        .get_mut(&donor)
        .ok_or(VegetationError::Domain("missing livewood pool"))?;
    let losses = [
        take_fraction(&mut source.display, fraction),
        take_fraction(&mut source.storage, fraction),
        take_fraction(&mut source.transfer, fraction),
    ];
    let target = tissues
        .get_mut(&receiver)
        .ok_or(VegetationError::Domain("missing deadwood pool"))?;
    for (destination, loss) in [
        &mut target.display,
        &mut target.storage,
        &mut target.transfer,
    ]
    .into_iter()
    .zip(losses)
    {
        destination.carbon += loss.carbon;
        destination.nitrogen += loss.nitrogen;
    }
    Ok(())
}

fn take_fraction(pool: &mut ElementPool, fraction: f64) -> ElementPool {
    let loss = ElementPool {
        carbon: pool.carbon * fraction,
        nitrogen: pool.nitrogen * fraction,
    };
    pool.carbon -= loss.carbon;
    pool.nitrogen -= loss.nitrogen;
    loss
}
fn route_fraction(
    tissues: &mut std::collections::BTreeMap<Tissue, TissuePool>,
    tissue: Tissue,
    fraction: f64,
    fractions: &[f64; 3],
    p: &CnParameters,
    out: &mut Vec<MaterialTransferAmounts>,
) -> Result<(), VegetationError> {
    let loss = debit_fraction(
        tissues
            .get_mut(&tissue)
            .ok_or(VegetationError::Domain("missing litter donor"))?,
        fraction,
    );
    for (receiver, share) in [
        ReceiverClass::Metabolic,
        ReceiverClass::Cellulose,
        ReceiverClass::Lignin,
    ]
    .into_iter()
    .zip(fractions)
    {
        out.push(material_transfer(
            tissue,
            receiver,
            loss.carbon * share,
            loss.nitrogen * share,
            p.drymatter_carbon_fraction,
        )?);
    }
    Ok(())
}
fn route_cwd(
    tissues: &mut std::collections::BTreeMap<Tissue, TissuePool>,
    tissue: Tissue,
    fraction: f64,
    p: &CnParameters,
    out: &mut Vec<MaterialTransferAmounts>,
) -> Result<(), VegetationError> {
    let loss = debit_fraction(
        tissues
            .get_mut(&tissue)
            .ok_or(VegetationError::Domain("missing CWD donor"))?,
        fraction,
    );
    out.push(material_transfer(
        tissue,
        ReceiverClass::CoarseWoodyDebris,
        loss.carbon,
        loss.nitrogen,
        p.drymatter_carbon_fraction,
    )?);
    Ok(())
}
fn validate_parameters(p: &CnParameters) -> Result<(), VegetationError> {
    let values = [
        p.growth_respiration_ratio,
        p.a1_froot_leaf,
        p.a2_croot_stem,
        p.a3_stem_leaf,
        p.a4_livewood_fraction,
        p.current_growth_fraction,
        p.cn_leaf,
        p.cn_leaf_litter,
        p.cn_froot,
        p.cn_livewood,
        p.cn_deadwood,
        p.drymatter_carbon_fraction,
        p.xs_recovery_days,
        p.leaf_lifetime_s,
        p.froot_lifetime_s,
        p.livewood_turnover_s,
        p.mortality_rate_s1,
    ];
    if values.iter().any(|v| !v.is_finite())
        || p.growth_respiration_ratio < 0.0
        || p.a1_froot_leaf < 0.0
        || p.a2_croot_stem < 0.0
        || p.a3_stem_leaf < 0.0
        || !(0.0..=1.0).contains(&p.a4_livewood_fraction)
        || !(0.0..=1.0).contains(&p.current_growth_fraction)
        || p.cn_leaf <= 0.0
        || p.cn_leaf_litter <= 0.0
        || p.cn_froot <= 0.0
        || p.cn_livewood <= 0.0
        || p.cn_deadwood <= 0.0
        || p.drymatter_carbon_fraction <= 0.0
        || p.drymatter_carbon_fraction > 1.0
        || p.xs_recovery_days <= 0.0
        || p.leaf_lifetime_s <= 0.0
        || p.froot_lifetime_s <= 0.0
        || p.livewood_turnover_s <= 0.0
        || p.mortality_rate_s1 < 0.0
    {
        return Err(VegetationError::Domain("C/N parameters"));
    }
    for fractions in [&p.leaf_litter_fractions, &p.froot_litter_fractions] {
        if fractions.iter().any(|v| !v.is_finite() || *v < 0.0)
            || (fractions.iter().sum::<f64>() - 1.0).abs() > 1e-12
        {
            return Err(VegetationError::Domain("litter fractions"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn parameters() -> CnParameters {
        CnParameters {
            growth_respiration_ratio: 0.1,
            a1_froot_leaf: 0.8,
            a2_croot_stem: 0.25,
            a3_stem_leaf: 0.35,
            a4_livewood_fraction: 0.2,
            current_growth_fraction: 0.6,
            cn_leaf: 30.0,
            cn_leaf_litter: 60.0,
            cn_froot: 45.0,
            cn_livewood: 55.0,
            cn_deadwood: 450.0,
            drymatter_carbon_fraction: 0.5,
            xs_recovery_days: 30.0,
            leaf_lifetime_s: 3.0 * 365.0 * 86_400.0,
            froot_lifetime_s: 2.0 * 365.0 * 86_400.0,
            livewood_turnover_s: 5.0 * 365.0 * 86_400.0,
            mortality_rate_s1: 0.0,
            leaf_litter_fractions: [0.25, 0.25, 0.5],
            froot_litter_fractions: [0.25, 0.25, 0.5],
        }
    }

    fn tissues_with_leaf(display: ElementPool) -> BTreeMap<Tissue, TissuePool> {
        [
            Tissue::Leaf,
            Tissue::FineRoot,
            Tissue::LiveStem,
            Tissue::DeadStem,
            Tissue::LiveCoarseRoot,
            Tissue::DeadCoarseRoot,
        ]
        .into_iter()
        .map(|tissue| {
            let pool = if tissue == Tissue::Leaf {
                TissuePool {
                    display,
                    ..TissuePool::default()
                }
            } else {
                TissuePool::default()
            };
            (tissue, pool)
        })
        .collect()
    }

    fn offset_once(
        tissues: &mut BTreeMap<Tissue, TissuePool>,
        parameters: &CnParameters,
    ) -> Result<PhenologyUpdate, VegetationError> {
        advance_phenology(
            tissues,
            PhenologyMode::SeasonalDeciduous,
            PhenologyPhase::Offset,
            0.0,
            4.0,
            0.2,
            0.2,
            1.0,
            0.6,
            0.3,
            3.0,
            4.0,
            parameters,
        )
    }

    #[test]
    fn deciduous_offset_uses_exact_cn_leaf_retranslocation_identity() {
        let parameters = parameters();
        let donor_n = 0.9;
        let mut tissues = tissues_with_leaf(ElementPool {
            carbon: 12.0,
            nitrogen: donor_n,
        });

        let update = offset_once(&mut tissues, &parameters).expect("valid offset debit");

        let fallen_c = 6.0;
        let prescribed_n_debit = fallen_c / parameters.cn_leaf;
        let expected_litter_n = fallen_c / parameters.cn_leaf_litter;
        let expected_retranslocated_n = prescribed_n_debit - expected_litter_n;
        let litter_c = update
            .transfers
            .iter()
            .map(|transfer| transfer.carbon)
            .sum::<f64>();
        let litter_n = update
            .transfers
            .iter()
            .map(|transfer| transfer.nitrogen)
            .sum::<f64>();
        let leaf = tissues.get(&Tissue::Leaf).expect("leaf donor");

        assert!((litter_c - fallen_c).abs() < 1e-15);
        assert!((litter_n - expected_litter_n).abs() < 1e-15);
        assert!((update.retranslocated_n - expected_retranslocated_n).abs() < 1e-15);
        assert!((leaf.display.carbon - (12.0 - fallen_c)).abs() < 1e-15);
        assert!((leaf.display.nitrogen - (donor_n - prescribed_n_debit)).abs() < 1e-15);
        assert!(
            (donor_n - leaf.display.nitrogen - litter_n - update.retranslocated_n).abs() < 1e-15
        );
        assert!((update.retranslocated_n - (donor_n - litter_n)).abs() > 0.5);
    }

    #[test]
    fn deciduous_offset_rejects_insufficient_prescribed_n_debit_without_mutation() {
        let parameters = parameters();
        let mut tissues = tissues_with_leaf(ElementPool {
            carbon: 12.0,
            nitrogen: 0.19,
        });
        let before = tissues.clone();

        let error = offset_once(&mut tissues, &parameters).expect_err("insufficient leaf N");

        assert_eq!(
            error,
            VegetationError::Domain("leaf offset donor cannot cover prescribed C/N debit")
        );
        assert_eq!(tissues, before);
    }

    #[test]
    fn v3_atkin_rd_and_carbon_debit_use_one_unit_conversion() {
        let rd25 = atkin_rd25(0.002, 2.0, 296.0, 1.25).expect("positive Atkin Rd25");
        let expected_rd25 = 1.25 + 0.2061 - 0.0402 * (296.0 - 273.15);
        assert!((rd25 - expected_rd25).abs() < 1.0e-15);
        let rd = leaf_rd_at_temperature(rd25, 298.15).expect("temperature response");
        assert!((rd - rd25).abs() < 1.0e-12);
        let debit = leaf_rd_carbon_debit(rd, 1.2, 1800.0, 0.35).expect("carbon debit");
        assert!((debit - rd * 1.0e-6 * 0.012_011 * 1.2 * 1800.0 * 0.35).abs() < 1.0e-18);
        assert!(atkin_rd25(0.0, 1.0, 350.0, 0.0).is_err());
        assert_eq!(atkin_rd25(0.0, 0.0, 296.0, 0.0), Ok(0.0));
    }

    #[test]
    fn root_maintenance_operands_preserve_layer_identity() {
        let mut tissues = tissues_with_leaf(ElementPool::default());
        tissues
            .get_mut(&Tissue::FineRoot)
            .expect("fine-root pool")
            .display
            .nitrogen = 0.004;
        tissues
            .get_mut(&Tissue::LiveStem)
            .expect("live-stem pool")
            .display
            .nitrogen = 0.002;
        tissues
            .get_mut(&Tissue::LiveCoarseRoot)
            .expect("live-coarse-root pool")
            .display
            .nitrogen = 0.001;
        let layer_a = RootRespirationOperand {
            layer_id: SoilLayerId::try_new("upper").expect("layer identity"),
            temperature_k: 293.15,
            nitrogen_fraction: 0.7,
        };
        let layer_b = RootRespirationOperand {
            layer_id: SoilLayerId::try_new("lower").expect("layer identity"),
            temperature_k: 303.15,
            nitrogen_fraction: 0.3,
        };
        let ordered = maintenance_respiration(
            &tissues,
            0.000_002,
            293.15,
            &[layer_a.clone(), layer_b.clone()],
            1.0e-7,
            2.0,
            3600.0,
        )
        .expect("typed root respiration");
        let reversed = maintenance_respiration(
            &tissues,
            0.000_002,
            293.15,
            &[layer_b, layer_a.clone()],
            1.0e-7,
            2.0,
            3600.0,
        )
        .expect("order-invariant typed root respiration");
        let expected_livewood = 0.003 * 1.0e-7 * 3600.0;
        let expected_fine_root = 0.004 * 1.0e-7 * (0.7 + 0.3 * 2.0) * 3600.0;
        assert!((ordered - (0.000_002 + expected_livewood + expected_fine_root)).abs() < 1e-18);
        assert_eq!(ordered.to_bits(), reversed.to_bits());

        let duplicate = RootRespirationOperand {
            layer_id: layer_a.layer_id.clone(),
            temperature_k: 303.15,
            nitrogen_fraction: 0.3,
        };
        assert_eq!(
            maintenance_respiration(
                &tissues,
                0.0,
                293.15,
                &[
                    RootRespirationOperand {
                        nitrogen_fraction: 0.7,
                        ..layer_a
                    },
                    duplicate,
                ],
                1.0e-7,
                2.0,
                3600.0,
            ),
            Err(VegetationError::Domain(
                "duplicate fine-root respiration layer"
            ))
        );
    }

    #[test]
    fn root_maintenance_rejects_incomplete_fraction_basis() {
        let tissues = tissues_with_leaf(ElementPool::default());
        let incomplete = [RootRespirationOperand {
            layer_id: SoilLayerId::try_new("upper").expect("layer identity"),
            temperature_k: 293.15,
            nitrogen_fraction: 0.8,
        }];
        assert_eq!(
            maintenance_respiration(&tissues, 0.0, 293.15, &incomplete, 1.0e-7, 2.0, 3600.0,),
            Err(VegetationError::Domain("maintenance respiration"))
        );
    }

    #[test]
    fn material_amounts_require_explicit_valid_binding() {
        let amounts = material_transfer(
            Tissue::Leaf,
            ReceiverClass::Metabolic,
            0.00432,
            0.000_100_285_714_285_714_27,
            0.48,
        )
        .expect("unbound material amounts");
        assert!((amounts.dry_matter - 0.009).abs() < 1e-15);
        assert!((amounts.carbon - amounts.dry_matter).abs() > 1e-6);
        let owner = ResourceOwnerId::try_new("vegetation:stratum-a").expect("owner identity");
        assert_eq!(
            amounts.bind(TransactionId(0), &owner, 1),
            Err(VegetationError::Domain("material transfer identity"))
        );
        assert_eq!(
            amounts.bind(TransactionId(9), &owner, 0),
            Err(VegetationError::Domain("material transfer identity"))
        );
        let forged = MaterialTransferAmounts {
            carbon: f64::NAN,
            ..amounts
        };
        assert_eq!(
            forged.bind(TransactionId(9), &owner, 1),
            Err(VegetationError::Domain("material transfer identity"))
        );
        let bound = amounts
            .bind(TransactionId(9), &owner, 7)
            .expect("bound material proposal");
        assert_eq!(bound.transaction_id, 9);
        assert_eq!(bound.owner_id, "vegetation:stratum-a");
        assert_eq!(bound.proposal_id, 7);
        assert_eq!(bound.carbon.to_bits(), amounts.carbon.to_bits());
        assert_eq!(bound.nitrogen.to_bits(), amounts.nitrogen.to_bits());
        assert_eq!(bound.dry_matter.to_bits(), amounts.dry_matter.to_bits());
    }
}
