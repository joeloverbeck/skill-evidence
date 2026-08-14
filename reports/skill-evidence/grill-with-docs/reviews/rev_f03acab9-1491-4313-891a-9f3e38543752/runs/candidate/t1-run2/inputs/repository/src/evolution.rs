pub fn validate_claim(target_hash: &str, operating_skill_hash: &str) -> Result<(), Refusal> {
    if target_hash == operating_skill_hash {
        return Err(Refusal::SelfTarget);
    }
    Ok(())
}

pub fn validate_partition(coverage: &[EventId], concluded: &[EventId], limited: &[EventId]) -> Result<(), Refusal> {
    require_exact_disjoint_partition(coverage, concluded, limited)
}
