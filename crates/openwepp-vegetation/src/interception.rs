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
    pub initial_drainage: f64,
    pub second_drainage: f64,
    pub wet_fraction: f64,
    pub closure_residual: f64,
}

impl InterceptionResult {
    #[must_use]
    pub fn drainage(&self) -> f64 {
        self.initial_drainage + self.second_drainage
    }
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
    let initial_drainage = (store - capacity).max(0.0);
    store -= initial_drainage;
    let wet_fraction = if capacity > 0.0 {
        (store / capacity).powf(2.0 / 3.0)
    } else {
        0.0
    };
    let (evaporation, condensation, second_drainage) = if i.vapor_amount >= 0.0 {
        let e = i.vapor_amount.min(store);
        store -= e;
        (e, 0.0, 0.0)
    } else {
        let c = -i.vapor_amount;
        store += c;
        let second_drainage = (store - capacity).max(0.0);
        store -= second_drainage;
        (0.0, c, second_drainage)
    };
    let closure = i.store0 + i.rain + condensation
        - store
        - evaporation
        - throughfall
        - stemflow
        - initial_drainage
        - second_drainage;
    Ok(InterceptionResult {
        store1: store,
        evaporation,
        condensation,
        throughfall,
        stemflow,
        initial_drainage,
        second_drainage,
        wet_fraction,
        closure_residual: closure,
    })
}

#[cfg(test)]
mod tests {
    use super::{InterceptionInput, liquid_interception};

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() <= 1e-14, "{actual} != {expected}");
    }

    fn input() -> InterceptionInput {
        InterceptionInput {
            store0: 0.0,
            rain: 0.0,
            vapor_amount: 0.0,
            lai: 1.0,
            sai: 0.0,
            alpha_liq: 0.0,
            p_liq: 0.5,
            stemflow_fraction: 0.0,
            leaf_temperature_k: 295.0,
        }
    }

    #[test]
    fn evaporation_has_no_second_drainage_and_uses_pre_vapor_wet_fraction() {
        let result = liquid_interception(InterceptionInput {
            store0: 0.2,
            vapor_amount: 0.1,
            ..input()
        })
        .expect("valid liquid interception");

        assert_close(result.evaporation, 0.1);
        assert_close(result.condensation, 0.0);
        assert_close(result.initial_drainage, 0.0);
        assert_close(result.second_drainage, 0.0);
        assert_close(result.store1, 0.1);
        assert_close(result.wet_fraction, 0.4_f64.powf(2.0 / 3.0));
    }

    #[test]
    fn condensation_exposes_positive_second_drainage() {
        let result = liquid_interception(InterceptionInput {
            store0: 0.5,
            vapor_amount: -0.25,
            ..input()
        })
        .expect("valid liquid interception");

        assert_close(result.evaporation, 0.0);
        assert_close(result.condensation, 0.25);
        assert_close(result.initial_drainage, 0.0);
        assert_close(result.second_drainage, 0.25);
        assert_close(result.drainage(), 0.25);
        assert_close(result.store1, 0.5);
    }

    #[test]
    fn stemflow_and_throughfall_partition_free_rain_exactly() {
        let result = liquid_interception(InterceptionInput {
            rain: 8.0,
            lai: 0.0,
            p_liq: 0.0,
            stemflow_fraction: 0.25,
            ..input()
        })
        .expect("valid liquid interception");

        assert_close(result.stemflow, 2.0);
        assert_close(result.throughfall, 6.0);
        assert_close(result.stemflow + result.throughfall, 8.0);
    }

    #[test]
    fn closure_reconstructs_with_both_drainage_operands() {
        let result = liquid_interception(InterceptionInput {
            store0: 0.75,
            rain: 1.0,
            vapor_amount: -0.5,
            stemflow_fraction: 0.25,
            ..input()
        })
        .expect("valid liquid interception");

        assert_close(result.initial_drainage, 0.25);
        assert_close(result.second_drainage, 0.5);
        assert_close(result.closure_residual, 0.0);
        assert_close(
            result.store1
                + result.evaporation
                + result.throughfall
                + result.stemflow
                + result.initial_drainage
                + result.second_drainage,
            0.75 + 1.0 + result.condensation,
        );
    }
}
