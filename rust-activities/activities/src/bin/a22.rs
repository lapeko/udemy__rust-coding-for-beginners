// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_lower() {
        assert_eq!(
            clamp(0, 10, 20),
            10,
            "Должно возвращать lower, если n меньше нижней границы"
        );
    }

    #[test]
    fn clamp_middle() {
        assert_eq!(
            clamp(15, 10, 20),
            15,
            "Должно возвращать n, если оно внутри диапазона"
        );
    }

    #[test]
    fn clamp_upper() {
        assert_eq!(
            clamp(30, 10, 20),
            20,
            "Должно возвращать upper, если n больше верхней границы"
        );
    }

    #[test]
    fn div_normal() {
        assert_eq!(
            div(6, 3),
            Some(2),
            "Обычное деление должно возвращать Some(результат)"
        );
    }

    #[test]
    fn div_by_zero() {
        assert_eq!(
            div(6, 0),
            None,
            "Деление на ноль должно возвращать None, а не паниковать"
        );
    }

    #[test]
    fn concat_standard() {
        assert_eq!(
            concat("foo", "bar"),
            "foobar",
            "Строки должны склеиваться без пробелов"
        );
    }

    #[test]
    fn concat_with_empty() {
        assert_eq!(
            concat("", "rust"),
            "rust",
            "Склеивание с пустой строкой не должно менять результат"
        );
    }
}
