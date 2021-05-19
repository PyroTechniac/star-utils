use super::{get_string, node_error};
use napi::*;
use std::fs::write;

#[js_function(2)]
pub fn write_file_sync(ctx: CallContext) -> Result<JsUndefined> {
    let filepath = get_string!(ctx.get::<JsString>(0)?)?;
    let buffer = ctx.get::<JsBuffer>(1)?.into_value()?;
    let err = write(filepath, buffer).map_err(|err| err.to_string());
    node_error!(err);
    Ok(ctx.env.get_undefined()?)
}

#[js_function(2)]
pub fn write_file(ctx: CallContext) -> Result<JsObject> {
    let input = ctx.get::<JsString>(0)?;
    let raw = ctx.get::<JsBuffer>(1)?;
    let writer = FileWriter::new(input, raw)?;
    ctx.env
        .spawn(writer)
        .map(|async_task| async_task.promise_object())
}

#[derive(Debug)]
pub struct FileWriter {
    filepath: String,
    data: Vec<u8>,
}

impl FileWriter {
    fn new(path: JsString, raw: JsBuffer) -> Result<Self> {
        let filepath = get_string!(path)?;
        let data = raw.into_value()?.to_vec();
        Ok(Self { filepath, data })
    }
}

impl Task for FileWriter {
    type Output = ();
    type JsValue = JsUndefined;

    fn compute(&mut self) -> Result<Self::Output> {
        write(&self.filepath, &self.data).map_err(|err| {
            Error::new(
                Status::GenericFailure,
                format!("failed to write file, {}", err),
            )
        })
    }

    fn resolve(self, env: Env, _: Self::Output) -> Result<Self::JsValue> {
        Ok(env.get_undefined()?)
    }
}
