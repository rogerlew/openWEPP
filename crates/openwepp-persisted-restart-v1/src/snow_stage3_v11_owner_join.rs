fn nested(error: impl std::fmt::Display) -> SnowStage3V11RestartError {
    SnowStage3V11RestartError::Nested(error.to_string())
}

fn nested_phase(phase: &'static str, error: impl std::fmt::Display) -> SnowStage3V11RestartError {
    SnowStage3V11RestartError::NestedPhase {
        phase,
        detail: error.to_string(),
    }
}

fn owner_transaction_id(bytes: &[u8]) -> Option<u128> {
    let value: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let transaction = value.get("last_transaction_id")?;
    transaction
        .as_u64()
        .map(u128::from)
        .or_else(|| transaction.as_str()?.parse().ok())
}

fn exact_checkpoint_owner_join(
    checkpoint: &V11ParentTransactionCheckpoint,
    clock: &openwepp_coupled_time::CoupledClockStateV1,
) -> Result<(), SnowStage3V11RestartError> {
    for owner_id in checkpoint
        .staged_complete_owners
        .keys()
        .map(String::as_str)
        .chain(clock.owners().iter().map(|owner| owner.owner_id()))
    {
        let expected = checkpoint
            .staged_complete_owners
            .get(owner_id)
            .map(|owner| owner.state_sha256);
        let actual_owner = clock
            .owners()
            .iter()
            .find(|owner| owner.owner_id() == owner_id);
        let actual = actual_owner.map(|owner| owner.state_digest());
        if expected != actual
            || checkpoint
                .staged_complete_owners
                .get(owner_id)
                .is_some_and(|expected| {
                    actual_owner.is_none_or(|actual| {
                        expected.state_bytes.as_slice() != actual.state_bytes()
                    })
                })
        {
            return Err(SnowStage3V11RestartError::V11ClockOwnerJoin {
                owner: owner_id.to_owned(),
                expected,
                actual,
                expected_transaction: checkpoint
                    .staged_complete_owners
                    .get(owner_id)
                    .and_then(|owner| owner_transaction_id(&owner.state_bytes)),
                actual_transaction: actual_owner
                    .and_then(|owner| owner_transaction_id(owner.state_bytes())),
            });
        }
    }
    Ok(())
}

fn reconstruct_completed_parent_candidate(
    checkpoint: &V11ParentTransactionCheckpoint,
    mut candidate: openwepp_vegetation::V11ParentCandidate,
    clock: &openwepp_coupled_time::CoupledClockStateV1,
    consumer: &DirectV10RealConsumerShadow,
) -> Result<openwepp_vegetation::V11ParentCandidate, SnowStage3V11RestartError> {
    let predecessor_owners = checkpoint
        .staged_complete_owners
        .values()
        .map(openwepp_vegetation::V11OwnerEnvelope::to_owner_state)
        .collect::<Result<Vec<_>, _>>()
        .map_err(nested)?;
    let predecessor_digest = complete_owner_set_digest(&predecessor_owners).map_err(nested)?;
    let ending_digest = complete_owner_set_digest(clock.owners()).map_err(nested)?;
    let finalization_event =
        clock
            .accepted_event_receipts()
            .last()
            .ok_or(SnowStage3V11RestartError::Identity(
                "completed parent finalization event omission",
            ))?;
    if finalization_event.parent_transaction_id() != clock.parent_transaction_id()
        || finalization_event.tick() != clock.accepted_until()
        || finalization_event.beginning_owner_set_digest() != predecessor_digest
        || finalization_event.ending_owner_set_digest() != ending_digest
    {
        return Err(SnowStage3V11RestartError::Identity(
            "completed parent finalization event join",
        ));
    }

    let candidate_by_owner = candidate
        .ending_complete_owners
        .iter()
        .map(|owner| (owner.owner_id(), owner))
        .collect::<std::collections::BTreeMap<_, _>>();
    let clock_by_owner = clock
        .owners()
        .iter()
        .map(|owner| (owner.owner_id(), owner))
        .collect::<std::collections::BTreeMap<_, _>>();
    if candidate_by_owner.len() != clock_by_owner.len() {
        return Err(SnowStage3V11RestartError::Identity(
            "completed parent owner cardinality",
        ));
    }
    let mut predecessor_bgc = consumer.restart_authority_biogeochemistry().clone();
    predecessor_bgc.last_transaction_id =
        predecessor_bgc.last_transaction_id.checked_sub(1).ok_or(
            SnowStage3V11RestartError::Identity("completed parent BGC successor"),
        )?;
    let predecessor_bgc_bytes = serde_json::to_vec(&predecessor_bgc).map_err(nested)?;
    let ending_bgc_bytes =
        serde_json::to_vec(consumer.restart_authority_biogeochemistry()).map_err(nested)?;
    for (owner_id, clock_owner) in &clock_by_owner {
        let candidate_owner =
            candidate_by_owner
                .get(owner_id)
                .ok_or(SnowStage3V11RestartError::Identity(
                    "completed parent owner manifest",
                ))?;
        if *owner_id == "bgc" {
            if candidate_owner.state_bytes() != predecessor_bgc_bytes
                || clock_owner.state_bytes() != ending_bgc_bytes
            {
                return Err(SnowStage3V11RestartError::Identity(
                    "completed parent BGC exact successor",
                ));
            }
        } else if *candidate_owner != *clock_owner {
            return Err(SnowStage3V11RestartError::Identity(
                "completed parent finalization mutation set",
            ));
        }
    }
    candidate.ending_complete_owners = candidate
        .ending_complete_owners
        .iter()
        .map(|owner| {
            clock_by_owner
                .get(owner.owner_id())
                .map(|ending| (*ending).clone())
                .ok_or(SnowStage3V11RestartError::Identity(
                    "completed parent ordered owner manifest",
                ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(candidate)
}
