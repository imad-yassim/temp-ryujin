import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RyujinSolana } from "../target/types/ryujin_solana";

import * as fs from "fs"

describe("ryujin-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RyujinSolana as Program<RyujinSolana>;

  it("Is initialized!", async () => {
    // Add your test here.

    const userSecretKey = JSON.parse(fs.readFileSync("./tests/RYTvFtn7thFaBshE472JD7oETcKTS14RTbXg56qZzzQ.json", 'utf8'));

    console.log({userSecretKey})
    const user = anchor.web3.Keypair.fromSecretKey(new Uint8Array(userSecretKey))

    const tx = await program.methods
      .startGameInstruction("jjkf")
      .accounts({ player: user.publicKey })
      .signers([ user ])
      .rpc();

      console.log("Your transaction signature", tx);
  });
});
