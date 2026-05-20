use amun_chain_position::ChainPosition;

/// Check if a child position is a direct descendant of a parent position.
pub fn is_descendant_of(child: ChainPosition, parent: ChainPosition) -> bool {
    if child.epoch < parent.epoch {
        return false;
    }
    if child.epoch == parent.epoch {
        return child.sequence == parent.sequence + 1;
    }
    // New epoch: sequence resets to 0, so child must be epoch+1, seq=0
    child.epoch == parent.epoch + 1 && child.sequence == 0
}
