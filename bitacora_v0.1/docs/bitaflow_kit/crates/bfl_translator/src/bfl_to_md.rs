use crate::parser::parse_bfl;

pub fn bt_to_md(bt: &str) -> Result<String, String> {
    let doc = parse_bfl(bt).map_err(|e| e.to_string())?;
    Ok(doc.body)
}
