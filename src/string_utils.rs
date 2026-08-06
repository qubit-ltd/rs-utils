/// Reports whether a string contains only Unicode whitespace characters.
///
/// Returns `true` for an empty string and for strings whose characters all
/// satisfy [`char::is_whitespace`]. Returns `false` when any non-whitespace
/// character is present.
pub fn is_blank(value: &str) -> bool {
    value.chars().all(char::is_whitespace)
}
