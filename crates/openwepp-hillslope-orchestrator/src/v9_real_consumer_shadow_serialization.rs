fn digest_serialized<T: Serialize>(value: &T) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))?;
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes))).map_err(Into::into)
}
