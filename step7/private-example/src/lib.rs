pub mod outermost {
    pub fn middle_function() {
    }

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
        }

        fn secret_function() {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
