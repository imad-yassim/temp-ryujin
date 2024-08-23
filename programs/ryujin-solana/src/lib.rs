pub mod instructions;
pub mod states;
pub mod events;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use states::vault_states::*;
use instructions::vault_managements::*;

declare_id!("RYUqKUNAX3nFc2kPPBXLHzm326n5qsUveCJ3Z6Jj1fX");


#[program]
pub mod ryujin_solana {

  // let GAME_PRICE: u64 = (1 / 10) * LAMPORTS_PER_SOL;

use events::game_events::{OGWinnerEvent, WLWinnerEvent};
use instructions::games_instructions::start_spinning;

use super::*;

  // pub fn add_to_vault(ctx: Context<FillVaultInstruction>, amount: u64) -> Result<()> {
  //   fill_vault(&ctx, amount * LAMPORTS_PER_SOL)
  // }


  pub fn start_game_instruction(ctx: Context<FillVaultInstruction>, discord_user_id: String) -> Result<()> {
    // RETRIEVE PAYMENT
    transfer_lamports(
      ctx.accounts.system_program.to_account_info().clone(),
      ctx.accounts.player.to_account_info().clone(),
      ctx.accounts.vault_account.to_account_info().clone(),
      (1 / 10) * LAMPORTS_PER_SOL
    )?;

    // START GAME
    // GENERATE RANDOM NUMBER
    let result: u16 = start_spinning();
    // let random_str = format!("{}", result);

    // msg!("Result is : {}", result);

    // USER WIN SOLS
    if result < 10 {
      transfer_lamports(
        ctx.accounts.system_program.to_account_info().clone(),
        ctx.accounts.vault_account.to_account_info().clone(),
        ctx.accounts.player.to_account_info().clone(),
        (1 / 100) * LAMPORTS_PER_SOL,
      )?;
      msg!("User win 100SOL");
    }


    // USER WIN WL
    else if result < 300 {
      emit!(WLWinnerEvent {
        winner_pubkey: *ctx.accounts.player.key,
        discord_user_id: discord_user_id,
    });
    }
    else if result < 900 {
      emit!(OGWinnerEvent {
        winner_pubkey: *ctx.accounts.player.key,
        discord_user_id: discord_user_id,
    });
    }

    Ok(())
  }
  
}






#[derive(Accounts)]
pub struct StartGameInstruction<'info> {
  #[account(mut)]
  pub player: Signer<'info>,

  #[account(
    mut,
    seeds = [b"vault_account"],
    bump,
  )]
  pub vault_account: Account<'info, VaultAccount>,
  pub system_program: Program<'info, System>,

 }


