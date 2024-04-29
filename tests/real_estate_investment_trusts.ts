import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RealEstateInvestmentTrusts } from "../target/types/real_estate_investment_trusts";

describe("real_estate_investment_trusts", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");

  const program = anchor.workspace
    .RealEstateInvestmentTrusts as Program<RealEstateInvestmentTrusts>;
  const adminOwner = anchor.web3.Keypair.generate();
  const investorOwner = anchor.web3.Keypair.generate();
  const trustSchemePromoter = anchor.web3.Keypair.generate();

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

  /* it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  }); */

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
    /* 1 - DevelopmentRealEstateInvestmentTrusts, // (D-REITs)
       2 - IncomeRealEstateInvestmentTrust,       // (I-REITs) 
    */
    let marketIssuer = {
      issuer: "Acorn Holdings Limited",
      name: "Acorn ASA",
      typeOfReit: 2, // IncomeRealEstateInvestmentTrust
      listingDate: "February 2021",
    };

    let initParams = {
      issuer: marketIssuer,
      country: "KE",
    };

    const tx = await program.methods
      .registerInvestmentTrustScheme(initParams)
      .accounts({
        owner: trustSchemePromoter.publicKey,
        realEstateInvestmentTrustScheme: realEstateInvestmentTrustScheme,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([trustSchemePromoter])
      .rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.realEstateInvestmentTrustScheme.fetch(
      realEstateInvestmentTrustScheme
    );
    console.log("real estate investment trust scheme: ", result);
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
});
