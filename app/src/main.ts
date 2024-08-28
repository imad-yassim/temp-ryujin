import { AnchorProvider, Idl, Program, Wallet } from "@coral-xyz/anchor";
import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  Queue,
} from "@switchboard-xyz/on-demand";
import dotenv from "dotenv";
import * as fs from 'fs';
import { readFileSync } from "fs";
import { createWheelSpinInstruction, ensureEscrowFunded, initializeGame, loadSbProgram, settleFlipInstruction, setupQueue } from "./utils";
import * as sb from '@switchboard-xyz/on-demand';
import { RyujinSolana } from "../../target/types/ryujin_solana";

const COMMITMENT = "confirmed";
const PLAYER_STATE_SEED = "playerState";
const ESCROW_SEED = "stateEscrow";

async function main() {
  dotenv.config();
  console.clear();
  try {

    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
    const userSecretKey = JSON.parse(fs.readFileSync("./wallet.json", 'utf8'));
    const keypair = Keypair.fromSecretKey(new Uint8Array(userSecretKey))
    const wallet = new Wallet(keypair);
    const keypairBalance = await connection.getBalance(keypair.publicKey);

    const provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());

    // const newBalance = await connection.requestAirdrop(
    //   keypair.publicKey,
    //   1 * LAMPORTS_PER_SOL,
    // );

    console.log(`User balance : ${keypairBalance / 1e9}SOL`)
    
    // Shuriken spin program
    console.log("Loading Shuriken spin program...")

    const programId = new PublicKey(process.env.PROGRAM_ID as string);
    if (!programId) throw Error("Program ID not found!")
    const idl: Idl = JSON.parse(readFileSync("./target/idl/ryujin_solana.json", "utf-8"));
    const program = new Program(idl, provider);

    console.log("Shuriken spin program loaded successfully")

    // SB Program
    const sbProgram = await loadSbProgram(program!.provider);
    let queue = await setupQueue(program!);


    const txOpts = {
      commitment: "processed" as Commitment,
      skipPreflight: false,
      maxRetries: 0,
    };

    // create randomness account and initializes it
    const rngKp = Keypair.generate();
    const [randomness, ix] = await sb.Randomness.create(sbProgram, rngKp, queue);
    console.log("\nCreated randomness account..");
    console.log("Randomness account", randomness.pubkey.toString());

    const createRandomnessTx = await sb.asV0Tx({
      connection: sbProgram.provider.connection,
      ixs: [ix],
      payer: keypair.publicKey,
      signers: [keypair, rngKp],
      computeUnitPrice: 75_000,
      computeUnitLimitMultiple: 1.3,
    });

    const sim = await connection.simulateTransaction(createRandomnessTx, txOpts);
    const sig1 = await connection.sendTransaction(createRandomnessTx, txOpts);
    await connection.confirmTransaction(sig1, COMMITMENT);
    console.log(
      "  Transaction Signature for randomness account creation: ",
      sig1
    );

  // initialize example program accounts
  const playerStateAccount = await PublicKey.findProgramAddressSync(
    [Buffer.from(PLAYER_STATE_SEED), keypair.publicKey.toBuffer()],
    sbProgram.programId
  );
  // Find the escrow account PDA and initialize the game
  const [escrowAccount, escrowBump] = await PublicKey.findProgramAddressSync(
    [Buffer.from(ESCROW_SEED)],
    program.programId
  );

  console.log("\nInitialize the game states...");


  await initializeGame(
    program,
    playerStateAccount,
    escrowAccount,
    keypair,
    sbProgram,
    connection
  );
  await ensureEscrowFunded(
    connection,
    escrowAccount,
    keypair,
    sbProgram,
    txOpts
  );

  const commitIx = await randomness.commitIx(queue);

 // Create Wheel spin Ix
  const coinFlipIx = await createWheelSpinInstruction(
    program,
    rngKp.publicKey,
    playerStateAccount,
    keypair,
    escrowAccount
  );

  const commitTx = await sb.asV0Tx({
    connection: sbProgram.provider.connection,
    ixs: [commitIx, coinFlipIx],
    payer: keypair.publicKey,
    signers: [keypair],
    computeUnitPrice: 75_000,
    computeUnitLimitMultiple: 1.3,
  });

  const sim4 = await connection.simulateTransaction(commitTx, txOpts);
  const sig4 = await connection.sendTransaction(commitTx, txOpts);
  await connection.confirmTransaction(sig4, COMMITMENT);
  console.log("  Transaction Signature commitTx", sig4);


   // Reveal the randomness Ix
   console.log("\nReveal the randomness...");
   const revealIx = await randomness.revealIx();
   const settleFlipIx = await settleFlipInstruction(
     program,
     escrowBump,
     playerStateAccount,
     rngKp.publicKey,
     escrowAccount,
     keypair
   );
 
   const revealTx = await sb.asV0Tx({
     connection: sbProgram.provider.connection,
     ixs: [revealIx, settleFlipIx],
     payer: keypair.publicKey,
     signers: [keypair],
     computeUnitPrice: 75_000,
     computeUnitLimitMultiple: 1.3,
   });
 
   const sim5 = await connection.simulateTransaction(revealTx, txOpts);
   const sig5 = await connection.sendTransaction(revealTx, txOpts);
   await connection.confirmTransaction(sig5, COMMITMENT);
   console.log("  Transaction Signature revealTx", sig5);

   const answer = await connection.getParsedTransaction(sig5, {
    maxSupportedTransactionVersion: 0,
  });
  let resultLog = answer?.meta?.logMessages?.filter((line) =>
    line.includes("Random number: ")
  )[0];
  let result = resultLog?.split(": ")[2];
  console.log("\Randomness result : ", result);


  } catch (error) {
    console.error("Failed to load Solana configuration:", error);
  }
}

main()