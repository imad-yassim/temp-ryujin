import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RyujinSolana } from "../target/types/ryujin_solana";

import { networkStateAccountAddress } from "@orao-network/solana-vrf";
import { Commitment, Keypair, PublicKey } from "@solana/web3.js";
import * as fs from "fs";
import { ensureEscrowFunded, loadSbProgram } from "../app/src/utils";
const PLAYER_STATE_SEED = "playerState";
const ESCROW_SEED = "stateEscrow";
const COMMITMENT = "confirmed";


describe("ryujin-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RyujinSolana as Program<RyujinSolana>;


  it("Is initialized!", async () => {
    // Add your test here.



    const userSecretKey = JSON.parse(fs.readFileSync("./wallet.json", 'utf8'));
    const user = anchor.web3.Keypair.fromSecretKey(new Uint8Array(userSecretKey))

    const playerStateAccount = await PublicKey.findProgramAddressSync(
      [Buffer.from(PLAYER_STATE_SEED), user.publicKey.toBuffer()],
      program.programId
    );

    console.log({userSecretKey})

    // Find the escrow account PDA and initliaze the game
  const [escrowAccount, escrowBump] = await PublicKey.findProgramAddressSync(
    [Buffer.from(ESCROW_SEED)],
    program.programId
  );

  const force = Keypair.generate().publicKey;

  const tx = await program.methods
  .startGame([ ...force.toBuffer() ])
  .accounts({
      user: user.publicKey,
      config: networkStateAccountAddress()
    })
    .signers([ user ])
    .rpc();

    const transaction = await program.provider.connection.getParsedTransaction(tx, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0, // For backward compatibility with older transaction formats
  });

  // Check if the transaction was found
  if (!transaction) {
      console.log('Transaction not found.');
      return;
  }

  // Access and display the logs
  const logs = transaction.meta?.logMessages;
  if (logs) {
      console.log('Transaction Logs:');
      logs.forEach((log, index) => {
          console.log(`${index + 1}: ${log}`);
      });
  } else {
      console.log('No logs found in this transaction.');
  }

  // await program.methods.wheelSpin().accounts({
  //   playerState:  playerStateAccount,
  // })

  // await ensureEscrowFunded(
  //   sbProgram.provider.connection,
  //   escrowAccount,
  //   user,
  //   sbProgram,
  //   txOpts
  // );

  // const balance = await sbProgram.provider.connection.getBalance(escrowAccount)
  // console.log(`Vault ${ensureEscrowFunded} balance : ${balance}`)


      console.log("Your transaction signature", tx);
  });
});
