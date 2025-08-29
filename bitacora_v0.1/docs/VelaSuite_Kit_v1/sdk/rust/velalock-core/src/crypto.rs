pub struct Dek(pub Vec<u8>);
pub struct Kek(pub Vec<u8>);

pub fn derive_kek_from_opaque(_opaque_export: &[u8]) -> Kek {
    // TODO: integrate opaque-ke; here we return placeholder
    Kek(vec![0u8;32])
}