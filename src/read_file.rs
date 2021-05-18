use super::{get_str_from_js, node_error};
use napi::*;
use std::fs::read;

#[js_function(1)]
pub fn read_file_sync(ctx: CallContext) -> Result<JsBuffer> {
    let filepath = get_str_from_js(ctx.get(0)?)?;
    let file = read(filepath).map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("failed to read file, {}", err),
        )
    });
    let bytes = node_error!(file);
    let buffer = ctx.env.create_buffer_with_data(bytes)?.into_raw();
    Ok(buffer)
}

#[js_function(1)]
pub fn read_file(ctx: CallContext) -> Result<JsObject> {
    let input = ctx.get::<JsString>(0)?;
    let reader = FileReader::new(input)?;
    ctx.env
        .spawn(reader)
        .map(|async_task| async_task.promise_object())
}

#[derive(Debug)]
pub struct FileReader {
    filepath: String,
}

impl FileReader {
    fn new(path: JsString) -> Result<Self> {
        let filepath = get_str_from_js(path)?;
        Ok(Self { filepath })
    }
}

impl Task for FileReader {
    type Output = Vec<u8>;
    type JsValue = JsBuffer;

    fn compute(&mut self) -> Result<Self::Output> {
        read(&self.filepath).map_err(|err| {
            Error::new(
                Status::GenericFailure,
                format!("failed to read file, {}", err),
            )
        })
    }

    fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(env.create_buffer_with_data(output)?.into_raw())
    }
}
