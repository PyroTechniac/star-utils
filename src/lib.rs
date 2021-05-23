#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use self::{
    read_file::{read_file, read_file_sync},
    write_file::{write_file, write_file_sync},
};
use napi::*;

mod internal;
mod read_file;
mod write_file;

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
    exports.create_named_method("readFileSync", read_file_sync)?;
    exports.create_named_method("readFile", read_file)?;
    exports.create_named_method("writeFileSync", write_file_sync)?;
    exports.create_named_method("writeFile", write_file)?;
    exports.set_named_property("napiVersion", env.create_uint32(env.get_napi_version()?)?)?;
    Ok(())
}
