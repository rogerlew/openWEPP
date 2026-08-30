use super::{SnowStage3V11QualifiedMassEnthalpyTotalV1, SnowStage3V11QualifiedSurfaceRouteV1};
use serde::{Deserialize, de::Error as _, ser::SerializeSeq as _};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SnowStage3V11QualifiedSurfaceRouteEntryV1 {
    route: SnowStage3V11QualifiedSurfaceRouteV1,
    total: SnowStage3V11QualifiedMassEnthalpyTotalV1,
}

pub(super) fn serialize<S>(
    map: &BTreeMap<SnowStage3V11QualifiedSurfaceRouteV1, SnowStage3V11QualifiedMassEnthalpyTotalV1>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut sequence = serializer.serialize_seq(Some(map.len()))?;
    for (route, total) in map {
        sequence.serialize_element(&SnowStage3V11QualifiedSurfaceRouteEntryV1 {
            route: route.clone(),
            total: *total,
        })?;
    }
    sequence.end()
}

pub(super) fn deserialize<'de, D>(
    deserializer: D,
) -> Result<
    BTreeMap<SnowStage3V11QualifiedSurfaceRouteV1, SnowStage3V11QualifiedMassEnthalpyTotalV1>,
    D::Error,
>
where
    D: serde::Deserializer<'de>,
{
    let entries = Vec::<SnowStage3V11QualifiedSurfaceRouteEntryV1>::deserialize(deserializer)?;
    if entries
        .windows(2)
        .any(|pair| pair[0].route >= pair[1].route)
    {
        return Err(D::Error::custom(
            "qualification surface routes are not strictly ordered",
        ));
    }
    Ok(entries
        .into_iter()
        .map(|entry| (entry.route, entry.total))
        .collect())
}
