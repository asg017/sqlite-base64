pub mod funcs;
pub mod meta;

use sqlite3_loadable::{
    errors::Result, scalar::define_scalar_function, sqlite3_entrypoint, sqlite3_imports,
};
use sqlite3ext_sys::sqlite3;

use crate::{
    funcs::{base64_decode, base64_encode},
    meta::{base64_debug, base64_version},
};

sqlite3_imports!();

#[sqlite3_entrypoint]
fn sqlite3_basesixfour_init(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(db, "base64_version", 0, base64_version)?;
    define_scalar_function(db, "base64_debug", 0, base64_debug)?;

    define_scalar_function(db, "base64_decode", 1, base64_decode)?;
    define_scalar_function(db, "base64_encode", 1, base64_encode)?;
    Ok(())
}
#[sqlite3_entrypoint]
fn sqlite3_base_decodeonly_init(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(db, "base64_decode", 1, base64_decode)?;
    Ok(())
}
