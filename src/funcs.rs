use base64;
use sqlite3_loadable::{
    api::{context_result_text, value_text},
    errors::{Error, Result},
};

use sqlite3ext_sys::{sqlite3_context, sqlite3_value};

pub fn base64_decode(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let contents = value_text(
        values
            .get(0)
            .ok_or(Error::new_message("expected 1st argument as contents"))?
            .to_owned(),
    )?;
    let res = base64::decode(contents).map_err(|e| Error::new_message(format!("error decoding: {}", e.to_string()).as_str()))?;
    let result = std::str::from_utf8(&res)?;
    context_result_text(context, result)?;
    Ok(())
}

pub fn base64_encode(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let contents = value_text(
        values
            .get(0)
            .ok_or(Error::new_message("expected 1st argument as contents"))?
            .to_owned(),
    )?;
    let res = base64::encode(&contents.as_bytes());
    context_result_text(context, &res)?;
    Ok(())
}
