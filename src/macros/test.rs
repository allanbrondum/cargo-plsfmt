use std::fmt;

#[macro_export]
macro_rules! assert_eq_text {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(
                        "assertion `left == right` failed\n  left:{}\n right:{}",
                        left_val, right_val
                    );
                }
            }
        }
    };
}
