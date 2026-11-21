use anchor_lang::prelude::*;

decl_id!("11111111111111111111111111111111");

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // [Issue #19] Validacao Ed25519 garantida
}
