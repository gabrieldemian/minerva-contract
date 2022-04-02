use anchor_lang::prelude::Pubkey;
use uuid::Uuid;

/* creates a unique ID for a mail using now, body, and sender as arguments */
pub fn get_uuid(now: &u32, body: &String, sender: &Pubkey) -> String {
    const V5NAMESPACE: &Uuid = &Uuid::from_bytes([
        16, 92, 30, 120, 224, 152, 10, 207, 140, 56, 246, 228, 206, 99, 196, 138,
    ]);

    let now = now.to_be_bytes();
    let body = body.as_bytes();
    let sender = sender.to_bytes();

    let mut vec = vec![];

    vec.extend_from_slice(&now);
    vec.extend_from_slice(&body);
    vec.extend_from_slice(&sender);

    Uuid::new_v5(V5NAMESPACE, &vec).to_string()
}
