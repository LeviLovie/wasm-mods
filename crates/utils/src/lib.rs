pub mod logging {
    use anyhow::Error;
    use tracing::error;

    pub trait LogError<T, E> {
        fn log_msg(self, msg: &str) -> Result<T, E>;
        fn log(self) -> Result<T, E>;
    }

    impl<T, E: std::fmt::Debug> LogError<T, E> for Result<T, E> {
        fn log_msg(self, msg: &str) -> Result<T, E> {
            if let Err(e) = &self {
                error!("{}: {:?}", msg, e);
            }
            self
        }

        fn log(self) -> Result<T, E> {
            if let Err(e) = &self {
                error!("{:?}", e);
            }
            self
        }
    }

    pub trait CheckOption<T> {
        fn check(self, msg: &str) -> Result<T, Error>;
    }

    impl<T> CheckOption<T> for Option<T> {
        fn check(self, msg: &str) -> Result<T, Error> {
            self.ok_or_else(|| Error::msg(msg.to_string()))
        }
    }

    pub trait CheckLog<T> {
        fn check_log(self, msg: &str) -> Result<T, Error>;
    }

    impl<T> CheckLog<T> for Option<T> {
        fn check_log(self, msg: &str) -> Result<T, Error> {
            if self.is_none() {
                error!(message = msg);
                return Err(Error::msg(msg.to_string()));
            }
            Ok(self.unwrap())
        }
    }
}
