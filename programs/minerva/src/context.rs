/* deserialized instruction data. these are the ctx structs of the processor functions */

use crate::state::{Mail, UserAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SendMail<'info> {
    #[account(
        init,
        payer = authority,
        space =
            8 +       // discriminator
            32 +      // from
            32 +      // to
            34 +      // id
            40 +      // subject
            512 +     // body
            32 +      // authority
            4 +       // created_at
            20 +      // salt
            36        // iv
    )]
    pub mail: Account<'info, Mail>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space =
            8  +          // discriminator
            4  + 64 +     // public key
            32 +          // authority
            1,            // bump
       seeds = [b"user-account", authority.key().as_ref()],
       bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccount<'info> {
    pub authority: Signer<'info>,
    #[account(
       mut,
       seeds = [b"user-account", authority.key().as_ref()],
       bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
}

/* helper table for calculating accounts spaces */
/*
    bool	        1 byte	    1 bit rounded up to 1 byte.
    u8 or i8	    1 byte
    u16 or i16	    2 bytes
    u32 or i32	    4 bytes
    u64 or i64	    8 bytes
    u128 or i128	16 bytes
    [u16; 32]	    64 bytes	32 items x 2 bytes. [itemSize; arrayLength]
    PubKey	        32 bytes	Same as [u8; 32]
    vec<u16>	    Any multiple of 2 bytes + 4 bytes for the prefix	Need to allocate the maximum amount of item that could be required.
    String	        Any multiple of 1 byte + 4 bytes for the prefix	Same as vec<u8>
*/
