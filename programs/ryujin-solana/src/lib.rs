
pub mod errors;
pub mod instructions;
pub mod states;
pub mod events;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use instructions::vault_managements::*;
use instructions::start_game::*;
use instructions::reveal_result::*;

use anchor_lang::{
  solana_program::account_info::AccountInfo,
  AccountDeserialize,
};

declare_id!("RYUqKUNAX3nFc2kPPBXLHzm326n5qsUveCJ3Z6Jj1fX");

const GAME_PRICE: u64 = (1 / 10) * LAMPORTS_PER_SOL;

#[program]
pub mod ryujin_solana {

  use super::*;

  // pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
  //   let player_state = &mut ctx.accounts.player_state;
  //   player_state.latest_flip_result = 366;
  //   player_state.randomness_account = Pubkey::default(); // Placeholder, will be set in coin_flip
  //   player_state.bump = ctx.bumps.player_state;
  //   player_state.allowed_user = ctx.accounts.user.key();

  //   Ok(())
  // }

    pub fn start_game(ctx: Context<StartSpinning>, force: [u8; 32]) -> Result<()> {
    // ***
    //  Taking the game collateral before requesting randomness.
    // ***
    msg!("Taking the game collateral before requesting randomness...");
    transfer(
      ctx.accounts.system_program.to_account_info(),
      ctx.accounts.user.to_account_info(),  // Include the user_account
      ctx.accounts.vault_account.to_account_info(),
      GAME_PRICE,
      None,
    )?;
    msg!("Payment retrieved successfully.");

    // ***
    //  Requesting orao randomness.
    // ***
    request_randomness(ctx, force)
  }

  pub fn stop_spinning(ctx: Context<StopSpinningInstruction>, _force: [u8; 32]) -> Result<()> {
    reveal_randomness(ctx)
  }


   // Settle the flip after randomness is revealed
  //  pub fn settle_flip(ctx: Context<SettleWheel>, escrow_bump: u8) -> Result<()> {
  //   let player_state: &mut Account<'_, PlayerState> = &mut ctx.accounts.player_state;
  //   let rand_acc = get_account_data(&ctx.accounts.randomness_account_data)?;

  //   let randomness = current_state(&rand_acc);
  //   if randomness == 0 {
  //       return err!(StillProcessing::StillProcessing)
  //   }
  //   let randomness_result: u16 = (randomness % 360) as u16;

  //   msg!("VRF result is: {}", randomness);


  //   // Update and log the result
  //   player_state.latest_flip_result = randomness_result;

  //   let seed_prefix = b"stateEscrow".as_ref();
  //   let escrow_seed = &[&seed_prefix[..], &[escrow_bump]];
  //   let seeds_slice: &[&[u8]] = escrow_seed;
  //   let binding = [seeds_slice];
  //   let seeds: Option<&[&[&[u8]]]> = Some(&binding);


  //   msg!("Random number:  {}", randomness_result);

  //   if randomness_result < 10 {
  //     msg!("You win!");
  //     let sol_win_amount: u64 = 1 / 10;

  //     let rent = Rent::get()?;
  //     let needed_lamports = sol_win_amount + rent.minimum_balance(ctx.accounts.vault_account.data_len());
  //     if needed_lamports > ctx.accounts.vault_account.lamports() {
  //         msg!("Not enough funds in treasury to pay out the user. Please try again later");
  //     } else {
  //       transfer(
  //         ctx.accounts.system_program.to_account_info(),
  //         ctx.accounts.vault_account.to_account_info(), // Transfer from the vault
  //         ctx.accounts.user.to_account_info(), // Payout to the user's wallet
  //         (1 / 100) * LAMPORTS_PER_SOL,
  //         seeds // Include seeds
  //       )?;
  //     }
  //   } else if randomness_result < 30 {

  //   } else if randomness_result < 200 {

  //   } else {
  //     // On lose, we keep the user's initial colletaral and they are
  //     // allowed to play again.
  //     msg!("You lose!");
  // }

  //   Ok(())
  // }

}



// === Accounts ===
#[account]
pub struct PlayerState {
    allowed_user: Pubkey,
    latest_flip_result: u16, // Stores the result of the latest flip
    randomness_account: Pubkey, // Reference to the Switchboard randomness account
    bump: u8,
    commit_slot: u64, // The slot at which the randomness was committed
    force: [u8; 32]
  }

#[error_code]
pub enum StillProcessing {
    #[msg("Randomness is still being fulfilled")]
    StillProcessing
}

#[error_code]
pub enum InvalidAmount {
    #[msg("Amount must be greater than 0.05 SOL")]
    InvalidAmount
}