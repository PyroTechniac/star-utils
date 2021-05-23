use super::{get_string, make_promise, node_error, ContextCreation};
use napi::*;
use std::fs::read;

macro_rules! create_buffer {
    (ctx, $ctx:expr, $bytes:expr) => {
        create_buffer!($ctx.env, $bytes)
    };
    ($env:expr, $bytes:expr) => {
        Ok($env.create_buffer_with_data($bytes)?.into_raw())
    };
}

#[js_function(1)]
pub fn read_file_sync(ctx: CallContext) -> Result<JsBuffer> {
    let filepath = get_string!(ctx.get::<JsString>(0)?)?;
    let file = read(filepath).map_err(|err| {
        Error::new(
            Status::GenericFailure,
            format!("failed to read file, {}", err),
        )
    });
    let bytes = node_error!(file);
    create_buffer!(ctx, ctx, bytes)
}

#[js_function(1)]
pub fn read_file(ctx: CallContext) -> Result<JsObject> {
    let reader = FileReader::new(&ctx)?;
    make_promise!(ctx, reader)
}

#[derive(Debug)]
pub struct FileReader {
    filepath: String,
}

impl ContextCreation for FileReader {
    fn new(ctx: &CallContext) -> Result<Self> {
        let filepath = get_string!(ctx.get::<JsString>(0)?)?;
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
        create_buffer!(env, output)
    }
}
