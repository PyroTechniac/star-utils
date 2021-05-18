#[macro_export]
macro_rules! node_error {
    ($res:expr) => {
        match $res {
            Ok(res) => res,
            Err(err) => return Err(napi::Error::from_reason(format!("{}", err))),
        }
    };
}

#[inline]
pub(crate) fn get_str_from_js(value: napi::JsString) -> napi::Result<String> {
    value.into_utf8()?.into_owned()
}