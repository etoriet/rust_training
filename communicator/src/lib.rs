pub mod client;

pub mod network;


mod outermost {
    pub fn middle_function() {}
    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {}
        pub fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}
use a::series::of;

fn try_series() {
    of::nested_modules();
}

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        client::connect();
    }
}
