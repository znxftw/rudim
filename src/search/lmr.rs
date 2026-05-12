#[inline(always)]
pub fn needs_reduction(depth: i32, number_of_legal_moves: usize) -> bool {
    if depth < 3 || number_of_legal_moves < 3 {
        return false;
    }

    true
}

// TODO: dynamic reduction
#[inline(always)]
pub fn get_reduction(_depth: i32, _number_of_legal_moves: usize) -> i32 {
    1
}
