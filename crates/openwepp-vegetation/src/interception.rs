//! E04--E05 finite liquid interception and surface partitioning.
use crate::VegetationError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InterceptionInput {
    pub store0: f64,
    pub rain: f64,
    pub vapor_amount: f64,
    pub lai: f64,
    pub sai: f64,
    pub alpha_liq: f64,
    pub p_liq: f64,
    pub stemflow_fraction: f64,
    pub leaf_temperature_k: f64,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InterceptionResult {
    pub store1: f64,
    pub evaporation: f64,
    pub condensation: f64,
    pub throughfall: f64,
    pub stemflow: f64,
    pub drainage: f64,
    pub wet_fraction: f64,
    pub closure_residual: f64,
}

pub fn liquid_interception(i: InterceptionInput) -> Result<InterceptionResult, VegetationError> {
    let values = [
        i.store0,
        i.rain,
        i.vapor_amount,
        i.lai,
        i.sai,
        i.alpha_liq,
        i.p_liq,
        i.stemflow_fraction,
        i.leaf_temperature_k,
    ];
    if values.iter().any(|v| !v.is_finite())
        || i.store0 < 0.0
        || i.rain < 0.0
        || i.lai < 0.0
        || i.sai < 0.0
        || !(0.0..=1.0).contains(&i.alpha_liq)
        || !(0.0..=1.0).contains(&i.stemflow_fraction)
        || i.p_liq < 0.0
    {
        return Err(VegetationError::Domain("liquid interception"));
    }
    if i.leaf_temperature_k < 273.15 {
        return Err(VegetationError::Unsupported("CANOPY_SNOW"));
    }
    let intercepted = i.alpha_liq * (i.lai + i.sai).tanh() * i.rain;
    let free = i.rain - intercepted;
    let stemflow = i.stemflow_fraction * free;
    let throughfall = (1.0 - i.stemflow_fraction) * free;
    let capacity = i.p_liq * (i.lai + i.sai);
    let mut store = i.store0 + intercepted;
    let mut drainage = (store - capacity).max(0.0);
    store -= drainage;
    let (evaporation, condensation) = if i.vapor_amount >= 0.0 {
        let e = i.vapor_amount.min(store);
        store -= e;
        (e, 0.0)
    } else {
        let c = -i.vapor_amount;
        store += c;
        let d2 = (store - capacity).max(0.0);
        store -= d2;
        drainage += d2;
        (0.0, c)
    };
    let wet_fraction = if capacity > 0.0 {
        (store / capacity).powf(2.0 / 3.0)
    } else {
        0.0
    };
    let closure =
        i.store0 + i.rain + condensation - store - evaporation - throughfall - stemflow - drainage;
    Ok(InterceptionResult {
        store1: store,
        evaporation,
        condensation,
        throughfall,
        stemflow,
        drainage,
        wet_fraction,
        closure_residual: closure,
    })
}
