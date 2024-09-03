use anchor_lang::prelude::*;
use orao_solana_vrf::{program::OraoVrf, state::NetworkState, CONFIG_ACCOUNT_SEED, RANDOMNESS_ACCOUNT_SEED};



pub fn request_randomness(ctx: Context<StartSpinning>, force: [u8; 32]) -> Result<()> {

  msg!("Requesting orao randomness...");

  let cpi_program = ctx.accounts.vrf.to_account_info();
  let cpi_accounts = orao_solana_vrf::cpi::accounts::Request {
    payer: ctx.accounts.user.to_account_info(),
    network_state: ctx.accounts.config.to_account_info(),
    treasury: ctx.accounts.vault_account.to_account_info(),
    request: ctx.accounts.randomness_account_data.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
  };

  let cpi_ctx: CpiContext<'_, '_, '_, '_, orao_solana_vrf::cpi::accounts::Request<'_>> = anchor_lang::context::CpiContext::new(cpi_program, cpi_accounts);
  orao_solana_vrf::cpi::request(cpi_ctx, force)?;

  msg!("Orao randomness requested successfully...");

  Ok(())
}


#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct StartSpinning<'info> {
    /// Player will be the `payer` account in the CPI call.
    #[account(mut)]
    pub user: Signer<'info>,

    /// This account is the current VRF request account, it'll be the `request` account in the CPI call.
    /// CHECK: The account's data is validated manually within the handler.
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub randomness_account_data: AccountInfo<'info>,

    /// CHECK: The account's data is validated manually within the handler.
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,
    
    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    /// VRF on-chain state account, it'll be the `network_state` account in the CPI call.
    pub config: Account<'info, NetworkState>,

    /// VRF program address to invoke CPI
    pub vrf: Program<'info, OraoVrf>,

    /// System program address to create player_state and to be used in CPI call.
    pub system_program: Program<'info, System>,
}

