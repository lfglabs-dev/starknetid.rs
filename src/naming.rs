use starknet::core::types::Felt;

#[derive(Debug)]
pub enum ResolvingError {
    ConnectionError(String),
    InvalidContractResult,
    InvalidDomain,
    NotSupported,
}

pub const SELECTOR_D2A: Felt = Felt::from_raw([
    261254206219932239,
    14839945581867836860,
    5414334324946440161,
    6985039847805449502,
]);

pub const MAINNET_CONTRACT: Felt = Felt::from_raw([
    327799339589885214,
    9525933456780166611,
    16204762974907305178,
    9876522541644636344,
]);

pub const GOERLI_CONTRACT: Felt = Felt::from_raw([
    452192057203262238,
    4558680749370441117,
    1453192132188820719,
    3991710935722461676,
]);

pub const SEPOLIA_CONTRACT: Felt = Felt::from_raw([
    212585471442387825,
    4554984079195314985,
    15656365478474140601,
    9545268995402309891,
]);

pub const SELECTOR_A2D: Felt = Felt::from_raw([
    40507466578104802,
    6971481136651747063,
    1760183467521543892,
    14453853710431432356,
]);
