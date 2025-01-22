use alkahest_rs::{contracts::IEAS::Attested, sol_types::EscrowClaimed};
use alloy::primitives::FixedBytes;
use pyo3::{exceptions::PyValueError, FromPyObject, IntoPyObject, PyErr, PyResult};

macro_rules! client_address_config {
    ($name:ident) => {
        #[derive(FromPyObject)]
        pub struct $name {
            pub eas: String,
            pub barter_utils: String,
            pub escrow_obligation: String,
            pub payment_obligation: String,
        }
    };
}

client_address_config!(Erc20Addresses);
client_address_config!(Erc721Addresses);
client_address_config!(Erc1155Addresses);
client_address_config!(TokenBundleAddresses);

#[derive(FromPyObject)]
pub struct AttestationAddresses {
    pub eas: String,
    pub eas_schema_registry: String,
    pub barter_utils: String,
    pub escrow_obligation: String,
    pub escrow_obligation_2: String,
}

#[derive(FromPyObject)]
pub struct AddressConfig {
    pub erc20_addresses: Option<Erc20Addresses>,
    pub erc721_addresses: Option<Erc721Addresses>,
    pub erc1155_addresses: Option<Erc1155Addresses>,
    pub token_bundle_addresses: Option<TokenBundleAddresses>,
    pub attestation_addresses: Option<AttestationAddresses>,
}

macro_rules! try_from_address_config {
    ( $from:path, $to:path) => {
        impl TryFrom<$from> for $to {
            type Error = PyErr;

            fn try_from(value: $from) -> PyResult<Self> {
                macro_rules! parse_address {
                    ($name:ident) => {
                        value
                            .$name
                            .parse()
                            .map_err(|_| PyValueError::new_err("invalid address"))?
                    };
                }

                Ok(Self {
                    eas: parse_address!(eas),
                    barter_utils: parse_address!(barter_utils),
                    escrow_obligation: parse_address!(escrow_obligation),
                    payment_obligation: parse_address!(payment_obligation),
                })
            }
        }
    };
}

try_from_address_config!(Erc20Addresses, alkahest_rs::clients::erc20::Erc20Addresses);
try_from_address_config!(
    Erc721Addresses,
    alkahest_rs::clients::erc721::Erc721Addresses
);
try_from_address_config!(
    Erc1155Addresses,
    alkahest_rs::clients::erc1155::Erc1155Addresses
);
try_from_address_config!(
    TokenBundleAddresses,
    alkahest_rs::clients::token_bundle::TokenBundleAddresses
);

impl TryFrom<AttestationAddresses> for alkahest_rs::clients::attestation::AttestationAddresses {
    type Error = PyErr;

    fn try_from(value: AttestationAddresses) -> PyResult<Self> {
        macro_rules! parse_address {
            ($name:ident) => {
                value
                    .$name
                    .parse()
                    .map_err(|_| PyValueError::new_err("invalid address"))?
            };
        }

        Ok(Self {
            eas: parse_address!(eas),
            eas_schema_registry: parse_address!(eas_schema_registry),
            barter_utils: parse_address!(barter_utils),
            escrow_obligation: parse_address!(escrow_obligation),
            escrow_obligation_2: parse_address!(escrow_obligation_2),
        })
    }
}

impl TryFrom<AddressConfig> for alkahest_rs::AddressConfig {
    type Error = PyErr;

    fn try_from(value: AddressConfig) -> PyResult<Self> {
        Ok(Self {
            erc20_addresses: value.erc20_addresses.and_then(|x| x.try_into().ok()),
            erc721_addresses: value.erc721_addresses.and_then(|x| x.try_into().ok()),
            erc1155_addresses: value.erc1155_addresses.and_then(|x| x.try_into().ok()),
            token_bundle_addresses: value.token_bundle_addresses.and_then(|x| x.try_into().ok()),
            attestation_addresses: value.attestation_addresses.and_then(|x| x.try_into().ok()),
        })
    }
}

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct ArbiterData {
    arbiter: String,
    demand: Vec<u8>,
}

impl TryFrom<ArbiterData> for alkahest_rs::types::ArbiterData {
    type Error = eyre::Error;

    fn try_from(value: ArbiterData) -> eyre::Result<Self> {
        Ok(Self {
            arbiter: value.arbiter.parse()?,
            demand: value.demand.into(),
        })
    }
}

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct Erc20Data {
    address: String,
    value: u128,
}

impl TryFrom<Erc20Data> for alkahest_rs::types::Erc20Data {
    type Error = eyre::Error;

    fn try_from(value: Erc20Data) -> eyre::Result<Self> {
        Ok(Self {
            address: value.address.parse()?,
            value: value.value.try_into()?,
        })
    }
}

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct Erc721Data {
    pub address: String,
    pub id: u128,
}

impl TryFrom<Erc721Data> for alkahest_rs::types::Erc721Data {
    type Error = eyre::Error;

    fn try_from(value: Erc721Data) -> eyre::Result<Self> {
        Ok(Self {
            address: value.address.parse()?,
            id: value.id.try_into()?,
        })
    }
}

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct Erc1155Data {
    address: String,
    id: u128,
    value: u128,
}

impl TryFrom<Erc1155Data> for alkahest_rs::types::Erc1155Data {
    type Error = eyre::Error;

    fn try_from(value: Erc1155Data) -> eyre::Result<Self> {
        Ok(Self {
            address: value.address.parse()?,
            id: value.id.try_into()?,
            value: value.value.try_into()?,
        })
    }
}

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct TokenBundleData {
    erc20s: Vec<Erc20Data>,
    erc721s: Vec<Erc721Data>,
    erc1155s: Vec<Erc1155Data>,
}

impl TryFrom<TokenBundleData> for alkahest_rs::types::TokenBundleData {
    type Error = eyre::Error;

    fn try_from(value: TokenBundleData) -> eyre::Result<Self> {
        let erc20s = value
            .erc20s
            .into_iter()
            .map(|x| x.try_into())
            .collect::<eyre::Result<Vec<_>>>()?;
        let erc721s = value
            .erc721s
            .into_iter()
            .map(|x| x.try_into())
            .collect::<eyre::Result<Vec<_>>>()?;
        let erc1155s = value
            .erc1155s
            .into_iter()
            .map(|x| x.try_into())
            .collect::<eyre::Result<Vec<_>>>()?;

        Ok(Self {
            erc20s,
            erc721s,
            erc1155s,
        })
    }
}

#[derive(IntoPyObject)]
pub struct EscowClaimedLog {
    pub payment: String,
    pub fulfillment: String,
    pub fulfiller: String,
}

impl From<EscrowClaimed> for EscowClaimedLog {
    fn from(value: EscrowClaimed) -> Self {
        Self {
            payment: value.payment.to_string(),
            fulfillment: value.fulfillment.to_string(),
            fulfiller: value.fulfiller.to_string(),
        }
    }
}

#[derive(IntoPyObject)]
pub struct AttestedLog {
    pub recipient: String,
    pub attester: String,
    pub uid: String,
    pub schema_uid: String,
}

impl From<Attested> for AttestedLog {
    fn from(value: Attested) -> Self {
        Self {
            recipient: value.recipient.to_string(),
            attester: value.attester.to_string(),
            uid: value.uid.to_string(),
            schema_uid: value.schemaUID.to_string(),
        }
    }
}

#[derive(FromPyObject)]
pub struct AttestationRequestData {
    pub recipient: String,
    pub expiration_time: u64,
    pub revocable: bool,
    pub ref_uid: String,
    pub data: Vec<u8>,
    pub value: u128,
}

#[derive(FromPyObject)]
pub struct AttestationRequest {
    pub schema: String,
    pub data: AttestationRequestData,
}

impl TryFrom<AttestationRequestData> for alkahest_rs::contracts::IEAS::AttestationRequestData {
    type Error = eyre::Error;

    fn try_from(value: AttestationRequestData) -> eyre::Result<Self> {
        Ok(Self {
            recipient: value.recipient.parse()?,
            expirationTime: value.expiration_time,
            revocable: value.revocable,
            refUID: value.ref_uid.parse()?,
            data: value.data.into(),
            value: value.value.try_into()?,
        })
    }
}

impl TryFrom<AttestationRequest> for alkahest_rs::contracts::IEAS::AttestationRequest {
    type Error = eyre::Error;

    fn try_from(value: AttestationRequest) -> eyre::Result<Self> {
        let schema: FixedBytes<32> = value.schema.parse()?;
        Ok(Self {
            schema,
            data: value.data.try_into()?,
        })
    }
}

#[derive(IntoPyObject)]
pub struct LogWithHash<T> {
    pub log: T,
    pub transaction_hash: String,
}
