/// Check if a round progression is legal.
pub fn is_legal_round_progression(current_round: u64, new_round: u64) -> bool {
    new_round >= current_round
}
