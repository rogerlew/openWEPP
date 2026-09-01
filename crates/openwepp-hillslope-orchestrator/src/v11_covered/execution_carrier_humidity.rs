/// One explicit default-off invocation of the actual `DirectV10` owner stack.
#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoveredCarrierLiveConsumptionRowV1 {
    pub lane_id: u32,
    pub forcing_sha256: Digest32,
    pub reference_specific_humidity_bits: u64,
    pub snow_specific_humidity_bits: u64,
    pub shared_specific_humidity_bits: u64,
    pub snow_vapor_into_surface_bits: u64,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoveredCarrierCondensationCreditAuditV1 {
    pub transaction_id: u128,
    pub hydrology_owner_id: String,
    pub ofe_id: String,
    pub tile_id: String,
    pub surface_id: String,
    pub amount_bits: u64,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OpenSnowLiveConsumptionRowV1 {
    pub lane_id: u32,
    pub forcing_sha256: Digest32,
    pub reference_specific_humidity_bits: u64,
    pub snow_specific_humidity_bits: u64,
    pub vapor_outward_bits: u64,
}

#[cfg(test)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct CoveredCarrierLiveConsumptionAuditV1 {
    pub carrier_rows: Vec<CoveredCarrierLiveConsumptionRowV1>,
    pub open_snow_rows: Vec<OpenSnowLiveConsumptionRowV1>,
    pub condensation_credits: Vec<CoveredCarrierCondensationCreditAuditV1>,
}

#[cfg(test)]
std::thread_local! {
    static COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT: std::cell::RefCell<Option<CoveredCarrierLiveConsumptionAuditV1>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn begin_covered_carrier_live_consumption_audit() {
    COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT.with(|audit| {
        *audit.borrow_mut() = Some(CoveredCarrierLiveConsumptionAuditV1::default());
    });
}

#[cfg(test)]
pub(crate) fn take_covered_carrier_live_consumption_audit() -> CoveredCarrierLiveConsumptionAuditV1
{
    COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT
        .with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
fn audit_covered_carrier_live_row(row: CoveredCarrierLiveConsumptionRowV1) {
    COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.carrier_rows.push(row);
        }
    });
}

#[cfg(test)]
fn audit_covered_carrier_condensation_credits(
    credits: &[openwepp_land_surface_energy::CondensationCredit],
) {
    COVERED_CARRIER_LIVE_CONSUMPTION_AUDIT.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit
                .condensation_credits
                .extend(
                    credits
                        .iter()
                        .map(|credit| CoveredCarrierCondensationCreditAuditV1 {
                            transaction_id: credit.transaction_id.0,
                            hydrology_owner_id: credit.hydrology_owner_id.as_str().to_owned(),
                            ofe_id: credit.ofe_id.as_str().to_owned(),
                            tile_id: credit.tile_id.as_str().to_owned(),
                            surface_id: credit.surface_id.as_str().to_owned(),
                            amount_bits: credit.amount_kg_m2_stand_ground.to_bits(),
                        }),
                );
        }
    });
}

#[cfg(not(test))]
fn audit_covered_carrier_condensation_credits(
    _: &[openwepp_land_surface_energy::CondensationCredit],
) {
}

fn shared_carrier_specific_humidity_v1(
    surfaces: &[CarrierSurface],
) -> Result<f64, DirectV11RealConsumerError> {
    let mut denominator = 0.0;
    let mut numerator = 0.0;
    let mut common_active_bits = None;
    let mut equal_active_nodes = true;
    for surface in surfaces {
        let conductance = surface.vapor_conductance_m_s;
        if !conductance.is_finite()
            || conductance < 0.0
            || !surface.specific_humidity.is_finite()
            || !(0.0..=1.0).contains(&surface.specific_humidity)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier humidity conductance domain",
            ));
        }
        if conductance.to_bits() == 0.0_f64.to_bits() {
            continue;
        }
        denominator += conductance;
        numerator += conductance * surface.specific_humidity;
        match common_active_bits {
            None => common_active_bits = Some(surface.specific_humidity.to_bits()),
            Some(bits) => equal_active_nodes &= bits == surface.specific_humidity.to_bits(),
        }
    }
    if !denominator.is_finite() || denominator <= 0.0 || !numerator.is_finite() {
        return Err(DirectV11RealConsumerError::Identity(
            "covered carrier humidity denominator",
        ));
    }
    if equal_active_nodes {
        return Ok(f64::from_bits(common_active_bits.ok_or(
            DirectV11RealConsumerError::Identity("covered carrier active humidity set"),
        )?));
    }
    let shared = numerator / denominator;
    if !shared.is_finite() || !(0.0..=1.0).contains(&shared) {
        return Err(DirectV11RealConsumerError::Identity(
            "covered carrier shared humidity domain",
        ));
    }
    Ok(shared)
}

#[cfg(test)]
mod shared_carrier_specific_humidity_tests {
    use super::*;

    fn surface(q: f64, conductance: f64) -> CarrierSurface {
        CarrierSurface {
            temperature_k: 273.15,
            specific_humidity: q,
            heat_conductance_m_s: conductance,
            vapor_conductance_m_s: conductance,
        }
    }

    #[test]
    fn equal_active_nodes_are_exact_and_active_one_bit_poison_uses_weighted_solve() {
        let q = 0.003_757_503_415_507_667_5_f64;
        let inactive_poison_q = f64::from_bits(q.to_bits() + 9);
        let exact = shared_carrier_specific_humidity_v1(&[
            surface(q, 1.0),
            surface(inactive_poison_q, 0.0),
            surface(q, 3.0),
        ])
        .expect("equal active-node humidity");
        assert_eq!(exact.to_bits(), q.to_bits());

        let active_poison_q = f64::from_bits(q.to_bits() + 1);
        let weighted = shared_carrier_specific_humidity_v1(&[
            surface(q, 1.0),
            surface(inactive_poison_q, 0.0),
            surface(active_poison_q, 1.0),
        ])
        .expect("one-bit active-node weighted humidity");
        let expected = (q + active_poison_q) / 2.0;
        assert_eq!(weighted.to_bits(), expected.to_bits());
        assert_ne!(weighted.to_bits(), q.to_bits());

        assert!(shared_carrier_specific_humidity_v1(&[surface(q, 0.0), surface(q, 0.0),]).is_err());
        assert!(shared_carrier_specific_humidity_v1(&[surface(q, -f64::MIN_POSITIVE)]).is_err());
    }
}
