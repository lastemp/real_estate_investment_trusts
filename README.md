# real_estate_investment_trusts

This is a [Real Estate Investment Trusts](https://www.nse.co.ke/real-estate-investment-trusts/) Rust Smart Contract(Solana Blockchain) built for educational purposes.
A REIT(Real Estate Investment Trusts) is a regulated collective investment vehicle that enables persons to contribute money’s worth as consideration for the acquisition of rights or interests in a trust that is divided into units with the intention of earning profits or income from real estate as beneficiaries of the trust.
This Smart Contract provides a marketplace where investors buy and sell real estate investment trusts.

There are two main types of REITs:
- Development Real Estate Investment Trusts (D-REITs)
- Income Real Estate Investment Trust (I-REITs)

Below are some features contained in the program:

- Register investment trust scheme
- Register investor
- Buy investment trusts
- Sell investment trusts

## Getting started

In order to run this example program you will need to install Rust and
Solana. Information about installing Rust can be found
[here](https://rustup.rs/) and information about installing Solana can
be found [here](https://docs.solana.com/cli/install-solana-cli-tools).

Once you've completed the Solana installation run the following
commands to configure your machine for local deployment:

```
solana config set --url localhost
solana-keygen new
```

These two commands create Solana config files in `~/.config/solana/`
which solana command line tools will read in to determine what cluster
to connect to and what keypair to use.

Having done that run a local Solana validator by executing:

```
solana-test-validator
```

This program must be left running in a separate terminal window.

## Deploying the Solana program

To deploy the Solana program in this repository to the Solana cluster
that you have configured run:

```
anchor build
```

```
anchor deploy
```

## Running the test program

To run the test program you must have already deployed the Solana
program. The test program sends a transaction to the Solana
blockchain asking it to execute the deployed program and reports the
results.

```
anchor test --skip-local-validator
```
