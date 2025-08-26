use crate::parser::FrontMatter;

pub fn serialize_front_matter(fm: &FrontMatter) -> String {
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str(&format!("alias: {}\n", fm.alias));
    if let Some(name) = &fm.name { out.push_str(&format!("name: {}\n", name)); }
    if let Some(slug) = &fm.slug { out.push_str(&format!("slug: {}\n", slug)); }
    if let Some(kind) = &fm.kind { out.push_str(&format!("kind: {}\n", kind)); }
    if let Some(version) = &fm.version { out.push_str(&format!("version: {}\n", version)); }
    if let Some(locale) = &fm.locale { out.push_str(&format!("locale: {}\n", locale)); }
    if let Some(req) = &fm.requires {
        out.push_str("requires:\n");
        for r in req { out.push_str(&format!("- {}\n", r)); }
    }
    out.push_str("---\n\n");
    out
}
