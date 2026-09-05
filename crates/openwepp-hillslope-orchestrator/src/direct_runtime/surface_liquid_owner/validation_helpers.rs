fn validate_same_ofe_value<T: Eq>(
    values: &mut BTreeMap<OfeId, T>,
    ofe_id: OfeId,
    value: T,
    error: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match values.entry(ofe_id) {
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(value);
        }
        std::collections::btree_map::Entry::Occupied(entry) if entry.get() != &value => {
            return Err(DirectSurfaceLiquidError::Identity(error));
        }
        std::collections::btree_map::Entry::Occupied(_) => {}
    }
    Ok(())
}

fn configured_ofes(configuration: &DirectSurfaceLiquidConfiguration) -> Vec<OfeId> {
    configuration.ofe_topology.clone()
}

fn require_positive(value: f64, field: &'static str) -> Result<(), DirectSurfaceLiquidError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(DirectSurfaceLiquidError::Domain(field))
    }
}

fn require_nonnegative(value: f64, field: &'static str) -> Result<(), DirectSurfaceLiquidError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(DirectSurfaceLiquidError::Domain(field))
    }
}

fn f64_bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

fn parse_f64_bits(value: &str) -> Result<f64, DirectSurfaceLiquidError> {
    if value.len() != 16
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(DirectSurfaceLiquidError::Schema(
            "canonical f64 must be 16 lowercase hexadecimal digits",
        ));
    }
    let bits = u64::from_str_radix(value, 16)
        .map_err(|_| DirectSurfaceLiquidError::Schema("canonical f64 parse"))?;
    Ok(f64::from_bits(bits))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
