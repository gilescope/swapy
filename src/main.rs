use crate::westmint::runtime_types::xcm::v3::{
    junction::Junction, junctions::Junctions::*, multilocation::MultiLocation,
};
use sp_keyring::{AccountKeyring, Sr25519Keyring};
use subxt::{tx::PairSigner, utils::MultiAddress, OnlineClient, SubstrateConfig};

#[subxt::subxt(
    runtime_metadata_url = "ws://localhost:9910",
    derive_for_all_types = "Clone"
)]
pub mod westmint {}

#[subxt::subxt(
    runtime_metadata_url = "ws://localhost:9944",
    derive_for_all_types = "Clone"
)]
pub mod westend {}

mod westend_context {
    use crate::westend::runtime_types::{
        pallet_xcm,
        sp_weights::weight_v2::Weight,
        westend_runtime::*,
        xcm::{v3::*, *},
    };

    //Weird v2:
    use crate::westend::runtime_types::xcm::v2::OriginKind;

    pub fn ask_westmint_to(encoded: Vec<u8>) -> RuntimeCall {
        RuntimeCall::XcmPallet(pallet_xcm::pallet::Call::send {
            dest: Box::new(VersionedMultiLocation::V3(multilocation::MultiLocation {
                parents: 0,
                interior: junctions::Junctions::X1(junction::Junction::Parachain(1000)),
            })),
            message: Box::new(VersionedXcm::V3(Xcm(vec![
                Instruction::UnpaidExecution {
                    check_origin: None,
                    weight_limit: WeightLimit::Unlimited,
                },
                Instruction::Transact {
                    origin_kind: OriginKind::SovereignAccount, //Superuser, //Native,
                    require_weight_at_most: Weight {
                        ref_time: 4_000_000_000,
                        proof_size: 20_024,
                    },
                    call: double_encoded::DoubleEncoded { encoded },
                },
            ]))),
        })
    }
}

mod westmint_context {
    use crate::westmint::runtime_types::{
        frame_system, pallet_asset_conversion, pallet_assets, pallet_utility,
        westmint_runtime::{RuntimeCall, *},
        xcm::{v3::*, *},
    };
    use parity_scale_codec::Encode;
    use sp_keyring::{AccountKeyring, Sr25519Keyring};
    use subxt::utils::MultiAddress;
    // use crate::westmint::runtime_types::sp_weights::weight_v2::Weight;
    // use subxt::utils::AccountId32;
    use subxt::tx::Payload;
    //  use crate::westmint::assets::calls::types::Create;
    use crate::westmint::utility::calls::types::Batch;
    pub fn asset_create() -> Payload<Batch> {
        let alice_pub = AccountKeyring::Alice.public();
        let alice_account = Sr25519Keyring::from_public(&alice_pub)
            .unwrap()
            .to_account_id();

        // RuntimeCall::Assets(pallet_assets::pallet::Call::create {
        //     id: 33,
        //     // is_sufficient: false,
        //     min_balance: 0,
        //     admin: MultiAddress::Id(alice_account.clone().into()),
        //     // subxt::utils::MultiAddress::<_, ()>::Id(AccountId32::new())
        // })
        // RuntimeCall::System(
        // frame_system::pallet::Call::remark_with_event {
        //     remark: "this is not a test.".as_bytes().to_vec(),
        // }
        // )

        // RuntimeCall::Utility(pallet_utility::pallet::Call::batch {
        //     calls: vec![

        // let encoded =                RuntimeCall::Assets(pallet_assets::pallet::Call::create {
        //                     id: 1,
        //                     admin: MultiAddress::Id(alice_account.clone().into()),
        //                     min_balance: 1,
        //                 })
        //   use subxt::SubstrateConfig; use subxt::OnlineClient;
        //   let westmint_api = OnlineClient::<SubstrateConfig>::from_url("ws://localhost:9910").await.unwrap();

        crate::westmint::tx()
            .utility()
            .batch(vec![RuntimeCall::ForeignAssets(
                pallet_assets::pallet::Call2::create {
                    id: multilocation::MultiLocation {
                        parents: 1,
                        interior: junctions::Junctions::Here,
                    },
                    admin: MultiAddress::Id(alice_account.clone().into()),
                    min_balance: 1,
                },
            )])

        //   let payload = crate::westmint::tx().assets().create(1,MultiAddress::Id(alice_account.clone().into()) ,1);
        //   payload

        // RuntimeCall::AssetConversion(pallet_asset_conversion::pallet::Call::create_pool {
        //     asset1: multilocation::MultiLocation {
        //         parents: 0,
        //         interior: junctions::Junctions::Here,
        //     },
        //     asset2: multilocation::MultiLocation {
        //         parents: 0,
        //         interior: junctions::Junctions::Here,
        //     },
        // })
        // RuntimeCall::System(frame_system::pallet::Call::remark_with_event {
        //     remark: "this is not a test.".as_bytes().to_vec(),
        // }),

        //     ],
        // })
        // .encode(); //.encode()

        // panic!("0x{}",hex::encode(encoded));

        // encoded
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    // Create a client to use:
    let westmint_api = OnlineClient::<SubstrateConfig>::from_url("ws://localhost:9910").await?;
    let westend_api =
        OnlineClient::<subxt::PolkadotConfig>::from_url("ws://localhost:9944").await?;

    let alice_pub = AccountKeyring::Alice.public();
    let alice_account = Sr25519Keyring::from_public(&alice_pub)
        .unwrap()
        .to_account_id();

    // let tx = westend::tx().system().remark_with_event(vec![1,2,3]);

    let tx = westend::tx().sudo().sudo(westend_context::ask_westmint_to(
        westmint_api
            .tx()
            .call_data(&westmint_context::asset_create())?,
    ));
    let _hash = westend_api
        .tx()
        .sign_and_submit_default(&tx, &signer)
        .await?;

    let tx = westmint::tx()
        .assets()
        .create(1, MultiAddress::Id(alice_account.clone().into()), 1);
    let _hash = westmint_api
        .tx()
        .sign_and_submit_default(&tx, &signer)
        .await?;
    println!("asset create hash: {:?}", _hash);

    let tx = westmint::tx()
        .assets()
        .mint(1, alice_account.clone().into(), 100_000_000_000_000);
    let _hash = westmint_api
        .tx()
        .sign_and_submit_default(&tx, &signer)
        .await?;
    println!("asset mint hash: {:?}", _hash);

    let dot = MultiLocation {
        parents: 0,
        interior: Here,
    };
    let asset1 = MultiLocation {
        parents: 0,
        interior: X2(Junction::PalletInstance(50), Junction::GeneralIndex(1)),
    };
    // let _asset66 = MultiLocation {
    //     parents: 0,
    //     interior: X2(Junction::PalletInstance(50), Junction::GeneralIndex(66)),
    // };
    let tx = westmint::tx()
        .asset_conversion()
        .create_pool(dot.clone(), asset1.clone());
    let _hash = westmint_api
        .tx()
        .sign_and_submit_default(&tx, &signer)
        .await?;
    println!("create pool hash: {:?}", _hash);

    let tx = westmint::tx().asset_conversion().add_liquidity(
        dot.clone(),
        asset1.clone(),
        10_000_000_000,
        100_000_000_000,
        1_000_000_000,
        1_000_000_000,
        alice_account.clone().into(),
    );
    let _hash = westmint_api
        .tx()
        .sign_and_submit_default(&tx, &signer)
        .await?;
    println!("add liquidity hash: {:?}", _hash);

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

    println!("done.");

    Ok(())
}
