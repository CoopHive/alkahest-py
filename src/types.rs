use alkahest_rs::{contracts::IEAS::Attested, sol_types::EscrowClaimed};
use alloy::primitives::{FixedBytes, U256};
use pyo3::{exceptions::PyValueError, pyclass, FromPyObject, IntoPyObject, PyErr, PyResult};

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
pub struct ArbitersAddresses {
    pub eas: String,
    pub trusted_party_arbiter: String,
    pub trivial_arbiter: String,
    pub specific_attestation_arbiter: String,
    pub trusted_oracle_arbiter: String,
    pub intrinsics_arbiter: String,
    pub intrinsics_arbiter_2: String,
    pub any_arbiter: String,
    pub all_arbiter: String,
    pub uid_arbiter: String,
    pub recipient_arbiter: String,
}

#[derive(FromPyObject)]
pub struct StringObligationAddresses {
    pub eas: String,
    pub obligation: String,
}

// Implement TryFrom for StringObligationAddresses
impl TryFrom<StringObligationAddresses>
    for alkahest_rs::clients::string_obligation::StringObligationAddresses
{
    type Error = PyErr;

    fn try_from(value: StringObligationAddresses) -> PyResult<Self> {
        Ok(Self {
            eas: value
                .eas
                .parse()
                .map_err(|_| PyValueError::new_err("invalid address"))?,
            obligation: value
                .obligation
                .parse()
                .map_err(|_| PyValueError::new_err("invalid address"))?,
        })
    }
}

#[derive(FromPyObject)]
pub struct AddressConfig {
    pub erc20_addresses: Option<Erc20Addresses>,
    pub erc721_addresses: Option<Erc721Addresses>,
    pub erc1155_addresses: Option<Erc1155Addresses>,
    pub token_bundle_addresses: Option<TokenBundleAddresses>,
    pub attestation_addresses: Option<AttestationAddresses>,
    pub arbiters_addresses: Option<ArbitersAddresses>,
    pub string_obligation_addresses: Option<StringObligationAddresses>,
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
            arbiters_addresses: value.arbiters_addresses.and_then(|x| x.try_into().ok()),
            string_obligation_addresses: value
                .string_obligation_addresses
                .and_then(|x| x.try_into().ok()),
        })
    }
}

// Implement TryFrom for ArbitersAddresses
impl TryFrom<ArbitersAddresses> for alkahest_rs::clients::arbiters::ArbitersAddresses {
    type Error = PyErr;

    fn try_from(value: ArbitersAddresses) -> PyResult<Self> {
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
            trusted_party_arbiter: parse_address!(trusted_party_arbiter),
            trivial_arbiter: parse_address!(trivial_arbiter),
            specific_attestation_arbiter: parse_address!(specific_attestation_arbiter),
            trusted_oracle_arbiter: parse_address!(trusted_oracle_arbiter),
            intrinsics_arbiter: parse_address!(intrinsics_arbiter),
            intrinsics_arbiter_2: parse_address!(intrinsics_arbiter_2),
            any_arbiter: parse_address!(any_arbiter),
            all_arbiter: parse_address!(all_arbiter),
            uid_arbiter: parse_address!(uid_arbiter),
            recipient_arbiter: parse_address!(recipient_arbiter),
        })
    }
}

#[derive(FromPyObject)]
#[pyo3(from_item_all)]
pub struct ArbiterData {
    pub arbiter: String,
    pub demand: Vec<u8>,
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
    pub address: String,
    pub value: u64,
}

impl TryFrom<Erc20Data> for alkahest_rs::types::Erc20Data {
    type Error = eyre::Error;

    fn try_from(value: Erc20Data) -> eyre::Result<Self> {
        Ok(Self {
            address: value.address.parse()?,
            value: U256::from(value.value),
        })
    }
}

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct PyErc20Data {
    #[pyo3(get)]
    pub address: String,

    #[pyo3(get)]
    pub value: u64,
}

#[pymethods]
impl PyErc20Data {
    #[new]
    pub fn new(address: String, value: u64) -> Self {
        Self { address, value }
    }
}

impl TryFrom<PyErc20Data> for alkahest_rs::types::Erc20Data {
    type Error = eyre::Error;

    fn try_from(value: PyErc20Data) -> eyre::Result<Self> {
        Ok(Self {
            address: value.address.parse()?,
            value: U256::from(value.value),
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

#[pyclass]
#[derive(Clone)]
pub struct PyAddressConfig {
    #[pyo3(get)]
    pub erc20_addresses: Option<PyErc20Addresses>,
    #[pyo3(get)]
    pub erc721_addresses: Option<PyErc721Addresses>,
    #[pyo3(get)]
    pub erc1155_addresses: Option<PyErc1155Addresses>,
    #[pyo3(get)]
    pub token_bundle_addresses: Option<PyTokenBundleAddresses>,
    #[pyo3(get)]
    pub attestation_addresses: Option<PyAttestationAddresses>,
    #[pyo3(get)]
    pub arbiters_addresses: Option<PyArbitersAddresses>,
    #[pyo3(get)]
    pub string_obligation_addresses: Option<PyStringObligationAddresses>,
}

impl From<&alkahest_rs::AddressConfig> for PyAddressConfig {
    fn from(data: &alkahest_rs::AddressConfig) -> Self {
        Self {
            erc20_addresses: data.erc20_addresses.as_ref().map(PyErc20Addresses::from),
            erc721_addresses: data.erc721_addresses.as_ref().map(PyErc721Addresses::from),
            erc1155_addresses: data
                .erc1155_addresses
                .as_ref()
                .map(PyErc1155Addresses::from),
            token_bundle_addresses: data
                .token_bundle_addresses
                .as_ref()
                .map(PyTokenBundleAddresses::from),
            attestation_addresses: data
                .attestation_addresses
                .as_ref()
                .map(PyAttestationAddresses::from),
            arbiters_addresses: data
                .arbiters_addresses
                .as_ref()
                .map(PyArbitersAddresses::from),
            string_obligation_addresses: data
                .string_obligation_addresses
                .as_ref()
                .map(PyStringObligationAddresses::from),
        }
    }
}

macro_rules! py_address_struct {
    ($name:ident, $src:path) => {
        #[pyclass]
        #[derive(Clone)]
        pub struct $name {
            #[pyo3(get)]
            pub eas: String,
            #[pyo3(get)]
            pub barter_utils: String,
            #[pyo3(get)]
            pub escrow_obligation: String,
            #[pyo3(get)]
            pub payment_obligation: String,
        }

        impl From<&$src> for $name {
            fn from(data: &$src) -> Self {
                Self {
                    eas: format!("{:?}", data.eas),
                    barter_utils: format!("{:?}", data.barter_utils),
                    escrow_obligation: format!("{:?}", data.escrow_obligation),
                    payment_obligation: format!("{:?}", data.payment_obligation),
                }
            }
        }
    };
}

py_address_struct!(
    PyErc20Addresses,
    alkahest_rs::clients::erc20::Erc20Addresses
);
py_address_struct!(
    PyErc721Addresses,
    alkahest_rs::clients::erc721::Erc721Addresses
);
py_address_struct!(
    PyErc1155Addresses,
    alkahest_rs::clients::erc1155::Erc1155Addresses
);
py_address_struct!(
    PyTokenBundleAddresses,
    alkahest_rs::clients::token_bundle::TokenBundleAddresses
);

#[pyclass]
#[derive(Clone)]
pub struct PyAttestationAddresses {
    #[pyo3(get)]
    pub eas: String,
    #[pyo3(get)]
    pub eas_schema_registry: String,
    #[pyo3(get)]
    pub barter_utils: String,
    #[pyo3(get)]
    pub escrow_obligation: String,
    #[pyo3(get)]
    pub escrow_obligation_2: String,
}
impl From<&alkahest_rs::clients::attestation::AttestationAddresses> for PyAttestationAddresses {
    fn from(data: &alkahest_rs::clients::attestation::AttestationAddresses) -> Self {
        Self {
            eas: format!("{:?}", data.eas),
            eas_schema_registry: format!("{:?}", data.eas_schema_registry),
            barter_utils: format!("{:?}", data.barter_utils),
            escrow_obligation: format!("{:?}", data.escrow_obligation),
            escrow_obligation_2: format!("{:?}", data.escrow_obligation_2),
        }
    }
}
#[pyclass]
#[derive(Clone)]
pub struct PyArbitersAddresses {
    #[pyo3(get)]
    pub eas: String,
    #[pyo3(get)]
    pub specific_attestation_arbiter: String,
    #[pyo3(get)]
    pub trivial_arbiter: String,
    #[pyo3(get)]
    pub trusted_oracle_arbiter: String,
    #[pyo3(get)]
    pub trusted_party_arbiter: String,
    #[pyo3(get)]
    pub intrinsics_arbiter: String,
    #[pyo3(get)]
    pub intrinsics_arbiter_2: String,
    #[pyo3(get)]
    pub any_arbiter: String,
    #[pyo3(get)]
    pub all_arbiter: String,
    #[pyo3(get)]
    pub uid_arbiter: String,
    #[pyo3(get)]
    pub recipient_arbiter: String,
}

impl From<&alkahest_rs::clients::arbiters::ArbitersAddresses> for PyArbitersAddresses {
    fn from(data: &alkahest_rs::clients::arbiters::ArbitersAddresses) -> Self {
        Self {
            eas: format!("{:?}", data.eas),
            specific_attestation_arbiter: format!("{:?}", data.specific_attestation_arbiter),
            trivial_arbiter: format!("{:?}", data.trivial_arbiter),
            trusted_oracle_arbiter: format!("{:?}", data.trusted_oracle_arbiter),
            trusted_party_arbiter: format!("{:?}", data.trusted_party_arbiter),
            intrinsics_arbiter: format!("{:?}", data.intrinsics_arbiter),
            intrinsics_arbiter_2: format!("{:?}", data.intrinsics_arbiter_2),
            any_arbiter: format!("{:?}", data.any_arbiter),
            all_arbiter: format!("{:?}", data.all_arbiter),
            uid_arbiter: format!("{:?}", data.uid_arbiter),
            recipient_arbiter: format!("{:?}", data.recipient_arbiter),
        }
    }
}
#[pyclass]
#[derive(Clone)]
pub struct PyStringObligationAddresses {
    #[pyo3(get)]
    pub eas: String,
    #[pyo3(get)]
    pub obligation: String,
}

impl From<&alkahest_rs::clients::string_obligation::StringObligationAddresses>
    for PyStringObligationAddresses
{
    fn from(data: &alkahest_rs::clients::string_obligation::StringObligationAddresses) -> Self {
        Self {
            eas: format!("{:?}", data.eas),
            obligation: format!("{:?}", data.obligation),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyERC20EscrowObligation {
    #[pyo3(get)]
    pub token: String,
    #[pyo3(get)]
    pub amount: u64,
    #[pyo3(get)]
    pub arbiter: String,
    #[pyo3(get)]
    pub demand: Vec<u8>,
}

#[pymethods]
impl PyERC20EscrowObligation {
    #[new]
    pub fn new(token: String, amount: u64, arbiter: String, demand: Vec<u8>) -> Self {
        Self {
            token,
            amount,
            arbiter,
            demand,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "PyERC20EscrowObligation(token='{}', amount={}, arbiter='{}', demand={:?})",
            self.token, self.amount, self.arbiter, self.demand
        )
    }
}

impl From<alkahest_rs::contracts::ERC20EscrowObligation::StatementData>
    for PyERC20EscrowObligation
{
    fn from(data: alkahest_rs::contracts::ERC20EscrowObligation::StatementData) -> Self {
        Self {
            token: format!("{:?}", data.token),
            amount: data.amount.try_into().unwrap_or(0), // Handle potential overflow
            arbiter: format!("{:?}", data.arbiter),
            demand: data.demand.to_vec(),
        }
    }
}
