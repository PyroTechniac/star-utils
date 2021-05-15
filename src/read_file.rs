use super::node_error;
use napi::*;
use std::fs::read;

#[js_function(1)]
pub fn read_file_sync(ctx: CallContext) -> Result<JsBuffer> {
    let filepath = ctx.get::<JsString>(0)?.into_utf8()?;
    let file = read(filepath.as_str()?).map_err(|err| err.to_string());
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
        let utf8_string = path.into_utf8()?;
        let utf8_owned = utf8_string.into_owned()?;
        Ok(Self {
            filepath: utf8_owned,
        })
    }
}

impl Task for FileReader {
    type Output = Vec<u8>;
    type JsValue = JsBuffer;

    fn compute(&mut self) -> Result<Self::Output> {
        read(&self.filepath).map_err(|err| Error::from_reason(err.to_string()))
    }

    fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(env.create_buffer_with_data(output)?.into_raw())
    }
}
