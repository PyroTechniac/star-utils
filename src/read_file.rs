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

#[derive(Debug)]
pub struct FileReader {
    filepath: String
}
