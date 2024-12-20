use anchor_lang::prelude::*;


pub fn initialize(ctx: Context<InitializePDAInstruction>) -> Result<()> {

  msg!("Creating pda with player info...");

  let player_state: &mut Account<'_, PlayerState> = &mut ctx.accounts.player_state;

  
  player_state.allowed_user = ctx.accounts.user.key();
  player_state.bump = ctx.bumps.player_state;
  player_state.allowed_user = ctx.accounts.user.key();
  player_state.obtained_og = 0;
  player_state.obtained_wl =  0;
  player_state.obtained_ryu =  0;
  player_state.obtained_sol =  0;
  player_state.obtained_nft = 0;
  player_state.current_force = [0; 32];

  msg!("Pda with player info Created.");

  Ok(())
}

#[account]
pub struct PlayerState {
  pub allowed_user: Pubkey,
  pub bump: u8,
  pub obtained_og: u8,
  pub obtained_wl: u8,
  pub obtained_ryu: u32,
  pub obtained_sol: u64,
  pub obtained_nft: u8,
  pub current_force: [u8; 32]
}


#[derive(Accounts)]
pub struct InitializePDAInstruction<'info> {
  /// Player will be the `payer` account in the CPI call.
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
    init,
    space = 8 + std::mem::size_of::<PlayerState>(),
    payer = user,
    seeds = [b"playerState", user.key.as_ref()],
    bump
  )]
  pub player_state: Account<'info, PlayerState>,

  pub system_program: Program<'info, System>,
}

