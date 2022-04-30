pub mod client;
pub mod network;

pub mod ex {
    fn inner() {
    }
    pub fn pub_inner() {
        modules::pub_inner();
    }
    
    mod modules {
        fn inner() {
            super::inner();
        }
        pub fn pub_inner() {}
    }
    pub mod pub_modules {
        fn inner() {}
        pub fn pub_inner() {}
    }
}

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
