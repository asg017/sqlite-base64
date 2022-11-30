use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, define_scalar_function, Error, Result};

/// base64_version()
pub fn base64_version(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, &format!("v{}", env!("CARGO_PKG_VERSION")))?;
    Ok(())
}

/// base64_debug()
pub fn base64_debug(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(
        context,
        &format!(
            "Version: v{}
Source: {}
",
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH")
        ),
    )?;
    Ok(())
}

/// base64_decode()
pub fn base64_decode(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let contents = api::value_text(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("expected 1st argument as contents"))?
            ,
    )?;
    let res = base64::decode(contents)
        .map_err(|e| Error::new_message(format!("error decoding: {}", e).as_str()))?;
    let result = std::str::from_utf8(&res)?;
    api::result_text(context, result)?;
    Ok(())
}
/// base64_encode(data)
pub fn base64_encode(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let contents = api::value_text(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("expected 1st argument as contents"))?
            ,
    )?;
    let res = base64::encode(contents.as_bytes());
    api::result_text(context, &res)?;
    Ok(())
}

#[sqlite_entrypoint]
fn sqlite3_basesixfour_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;
    define_scalar_function(db, "base64_version", 0, base64_version, flags)?;
    define_scalar_function(db, "base64_debug", 0, base64_debug, flags)?;

    define_scalar_function(db, "base64_decode", 1, base64_decode, flags)?;
    define_scalar_function(db, "base64_encode", 1, base64_encode, flags)?;
    Ok(())
}
