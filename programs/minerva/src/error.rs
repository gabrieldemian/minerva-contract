use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid instruction")]
    InvalidInstruction,

    #[msg("The body of your email is too long. The max is 512 chars")]
    InvalidBody,

    #[msg("The subject of your email is too long. The max is 40 chars")]
    InvalidSubject,

    #[msg("The salt should be exactly 16 chars")]
    InvalidSalt,

    #[msg("The IV should be exactly 32 chars")]
    InvalidIv,

    #[msg("The diffie publickey should be exactly 64 chars")]
    InvalidDiffie,
}
