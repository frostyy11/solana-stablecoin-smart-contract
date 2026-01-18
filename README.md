Key Features:

* Initialize: Sets up the stablecoin with a mint authority and configurable decimals
* Mint: Central authority can mint new tokens to any account
* Burn: Users can burn their own tokens
* Transfer: Standard token transfers between accounts
* Pause/Unpause: Emergency controls for the central authority
* Tracking: Keeps track of total minted and burned amounts

Security Features:

* Only the authority can mint tokens and pause/unpause the contract
* Overflow protection on calculations
* Pause functionality for emergency situations

To deploy this:

* Install Anchor: `cargo install --git https://github.com/coral-xyz/anchor avm --locked && avm install latest && avm use latest`
* Create a new Anchor project: `anchor init my_stablecoin`
* Replace the lib.rs file with this code
* Update Anchor.toml with your program ID
* Build: `anchor build`
* Deploy: `anchor deploy`

You'll also need to update the declare_id! macro with your actual program ID after building.
The stablecoin uses Solana's SPL Token standard, so it's compatible with all Solana wallets and exchanges.
