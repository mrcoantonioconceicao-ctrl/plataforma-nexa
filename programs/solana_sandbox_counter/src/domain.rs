//! Domain-Driven Design (DDD) - Bounded Context: Solana UserCounter
//! Modulo: programs/solana_sandbox_counter/src/domain.rs
//! Autoria: Marco Antonio Conceicao
//!
//! Modulo complementar e incremental (Regra C44 - Nao-Destrutiva).
//! Define a raiz de agregacao (Aggregate Root), especificacoes de invariantes
//! e regras de negocio de dominio para o smart contract UserCounter.

use anchor_lang::prelude::*;

/// Raiz de Agregacao (Aggregate Root) no contexto DDD
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct UserCounterAggregate {
    pub authority: Pubkey,
    pub count: u64,
    pub bump: u8,
}

/// Especificacao formal de invariantes do agregado UserCounter
pub struct UserCounterDomainSpec;

impl UserCounterDomainSpec {
    /// Invariante DDD INV-RENT-EXEMPT: 49 bytes exatos
    pub const EXACT_ACCOUNT_SPACE: usize = 8 + 32 + 8 + 1;

    /// Invariante DDD INV-DETERMINISTIC-PDA
    pub const SEED_PREFIX: &'static [u8] = b"counter";

    /// Valida se uma transicao de incremento preserva o invariante de nao-estouro
    pub fn validate_increment_invariant(current: u64, amount: u64) -> Result<u64> {
        current.checked_add(amount).ok_or_else(|| {
            error!(crate::SecurityErrorCode::NumericalOverflow)
        })
    }

    /// Valida se uma transicao de decremento preserva o invariante de nao-negatividade
    pub fn validate_decrement_invariant(current: u64, amount: u64) -> Result<u64> {
        current.checked_sub(amount).ok_or_else(|| {
            error!(crate::SecurityErrorCode::NumericalUnderflow)
        })
    }

    /// Valida se a autoridade informada corresponde estritamente ao proprietario do agregado
    pub fn validate_authority_invariant(registered: &Pubkey, signer: &Pubkey) -> bool {
        registered == signer
    }
}
