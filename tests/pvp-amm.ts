import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PvpAmm } from "../target/types/pvp_amm";
import * as assert from "assert";
import * as bs58 from "bs58";
import * as serumCmn from "@project-serum/common";
import { TokenInstructions } from  "@project-serum/serum";

describe("pvp-amm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PvpAmm as Program<PvpAmm>;

  let usdcMint = null;
  let gdTokenMint = null;  
  let poolusdc = null;
  let longusdc = null;
  let shortusdc = null;
  let poolgd = null;
  let longgd = null;
  let shortgd = null;

  it("Is initialized!", async () => {
    // Add your test here.
   //usdcMint = await spl.Token.createMint(program.provider.connection, program.provider.wallet.publicKey, program.provider.wallet.publicKey, program.provider.wallet.publicKey, 0, program.programId);
    gdTokenMint = await createMint(program.provider);
    // poolusdc = await createTokenAccount(program.provider, usdcMint, program.provider.wallet.publicKey);
    // longusdc = await createTokenAccount(program.provider, usdcMint, program.provider.wallet.publicKey);
    // shortusdc = await createTokenAccount(program.provider, usdcMint, program.provider.wallet.publicKey);
    // poolgd = await createTokenAccount(program.provider, gdTokenMint, program.provider.wallet.publicKey);
    // longgd = await createTokenAccount(program.provider, gdTokenMint, program.provider.wallet.publicKey);
    // shortgd = await createTokenAccount(program.provider, gdTokenMint, program.provider.wallet.publicKey);

  });

  it("Can create a new pool", async () => {
    // Add your test here.   
    let pool = anchor.web3.Keypair.generate();
    const tx = await program.rpc.createPool(new anchor.BN(100), {
        accounts: {
            pool: pool.publicKey,
            author: program.provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [pool],
    });
    console.log("Your transaction signature", tx);
  });
});

// const program = anchor.workspace.PvpAmm as Program<PvpAmm>;

const TOKEN_PROGRAM_ID = new anchor.web3.PublicKey(
  TokenInstructions.TOKEN_PROGRAM_ID.toString()
);

async function getTokenAccount(provider, addr) {
  return await serumCmn.getTokenAccount(provider, addr);
}

async function getMintInfo(provider, mintAddr) {
  return await serumCmn.getMintInfo(provider, mintAddr);
}

async function createMint(provider, authority) {
  if (authority === undefined) {
    authority = provider.wallet.publicKey;
  }
  const mint = anchor.web3.Keypair.generate();
  const instructions = await createMintInstructions(
    provider,
    authority,
    mint.publicKey
  );

  const tx = new anchor.web3.Transaction();
  tx.add(...instructions);

  await provider.send(tx, [mint]);

  return mint.publicKey;
}

async function createMintInstructions(provider, authority, mint) {
  let instructions = [
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: mint,
      space: 82,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
      programId: TOKEN_PROGRAM_ID,
    }),
    TokenInstructions.initializeMint({
      mint,
      decimals: 9,
      mintAuthority: authority,
    }),
  ];
  return instructions;
}

async function createTokenAccount(provider, mint, owner) {
  const vault = anchor.web3.Keypair.generate();
  const tx = new anchor.web3.Transaction();
  tx.add(
    ...(await createTokenAccountInstrs(program.provider, vault.publicKey, mint, owner))
  );
  await provider.send(tx, [vault]);
  return vault.publicKey;
}

async function createTokenAccountInstrs(
  provider,
  newAccountPubkey,
  mint,
  owner,
  lamports
) {
  if (lamports === undefined) {
    lamports = await provider.connection.getMinimumBalanceForRentExemption(165);
  }
  return [
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey,
      space: 165,
      lamports,
      programId: TOKEN_PROGRAM_ID,
    }),
    TokenInstructions.initializeAccount({
      account: newAccountPubkey,
      mint,
      owner,
    }),
  ];
}
