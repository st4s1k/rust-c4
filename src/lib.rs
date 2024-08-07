#[macro_export]
macro_rules! c4 {
    (for (;;$($iter:stmt),*) $code_block:block) => {
        c4! { for (();true;$($iter),*) $code_block }
    };
    (for ($($init:stmt),+;;$($iter:stmt),*) $code_block:block) => {
        c4! { for ($($init),+;true;$($iter),*) $code_block }
    };
    (for (;$loop_condition:expr;$($iter:stmt),*) $code_block:block) => {
        c4! { for (();$loop_condition;$($iter),*) $code_block }
    };
    (
        for (
            $($init:stmt),+;
            $loop_condition:expr;
            $($iter:stmt),*
        ) $code_block:block
    ) => {
        {
            let mut first_cycle = true;
            $($init)+
            loop {
                if !first_cycle {
                    $($iter)*
                    if !$loop_condition {
                        break;
                    }
                } else {
                    first_cycle = false;
                }
                $code_block
            };
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_endless() {
        let mut result = String::new();
        let mut i = 1;

        c4! {
            for (;;) {
                if i > 10 {
                    break;
                }
                result.push_str(&format!("9 * {:<2} = {}{}", i, i - 1, 10 - i));
                i += 1;
            }
        }

        assert_eq!(
            result,
            concat![
            "9 * 1  = 09",
            "9 * 2  = 18",
            "9 * 3  = 27",
            "9 * 4  = 36",
            "9 * 5  = 45",
            "9 * 6  = 54",
            "9 * 7  = 63",
            "9 * 8  = 72",
            "9 * 9  = 81",
            "9 * 10 = 90"
            ]
        );
    }

    #[test]
    fn test_iter() {
        let mut result = String::new();
        let mut i = 1;

        c4! {
            for (;;i += 1) {
                if i > 10 {
                    break;
                }
                result.push_str(&format!("9 * {:<2} = {}{}", i, i - 1, 10 - i));
            }
        }

        assert_eq!(
            result,
            concat![
            "9 * 1  = 09",
            "9 * 2  = 18",
            "9 * 3  = 27",
            "9 * 4  = 36",
            "9 * 5  = 45",
            "9 * 6  = 54",
            "9 * 7  = 63",
            "9 * 8  = 72",
            "9 * 9  = 81",
            "9 * 10 = 90"
            ]
        );
    }

    #[test]
    fn test_loop_condition() {
        let mut result = String::new();
        let mut i = 1;

        c4! {
            for (;i <= 10;) {
                result.push_str(&format!("9 * {:<2} = {}{}", i, i - 1, 10 - i));
                i += 1;
            }
        }

        assert_eq!(
            result,
            concat![
            "9 * 1  = 09",
            "9 * 2  = 18",
            "9 * 3  = 27",
            "9 * 4  = 36",
            "9 * 5  = 45",
            "9 * 6  = 54",
            "9 * 7  = 63",
            "9 * 8  = 72",
            "9 * 9  = 81",
            "9 * 10 = 90"
            ]
        );
    }

    #[test]
    fn test_loop_condition_iter() {
        let mut result = String::new();
        let mut i = 1;

        c4! {
            for (;i <= 10; i += 1) {
                result.push_str(&format!("9 * {:<2} = {}{}", i, i - 1, 10 - i));
            }
        }

        assert_eq!(
            result,
            concat![
            "9 * 1  = 09",
            "9 * 2  = 18",
            "9 * 3  = 27",
            "9 * 4  = 36",
            "9 * 5  = 45",
            "9 * 6  = 54",
            "9 * 7  = 63",
            "9 * 8  = 72",
            "9 * 9  = 81",
            "9 * 10 = 90"
            ]
        );
    }

    #[test]
    fn test_init() {
        let mut result = String::new();

        c4! {
            for (let mut i = 1;;) {
                if i > 10 {
                    break;
                }
                result.push_str(&format!("9 * {:<2} = {}{}", i, i - 1, 10 - i));
                i += 1;
            }
        }

        assert_eq!(
            result,
            concat![
            "9 * 1  = 09",
            "9 * 2  = 18",
            "9 * 3  = 27",
            "9 * 4  = 36",
            "9 * 5  = 45",
            "9 * 6  = 54",
            "9 * 7  = 63",
            "9 * 8  = 72",
            "9 * 9  = 81",
            "9 * 10 = 90"
            ]
        );
    }

    #[test]
    fn test_init_iter() {
        let mut result = String::new();

        c4! {
            for (let mut i = 1;; i += 1) {
                if i > 10 {
                    break;
                }
                result.push_str(&format!("9 * {:<2} = {}{}", i, i - 1, 10 - i));
            }
        }

        assert_eq!(
            result,
            concat![
            "9 * 1  = 09",
            "9 * 2  = 18",
            "9 * 3  = 27",
            "9 * 4  = 36",
            "9 * 5  = 45",
            "9 * 6  = 54",
            "9 * 7  = 63",
            "9 * 8  = 72",
            "9 * 9  = 81",
            "9 * 10 = 90"
            ]
        );
    }

    #[test]
    fn test_init_loop_condition() {
        let mut result = String::new();

        c4! {
            for (let mut i = 1; i <= 10;) {
                result.push_str(&format!("9 * {:<2} = {}{}", i, i - 1, 10 - i));
                i += 1;
            }
        }

        assert_eq!(
            result,
            concat![
            "9 * 1  = 09",
            "9 * 2  = 18",
            "9 * 3  = 27",
            "9 * 4  = 36",
            "9 * 5  = 45",
            "9 * 6  = 54",
            "9 * 7  = 63",
            "9 * 8  = 72",
            "9 * 9  = 81",
            "9 * 10 = 90"
            ]
        );
    }

    #[test]
    fn test_init_loop_condition_iter() {
        let mut result = String::new();

        c4! {
            for (let mut i = 1; i <= 10; i += 1) {
                if i == 5 {
                    continue;
                }
                result.push_str(&format!("9 * {:<2} = {}{}", i, i - 1, 10 - i));
            }
        }

        assert_eq!(
            result,
            concat![
            "9 * 1  = 09",
            "9 * 2  = 18",
            "9 * 3  = 27",
            "9 * 4  = 36",
            // "9 * 5  = 45",
            "9 * 6  = 54",
            "9 * 7  = 63",
            "9 * 8  = 72",
            "9 * 9  = 81",
            "9 * 10 = 90"
            ]
        );
    }

    #[test]
    fn test_multiple_statements() {
        let mut result = String::new();

        c4! {
            for (
                let mut i = 0,
                let mut j = 0,
                let mut s = "some dummy word".to_string();
                i * j <= s.len();
                i += 1,
                s = format!("{} {} {}", &s, i, j)
            ) {
                j += 1;
                result.push_str(&format!("i: {}; j: {}; s: {}", i, j, s));
            }
        }

        assert_eq!(
            result,
            concat![
            "i: 0; j: 1; s: some dummy word",
            "i: 1; j: 2; s: some dummy word 1 1",
            "i: 2; j: 3; s: some dummy word 1 1 2 2",
            "i: 3; j: 4; s: some dummy word 1 1 2 2 3 3",
            "i: 4; j: 5; s: some dummy word 1 1 2 2 3 3 4 4",
            "i: 5; j: 6; s: some dummy word 1 1 2 2 3 3 4 4 5 5",
            "i: 6; j: 7; s: some dummy word 1 1 2 2 3 3 4 4 5 5 6 6"
            ]
        );
    }
}