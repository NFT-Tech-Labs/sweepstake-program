import * as anchor from "@project-serum/anchor";
import { LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { DagoatsSweepstake } from "../target/types/dagoats_sweepstake";
import { before } from "mocha";
import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";

use(chaiAsPromised);

describe("DaGOATs Sweepstake", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace
    .DagoatsSweepstake as anchor.Program<DagoatsSweepstake>;

  const userState = anchor.web3.Keypair.generate();
  const player_a = anchor.web3.Keypair.generate();
  const player_b = anchor.web3.Keypair.generate();
  const player_a_sweepstake = anchor.web3.Keypair.generate();

  const airdrop = async (
    user: anchor.web3.Keypair,
    amount = LAMPORTS_PER_SOL
  ) => {
    const signature = await provider.connection.requestAirdrop(
      user.publicKey,
      amount
    );
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      ...latestBlockHash,
      signature,
    });
  };

  const getAccountInfo = async (account: anchor.web3.Keypair) => {
    const { owner } = await provider.connection.getAccountInfo(
      account.publicKey
    );
    return {
      owner: owner.toBase58(),
    };
  };

  before(() => Promise.all([airdrop(player_a), airdrop(player_b)]));

  describe("Initialization", () => {
    it("Should initialize user state", async () => {
      await program.methods
        .createUser(new anchor.BN(1))
        .accounts({
          userState: userState.publicKey,
          authority: player_a.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([userState, player_a])
        .rpc();
      const { owner } = await getAccountInfo(userState);
      expect(owner).eq(program.programId.toBase58());
    });

    it("Should not initialize user state twice", () =>
      expect(
        program.methods
          .createUser(new anchor.BN(1))
          .accounts({
            userState: userState.publicKey,
            authority: player_a.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([userState, player_a])
          .rpc()
      ).to.be.rejectedWith(/This transaction has already been processed/));
  });

  describe("Sweepstake", () => {
    const input = {
      id: new anchor.BN(1),
      worldChampion: "NL",
      finalGame: "NL-PL=1:0",
      thirdPlaceGame: "ES-CZ=0:0",
      semifinals: "NL-PL=1:0;NL-PL=1:0",
      quarterFinals: "NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0",
      roundOf16:
        "NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0",
      groupStage3:
        "NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0",
      groupStage2:
        "NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0",
      groupStage1:
        "NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0;NL-PL=1:0",
    };
    it("Should not create sweepstake with invalid input data", () =>
      expect(
        program.methods
          .createSweepstake({
            id: new anchor.BN(1),
            finalGame: "",
            groupStage1: "",
            groupStage2: "",
            groupStage3: "",
            quarterFinals: "",
            roundOf16: "",
            semifinals: "",
            thirdPlaceGame: "",
            worldChampion: "",
          })
          .accounts({
            userState: userState.publicKey,
            authority: player_a.publicKey,
            systemProgram: SystemProgram.programId,
            sweepstakeState: player_a_sweepstake.publicKey,
          })
          .signers([player_a, player_a_sweepstake])
          .rpc()
      ).to.be.rejectedWith(/Sweepstake data has invalid length/));

    it("User should not create sweepstake for a different user", () =>
      expect(
        program.methods
          .createSweepstake(input)
          .accounts({
            userState: userState.publicKey,
            authority: player_b.publicKey,
            systemProgram: SystemProgram.programId,
            sweepstakeState: player_a_sweepstake.publicKey,
          })
          .signers([player_b, player_a_sweepstake])
          .rpc()
      ).to.be.rejected);

    it("Should create sweepstake", async () => {
      const tx = await program.methods
        .createSweepstake(input)
        .accounts({
          userState: userState.publicKey,
          authority: player_a.publicKey,
          systemProgram: SystemProgram.programId,
          sweepstakeState: player_a_sweepstake.publicKey,
        })
        .signers([player_a, player_a_sweepstake])
        .rpc();
      expect(tx).not.to.be.empty;
    });

    it("Should not create sweepstake to exceed limit per wallet", () => {
      const sweepstake = anchor.web3.Keypair.generate();
      return expect(
        program.methods
          .createSweepstake(input)
          .accounts({
            userState: userState.publicKey,
            authority: player_a.publicKey,
            systemProgram: SystemProgram.programId,
            sweepstakeState: sweepstake.publicKey,
          })
          .signers([player_a, sweepstake])
          .rpc()
      ).to.be.rejectedWith(/Exceeded number of sweepstakes per wallet/);
    });
  });
});
