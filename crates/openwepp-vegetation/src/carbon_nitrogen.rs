//! E16--E22 persistent carbon, nitrogen, phenology, and material transfer.
use crate::VegetationError;

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

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tissue {
    Leaf,
    FineRoot,
    LiveStem,
    DeadStem,
    LiveCoarseRoot,
    DeadCoarseRoot,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReceiverClass {
    Metabolic,
    Cellulose,
    Lignin,
    CoarseWoodyDebris,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialTransfer {
    pub tissue: Tissue,
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
        tissue,
        receiver,
        carbon,
        nitrogen,
        dry_matter: carbon / carbon_fraction,
    })
}
