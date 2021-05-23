#[macro_export]
macro_rules! node_error {
    ($res:expr) => {
        match $res {
            Ok(res) => res,
            Err(err) => return Err(napi::Error::from_reason(format!("{}", err))),
        }
    };
}

#[macro_export]
macro_rules! get_string {
    ($res:expr) => {
        $res.into_utf8()?.into_owned()
    };
}

#[macro_export]
macro_rules! make_promise {
    ($ctx:expr, $task:expr) => {
        $ctx.env
            .spawn($task)
            .map(|async_task| async_task.promise_object())
    };
}
