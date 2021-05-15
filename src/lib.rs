#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::*;
use self::read_file::{read_file_sync, read_file};

mod internal;
mod read_file;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("readFileSync", read_file_sync)?;
    exports.create_named_method("readFile", read_file)?;
    Ok(())
}
