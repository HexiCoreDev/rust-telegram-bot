/// Returns `true` when every row is a non-string sequence — always valid at the Rust type level.
pub fn check_keyboard_type<T>(_keyboard: &[Vec<T>]) -> bool {
    true
}
