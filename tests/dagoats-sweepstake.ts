import * as anchor from "@project-serum/anchor";
import { LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { DagoatsSweepstake } from "../target/types/dagoats_sweepstake";
import { before } from "mocha";
import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import * as crypto from "crypto";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

use(chaiAsPromised);

const sha = crypto.createHash("sha1");

describe("DaGOATs Sweepstake", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace
    .DagoatsSweepstake as anchor.Program<DagoatsSweepstake>;

  let dagoatsAssociatedTokenAccount, fakeAssociatedTokenAccount;
  const solPerSweepstake = LAMPORTS_PER_SOL;
  const dustPerSweepstake = 10 * LAMPORTS_PER_SOL;
  const dagoatsSolWalletAddress = new anchor.web3.PublicKey(
    "53Xa3PVBki4ZT2qJoJPfiGiA42SyuvQ6WXj5ysw8TRv1"
  );
  const fakeDagoatsSolWalletAddress = new anchor.web3.PublicKey(
    "328vR2LdB9PaLq9eR7ad3Hj8shX9SzSZJhFpeeSkKXa4"
  );

  const user = anchor.web3.Keypair.generate();
  const anotherUser = anchor.web3.Keypair.generate();
  const dustUser = anchor.web3.Keypair.generate();
  const userSweepstake = anchor.web3.Keypair.generate();
  const dustSweepstake = anchor.web3.Keypair.generate();
  const fakeDustSweepstake = anchor.web3.Keypair.generate();
  const userState = anchor.web3.Keypair.generate();
  const fakeDustState = anchor.web3.Keypair.generate();
  const dustUserState = anchor.web3.Keypair.generate();
  const fakeDust = anchor.web3.Keypair.generate();
  const dust = anchor.web3.Keypair.fromSeed(
    Uint8Array.from([
      1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
      1, 1, 1, 1, 1, 1, 1,
    ])
  );

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

  const getSplBalance = async (pubKey: anchor.web3.PublicKey) => {
    const balance = await provider.connection.getTokenAccountBalance(pubKey);
    return Number(balance.value.amount);
  };

  const initializeMint = async (mintAccount: anchor.web3.Keypair) => {
    const authority = anchor.web3.Keypair.generate();
    await airdrop(authority);
    const mint = await createMint(
      provider.connection,
      authority,
      authority.publicKey,
      null,
      9,
      mintAccount
    );
    const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      authority,
      mint,
      authority.publicKey
    );
    await mintTo(
      provider.connection,
      authority,
      mint,
      associatedTokenAccount.address,
      authority,
      10 * LAMPORTS_PER_SOL
    );
    return { associatedTokenAccount, authority, mint };
  };

  before(() =>
    Promise.all([
      airdrop(user, LAMPORTS_PER_SOL / 2),
      airdrop(anotherUser),
      airdrop(dustUser),
    ])
  );

  describe("Initialization", () => {
    it("Should initialize user state", async () => {
      await program.methods
        .createUser(new anchor.BN(1))
        .accounts({
          userState: userState.publicKey,
          authority: user.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([userState, user])
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
            authority: user.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([userState, user])
          .rpc()
      ).to.be.rejectedWith(/This transaction has already been processed/));
  });

  const input = {
    id: new anchor.BN(1),
    worldChampion: "NL",
    finalGame: "NL-PL=10:10",
    thirdPlaceGame: "ES-CZ=0:0",
    semifinals: "NL-PL=10:10;NL-PL=10:10",
    quarterFinals: "NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10",
    roundOf16:
      "NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10",
    groupStage3:
      "NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10",
    groupStage2:
      "NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10",
    groupStage1:
      "NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10;NL-PL=10:10",
  };
  const { id, ...data } = input;
  const shaInput = {
    id,
    predictions: sha.update(JSON.stringify(data)).digest("hex"),
  };

  describe("Sweepstake SOL", () => {
    it("User should not create sweepstake for a different user", () =>
      expect(
        program.methods
          .createSweepstakeSol(shaInput)
          .accounts({
            userState: userState.publicKey,
            authority: anotherUser.publicKey,
            dagoatsWallet: dagoatsSolWalletAddress,
            systemProgram: SystemProgram.programId,
            sweepstakeState: userSweepstake.publicKey,
          })
          .signers([anotherUser, userSweepstake])
          .rpc()
      ).to.be.rejected);

    it("User should not create sweepstake with not enough SOLs", () =>
      expect(
        program.methods
          .createSweepstakeSol(shaInput)
          .accounts({
            userState: userState.publicKey,
            authority: user.publicKey,
            dagoatsWallet: dagoatsSolWalletAddress,
            systemProgram: SystemProgram.programId,
            sweepstakeState: userSweepstake.publicKey,
          })
          .signers([user, userSweepstake])
          .rpc()
      ).to.be.rejected);

    it("User should not create sweepstake with a bad dagoats wallet", () =>
      expect(
        program.methods
          .createSweepstakeSol(shaInput)
          .accounts({
            userState: userState.publicKey,
            authority: user.publicKey,
            dagoatsWallet: fakeDagoatsSolWalletAddress,
            systemProgram: SystemProgram.programId,
            sweepstakeState: userSweepstake.publicKey,
          })
          .signers([user, userSweepstake])
          .rpc()
      ).to.be.rejected);

    it("Should create sweepstake", async () => {
      await airdrop(user, 2 * LAMPORTS_PER_SOL);
      const preBalance = await getBalance(dagoatsSolWalletAddress);
      const tx = await program.methods
        .createSweepstakeSol(shaInput)
        .accounts({
          userState: userState.publicKey,
          authority: user.publicKey,
          dagoatsWallet: dagoatsSolWalletAddress,
          systemProgram: SystemProgram.programId,
          sweepstakeState: userSweepstake.publicKey,
        })
        .signers([user, userSweepstake])
        .rpc();
      const sweepstake = await program.account.sweepstake.fetch(
        userSweepstake.publicKey
      );
      const createdUser = await program.account.user.fetch(userState.publicKey);
      expect(tx).not.to.be.empty;

      expect(createdUser.sweepstakesSubmitted).eq(1);
      expect(createdUser.currentSweepstakeKey.toBase58()).eq(
        userSweepstake.publicKey.toBase58()
      );

      expect(sweepstake.predictions).eq(shaInput.predictions);
      expect(sweepstake.id.toString()).eq(input.id.toString());
      expect(sweepstake.authority.toBase58()).eq(user.publicKey.toBase58());
      expect(sweepstake.preSweepstakeKey).to.be.null;

      const balance = await getBalance(dagoatsSolWalletAddress);
      expect(balance - preBalance).eq(solPerSweepstake);
    });

    it("Should not create sweepstake to exceed limit per wallet", () => {
      const sweepstake = anchor.web3.Keypair.generate();
      return expect(
        program.methods
          .createSweepstakeSol(shaInput)
          .accounts({
            userState: userState.publicKey,
            authority: user.publicKey,
            dagoatsWallet: dagoatsSolWalletAddress,
            systemProgram: SystemProgram.programId,
            sweepstakeState: sweepstake.publicKey,
          })
          .signers([user, sweepstake])
          .rpc()
      ).to.be.rejectedWith(/Exceeded number of sweepstakes per wallet/);
    });
  });

  describe("Sweepstake DUST", async () => {
    let associatedTokenAccount,
      authority,
      fakeAccount,
      fakeAuthority,
      mint,
      fakeMint;
    before(async () => {
      const [ok, fake] = await Promise.all([
        initializeMint(dust),
        initializeMint(fakeDust),
      ]);
      associatedTokenAccount = ok.associatedTokenAccount;
      authority = ok.authority;
      fakeAuthority = fake.authority;
      fakeAccount = fake.associatedTokenAccount;
      mint = ok.mint;
      fakeMint = fake.mint;
      dagoatsAssociatedTokenAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        authority,
        mint,
        dagoatsSolWalletAddress
      );
      fakeAssociatedTokenAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        authority,
        mint,
        fakeDagoatsSolWalletAddress
      );
    });
    it("Should initialize user state", async () => {
      await program.methods
        .createUser(new anchor.BN(1))
        .accounts({
          userState: dustUserState.publicKey,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([dustUserState, authority])
        .rpc();
      const { owner } = await getAccountInfo(dustUserState);
      expect(owner).eq(program.programId.toBase58());

      await program.methods
        .createUser(new anchor.BN(1))
        .accounts({
          userState: fakeDustState.publicKey,
          authority: fakeAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([fakeDustState, fakeAuthority])
        .rpc();
      const { owner: fake } = await getAccountInfo(fakeDustState);
      expect(fake).eq(program.programId.toBase58());
    });
    it("Should not create sweepstake with a bad accounts", () =>
      expect(
        program.methods
          .createSweepstakeSpl(shaInput)
          .accounts({
            mint,
            userState: fakeDustState.publicKey,
            authority: fakeAuthority.publicKey,
            dagoatsWallet: dagoatsAssociatedTokenAccount.address,
            systemProgram: SystemProgram.programId,
            sweepstakeState: fakeDustSweepstake.publicKey,
            userWallet: fakeAssociatedTokenAccount.address,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([fakeAuthority, fakeDustSweepstake])
          .rpc()
      ).to.be.rejectedWith(/A token owner constraint was violated/));
    it("Should create sweepstake", async () => {
      const preBalance = await getSplBalance(
        dagoatsAssociatedTokenAccount.address
      );
      const tx = await program.methods
        .createSweepstakeSpl(shaInput)
        .accounts({
          mint,
          userState: dustUserState.publicKey,
          authority: authority.publicKey,
          dagoatsWallet: dagoatsAssociatedTokenAccount.address,
          systemProgram: SystemProgram.programId,
          sweepstakeState: dustSweepstake.publicKey,
          userWallet: associatedTokenAccount.address,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([authority, dustSweepstake])
        .rpc();
      const sweepstake = await program.account.sweepstake.fetch(
        dustSweepstake.publicKey
      );
      const createdUser = await program.account.user.fetch(
        dustUserState.publicKey
      );
      expect(tx).not.to.be.empty;

      expect(createdUser.sweepstakesSubmitted).eq(1);
      expect(createdUser.currentSweepstakeKey.toBase58()).eq(
        dustSweepstake.publicKey.toBase58()
      );

      expect(sweepstake.predictions).eq(shaInput.predictions);
      expect(sweepstake.id.toString()).eq(input.id.toString());
      expect(sweepstake.authority.toBase58()).eq(
        authority.publicKey.toBase58()
      );
      expect(sweepstake.preSweepstakeKey).to.be.null;

      const balance = await getSplBalance(
        dagoatsAssociatedTokenAccount.address
      );
      expect(balance - preBalance).eq(dustPerSweepstake);
    });
  });
});
