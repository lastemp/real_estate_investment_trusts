import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealEstateInvestmentTrusts } from "../target/types/real_estate_investment_trusts";
import {
  Account,
  createAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("real_estate_investment_trusts", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace
    .RealEstateInvestmentTrusts as Program<RealEstateInvestmentTrusts>;
  const adminOwner = anchor.web3.Keypair.generate();
  const trustSchemePromoter = anchor.web3.Keypair.generate();
  const depositAccount = anchor.web3.Keypair.generate();
  /* const usdcMint = new anchor.web3.PublicKey(
    "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
  ); // USDC devnet */

  const payer = wallet.payer;
  const associateTokenProgram = new anchor.web3.PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );
  const mintToken = anchor.web3.Keypair.generate(); // dummy usdc token created for test purposes
  const tokenAccount = anchor.utils.token.associatedAddress({
    mint: mintToken.publicKey,
    owner: payer.publicKey,
  });

  let investorOwner = anchor.web3.Keypair.generate();
  let investorOwnerATA = anchor.web3.Keypair.generate();

  let treasuryVaultATA: Account;

  // pdaAuth
  let [pdaAuth, adminPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("auth"),
      depositAccount.publicKey.toBuffer(),
    ],
    program.programId
  );
  let [treasuryVault, adminTreasuryBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("treasury-vault"), pdaAuth.toBuffer()],
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

  // admin owner
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

  // investor owner
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

  // trust scheme promoter
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
    try {
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
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.investmentTrustsConfigs.fetch(
        investmentTrustsConfigs
      );
      console.log("investmentTrustsConfigs: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register investment trust scheme!", async () => {
    // typeOfReit
    // 1 - DevelopmentRealEstateInvestmentTrusts i.e (D-REITs)
    // 2 - IncomeRealEstateInvestmentTrust i.e (I-REITs)

    try {
      let marketIssuer = {
        issuer: "Acorn Holdings Limited",
        name: "Acorn ASA",
        typeOfReit: 2, // IncomeRealEstateInvestmentTrust
        listingDate: "April 2024",
      };

      let initParams = {
        issuer: marketIssuer,
        country: "KE",
        unitCostOfInvestmentTrusts: 1, // unit cost of investment trusts
        decimals: 9, // token mint in smallest unit i.e 9 decimals
      };

      const tx = await program.methods
        .registerInvestmentTrustScheme(initParams)
        .accounts({
          owner: trustSchemePromoter.publicKey,
          investmentTrustsConfigs: investmentTrustsConfigs,
          realEstateInvestmentTrustScheme: realEstateInvestmentTrustScheme,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([trustSchemePromoter, depositAccount])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
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
    } catch (error) {
      console.log(error);
    }
  });

  it("Is create token!", async () => {
    console.log("mint token: ", mintToken.publicKey.toBase58());
    console.log("token account: ", tokenAccount.toBase58());

    try {
      let initParams = {
        amount: new anchor.BN(100),
      };

      const tx = await program.methods
        .createToken(initParams)
        .accounts({
          owner: payer.publicKey,
          realEstateInvestmentTrustScheme: realEstateInvestmentTrustScheme,
          mintToken: mintToken.publicKey,
          tokenAccount: tokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is token transfer", async () => {
    console.log(
      "investor owner token account: ",
      investorOwnerATA.publicKey.toBase58()
    );

    try {
      await createAccount(
        provider.connection,
        investorOwner,
        mintToken.publicKey,
        investorOwner.publicKey,
        investorOwnerATA
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let initParams = {
        amount: new anchor.BN(70),
      };
      const tx = await program.methods
        .transferToken(initParams)
        .accounts({
          owner: payer.publicKey,
          realEstateInvestmentTrustScheme: realEstateInvestmentTrustScheme,
          mintToken: mintToken.publicKey,
          fromAccount: tokenAccount,
          toAccount: investorOwnerATA.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([mintToken])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is register first investor!", async () => {
    try {
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
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.investor.fetch(investor);
      console.log("investor: ", result);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is buy investment trusts!", async () => {
    try {
      treasuryVaultATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mintToken.publicKey,
        treasuryVault,
        true
      );
      console.log(
        "treasuryVaultATA address: " + treasuryVaultATA.address.toBase58()
      );
    } catch (error) {
      console.log(error);
    }

    try {
      let initParams = {
        // 10 amount of token to transfer (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(10),
      };

      const tx = await program.methods
        .buyInvestmentTrusts(initParams)
        .accounts({
          owner: investorOwner.publicKey,
          realEstateInvestmentTrustScheme: realEstateInvestmentTrustScheme,
          investor: investor,
          senderTokens: investorOwnerATA.publicKey,
          recipientTokens: treasuryVaultATA.address,
          mintToken: mintToken.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([investorOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.investor.fetch(investor);
      console.log("investor: ", result);

      let result2 = await program.account.realEstateInvestmentTrustScheme.fetch(
        realEstateInvestmentTrustScheme
      );
      console.log("real estate investment trust scheme: ", result2);
    } catch (error) {
      console.log(error);
    }
  });

  it("Is sell investment trusts!", async () => {
    try {
      let initParams = {
        // 3 amount of token to transfer (in smallest unit i.e 9 decimals)
        amount: new anchor.BN(3),
      };
      const tx = await program.methods
        .sellInvestmentTrusts(initParams)
        .accounts({
          owner: investorOwner.publicKey,
          realEstateInvestmentTrustScheme: realEstateInvestmentTrustScheme,
          investor: investor,
          senderTokens: treasuryVaultATA.address,
          recipientTokens: investorOwnerATA.publicKey,
          mintToken: mintToken.publicKey,
          depositAccount: depositAccount.publicKey,
          pdaAuth: pdaAuth,
          treasuryVault: treasuryVault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associateTokenProgram: associateTokenProgram,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([investorOwner])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }

    try {
      let result = await program.account.depositBase.fetch(
        depositAccount.publicKey
      );
      console.log("deposit account: ", result);

      let result2 = await program.account.investor.fetch(investor);
      console.log("investor: ", result2);

      let result3 = await program.account.realEstateInvestmentTrustScheme.fetch(
        realEstateInvestmentTrustScheme
      );
      console.log("real estate investment trust scheme: ", result3);
    } catch (error) {
      console.log(error);
    }
  });
});
