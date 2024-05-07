#![allow(missing_docs)]
//! Accounts structs for StableSwap.

use anchor_lang::prelude::*;

/// Accounts for an [crate::initialize] instruction.
#[derive(Accounts, Clone)]
pub struct Initialize<'info> {
    /// The swap.
    /// CHECK:
    #[account(signer)]
    pub swap: AccountInfo<'info>,
    /// The authority of the swap.
    /// CHECK:
    pub swap_authority: AccountInfo<'info>,
    /// The admin of the swap.
    /// CHECK:
    pub admin: AccountInfo<'info>,
    /// The A token of the swap.
    /// CHECK:
    pub token_a: InitToken<'info>,
    /// The B token of the swap.
    /// CHECK:
    pub token_b: InitToken<'info>,
    /// The pool mint of the swap.
    /// CHECK:
    pub pool_mint: AccountInfo<'info>,
    /// The output account for LP tokens.
    /// CHECK:
    pub output_lp: AccountInfo<'info>,
    /// The spl_token program.
    /// CHECK:
    pub token_program: AccountInfo<'info>,
}

/// Accounts for a [crate::deposit] instruction.
#[derive(Accounts, Clone)]
pub struct Deposit<'info> {
    /// The context of the user.
    /// CHECK:
    pub user: SwapUserContext<'info>,
    /// The A token of the swap.
    /// CHECK:
    pub input_a: SwapToken<'info>,
    /// The B token of the swap.
    /// CHECK:
    pub input_b: SwapToken<'info>,
    /// The pool mint of the swap.
    /// CHECK:
    pub pool_mint: AccountInfo<'info>,
    /// The output account for LP tokens.
    /// CHECK:
    pub output_lp: AccountInfo<'info>,
}

/// Accounts for a [crate::swap] instruction.
#[derive(Accounts, Clone)]
pub struct Swap<'info> {
    /// The context of the user.
    /// CHECK:
    pub user: SwapUserContext<'info>,
    /// Accounts for input tokens.
    /// CHECK:
    pub input: SwapToken<'info>,
    /// Accounts for output tokens.
    /// CHECK:
    pub output: SwapOutput<'info>,
}

/// Accounts for a [crate::withdraw_one] instruction.
#[derive(Accounts, Clone)]
pub struct WithdrawOne<'info> {
    /// The context of the user.
    /// CHECK:
    pub user: SwapUserContext<'info>,
    /// The pool mint of the swap.
    /// CHECK:
    pub pool_mint: AccountInfo<'info>,
    /// The input (user)'s LP token account
    /// CHECK:
    pub input_lp: AccountInfo<'info>,
    /// The TokenAccount holding the swap's reserves of quote tokens; i.e., the token not being withdrawn.
    ///
    /// - If withdrawing token A, this is `swap_info.token_b.reserves`.
    /// - If withdrawing token B, this is `swap_info.token_a.reserves`.
    ///
    /// These reserves are needed for the withdraw_one instruction since the
    /// StableSwap `D` invariant requires both the base and quote reserves
    /// to determine how many tokens are paid out to users withdrawing from
    /// the swap.
    ///
    /// *For more info, see [stable_swap_client::state::SwapTokenInfo::reserves].*
    /// CHECK:
    pub quote_reserves: AccountInfo<'info>,
    /// Accounts for output tokens.
    /// CHECK:
    pub output: SwapOutput<'info>,
}

/// Accounts for a [crate::withdraw] instruction.
#[derive(Accounts, Clone)]
pub struct Withdraw<'info> {
    /// The context of the user.
    /// CHECK:
    pub user: SwapUserContext<'info>,
    /// The input account for LP tokens.
    /// CHECK:
    pub input_lp: AccountInfo<'info>,
    /// The pool mint of the swap.
    /// CHECK:
    pub pool_mint: AccountInfo<'info>,
    /// The A token of the swap.
    /// CHECK:
    pub output_a: SwapOutput<'info>,
    /// The B token of the swap.
    /// CHECK:
    pub output_b: SwapOutput<'info>,
}

/// Accounts for a [crate::set_fee_account] instruction.
#[derive(Accounts, Clone)]
pub struct SetFeeAccount<'info> {
    /// The context of the admin user
    /// CHECK:
    pub admin_ctx: AdminUserContext<'info>,
    /// The new token account for fees
    /// CHECK:
    pub fee_account: AccountInfo<'info>,
}

/// Accounts for a [crate::apply_new_admin] instruction.
#[derive(Accounts, Clone)]
pub struct CommitNewAdmin<'info> {
    /// The context of the admin user.
    /// CHECK:
    pub admin_ctx: AdminUserContext<'info>,
    /// The account of the new admin.
    /// CHECK:
    pub new_admin: AccountInfo<'info>,
}

// --------------------------------
// Various accounts
// --------------------------------

/// Token accounts for initializing a [crate::SwapInfo].
#[derive(Accounts, Clone)]
pub struct InitToken<'info> {
    /// The token account for the pool's reserves of this token.
    /// CHECK:
    pub reserve: AccountInfo<'info>,
    /// The token account for the fees associated with the token.
    /// CHECK:
    pub fees: AccountInfo<'info>,
    /// The mint of the token.
    /// CHECK:
    pub mint: AccountInfo<'info>,
}

/// Token accounts for a [crate::swap] instruction.
#[derive(Accounts, Clone)]
pub struct SwapToken<'info> {
    /// The token account associated with the user.
    /// CHECK:
    pub user: AccountInfo<'info>,
    /// The token account for the pool's reserves of this token.
    /// CHECK:
    pub reserve: AccountInfo<'info>,
}

/// Token accounts for the output of a StableSwap instruction.
#[derive(Accounts, Clone)]
pub struct SwapOutput<'info> {
    /// The token accounts of the user and the token.
    /// CHECK:
    pub user_token: SwapToken<'info>,
    /// The token account for the fees associated with the token.
    /// CHECK:
    pub fees: AccountInfo<'info>,
}

/// Accounts for an instruction that interacts with the swap.
#[derive(Accounts, Clone)]
pub struct SwapUserContext<'info> {
    /// The spl_token program.
    /// CHECK:
    pub token_program: AccountInfo<'info>,
    /// The authority of the swap.
    /// CHECK:
    pub swap_authority: AccountInfo<'info>,
    /// The authority of the user.
    /// CHECK:
    #[account(signer)]
    pub user_authority: AccountInfo<'info>,
    /// The swap.
    /// CHECK:
    pub swap: AccountInfo<'info>,
}

/// Accounts for an instruction that requires admin permission.
#[derive(Accounts, Clone)]
pub struct AdminUserContext<'info> {
    /// The public key of the admin account.
    ///
    /// *Note: must be a signer.*
    /// CHECK:
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    /// The swap.
    /// CHECK:
    pub swap: AccountInfo<'info>,
}
