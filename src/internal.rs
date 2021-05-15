#[macro_export]
macro_rules! node_error {
    ($res:expr) => {
        match $res {
            Ok(res) => res,
            Err(err) => return Err(napi::Error::from_reason(format!("{}", err))),
        }
    };
}
