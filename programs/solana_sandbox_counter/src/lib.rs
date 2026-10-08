use anchor_lang::prelude::*;

declare_id!("CntSandbox111111111111111111111111111111111");

#[program]
pub mod solana_sandbox_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_count: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.authority.key();
        counter.count = initial_count;
        counter.bump = ctx.bumps.counter;
        msg!("UserCounter initialized with bump {}", counter.bump);
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateCounter>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_add(amount).ok_or(SecurityErrorCode::NumericalOverflow)?;
        Ok(())
    }

    pub fn decrement(ctx: Context<UpdateCounter>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_sub(amount).ok_or(SecurityErrorCode::NumericalUnderflow)?;
        Ok(())
    }

    pub fn reset(ctx: Context<UpdateCounter>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        Ok(())
    }

    pub fn close(ctx: Context<CloseCounter>) -> Result<()> {
        msg!("Account closed successfully");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = UserCounter::ACCOUNT_SPACE,
        seeds = [b"counter", authority.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, UserCounter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    #[account(
        mut,
        seeds = [b"counter", authority.key().as_ref()],
        bump = counter.bump,
        has_one = authority @ SecurityErrorCode::UnauthorizedAuthority
    )]
    pub counter: Account<'info, UserCounter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseCounter<'info> {
    #[account(
        mut,
        seeds = [b"counter", authority.key().as_ref()],
        bump = counter.bump,
        has_one = authority @ SecurityErrorCode::UnauthorizedAuthority,
        close = authority
    )]
    pub counter: Account<'info, UserCounter>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct UserCounter {
    pub authority: Pubkey,
    pub count: u64,
    pub bump: u8,
}

impl UserCounter {
    pub const ACCOUNT_SPACE: usize = 8 + 32 + 8 + 1; // 49 bytes
}

#[error_code]
pub enum SecurityErrorCode {
    #[msg("Arithmetic overflow occurred during counter operation")]
    NumericalOverflow,
    #[msg("Arithmetic underflow occurred during counter operation")]
    NumericalUnderflow,
    #[msg("Unauthorized: Signer does not match authority")]
    UnauthorizedAuthority,
}
