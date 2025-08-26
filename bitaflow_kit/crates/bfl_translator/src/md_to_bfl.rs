use crate::parser::FrontMatter;
use crate::serializer::serialize_front_matter;

pub fn md_to_bt(md: &str, target_alias: &str) -> Result<String, String> {
    let fm = FrontMatter {
        alias: target_alias.to_string(),
        name: None, slug: None, kind: Some("TPL".into()),
        version: Some(1), locale: None, requires: None
    };
    let mut out = serialize_front_matter(&fm);
    out.push_str(md.trim());
    Ok(out)
}
