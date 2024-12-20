use anchor_lang::prelude::*;
use orao_solana_vrf::{program::OraoVrf, state::NetworkState, CONFIG_ACCOUNT_SEED, RANDOMNESS_ACCOUNT_SEED};


use super::initialize_pda::PlayerState;



pub fn request_randomness(ctx: Context<StartSpinning>, force: [u8; 32]) -> Result<()> {
  let player_state: &mut Account<'_, PlayerState> = &mut ctx.accounts.player_state;


  msg!("Requesting orao randomness...");

  let cpi_program = ctx.accounts.vrf.to_account_info();
  let cpi_accounts = orao_solana_vrf::cpi::accounts::Request {
    payer: ctx.accounts.user.to_account_info(),
    network_state: ctx.accounts.config.to_account_info(),
    treasury: ctx.accounts.treasury.to_account_info(),
    request: ctx.accounts.randomness_account_data.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
  };
  let cpi_ctx = anchor_lang::context::CpiContext::new(cpi_program, cpi_accounts);
  orao_solana_vrf::cpi::request(cpi_ctx, force)?;

  msg!("Orao randomness requested successfully.");
  player_state.current_force = force;

  Ok(())
}


#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct StartSpinning<'info> {
  #[account(mut, seeds = [b"playerState", user.key.as_ref()], bump)]
  pub player_state: Account<'info, PlayerState>,

  /// Player will be the `payer` account in the CPI call.
  #[account(mut)]
  pub user: Signer<'info>,
  /// CHECK: Treasury
  #[account(mut)]
  pub treasury: AccountInfo<'info>,
  /// This account is the current VRF request account, it'll be the `request` account in the CPI call.
  /// CHECK: The account's data is validated manually within the handler.
  #[account(
      mut,
      seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), &force],
      bump,
      seeds::program = orao_solana_vrf::ID
  )]
  pub randomness_account_data: AccountInfo<'info>,
  /// CHECK: The account's data is validated manually within the handler.
  #[account(mut, seeds = [b"vaultAccount".as_ref()], bump )]
  pub vault_account: AccountInfo<'info>,
  #[account(
    mut,
    seeds = [CONFIG_ACCOUNT_SEED.as_ref()],
    bump,
    seeds::program = orao_solana_vrf::ID
  )]
  pub config: Account<'info, NetworkState>,
  pub vrf: Program<'info, OraoVrf>,
  pub system_program: Program<'info, System>,
}

