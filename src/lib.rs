use alkahest_rs::{
    clients::{attestation, erc1155, erc20, erc721, token_bundle},
    sol_types::EscrowClaimed,
};
use alloy::primitives::{Address, FixedBytes};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass]
#[derive(Clone)]
pub struct AlkahestClient {
    inner: alkahest_rs::AlkahestClient,
    pub erc_20: Erc20Client,
    pub erc_721: Erc721Client,
    pub erc_1155: Erc1155Client,
    pub token_bundle: TokenBundleClient,
    pub attestation: AttestationClient,
}

#[pyclass]
#[derive(Clone)]
pub struct Erc20Client {
    inner: erc20::Erc20Client,
}

#[pyclass]
#[derive(Clone)]
pub struct Erc721Client {
    inner: erc721::Erc721Client,
}

#[pyclass]
#[derive(Clone)]
pub struct Erc1155Client {
    inner: erc1155::Erc1155Client,
}

#[pyclass]
#[derive(Clone)]
pub struct TokenBundleClient {
    inner: token_bundle::TokenBundleClient,
}

#[pyclass]
#[derive(Clone)]
pub struct AttestationClient {
    inner: attestation::AttestationClient,
}

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

#[pymethods]
impl AlkahestClient {
    #[new]
    #[pyo3(signature = (private_key, rpc_url, address_config=None))]
    pub fn new(
        private_key: String,
        rpc_url: String,
        address_config: Option<AddressConfig>,
    ) -> PyResult<Self> {
        let address_config = address_config.map(|x| x.try_into()).transpose()?;
        let client = alkahest_rs::AlkahestClient::new(private_key, rpc_url, address_config)?;

        let client = Self {
            inner: client.clone(),
            erc_20: Erc20Client {
                inner: client.erc20,
            },
            erc_721: Erc721Client {
                inner: client.erc721,
            },
            erc_1155: Erc1155Client {
                inner: client.erc1155,
            },
            token_bundle: TokenBundleClient {
                inner: client.token_bundle,
            },
            attestation: AttestationClient {
                inner: client.attestation,
            },
        };

        Ok(client)
    }

    #[pyo3(signature = (contract_address, buy_attestation, from_block=None))]
    pub async fn wait_for_fulfillment(
        &self,
        contract_address: String,
        buy_attestation: String,
        from_block: Option<u64>,
    ) -> eyre::Result<EscowClaimedLog> {
        let contract_address: Address = contract_address.parse()?;
        let buy_attestation: FixedBytes<32> = buy_attestation.parse()?;
        let res = self
            .inner
            .wait_for_fulfillment(contract_address, buy_attestation, from_block)
            .await?;
        Ok(res.data.into())
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

#[derive(FromPyObject)]
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
pub struct Erc721Data {
    address: String,
    id: u128,
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

#[pymethods]
impl Erc20Client {
    pub async fn approve(&self, token: Erc20Data, purpose: String) -> eyre::Result<String> {
        let purpose = match purpose.as_str() {
            "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
            "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
            _ => return Err(eyre::eyre!("Invalid purpose")),
        };
        let receipt = self.inner.approve(token.try_into()?, purpose).await?;

        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn approve_if_less(
        &self,
        token: Erc20Data,
        purpose: String,
    ) -> eyre::Result<Option<String>> {
        let purpose = match purpose.as_str() {
            "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
            "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
            _ => return Err(eyre::eyre!("Invalid purpose")),
        };
        let receipt = self
            .inner
            .approve_if_less(token.try_into()?, purpose)
            .await?;

        Ok(receipt.map(|x| x.transaction_hash.to_string()))
    }

    pub async fn collect_payment(
        &self,
        buy_attestation: String,
        fulfillment: String,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .collect_payment(buy_attestation.parse()?, fulfillment.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn collect_expired(&self, buy_attestation: String) -> eyre::Result<String> {
        let receipt = self.inner.collect_expired(buy_attestation.parse()?).await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_with_erc20(
        &self,
        price: Erc20Data,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_with_erc20(price.try_into()?, item.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn permit_and_buy_with_erc20(
        &self,
        price: Erc20Data,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .permit_and_buy_with_erc20(price.try_into()?, item.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn pay_with_erc20(&self, price: Erc20Data, payee: String) -> eyre::Result<String> {
        let receipt = self
            .inner
            .pay_with_erc20(price.try_into()?, payee.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn permit_and_pay_with_erc20(
        &self,
        price: Erc20Data,
        payee: String,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .permit_and_pay_with_erc20(price.try_into()?, payee.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_erc20_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc20Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_erc20_for_erc20(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn permit_and_buy_erc20_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc20Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .permit_and_buy_erc20_for_erc20(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn pay_erc20_for_erc20(&self, buy_attestation: String) -> eyre::Result<String> {
        let receipt = self
            .inner
            .pay_erc20_for_erc20(buy_attestation.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn permit_and_pay_erc20_for_erc20(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .permit_and_pay_erc20_for_erc20(buy_attestation.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_erc721_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc721Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_erc721_for_erc20(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn permit_and_buy_erc721_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc721Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .permit_and_buy_erc721_for_erc20(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_erc1155_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc1155Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_erc1155_for_erc20(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn permit_and_buy_erc1155_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc1155Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .permit_and_buy_erc1155_for_erc20(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_bundle_for_erc20(
        &self,
        bid: Erc20Data,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_bundle_for_erc20(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn permit_and_buy_bundle_for_erc20(
        &self,
        bid: Erc20Data,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .permit_and_buy_bundle_for_erc20(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }
}

#[pymethods]
impl Erc721Client {
    pub async fn approve(&self, token: Erc721Data, purpose: String) -> eyre::Result<String> {
        let purpose = match purpose.as_str() {
            "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
            "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
            _ => return Err(eyre::eyre!("Invalid purpose")),
        };
        let receipt = self.inner.approve(token.try_into()?, purpose).await?;

        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn approve_all(
        &self,
        token_contract: String,
        purpose: String,
    ) -> eyre::Result<String> {
        let token_contract: Address = token_contract.parse()?;
        let purpose = match purpose.as_str() {
            "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
            "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
            _ => return Err(eyre::eyre!("Invalid purpose")),
        };
        let receipt = self.inner.approve_all(token_contract, purpose).await?;

        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn revoke_all(
        &self,
        token_contract: String,
        purpose: String,
    ) -> eyre::Result<String> {
        let token_contract: Address = token_contract.parse()?;
        let purpose = match purpose.as_str() {
            "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
            "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
            _ => return Err(eyre::eyre!("Invalid purpose")),
        };
        let receipt = self.inner.revoke_all(token_contract, purpose).await?;

        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn collect_payment(
        &self,
        buy_attestation: String,
        fulfillment: String,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .collect_payment(buy_attestation.parse()?, fulfillment.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn collect_expired(&self, buy_attestation: String) -> eyre::Result<String> {
        let receipt = self.inner.collect_expired(buy_attestation.parse()?).await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_with_erc721(
        &self,
        price: Erc721Data,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_with_erc721(price.try_into()?, item.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn pay_with_erc721(&self, price: Erc721Data, payee: String) -> eyre::Result<String> {
        let receipt = self
            .inner
            .pay_with_erc721(price.try_into()?, payee.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_erc_721_for_erc_721(
        &self,
        bid: Erc721Data,
        ask: Erc721Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_erc_721_for_erc_721(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn pay_erc_721_for_erc_721(&self, buy_attestation: String) -> eyre::Result<String> {
        let receipt = self
            .inner
            .pay_erc_721_for_erc_721(buy_attestation.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_erc20_with_erc721(
        &self,
        bid: Erc721Data,
        ask: Erc20Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_erc20_with_erc721(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_erc1155_with_erc721(
        &self,
        bid: Erc721Data,
        ask: Erc1155Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_erc1155_with_erc721(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_bundle_with_erc721(
        &self,
        bid: Erc721Data,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_bundle_with_erc721(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }
}

#[pymethods]
impl Erc1155Client {
    pub async fn approve_all(
        &self,
        token_contract: String,
        purpose: String,
    ) -> eyre::Result<String> {
        let token_contract: Address = token_contract.parse()?;
        let purpose = match purpose.as_str() {
            "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
            "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
            _ => return Err(eyre::eyre!("Invalid purpose")),
        };
        let receipt = self.inner.approve_all(token_contract, purpose).await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn revoke_all(
        &self,
        token_contract: String,
        purpose: String,
    ) -> eyre::Result<String> {
        let token_contract: Address = token_contract.parse()?;
        let purpose = match purpose.as_str() {
            "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
            "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
            _ => return Err(eyre::eyre!("Invalid purpose")),
        };
        let receipt = self.inner.revoke_all(token_contract, purpose).await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn collect_payment(
        &self,
        buy_attestation: String,
        fulfillment: String,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .collect_payment(buy_attestation.parse()?, fulfillment.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn collect_expired(&self, buy_attestation: String) -> eyre::Result<String> {
        let receipt = self.inner.collect_expired(buy_attestation.parse()?).await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_with_erc1155(
        &self,
        price: Erc1155Data,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_with_erc1155(price.try_into()?, item.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn pay_with_erc_1155(
        &self,
        price: Erc1155Data,
        payee: String,
    ) -> eyre::Result<String> {
        let payee: Address = payee.parse()?;
        let receipt = self
            .inner
            .pay_with_erc1155(price.try_into()?, payee)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_erc1155_for_erc1155(
        &self,
        bid: Erc1155Data,
        ask: Erc1155Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_erc1155_for_erc1155(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn pay_erc1155_for_erc1155(&self, buy_attestation: String) -> eyre::Result<String> {
        let receipt = self
            .inner
            .pay_erc1155_for_erc1155(buy_attestation.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_erc20_with_erc1155(
        &self,
        bid: Erc1155Data,
        ask: Erc20Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_erc20_with_erc1155(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_erc721_with_erc1155(
        &self,
        bid: Erc1155Data,
        ask: Erc721Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_erc721_with_erc1155(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_bundle_with_erc1155(
        &self,
        bid: Erc1155Data,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_bundle_with_erc1155(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }
}

#[pymethods]
impl TokenBundleClient {
    pub async fn collect_payment(
        &self,
        buy_attestation: String,
        fulfillment: String,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .collect_payment(buy_attestation.parse()?, fulfillment.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn collect_expired(&self, buy_attestation: String) -> eyre::Result<String> {
        let receipt = self.inner.collect_expired(buy_attestation.parse()?).await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_with_bundle(
        &self,
        price: TokenBundleData,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_with_bundle(price.try_into()?, item.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn pay_with_bundle(
        &self,
        price: TokenBundleData,
        payee: String,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .pay_with_bundle(price.try_into()?, payee.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn buy_bundle_for_bundle(
        &self,
        bid: TokenBundleData,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .buy_bundle_for_bundle(bid.try_into()?, ask.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn pay_bundle_for_bundle(&self, buy_attestation: String) -> eyre::Result<String> {
        let receipt = self
            .inner
            .pay_bundle_for_bundle(buy_attestation.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
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

#[pymethods]
impl AttestationClient {
    pub async fn register_schema(
        &self,
        schema: String,
        resolver: String,
        revocable: bool,
    ) -> eyre::Result<String> {
        let schema: FixedBytes<32> = schema.parse()?;
        let resolver: Address = resolver.parse()?;
        let receipt = self
            .inner
            .register_schema(schema.to_string(), resolver, revocable)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn attest(&self, attestation: AttestationRequest) -> eyre::Result<String> {
        let receipt = self.inner.attest(attestation.try_into()?).await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn collect_payment(
        &self,
        buy_attestation: String,
        fulfillment: String,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .collect_payment(buy_attestation.parse()?, fulfillment.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn collect_payment_2(
        &self,
        buy_attestation: String,
        fulfillment: String,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .collect_payment_2(buy_attestation.parse()?, fulfillment.parse()?)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn create_escrow(
        &self,
        attestation: AttestationRequest,
        demand: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .create_escrow(attestation.try_into()?, demand.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn create_escrow_2(
        &self,
        attestation: String,
        demand: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .create_escrow_2(attestation.parse()?, demand.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }

    pub async fn attest_and_create_escrow(
        &self,
        attestation: AttestationRequest,
        demand: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        let receipt = self
            .inner
            .attest_and_create_escrow(attestation.try_into()?, demand.try_into()?, expiration)
            .await?;
        Ok(receipt.transaction_hash.to_string())
    }
}
