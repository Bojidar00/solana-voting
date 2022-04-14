import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Voting } from "../target/types/voting";
import { PublicKey, SystemProgram } from '@solana/web3.js';

describe("voting", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Voting as Program<Voting>;

  it("Create voting topic!", async () => {
    const vote_account = anchor.web3.Keypair.generate();
    // Add your test here.



    const tx = await program.rpc.create({
      accounts: {
        voteAccount:vote_account,
        user: anchor.getProvider().wallet.publicKey,
        systemProgram: SystemProgram.programId
      }
    });
    console.log("Your transaction signature", tx);
  });
});
