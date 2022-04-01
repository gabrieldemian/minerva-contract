use {crate::error::ErrorCode, anchor_lang::prelude::*, context::*, utils::*};
pub mod context;
pub mod error;
pub mod state;
pub mod utils;

declare_id!("9KVS65SWuX5jnmJkzpyXMCdeKpad9G5sSoKopUUgDiA");

#[program]
pub mod minerva {
    use super::*;
    use anchor_lang::Key;

    pub fn send_email(
        ctx: Context<SendMail>,
        subject: String,
        body: String,
        from: Pubkey,
        to: Pubkey,
        salt: String,
        iv: String
    ) -> Result<()> {
        require!(subject.chars().count() < 50, ErrorCode::InvalidSubject);
        require!(body.chars().count() < 280, ErrorCode::InvalidBody);
        require!(salt.chars().count() == 16, ErrorCode::InvalidSalt);
        require!(iv.chars().count() == 32, ErrorCode::InvalidIv);

        let now = Clock::get().unwrap().unix_timestamp as u32;
        let mail = &mut ctx.accounts.mail;
        let id = get_uuid(&now, &body, &mail.key());

        mail.from = from;
        mail.to = to;
        mail.id = id.clone();
        mail.subject = subject;
        mail.body = body; // encrypted body, a ciphertext
        mail.created_at = now;
        mail.salt = salt;
        mail.iv = iv;
        mail.authority = *ctx.accounts.authority.key;

        emit!(state::NewEmailEvent {
            from,
            to,
            id
        });

        Ok(())
    }

    pub fn register(ctx: Context<Register>, diffie_pubkey: String) -> Result<()> {
        require!(diffie_pubkey.chars().count() == 64, ErrorCode::InvalidDiffie);

        let user_account = &mut ctx.accounts.user_account;

        user_account.diffie_pubkey = diffie_pubkey;
        user_account.authority = *ctx.accounts.authority.key;
        user_account.bump = *ctx.bumps.get("user_account").unwrap();

        Ok(())
    }

    pub fn update_account(ctx: Context<UpdateAccount>, diffie_pubkey: String) -> Result<()> {
        require!(diffie_pubkey.chars().count() == 64, ErrorCode::InvalidDiffie);

        ctx.accounts.user_account.diffie_pubkey = diffie_pubkey;

        Ok(())
    }
}
