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
  let trade = null;

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

  it("Can create a new PvP trade", async () => {
    // Add your test here.   
    trade = anchor.web3.Keypair.generate();
    const tx = await program.rpc.createTrade(
        new anchor.BN(1000), 
        new anchor.BN(2000), 
        new anchor.BN(10000), 
        new anchor.BN(20000), 
        new anchor.BN(100), {
        accounts: {
            trade: trade.publicKey,
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
        signers: [trade, longKeypair, shortKeypair, gdTokenKeypair],
    });
    
    const USDCPool = await program.provider.connection.getTokenAccountBalance(poolusdc);
    console.log(USDCPool.value.amount + " USDC sent to Pool");

    const gdPool = await program.provider.connection.getTokenAccountBalance(poolgd);
    console.log(gdPool.value.amount + " GD minted to Pool");

    const tradeAccount = await program.account.trade.fetch(trade.publicKey);
    console.log("Trade Account Timestamp: " + tradeAccount.timestamp);
    console.log("Trade Account Asset Price: " + tradeAccount.assetPrice);
    console.log("Trade Account Long: " + tradeAccount.longPayer);
    console.log("Trade Account Short: " + tradeAccount.shortPayer);
    console.log("Trade Account Long Collateral: " + tradeAccount.longCol);
    console.log("Trade Account Short Collateral: " + tradeAccount.shortCol);
    console.log("Trade Account Long Position: " + tradeAccount.longPos);
    console.log("Trade Account Short Position: " + tradeAccount.shortPos);
    console.log("Trade Account Long is Open: " + tradeAccount.longOpen);
    console.log("Trade Account Short is Open: " + tradeAccount.shortOpen);
    console.log("Trade Account transaction signature: ", tx);
  });

  it("Can close a PvP trade with asset price increase", async () => {
    // Add your test here.
    const tx = await program.rpc.closeTrade(new anchor.BN(105), {
        accounts: {
            tradeAccount: trade.publicKey,
            longPayer: longKeypair.publicKey,
            shortPayer: shortKeypair.publicKey,
            transferFrom: poolgd,
            authority: poolKeypair.publicKey,
            transferTo: longgd,
            transferTo2: shortgd,
            gdMint: gdTokenMint,
            mintTo: poolgd,
            authority1: gdTokenKeypair.publicKey,
            tokenProgram: Spl.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [longKeypair, shortKeypair, poolKeypair],
    });

    const longbal = await program.provider.connection.getTokenAccountBalance(longgd);
    console.log(longbal.value.amount + " GD transfered to Long");
    const shortbal = await program.provider.connection.getTokenAccountBalance(shortgd);
    console.log(shortbal.value.amount + " GD transfered to Short");
    
    const tradeAccount = await program.account.trade.fetch(trade.publicKey);
    console.log("Trade Account Timestamp Created: " + tradeAccount.timestamp);
    console.log("Trade Account Asset Price: " + tradeAccount.assetPrice);
    console.log("Trade Account Final Price: " + tradeAccount.finalPrice);
    console.log("Trade Account Long: " + tradeAccount.longPayer);
    console.log("Trade Account Short: " + tradeAccount.shortPayer);
    console.log("Trade Account Long Collateral: " + tradeAccount.longCol);
    console.log("Trade Account Short Collateral: " + tradeAccount.shortCol);
    console.log("Trade Account Long Position: " + tradeAccount.longPos);
    console.log("Trade Account Short Position: " + tradeAccount.shortPos);
    console.log("Trade Account Long GD Token Distribution: " + tradeAccount.longDist);
    console.log("Trade Account Short GD Token Distribution: " + tradeAccount.shortDist);
    console.log("Trade Account Long is Open: " + tradeAccount.longOpen);
    console.log("Trade Account Short is Open: " + tradeAccount.shortOpen);
    console.log("Trade Account Timestamp Closed: " + tradeAccount.timestampClose);
    console.log("Trade Account transaction signature: ", tx);
  });

  it("Can create a new PvP trade", async () => {
    // Add your test here.   
    trade = anchor.web3.Keypair.generate();
    const tx = await program.rpc.createTrade(
        new anchor.BN(1000), 
        new anchor.BN(2000), 
        new anchor.BN(10000), 
        new anchor.BN(20000), 
        new anchor.BN(100), {
        accounts: {
            trade: trade.publicKey,
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
        signers: [trade, longKeypair, shortKeypair, gdTokenKeypair],
    });
    
    const USDCPool = await program.provider.connection.getTokenAccountBalance(poolusdc);
    console.log(USDCPool.value.amount + " USDC sent to Pool");

    const gdPool = await program.provider.connection.getTokenAccountBalance(poolgd);
    console.log(gdPool.value.amount + " GD minted to Pool");

    const tradeAccount = await program.account.trade.fetch(trade.publicKey);
    console.log("Trade Account Timestamp: " + tradeAccount.timestamp);
    console.log("Trade Account Asset Price: " + tradeAccount.assetPrice);
    console.log("Trade Account Long: " + tradeAccount.longPayer);
    console.log("Trade Account Short: " + tradeAccount.shortPayer);
    console.log("Trade Account Long Collateral: " + tradeAccount.longCol);
    console.log("Trade Account Short Collateral: " + tradeAccount.shortCol);
    console.log("Trade Account Long Position: " + tradeAccount.longPos);
    console.log("Trade Account Short Position: " + tradeAccount.shortPos);
    console.log("Trade Account Long is Open: " + tradeAccount.longOpen);
    console.log("Trade Account Short is Open: " + tradeAccount.shortOpen);
    console.log("Trade Account transaction signature: ", tx);
  });

  it("Can close a PvP trade with asset price decrease", async () => {
    const tx = await program.rpc.closeTrade(new anchor.BN(95), {
        accounts: {
            tradeAccount: trade.publicKey,
            longPayer: longKeypair.publicKey,
            shortPayer: shortKeypair.publicKey,
            transferFrom: poolgd,
            authority: poolKeypair.publicKey,
            transferTo: longgd,
            transferTo2: shortgd,
            gdMint: gdTokenMint,
            mintTo: poolgd,
            authority1: gdTokenKeypair.publicKey,
            tokenProgram: Spl.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [longKeypair, shortKeypair, poolKeypair],
    });

    const longbal = await program.provider.connection.getTokenAccountBalance(longgd);
    console.log(longbal.value.amount + " GD transfered to Long");
    const shortbal = await program.provider.connection.getTokenAccountBalance(shortgd);
    console.log(shortbal.value.amount + " GD transfered to Short");
    
    const tradeAccount = await program.account.trade.fetch(trade.publicKey);
    console.log("Trade Account Timestamp Created: " + tradeAccount.timestamp);
    console.log("Trade Account Asset Price: " + tradeAccount.assetPrice);
    console.log("Trade Account Final Price: " + tradeAccount.finalPrice);
    console.log("Trade Account Long: " + tradeAccount.longPayer);
    console.log("Trade Account Short: " + tradeAccount.shortPayer);
    console.log("Trade Account Long Collateral: " + tradeAccount.longCol);
    console.log("Trade Account Short Collateral: " + tradeAccount.shortCol);
    console.log("Trade Account Long Position: " + tradeAccount.longPos);
    console.log("Trade Account Short Position: " + tradeAccount.shortPos);
    console.log("Trade Account Long GD Token Distribution: " + tradeAccount.longDist);
    console.log("Trade Account Short GD Token Distribution: " + tradeAccount.shortDist);
    console.log("Trade Account Long is Open: " + tradeAccount.longOpen);
    console.log("Trade Account Short is Open: " + tradeAccount.shortOpen);
    console.log("Trade Account Timestamp Closed: " + tradeAccount.timestampClose);
    console.log("Trade Account transaction signature: ", tx);
  });
});