import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PvpAmm } from "../target/types/pvp_amm";
import * as assert from "assert";
import * as bs58 from "bs58";
import * as serumCmn from "@project-serum/common";
import { TokenInstructions } from  "@project-serum/serum";
import * as Spl from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

describe("pvp-amm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PvpAmm as Program<PvpAmm>;

  let usdcKeypair = null;
  let usdcMint = null;
  let gdTokenKeypair = null;
  let gdTokenMint = null;  
  let poolKeypair = null;
  let poolusdc = null;
  let poolgd = null;
  let longKeypair = null;
  let shortKeypair = null;
  let longusdc = null;
  let shortusdc = null;
  let longgd = null;
  let shortgd = null;
  let pool = null;

  it("Is initialized!", async () => {
    usdcKeypair = await anchor.web3.Keypair.generate();
    const signature = await program.provider.connection.requestAirdrop(usdcKeypair.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature);
    
    usdcMint = await Spl.createMint(program.provider.connection, 
        usdcKeypair,           
        usdcKeypair.publicKey,       
        null,                         
        6);
    console.log("Created USDC Mint: " + usdcMint.toBase58());

    gdTokenKeypair = await anchor.web3.Keypair.generate();
    const signature1 = await program.provider.connection.requestAirdrop(gdTokenKeypair.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature1);
    
    gdTokenMint = await Spl.createMint(
        program.provider.connection, 
        gdTokenKeypair,  
        gdTokenKeypair.publicKey,          
        null,                          
        12);
    console.log("Created GD Token Mint: " + gdTokenMint.toBase58());

    poolKeypair = await anchor.web3.Keypair.generate();
    const signature2 = await program.provider.connection.requestAirdrop(poolKeypair.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature2);

    const usdcMintPublicKey = new PublicKey(usdcMint);
    const gdTokenMintPublicKey = new PublicKey(gdTokenMint);

    poolusdc = await Spl.createAssociatedTokenAccount(
        program.provider.connection, 
        poolKeypair,  
        usdcMintPublicKey,
        poolKeypair.publicKey
    );
    console.log("Created USDC Pool: " + poolusdc.toBase58());

    poolgd = await Spl.createAssociatedTokenAccount(
        program.provider.connection, 
        poolKeypair,  
        gdTokenMintPublicKey,
        poolKeypair.publicKey
    );
    console.log("Created GD Token Pool: " + poolgd.toBase58());

    longKeypair = await anchor.web3.Keypair.generate();
    const signature3 = await program.provider.connection.requestAirdrop(longKeypair.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature3);
    
    longusdc = await Spl.createAssociatedTokenAccount(
        program.provider.connection, 
        longKeypair,  
        usdcMintPublicKey,
        longKeypair.publicKey
    );
    console.log("Created Long USDC ATA: " + longusdc.toBase58());

    shortKeypair = await anchor.web3.Keypair.generate();
    const signature4 = await program.provider.connection.requestAirdrop(shortKeypair.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature4);
    
    shortusdc = await Spl.createAssociatedTokenAccount(
        program.provider.connection, 
        shortKeypair,  
        usdcMintPublicKey,
        shortKeypair.publicKey
    );
    console.log("Created Short USDC ATA: " + shortusdc.toBase58());

    // longgd = await createTokenAccount(program.provider, gdTokenMint, program.provider.wallet.publicKey);
    longgd = await Spl.createAssociatedTokenAccount(
        program.provider.connection, 
        longKeypair,
        gdTokenMintPublicKey,
        longKeypair.publicKey
    );
    console.log("Created Long GD Token ATA: " + longgd.toBase58());

    // shortgd = await createTokenAccount(program.provider, gdTokenMint, program.provider.wallet.publicKey);
    shortgd = await Spl.createAssociatedTokenAccount(
        program.provider.connection, 
        shortKeypair,  
        gdTokenMintPublicKey,
        shortKeypair.publicKey
    );
    console.log("Created Short GD Token ATA: " + shortgd.toBase58());

    const longUSDCPublicKey = new PublicKey(longusdc);
    let tx = await Spl.mintTo(
        program.provider.connection,
        longKeypair,
        usdcMintPublicKey,
        longUSDCPublicKey,
        usdcKeypair,
        10000,
    );
    const usdcMintedToLong = await program.provider.connection.getTokenAccountBalance(longusdc);
    console.log("Minted " + usdcMintedToLong.value.amount + " USDC to Long");

    const shortMintPublicKey = new PublicKey(shortusdc);
    let tx1 = await Spl.mintTo(
        program.provider.connection,
        shortKeypair,
        usdcMintPublicKey,
        shortMintPublicKey,
        usdcKeypair,
        10000,
    );
    const usdcMintedToShort = await program.provider.connection.getTokenAccountBalance(shortusdc);
    console.log("Minted " + usdcMintedToShort.value.amount + " USDC to Short");

  });

  it("Can create a new pool", async () => {
    // Add your test here.   
    pool = anchor.web3.Keypair.generate();
    const tx = await program.rpc.createPool(
        new anchor.BN(1000), 
        new anchor.BN(2000), 
        new anchor.BN(10000), 
        new anchor.BN(20000), 
        new anchor.BN(100), {
        accounts: {
            pool: pool.publicKey,
            longPayer: longKeypair.publicKey,
            shortPayer: shortKeypair.publicKey,
            mint: gdTokenMint,
            authority: gdTokenKeypair.publicKey,
            from: longusdc,
            from2: shortusdc,
            // transferTo: poolusdc,
            mintTo: poolgd,
            tokenProgram: Spl.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [pool, longKeypair, shortKeypair, gdTokenKeypair],
    });
    
    const USDCPool = await program.provider.connection.getTokenAccountBalance(poolusdc);
    console.log(USDCPool.value.amount + " USDC sent to Pool");

    const gdPool = await program.provider.connection.getTokenAccountBalance(poolgd);
    console.log(gdPool.value.amount + " GD minted to Pool");

    const poolAccount = await program.account.pool.fetch(pool.publicKey);
    console.log("Created Pool Account Timestamp: " + poolAccount.timestamp);
    console.log("Created Pool Account Asset Price: " + poolAccount.assetPrice);
    console.log("Created Pool Account Long: " + poolAccount.longPayer);
    console.log("Created Pool Account Short: " + poolAccount.shortPayer);
    console.log("Created Pool Account Long Collateral: " + poolAccount.longCol);
    console.log("Created Pool Account Short Collateral: " + poolAccount.shortCol);
    console.log("Created Pool Account Long Position: " + poolAccount.longPos);
    console.log("Created Pool Account Short Position: " + poolAccount.shortPos);
    
    console.log("Pool Account transaction signature", tx);
  });

  it("Can close a pool", async () => {
    // Add your test here.
    const tx = await program.rpc.closePool(new anchor.BN(105), {
        accounts: {
            poolAccount: pool.publicKey,
            longPayer: longKeypair.publicKey,
            shortPayer: shortKeypair.publicKey,
            tokenProgram: Spl.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [longKeypair, shortKeypair],
    });
    
    const poolAccount = await program.account.pool.fetch(pool.publicKey);
    console.log(poolAccount.assetPrice.toString());
    console.log("Pool Account transaction signature", tx);
  });
});