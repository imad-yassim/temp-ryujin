// import * as anchor from "@coral-xyz/anchor";
// import { Idl, Program } from "@coral-xyz/anchor";
// import { RyujinSolana } from "../target/types/ryujin_solana";

// import * as fs from "fs"
// import { ensureEscrowFunded, loadSbProgram, setupQueue } from "../app/src/utils";
// import * as sb from '@switchboard-xyz/on-demand';
// import { Commitment, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
// const PLAYER_STATE_SEED = "playerState";
// const ESCROW_SEED = "stateEscrow";
// const COMMITMENT = "confirmed";


// describe("ryujin-solana", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.RyujinSolana as Program<RyujinSolana>;


//   it("Is initialized!", async () => {
//     // Add your test here.
//     let queue = await setupQueue(program as unknown  as Program<Idl>);

//     console.log({queue})


//     const userSecretKey = JSON.parse(fs.readFileSync("./tests/RYTvFtn7thFaBshE472JD7oETcKTS14RTbXg56qZzzQ.json", 'utf8'));
//     const user = anchor.web3.Keypair.fromSecretKey(new Uint8Array(userSecretKey))

//     const playerStateAccount = await PublicKey.findProgramAddressSync(
//       [Buffer.from(PLAYER_STATE_SEED), user.publicKey.toBuffer()],
//       program.programId
//     );


//     console.log({userSecretKey})

//     const sbProgram = await loadSbProgram(program!.provider);
//     const txOpts = {
//       commitment: "processed" as Commitment,
//       skipPreflight: false,
//       maxRetries: 0,
//     };

//     const rngKp = Keypair.generate();

//     const [randomness, ix] = await sb.Randomness.create(sbProgram, rngKp, queue);

//     console.log("\nCreated randomness account..");
//     console.log("Randomness account", randomness.pubkey.toString());

//     const createRandomnessTx = await sb.asV0Tx({
//       connection: sbProgram.provider.connection,
//       ixs: [ix],
//       payer: user.publicKey,
//       signers: [user, rngKp],
//       computeUnitPrice: 75_000,
//       computeUnitLimitMultiple: 1.3,
//     });

//     // Find the escrow account PDA and initliaze the game
//   const [escrowAccount, escrowBump] = await PublicKey.findProgramAddressSync(
//     [Buffer.from(ESCROW_SEED)],
//     program.programId
//   );

//   // const tx = await program.methods
//   //   .initializeGame()
//   //   .accounts({
//   //     playerState: playerStateAccount[0],
//   //     user: user.publicKey,
//   //     systemProgram: SystemProgram.programId,
//   //   })
//   //   .signers([ user ])
//   //   .rpc();

//   // await program.methods.wheelSpin().accounts({
//   //   playerState:  playerStateAccount,
//   // })

//   await ensureEscrowFunded(
//     sbProgram.provider.connection,
//     escrowAccount,
//     user,
//     sbProgram,
//     txOpts
//   );

//   const balance = await sbProgram.provider.connection.getBalance(escrowAccount)
//   console.log(`Vault ${ensureEscrowFunded} balance : ${balance}`)


//       console.log("Your transaction signature", tx);
//   });
// });
