use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};
use serde::{Deserialize, Serialize};

use crate::CoupledTimeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ModelTimeNs(pub u128);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeSupport {
    pub start_ns: ModelTimeNs,
    pub end_ns: ModelTimeNs,
}

impl TimeSupport {
    pub fn new(start_ns: ModelTimeNs, end_ns: ModelTimeNs) -> Result<Self, CoupledTimeError> {
        if start_ns >= end_ns {
            return Err(CoupledTimeError::InvalidSupport);
        }
        Ok(Self { start_ns, end_ns })
    }

    #[must_use]
    pub const fn duration_ns(self) -> u128 {
        self.end_ns.0 - self.start_ns.0
    }

    /// Derive the one common binary64 seconds operand for every participant.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // The contract explicitly requires correctly rounded u128 -> binary64.
    pub fn duration_s_bits(self) -> u64 {
        ((self.duration_ns() as f64) / 1_000_000_000.0).to_bits()
    }
}

/// Quantize finite nonnegative seconds to nanoseconds, nearest/ties-to-even.
pub fn quantize_seconds_to_tick(
    parent_start: ModelTimeNs,
    parent_end: ModelTimeNs,
    seconds: f64,
) -> Result<ModelTimeNs, CoupledTimeError> {
    if !seconds.is_finite() || seconds.is_sign_negative() && seconds != 0.0 {
        return Err(CoupledTimeError::EventProposal);
    }
    let bits = seconds.to_bits();
    let fraction = bits & ((1_u64 << 52) - 1);
    let biased = ((bits >> 52) & 0x7ff) as i32;
    let (significand, exponent) = if biased == 0 {
        (fraction, -1074)
    } else {
        ((1_u64 << 52) | fraction, biased - 1023 - 52)
    };
    let mut numerator = BigUint::from(significand) * BigUint::from(1_000_000_000_u64);
    let rounded = if exponent >= 0 {
        numerator <<= usize::try_from(exponent).map_err(|_| CoupledTimeError::EventProposal)?;
        numerator
    } else {
        let shift = usize::try_from(-exponent).map_err(|_| CoupledTimeError::EventProposal)?;
        let denominator = BigUint::from(1_u8) << shift;
        let quotient = &numerator / &denominator;
        let remainder = numerator % &denominator;
        let twice = &remainder << 1_usize;
        if twice > denominator
            || twice == denominator && (&quotient & BigUint::from(1_u8)) != BigUint::zero()
        {
            quotient + BigUint::from(1_u8)
        } else {
            quotient
        }
    };
    let delta = rounded.to_u128().ok_or(CoupledTimeError::EventProposal)?;
    let tick = parent_start
        .0
        .checked_add(delta)
        .ok_or(CoupledTimeError::ArithmeticOverflow)?;
    if tick > parent_end.0 {
        return Err(CoupledTimeError::EventProposal);
    }
    Ok(ModelTimeNs(tick))
}
