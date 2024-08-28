pub mod errors;
pub mod instructions;
pub mod states;
pub mod events;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use switchboard_on_demand::accounts::RandomnessAccountData;

use instructions::vault_managements::*;
use errors::ErrorCode;

declare_id!("RYUqKUNAX3nFc2kPPBXLHzm326n5qsUveCJ3Z6Jj1fX");


const GAME_PRICE: u64 = (1 / 10) * LAMPORTS_PER_SOL;


#[program]
pub mod ryujin_solana {
  use super::*;

  // pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
  //   let player_state = &mut ctx.accounts.player_state;
  //   player_state.randomness_account = Pubkey::default(); // Placeholder, will be set in coin_flip
  //   player_state.bump = ctx.bumps.player_state;
  //   player_state.allowed_user = ctx.accounts.user.key();

  //   Ok(())
  // }

  pub fn wheel_spin(ctx: Context<WheelSpin>, randomness_account: Pubkey) -> Result<()> {
    let clock = Clock::get()?;
    // let player_state: &mut Account<PlayerState> = &mut ctx.accounts.player_state;
    // Record the user's guess
    let randomness_data = RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();

    if randomness_data.seed_slot != clock.slot - 1 {
        msg!("seed_slot: {}", randomness_data.seed_slot);
        msg!("slot: {}", clock.slot);
        return Err(ErrorCode::RandomnessAlreadyRevealed.into());
    }

    // Track the player's committed values so you know they don't request randomness
    // multiple times.
    // player_state.commit_slot = randomness_data.seed_slot;

    // ***
    // IMPORTANT: Remember, in Switchboard Randomness, it's the responsibility of the caller to reveal the randomness.
    // Therefore, the game collateral MUST be taken upon randomness request, not on reveal.
    // ***
    transfer(
      ctx.accounts.system_program.to_account_info(),
      ctx.accounts.user.to_account_info(),  // Include the user_account
      ctx.accounts.vault_account.to_account_info(),
      GAME_PRICE,
      None,
    )?;

    // Store flip commit
    // player_state.randomness_account = randomness_account;

    // Log the result
    msg!("Wheel spinning initiated, randomness requested.");
    Ok(())
  }


   // Settle the flip after randomness is revealed
   pub fn settle_flip(ctx: Context<SettleWheel>, escrow_bump: u8) -> Result<()> {

    let clock: Clock = Clock::get()?;
    let player_state = &mut ctx.accounts.player_state;

    // call the switchboard on-demand parse function to get the randomness data
    let randomness_data = RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    if randomness_data.seed_slot != player_state.commit_slot {
        return Err(ErrorCode::RandomnessExpired.into());
    }

    // call the switchboard on-demand get_value function to get the revealed random value
    let revealed_random_value = randomness_data.get_value(&clock)
        .map_err(|_| ErrorCode::RandomnessNotResolved)?;

    // Use the revealed random value to determine number between 0 and 359
    let mut rng_num: u32 = 0;
    for i in 0..4 {
        rng_num = rng_num << 8 | revealed_random_value[i] as u32;
    }

    let randomness_result: u16 = (rng_num % 360) as u16;

    // Update and log the result
    player_state.latest_flip_result = randomness_result;

    let seed_prefix = b"stateEscrow".as_ref();
    let escrow_seed = &[&seed_prefix[..], &[escrow_bump]];
    let seeds_slice: &[&[u8]] = escrow_seed;
    let binding = [seeds_slice];
    let seeds: Option<&[&[&[u8]]]> = Some(&binding);


    msg!("Random number:  {}", randomness_result);

    if randomness_result < 10 {
      msg!("You win!");
      let sol_win_amount = 1 / 10;

      let rent = Rent::get()?;
      let needed_lamports = sol_win_amount + rent.minimum_balance(ctx.accounts.vault_account.data_len());
      if needed_lamports > ctx.accounts.vault_account.lamports() {
          msg!("Not enough funds in treasury to pay out the user. Please try again later");
      } else {
        transfer(
          ctx.accounts.system_program.to_account_info(),
          ctx.accounts.vault_account.to_account_info(), // Transfer from the vault
          ctx.accounts.user.to_account_info(), // Payout to the user's wallet
          (1 / 100) * LAMPORTS_PER_SOL,
          seeds // Include seeds
      )?;
    }
    } else if randomness_result < 30 {

    } else if randomness_result < 200 {

    } else {
      // On lose, we keep the user's initial colletaral and they are
      // allowed to play again.
      msg!("You lose!");
  }

    Ok(())
  }

}


// === Instructions ===
#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(init,
        payer = user,
        seeds = [b"playerState".as_ref(), user.key().as_ref()],
        space = 8 + 100,
        bump)]
    pub player_state: Account<'info, PlayerState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WheelSpin<'info> {
    #[account(
      mut,
      seeds = [b"playerState".as_ref(), user.key().as_ref()],
      bump = player_state.bump
    )]
    pub player_state: Account<'info, PlayerState>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
    /// CHECK: This is a simple Solana account holding SOL.
    #[account(mut, seeds = [b"stateEscrow".as_ref()], bump)]
    pub vault_account: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettleWheel<'info> {
    #[account(mut,
        seeds = [b"playerState".as_ref(), user.key().as_ref()],
        bump = player_state.bump)]
    pub player_state: Account<'info, PlayerState>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,
     /// CHECK: This is a simple Solana account holding SOL.
    #[account(mut, seeds = [b"stateEscrow".as_ref()], bump )]
    pub vault_account: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// === Accounts ===
#[account]
pub struct PlayerState {
    allowed_user: Pubkey,
    latest_flip_result: u16, // Stores the result of the latest flip
    randomness_account: Pubkey, // Reference to the Switchboard randomness account
    bump: u8,
    commit_slot: u64, // The slot at which the randomness was committed
}
