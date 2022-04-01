use anchor_lang::prelude::*;

#[account]
pub struct Mail {
    pub from: Pubkey,
    pub to: Pubkey,
    pub id: String,
    pub subject: String,
    pub body: String,
    pub authority: Pubkey,
    pub created_at: u32,
    pub iv: String,
    pub salt: String,
}

#[account]
pub struct UserAccount {
    /* his pubkey from the encryption key from diffie helman exchange */
    pub diffie_pubkey: String,
    pub authority: Pubkey,
    pub bump: u8,
}

#[event]
pub struct NewEmailEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub id: String,
}
