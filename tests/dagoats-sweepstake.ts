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

  const solPerSweepstake = LAMPORTS_PER_SOL;
  const dagoatsSolWalletAddress = new anchor.web3.PublicKey(
    "328vR2LdB9PaLq9eR7ad3Hj8shX9SzSZJhFpeeSkKXa4"
  );
  const fakeDagoatsSolWalletAddress = new anchor.web3.PublicKey(
    "53Xa3PVBki4ZT2qJoJPfiGiA42SyuvQ6WXj5ysw8TRv1"
  );
  const userState = anchor.web3.Keypair.generate();
  const player_a = anchor.web3.Keypair.generate();
  const player_b = anchor.web3.Keypair.generate();
  const player_a_sweepstake = anchor.web3.Keypair.generate();

  const airdrop = async (
    user: anchor.web3.Keypair,
    amount = 3 * LAMPORTS_PER_SOL
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

  const getBalance = (pubKey: anchor.web3.PublicKey) =>
    provider.connection.getBalance(pubKey);

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
      paymentAmount: new anchor.BN(solPerSweepstake),
    };
    it("Should not create sweepstake with invalid input data", () =>
      expect(
        program.methods
          .createSweepstakeSol({
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
            paymentAmount: new anchor.BN(solPerSweepstake),
          })
          .accounts({
            userState: userState.publicKey,
            authority: player_a.publicKey,
            dagoatsWallet: dagoatsSolWalletAddress,
            systemProgram: SystemProgram.programId,
            sweepstakeState: player_a_sweepstake.publicKey,
          })
          .signers([player_a, player_a_sweepstake])
          .rpc()
      ).to.be.rejectedWith(/Sweepstake data has invalid length/));

    it("User should not create sweepstake for a different user", () =>
      expect(
        program.methods
          .createSweepstakeSol(input)
          .accounts({
            userState: userState.publicKey,
            authority: player_b.publicKey,
            dagoatsWallet: dagoatsSolWalletAddress,
            systemProgram: SystemProgram.programId,
            sweepstakeState: player_a_sweepstake.publicKey,
          })
          .signers([player_b, player_a_sweepstake])
          .rpc()
      ).to.be.rejected);

    it("User should not create sweepstake with a bad dagoats wallet", () =>
      expect(
        program.methods
          .createSweepstakeSol(input)
          .accounts({
            userState: userState.publicKey,
            authority: player_a.publicKey,
            dagoatsWallet: fakeDagoatsSolWalletAddress,
            systemProgram: SystemProgram.programId,
            sweepstakeState: player_a_sweepstake.publicKey,
          })
          .signers([player_a, player_a_sweepstake])
          .rpc()
      ).to.be.rejected);

    it("Should create sweepstake", async () => {
      const preBalance = await getBalance(dagoatsSolWalletAddress);
      const tx = await program.methods
        .createSweepstakeSol(input)
        .accounts({
          userState: userState.publicKey,
          authority: player_a.publicKey,
          dagoatsWallet: dagoatsSolWalletAddress,
          systemProgram: SystemProgram.programId,
          sweepstakeState: player_a_sweepstake.publicKey,
        })
        .signers([player_a, player_a_sweepstake])
        .rpc();
      const sweepstake = await program.account.sweepstake.fetch(
        player_a_sweepstake.publicKey
      );
      const user = await program.account.user.fetch(userState.publicKey);
      expect(tx).not.to.be.empty;

      expect(user.sweepstakesSubmitted).eq(1);
      expect(user.currentSweepstakeKey.toBase58()).eq(
        player_a_sweepstake.publicKey.toBase58()
      );

      expect(sweepstake.worldChampion).eq(input.worldChampion);
      expect(sweepstake.thirdPlaceGame).eq(input.thirdPlaceGame);
      expect(sweepstake.finalGame).eq(input.finalGame);
      expect(sweepstake.semifinals).eq(input.semifinals);
      expect(sweepstake.groupStage1).eq(input.groupStage1);
      expect(sweepstake.roundOf16).eq(input.roundOf16);
      expect(sweepstake.groupStage2).eq(input.groupStage2);
      expect(sweepstake.groupStage3).eq(input.groupStage3);
      expect(sweepstake.quarterFinals).eq(input.quarterFinals);
      expect(sweepstake.id.toString()).eq(input.id.toString());
      expect(sweepstake.authority.toBase58()).eq(player_a.publicKey.toBase58());
      expect(sweepstake.preSweepstakeKey).to.be.null;

      const balance = await getBalance(dagoatsSolWalletAddress);
      expect(balance - preBalance).eq(solPerSweepstake);
    });

    it("Should not create sweepstake to exceed limit per wallet", () => {
      const sweepstake = anchor.web3.Keypair.generate();
      return expect(
        program.methods
          .createSweepstakeSol(input)
          .accounts({
            userState: userState.publicKey,
            authority: player_a.publicKey,
            dagoatsWallet: dagoatsSolWalletAddress,
            systemProgram: SystemProgram.programId,
            sweepstakeState: sweepstake.publicKey,
          })
          .signers([player_a, sweepstake])
          .rpc()
      ).to.be.rejectedWith(/Exceeded number of sweepstakes per wallet/);
    });
  });
});
