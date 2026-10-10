use anchor_lang::prelude::*;

pub mod domain;

// Standard Sandbox Program ID for the Anchor Counter DevSecOps environment
declare_id!("CntSandbox111111111111111111111111111111111");

#[program]
pub mod solana_sandbox_counter {
    use super::*;

    /// Initializes a deterministically derived PDA counter account for the signing authority.
    /// Allocates exact 49 bytes for optimal rent-exempt calculation.
    pub fn initialize(ctx: Context<Initialize>, initial_count: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.authority.key();
        counter.count = initial_count;
        counter.bump = ctx.bumps.counter;

        msg!("Initialized UserCounter PDA for authority: {}", counter.authority);
        msg!("Initial count: {}, Canonical bump: {}", counter.count, counter.bump);
        Ok(())
    }

    /// Increments the counter with native checked arithmetic protection and authority verification.
    pub fn increment(ctx: Context<UpdateCounter>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        // Native arithmetic overflow protection via checked_add
        counter.count = counter
            .count
            .checked_add(amount)
            .ok_or(SecurityErrorCode::NumericalOverflow)?;

        msg!("Counter incremented by {}. New count: {}", amount, counter.count);
        Ok(())
    }

    /// Decrements the counter with native checked arithmetic protection to prevent underflow.
    pub fn decrement(ctx: Context<UpdateCounter>, amount: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        // Native arithmetic underflow protection via checked_sub
        counter.count = counter
            .count
            .checked_sub(amount)
            .ok_or(SecurityErrorCode::NumericalUnderflow)?;

        msg!("Counter decremented by {}. New count: {}", amount, counter.count);
        Ok(())
    }

    /// Resets the counter value to zero, strictly authorized by the counter owner.
    pub fn reset(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;

        msg!("Counter reset to zero by authority: {}", ctx.accounts.authority.key());
        Ok(())
    }

    /// Safely closes the PDA account and reclaims rent-exempt lamports back to the authority.
    pub fn close(ctx: Context<CloseCounter>) -> Result<()> {
        msg!("UserCounter PDA closed. Rent reclaimed by: {}", ctx.accounts.authority.key());
        Ok(())
    }
}

// ==========================================
// DECLARATIVE ANCHOR INSTRUCTION CONTEXTS
// ==========================================

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

// ==========================================
// STATE DATA STRUCTURE & MEMORY ALLOCATION
// ==========================================

#[account]
pub struct UserCounter {
    pub authority: Pubkey, // 32 bytes
    pub count: u64,        // 8 bytes
    pub bump: u8,          // 1 byte
}

impl UserCounter {
    // Explicit rent-exempt memory allocation layout
    pub const DISCRIMINATOR_SPACE: usize = 8;
    pub const AUTHORITY_SPACE: usize = 32;
    pub const COUNT_SPACE: usize = 8;
    pub const BUMP_SPACE: usize = 1;

    /// Total space: 8 + 32 + 8 + 1 = 49 bytes
    pub const ACCOUNT_SPACE: usize = Self::DISCRIMINATOR_SPACE
        + Self::AUTHORITY_SPACE
        + Self::COUNT_SPACE
        + Self::BUMP_SPACE;
}

// ==========================================
// SECURITY ERROR CODES
// ==========================================

#[error_code]
pub enum SecurityErrorCode {
    #[msg("Arithmetic overflow occurred during counter operation")]
    NumericalOverflow,

    #[msg("Arithmetic underflow occurred: counter cannot be negative")]
    NumericalUnderflow,

    #[msg("Access Denied: Signer does not match the registered counter authority")]
    UnauthorizedAuthority,

    #[msg("Invalid canonical bump seed provided")]
    InvalidBumpSeed,
}
