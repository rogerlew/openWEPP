//! E16--E22 persistent carbon, nitrogen, phenology, and material transfer.
use crate::VegetationError;
use crate::transaction::PhenologyPhase;
use openwepp_kernel_contract::MaterialDonorClass;

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
    leaf_area: f64,
    t10_k: f64,
    air_temperature_k: f64,
    fine_root_temperatures_k: &[f64],
    fine_root_fractions: &[f64],
    atkin_intercept: f64,
    base_rate: f64,
    q10: f64,
    dt_s: f64,
) -> Result<f64, VegetationError> {
    if ![
        leaf_area,
        t10_k,
        air_temperature_k,
        atkin_intercept,
        base_rate,
        q10,
        dt_s,
    ]
    .iter()
    .all(|value| value.is_finite())
        || leaf_area < 0.0
        || t10_k <= 0.0
        || air_temperature_k <= 0.0
        || base_rate < 0.0
        || q10 <= 0.0
        || dt_s <= 0.0
        || fine_root_temperatures_k.len() != fine_root_fractions.len()
        || fine_root_temperatures_k
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.0)
        || fine_root_fractions
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        || (fine_root_fractions.iter().sum::<f64>() - 1.0).abs() > 1e-12
    {
        return Err(VegetationError::Domain("maintenance respiration"));
    }
    let nitrogen = |tissue: Tissue| -> Result<f64, VegetationError> {
        let pool = tissues
            .get(&tissue)
            .ok_or(VegetationError::Domain("missing tissue pool"))?;
        Ok(pool.display.nitrogen + pool.storage.nitrogen + pool.transfer.nitrogen)
    };
    let leaf_n = nitrogen(Tissue::Leaf)?;
    let n_area = if leaf_area == 0.0 {
        0.0
    } else {
        leaf_n / leaf_area
    };
    // Atkin source units are g C m-2 leaf d-1; convert to kg C and the interval.
    let n_area_g_m2 = n_area * 1_000.0;
    let leaf_rate_g_day = atkin_intercept + 0.2061 * n_area_g_m2 - 0.0402 * (t10_k - 273.15);
    if !leaf_rate_g_day.is_finite() || leaf_rate_g_day < 0.0 {
        return Err(VegetationError::Domain("negative Atkin leaf respiration"));
    }
    let leaf = leaf_rate_g_day * 1e-3 * leaf_area * dt_s / 86_400.0;
    let livewood_n = nitrogen(Tissue::LiveStem)? + nitrogen(Tissue::LiveCoarseRoot)?;
    let livewood = livewood_n * base_rate * q10.powf((air_temperature_k - 293.15) / 10.0) * dt_s;
    let fine_root_n = nitrogen(Tissue::FineRoot)?;
    let fine_root = fine_root_temperatures_k
        .iter()
        .zip(fine_root_fractions)
        .map(|(temperature, fraction)| {
            fine_root_n * fraction * base_rate * q10.powf((*temperature - 293.15) / 10.0) * dt_s
        })
        .sum::<f64>();
    let total = leaf + livewood + fine_root;
    if total.is_finite() {
        Ok(total)
    } else {
        Err(VegetationError::Domain("maintenance response"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Allocation {
    pub eta: f64,
    pub leaf_c: f64,
    pub froot_c: f64,
    pub stem_c: f64,
    pub croot_c: f64,
    pub growth_respiration: f64,
    pub n_used: f64,
    pub nsc_end: f64,
}
#[derive(Clone, Copy, Debug)]
pub struct AllocationInput {
    pub carbon_offer: f64,
    pub nitrogen_available: f64,
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub growth_resp_ratio: f64,
    pub cn_leaf: f64,
    pub cn_froot: f64,
    pub cn_wood: f64,
}
pub fn allocate(i: AllocationInput) -> Result<Allocation, VegetationError> {
    let vals = [
        i.carbon_offer,
        i.nitrogen_available,
        i.a1,
        i.a2,
        i.a3,
        i.growth_resp_ratio,
        i.cn_leaf,
        i.cn_froot,
        i.cn_wood,
    ];
    if vals.iter().any(|v| !v.is_finite())
        || i.carbon_offer < 0.0
        || i.nitrogen_available < 0.0
        || i.a1 < 0.0
        || i.a2 < 0.0
        || i.a3 < 0.0
        || !(0.0..1.0).contains(&i.growth_resp_ratio)
        || i.cn_leaf <= 0.0
        || i.cn_froot <= 0.0
        || i.cn_wood <= 0.0
    {
        return Err(VegetationError::Domain("allocation"));
    }
    let callom = (1.0 + i.growth_resp_ratio) * (1.0 + i.a1 + i.a3 * (1.0 + i.a2));
    let nallom = 1.0 / i.cn_leaf + i.a1 / i.cn_froot + i.a3 * (1.0 + i.a2) / i.cn_wood;
    let demand = i.carbon_offer * nallom / callom;
    let eta = if demand == 0.0 {
        1.0
    } else {
        (i.nitrogen_available / demand).min(1.0)
    };
    let leaf = eta * i.carbon_offer / callom;
    let froot = i.a1 * leaf;
    let stem = i.a3 * leaf;
    let croot = i.a2 * stem;
    let structural = leaf + froot + stem + croot;
    let respiration = i.growth_resp_ratio * structural;
    Ok(Allocation {
        eta,
        leaf_c: leaf,
        froot_c: froot,
        stem_c: stem,
        croot_c: croot,
        growth_respiration: respiration,
        n_used: eta * demand,
        nsc_end: (1.0 - eta) * i.carbon_offer,
    })
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
) -> Result<MaterialTransfer, VegetationError> {
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
    Ok(MaterialTransfer {
        transaction_id: 0,
        owner_id: String::new(),
        proposal_id: 0,
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
) -> Result<Vec<MaterialTransfer>, VegetationError> {
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
    pub transfers: Vec<MaterialTransfer>,
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
    out: &mut Vec<MaterialTransfer>,
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
    out: &mut Vec<MaterialTransfer>,
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
}
