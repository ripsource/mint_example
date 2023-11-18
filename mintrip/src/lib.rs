use scrypto::prelude::*;

#[blueprint]
mod bootstrap {

    // This is a struct holds our resource manager we'll use to crank out the NFTs.
    struct Bootstrap {
        rippy_manager: ResourceManager,
        resource_addr: ResourceAddress,
    }

    impl Bootstrap {
        // Creates a number of NFT collections used for testing of the NFT marketplace blueprints.
        pub fn bootstrap() -> (Global<Bootstrap>, NonFungibleBucket) {
            let (address_reservation, _component_address) =
                Runtime::allocate_component_address(Bootstrap::blueprint_id());

            // This mints an admin badge that means only the holder can mint NFTs/update metadata/lock minting etc.
            let admin = ResourceBuilder::new_string_non_fungible::<Riplord>(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "Rip Lord".to_owned(), locked;
                        "icon_url" => Url::of("https://rippyclip.com/rippy.webp"), locked;
                        "tags" => vec!["badge".to_string()], locked;
                    }
                ))
                .mint_initial_supply([(
                    "Rippy_Lord".try_into().unwrap(),
                    Riplord {
                        name: "Power of the Rippy".to_owned(),
                    },
                )]);

            // Creating the resource manager used for the NFT -> includes collection level metadata and
            // sets the rules for miniting/updating metadata/locking minting etc.

            // There are potentially some other fields that we could add here relating to royalties
            // On Rippys I decided not to have royalties so I didn't bother to add them.
            let nfts = ResourceBuilder::new_string_non_fungible::<NFT>(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "Rippy Clip".to_owned(), locked;
                        "description" => "The Unofficial Radix Assistant".to_owned(), locked;
                        "creator" => "ripsource", locked;
                        "symbol" => "RIP".to_owned(), locked;
                        "icon_url" => Url::of("https://rippyclip.com/rippy.webp"), locked;
                        "tags" => vec!["Collectible".to_string()], locked;
                        "info_url" => Url::of("https://rippyclip.com"), locked;
                    }
                ))
                .mint_roles(mint_roles!(
                   minter => rule!(require(admin.resource_address()));
                   minter_updater => rule!(require(admin.resource_address()));
                ))
                .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                    non_fungible_data_updater => rule!(require(admin.resource_address()));
                    non_fungible_data_updater_updater => rule!(require(admin.resource_address()));
                ))
                .create_with_no_initial_supply();

            let component = Self {
                rippy_manager: nfts,
                resource_addr: nfts.address(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize();

            (component, admin)
        }

        /// This function is used after minting in order to lock the minting of new NFTs.

        pub fn lock_mint(&mut self, badge: NonFungibleBucket) -> NonFungibleBucket {
            badge.authorize_with_all(|| self.rippy_manager.set_mintable(AccessRule::DenyAll));
            badge.authorize_with_all(|| {
                self.rippy_manager.lock_mintable();
            });

            badge
        }

        /// This function is used to mint a new NFT. It takes in a vector of strings which are the metadata for the NFT.

        pub fn minty(&mut self, metadata: Vec<String>) -> Bucket {
            let background = metadata[0].clone();
            let body = metadata[1].clone();
            let brow = metadata[2].clone();
            let ear = metadata[3].clone();
            let eyes = metadata[4].clone();
            let ground = metadata[5].clone();
            let tail = metadata[6].clone();
            let rank = metadata[7].clone();
            let key_image = metadata[8].clone();
            let image_store = metadata[9].clone();
            let format_number = metadata[10].clone();

            let backgroundmap = background.clone();
            let bodymap = body.clone();
            let browmap = brow.clone();
            let earmap = ear.clone();
            let eyesmap = eyes.clone();
            let groundmap = ground.clone();
            let tailmap = tail.clone();
            let rankmap = rank.clone();

            let map: HashMap<String, String> = vec![
                ("Background".to_string(), backgroundmap),
                ("Body".to_string(), bodymap),
                ("Brow".to_string(), browmap),
                ("Ear".to_string(), earmap),
                ("Eyes".to_string(), eyesmap),
                ("Ground".to_string(), groundmap),
                ("Tail".to_string(), tailmap),
                ("Rank".to_string(), rankmap),
            ]
            .iter()
            .cloned()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

            let title = "#".to_string() + &format_number;

            let fill = NFT {
                name: title,
                description: "The Unofficial Radix Assistant".to_owned(),
                key_image_url: Url::of(key_image),
                attributes: map,
                background,
                body,
                brow,
                ear,
                eyes,
                ground,
                tail,
                rank,
                ipfs: image_store,
            };

            let mut rip_str = "Rippy_".to_string();
            rip_str.push_str(&format_number);
            let non_fun_id = NonFungibleLocalId::String(rip_str.try_into().unwrap());

            let rippy = self.rippy_manager.mint_non_fungible(&non_fun_id, fill);

            rippy
        }
    }
}

// This is the struct for the NFTs which we are minting. It contains all of the metadata for the NFTs.
// The indiviudal string fields are redudant, however I use them so that traits show up in the Radix Wallet.

#[derive(NonFungibleData, ScryptoSbor)]
struct NFT {
    name: String,
    description: String,
    // I currently make my key_image_url mutable. This is the image that shows in the wallet and currently
    // the wallet doesn't show IPFS images - and is counterproductive to use a ipfs http link I think.
    // I used a CDN/website link from my website, but I maye update to arweave potentially.
    // Having the #[mutable] attribute above a field is used to make the field mutable, the rest are immutable still.
    #[mutable]
    key_image_url: Url,
    attributes: HashMap<String, String>,
    background: String,
    body: String,
    brow: String,
    ear: String,
    eyes: String,
    ground: String,
    tail: String,
    rank: String,
    ipfs: String,
}

#[derive(NonFungibleData, ScryptoSbor)]
struct Riplord {
    name: String,
}
