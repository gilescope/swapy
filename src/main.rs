use crate::westmint::runtime_types::{
    bounded_collections::bounded_vec::BoundedVec,
    xcm::v3::{junction::Junction, junctions::Junctions::*, multilocation::MultiLocation},
};
use sp_keyring::{AccountKeyring, Sr25519Keyring};
use subxt::{tx::PairSigner, utils::MultiAddress, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_url = "wss://westmint-rpc.polkadot.io:443")]
// #[subxt::subxt(runtime_metadata_url = "ws://localhost:9910")]
pub mod westmint {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let alice_pub = AccountKeyring::Alice.public();
    let alice_account = Sr25519Keyring::from_public(&alice_pub)
        .unwrap()
        .to_account_id();

    // let tx = westmint::tx().system().remark("Hello, world!".into());

    let tx = westmint::tx()
        .assets()
        .create(1, MultiAddress::Id(alice_account.clone().into()), 1);

    // let dot = MultiLocation {
    //     parents: 0,
    //     interior: Here,
    // };
    // let dot2 = MultiLocation {
    //     parents: 0,
    //     interior: Here,
    // };
    // let dot3 = MultiLocation {
    //     parents: 0,
    //     interior: Here,
    // };
    // let asset1 = MultiLocation {
    //     parents: 0,
    //     interior: X2(Junction::PalletInstance(50), Junction::GeneralIndex(1)),
    // };
    // let asset1a = MultiLocation {
    //     parents: 0,
    //     interior: X2(Junction::PalletInstance(50), Junction::GeneralIndex(1)),
    // };
    // let asset1b = MultiLocation {
    //     parents: 0,
    //     interior: X2(Junction::PalletInstance(50), Junction::GeneralIndex(1)),
    // };
    // let tx = westmint::tx().asset_conversion().create_pool(dot, asset1);

    // let tx = westmint::tx().asset_conversion().add_liquidity(
    //     dot2,
    //     asset1a,
    //     100,
    //     100,
    //     1,
    //     1,
    //     alice_account.clone().into(),
    // );

    // let tx = westmint::tx()
    //     .asset_conversion()
    //     .swap_exact_tokens_for_tokens(
    //         BoundedVec(vec![dot3, asset1b]),
    //         10,
    //         1,
    //         alice_account.into(),
    //         true,
    //     );

    // Submit the transaction with default params:
    let _hash = api.tx().sign_and_submit_default(&tx, &signer).await?;
    println!("Hello, world! hash: {:?}", _hash);

    Ok(())
}
