fn compose_terminal_precipitation_sets_v1(
    phases: &[CoveredCarrierPhaseResultV1],
    accepted_support: TimeSupport,
) -> Result<BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>, DirectV11RealConsumerError> {
    let first = phases.first().ok_or(DirectV11RealConsumerError::Identity(
        "terminal precipitation empty carrier chain",
    ))?;
    if first.transition.boundary.support.start_ns() != accepted_support.start_ns()
        || phases.last().is_none_or(|phase| {
            phase.transition.boundary.support.end_ns() != accepted_support.end_ns()
        })
        || phases.windows(2).any(|pair| {
            pair[0].transition.boundary.support.end_ns()
                != pair[1].transition.boundary.support.start_ns()
        })
    {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal precipitation carrier-chain support",
        ));
    }
    let lanes = first
        .precipitation_sets
        .keys()
        .copied()
        .collect::<BTreeSet<_>>();
    if lanes.is_empty()
        || phases.iter().any(|phase| {
            phase
                .precipitation_sets
                .keys()
                .copied()
                .collect::<BTreeSet<_>>()
                != lanes
        })
    {
        return Err(DirectV11RealConsumerError::Identity(
            "terminal precipitation carrier-chain topology",
        ));
    }

    let mut composed = BTreeMap::new();
    for lane_id in lanes {
        let first_set = first.precipitation_sets.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("terminal precipitation first lane"),
        )?;
        crate::snow_stage3_v11_attachment::validate_precipitation_phase_parcel_set(first_set).map_err(|_| {
            DirectV11RealConsumerError::Identity("terminal precipitation child seal")
        })?;
        let mut parcels = Vec::new();
        let mut next_ordinal = BTreeMap::<(
            u32,
            Stage3PrecipitationPhaseV1,
            Stage3PrecipitationSourceV1,
        ), u32>::new();
        for phase in phases {
            let set = phase.precipitation_sets.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("terminal precipitation child lane"),
            )?;
            crate::snow_stage3_v11_attachment::validate_precipitation_phase_parcel_set(set).map_err(|_| {
                DirectV11RealConsumerError::Identity("terminal precipitation child seal")
            })?;
            if set.support != phase.transition.boundary.support
                || set.lane_id != lane_id
                || set.ofe_id != first_set.ofe_id
                || set.ofe_ground_basis != first_set.ofe_ground_basis
                || set.topology_identity_sha256 != first_set.topology_identity_sha256
                || set.destinations != first_set.destinations
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "terminal precipitation child topology identity",
                ));
            }
            for parcel in &set.parcels {
                let key = (parcel.destination_topology_index, parcel.phase, parcel.source);
                let ordinal = next_ordinal.entry(key).or_insert(0);
                let mut rebound = parcel.clone();
                rebound.support = accepted_support;
                rebound.semantic_receipt_ordinal = *ordinal;
                *ordinal = ordinal.checked_add(1).ok_or(
                    DirectV11RealConsumerError::Identity(
                        "terminal precipitation semantic ordinal overflow",
                    ),
                )?;
                rebound.receipt_sha256 = Digest32::zero();
                parcels.push(rebound.seal().map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "terminal precipitation composed parcel seal",
                    )
                })?);
            }
        }
        parcels.sort_by(|left, right| {
            (
                left.lane_id,
                left.destination_topology_index,
                left.phase,
                left.source,
                left.semantic_receipt_ordinal,
            )
                .cmp(&(
                    right.lane_id,
                    right.destination_topology_index,
                    right.phase,
                    right.source,
                    right.semantic_receipt_ordinal,
                ))
        });
        let mut set = first_set.clone();
        set.support = accepted_support;
        set.parcels = parcels;
        set.receipt_sha256 = Digest32::zero();
        composed.insert(
            lane_id,
            set.seal().map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "terminal precipitation composed parcel-set seal",
                )
            })?,
        );
    }
    Ok(composed)
}
