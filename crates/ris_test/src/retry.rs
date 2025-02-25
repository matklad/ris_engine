pub fn test_retry(count: usize, test: fn() -> ()) {
    for _ in 0..count - 1 {
        let result = std::panic::catch_unwind(test);

        if result.is_ok() {
            return;
        }
    }

    test();
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut RETRY_SHOULD_SUCCEED_COUNT: i32 = 0;
    #[test]
    fn should_succeed() {
        unsafe {
            RETRY_SHOULD_SUCCEED_COUNT = 0;

            test_retry(10, || {
                RETRY_SHOULD_SUCCEED_COUNT += 1;
                if RETRY_SHOULD_SUCCEED_COUNT < 5 {
                    panic!();
                }
            });

            assert_eq!(RETRY_SHOULD_SUCCEED_COUNT, 5);
        }
    }

    static mut RETRY_SHOULD_FAIL_COUNT: i32 = 0;
    #[test]
    fn should_fail() {
        unsafe {
            RETRY_SHOULD_FAIL_COUNT = 0;

            let result = std::panic::catch_unwind(|| {
                test_retry(10, || {
                    RETRY_SHOULD_FAIL_COUNT += 1;
                    panic!();
                })
            });

            assert_eq!(RETRY_SHOULD_FAIL_COUNT, 10);
            assert!(result.is_err());
        }
    }
}
