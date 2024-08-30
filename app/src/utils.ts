import { Provider, Program, Idl, web3 } from "@coral-xyz/anchor";
import { Commitment, Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import * as sb from '@switchboard-xyz/on-demand';
import { RyujinSolana } from "../../target/types/ryujin_solana";

const COMMITMENT = "confirmed";

export async function loadSbProgram(
  provider: Provider
): Promise<Program> {
  const sbProgramId = await sb.getProgramId(provider.connection);
  const sbIdl = await Program.fetchIdl(sbProgramId, provider);
  const sbProgram = new Program(sbIdl!, provider);
  return sbProgram;
}

export async function setupQueue(program: Program<Idl>): Promise<PublicKey> {
  const queueAccount = await sb.getDefaultQueue(
    program.provider.connection.rpcEndpoint
  );
  console.log("Queue account", queueAccount.pubkey.toString());
  try {
    await queueAccount.loadData();
  } catch (err) {
    console.error("Queue not found, ensure you are using devnet in your env");
    process.exit(1);
  }
  return queueAccount.pubkey;
}

export async function myAnchorProgram(
  provider: Provider,
  keypath: string
): Promise<Program> {
  const myProgramKeypair = await sb.AnchorUtils.initKeypairFromFile(keypath);
  const pid = myProgramKeypair.publicKey;
  const idl = (await Program.fetchIdl(pid, provider))!;
  const program = new Program(idl, provider);
  return program;
}



/**
 * Creates, simulates, sends, and confirms a transaction.
 * @param sbProgram - The Switchboard program.
 * @param connection - The Solana connection object.
 * @param ix - The instruction array for the transaction.
 * @param keypair - The keypair of the payer.
 * @param signers - The array of signers for the transaction.
 * @param txOpts - The transaction options.
 * @returns The transaction signature.
 */
export async function handleTransaction(
  sbProgram: Program,
  connection: Connection,
  ix: web3.TransactionInstruction[],
  keypair: Keypair,
  signers: Keypair[],
  txOpts: any
): Promise<string> {
  const createTx = await sb.asV0Tx({
    connection: sbProgram.provider.connection,
    ixs: ix,
    payer: keypair.publicKey,
    signers: signers,
    computeUnitPrice: 75_000,
    computeUnitLimitMultiple: 1.3,
  });

  const sim = await connection.simulateTransaction(createTx, txOpts);
  const sig = await connection.sendTransaction(createTx, txOpts);
  await connection.confirmTransaction(sig, COMMITMENT);
  console.log("  Transaction Signature", sig);
  return sig;
}


export async function initializeGame(
  myProgram: Program,
  playerStateAccount: [web3.PublicKey, number],
  escrowAccount: PublicKey,
  keypair: Keypair,
  sbProgram: Program,
  connection: Connection
): Promise<web3.TransactionInstruction> {
  const initIx = await myProgram.methods
    .initializeGame()
    .accounts({
      playerState: playerStateAccount,
      escrowAccount: escrowAccount,
      user: keypair.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .instruction();

  const txOpts = {
    commitment: "processed" as Commitment,
    skipPreflight: true,
    maxRetries: 0,
  };

  return initIx;
  // await handleTransaction(
  //   sbProgram,
  //   connection,
  //   [initIx],
  //   keypair,
  //   [keypair],
  //   txOpts
  // );
}


export async function ensureEscrowFunded(
  connection: Connection,
  escrowAccount: PublicKey,
  keypair: Keypair,
  sbProgram: Program,
  txOpts: any
): Promise<w> {
  const accountBalance = await connection.getBalance(escrowAccount);
  const minRentExemption =
    await connection.getMinimumBalanceForRentExemption(0);

  const requiredBalance = minRentExemption;
  const amountToFund = requiredBalance - accountBalance;

  // if (accountBalance < requiredBalance) {
  //   const amountToFund = requiredBalance - accountBalance;
  //   console.log(
  //     `Funding account with ${amountToFund} lamports to meet rent exemption threshold.`
  //   );

    const transferIx = SystemProgram.transfer({
      fromPubkey: keypair.publicKey,
      toPubkey: escrowAccount,
      lamports: amountToFund,
    });

    return transferIx;
    // const transferTx = await sb.asV0Tx({
    //   connection: sbProgram.provider.connection,
    //   ixs: [transferIx],
    //   payer: keypair.publicKey,
    //   signers: [keypair],
    //   computeUnitPrice: 75_000,
    //   computeUnitLimitMultiple: 1.3,
    // });

  //   const sim3 = await connection.simulateTransaction(transferTx, txOpts);
  //   const sig3 = await connection.sendTransaction(transferTx, txOpts);
  //   await connection.confirmTransaction(sig3, COMMITMENT);
  //   console.log("  Transaction Signature ", sig3);
  // } else {
  //   console.log("  Escrow account funded already");
  // }
}

export /**
 * Creates the coin flip instruction for the given program.
 * @param myProgram - The Anchor program.
 * @param rngKpPublicKey - The public key of the randomness keypair.
 * @param userGuess - The user's guess (heads or tails).
 * @param playerStateAccount - The player's state account public key.
 * @param keypair - The keypair of the user.
 * @param escrowAccount - The escrow account public key.
 * @returns The coin flip instruction.
 */
async function createWheelSpinInstruction(
  myProgram: Program<Idl>,
  rngKpPublicKey: PublicKey,
  playerStateAccount: [web3.PublicKey, number],
  keypair: Keypair,
  escrowAccount: PublicKey
): Promise<web3.TransactionInstruction> {
  return await myProgram.methods
    .wheelSpin(rngKpPublicKey)
    .accounts({
      playerState: playerStateAccount,
      user: keypair.publicKey,
      randomnessAccountData: rngKpPublicKey,
      vaultAccount: escrowAccount,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/**
 * Creates the settle flip instruction for the given program.
 * @param myProgram - The Anchor program.
 * @param escrowBump - The bump seed for the escrow account.
 * @param playerStateAccount - The player's state account public key.
 * @param rngKpPublicKey - The public key of the randomness keypair.
 * @param escrowAccount - The escrow account public key.
 * @param keypair - The keypair of the user.
 * @returns The settle flip instruction.
 */
export async function settleFlipInstruction(
  myProgram: Program,
  escrowBump: number,
  playerStateAccount: [web3.PublicKey, number],
  rngKpPublicKey: PublicKey,
  escrowAccount: PublicKey,
  keypair: Keypair
): Promise<web3.TransactionInstruction> {
  return await myProgram.methods
    .settleFlip(escrowBump)
    .accounts({
      playerState: playerStateAccount,
      randomnessAccountData: rngKpPublicKey,
      escrowAccount: escrowAccount,
      user: keypair.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}