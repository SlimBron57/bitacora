use velalock_core::{enroll::enroll_device, vault::Vault, crypto::derive_kek_from_opaque};

fn main() {
    let pass = b"1234";
    let ctx = enroll_device(pass).expect("enroll");
    let kek = derive_kek_from_opaque(pass);
    let _dek = Vault::open(&ctx.wrap_dek_b64, &kek).expect("vault");
    println!("Enrolled DID: {}", ctx.did);
}