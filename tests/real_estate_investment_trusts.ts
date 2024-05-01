import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealEstateInvestmentTrusts } from "../target/types/real_estate_investment_trusts";
import {
  Account,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
//import { AnchorSplToken } from "../target/types/anchor_spl_token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
//import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("real_estate_investment_trusts", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  //const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace
    .RealEstateInvestmentTrusts as Program<RealEstateInvestmentTrusts>;
  const adminOwner = anchor.web3.Keypair.generate();
  const investorOwner = anchor.web3.Keypair.generate();
  const trustSchemePromoter = anchor.web3.Keypair.generate();
  const depositAccount = anchor.web3.Keypair.generate();
  /* const usdcMint = new anchor.web3.PublicKey(
    "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
  ); // USDC devnet */
  const usdcMint = new anchor.web3.PublicKey(
    "F9LbqkLDEWzwd1Hz8HeQgd56utrvqgPtXmhWX2QN5j5n"
  ); // test token
  const payer = wallet.payer;
  //const recipient = anchor.web3.Keypair.generate();
  const associateTokenProgram = new anchor.web3.PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );

  let payerATA: Account;
  //let recipientATA: Account;
  let treasuryVaultATA: Account;

  // pdaAuth
  let [pdaAuth, adminPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("auth"),
      depositAccount.publicKey.toBuffer(),
    ],
    program.programId
  );
  let [solVault, adminSolBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("sol-vault"), pdaAuth.toBuffer()],
    program.programId
  );

  let [investmentTrustsConfigs] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("investment-trusts-configs")],
    program.programId
  );

  let [realEstateInvestmentTrustScheme] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("investment-trust-scheme"),
        trustSchemePromoter.publicKey.toBuffer(),
      ],
      program.programId
    );

  let [investor] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("investor"),
      investorOwner.publicKey.toBuffer(),
    ],
    program.programId
  );

  // senderATA & treasuryVaultATA
  before(async () => {
    payerATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      usdcMint,
      payer.publicKey
    );

    treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      usdcMint,
      solVault, //recipient.publicKey
      true
    );
    console.log("senderATA address: " + payerATA.address.toBase58());
    console.log("recipientATA address: " + treasuryVaultATA.address.toBase58());
  });

  // admin Owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      adminOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // investor Owner
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      investorOwner.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  // trust Scheme Promoter
  before(async () => {
    let res = await provider.connection.requestAirdrop(
      trustSchemePromoter.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is initialized!", async () => {
    let initParams = {
      isInitialized: true,
    };

    const tx = await program.methods
      .init(initParams)
      .accounts({
        owner: adminOwner.publicKey,
        investmentTrustsConfigs: investmentTrustsConfigs,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([adminOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.investmentTrustsConfigs.fetch(
      investmentTrustsConfigs
    );
    console.log("investmentTrustsConfigs: ", result);
  });

  it("Is register investment trust scheme!", async () => {
    // typeOfReit
    // 1 - DevelopmentRealEstateInvestmentTrusts i.e (D-REITs)
    // 2 - IncomeRealEstateInvestmentTrust i.e (I-REITs)

    let marketIssuer = {
      issuer: "Acorn Holdings Limited",
      name: "Acorn ASA",
      typeOfReit: 2, // IncomeRealEstateInvestmentTrust
      listingDate: "April 2024",
    };

    let initParams = {
      issuer: marketIssuer,
      country: "KE",
    };

    const tx = await program.methods
      .registerInvestmentTrustScheme(initParams)
      .accounts({
        owner: trustSchemePromoter.publicKey,
        investmentTrustsConfigs: investmentTrustsConfigs,
        realEstateInvestmentTrustScheme: realEstateInvestmentTrustScheme,
        depositAccount: depositAccount.publicKey,
        pdaAuth: pdaAuth,
        solVault: solVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([trustSchemePromoter, depositAccount])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.realEstateInvestmentTrustScheme.fetch(
      realEstateInvestmentTrustScheme
    );
    let result2 = await program.account.depositBase.fetch(
      depositAccount.publicKey
    );
    let result3 = await program.account.investmentTrustsConfigs.fetch(
      investmentTrustsConfigs
    );
    console.log("real estate investment trust scheme: ", result);
    console.log("deposit account: ", result2);
    console.log("investment trusts configs: ", result3);
  });

  it("Is register first investor!", async () => {
    let initParams = {
      fullNames: "paul john",
      country: "KE",
    };

    const tx = await program.methods
      .registerInvestor(initParams)
      .accounts({
        owner: investorOwner.publicKey,
        investor: investor,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([investorOwner])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.investor.fetch(investor);
    console.log("investor: ", result);
  });

  it("Is buy investment trusts!", async () => {
    /* The set transfer amount value of 10000000 is equal to 10 USDC. 
    This setting is because USDC uses a 6 decimal place format, 
    and the amount value should be in the smallest unit. */
    let initParams = {
      //amount: new anchor.BN(10000000), // 10 amount of USDC to transfer (in smallest unit)
      amount: new anchor.BN(10 ** 9 * 10), // 10 amount of token to transfer (in smallest unit i.e 9 decimals)
    };

    const tx = await program.methods
      .buyInvestmentTrusts(initParams)
      .accounts({
        owner: payer.publicKey,
        senderTokens: payerATA.address,
        recipientTokens: treasuryVaultATA.address,
        mintToken: usdcMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        associateTokenProgram: associateTokenProgram,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc();
    console.log("Your transaction signature", tx);

    /* let result = await program.account.investmentTrustsConfigs.fetch(
      investmentTrustsConfigs
    );
    console.log("investmentTrustsConfigs: ", result); */
  });

  it("Is sell investment trusts!", async () => {
    /* The set transfer amount value of 10000000 is equal to 10 USDC. 
    This setting is because USDC uses a 6 decimal place format, 
    and the amount value should be in the smallest unit. */
    let initParams = {
      amount: new anchor.BN(10 ** 9 * 3), // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
    };

    const tx = await program.methods
      .sellInvestmentTrusts(initParams)
      .accounts({
        owner: payer.publicKey,
        senderTokens: treasuryVaultATA.address,
        recipientTokens: payerATA.address,
        mintToken: usdcMint,
        depositAccount: depositAccount.publicKey,
        pdaAuth: pdaAuth,
        solVault: solVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associateTokenProgram: associateTokenProgram,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.depositBase.fetch(
      depositAccount.publicKey
    );
    console.log("deposit account: ", result);
  });
});
