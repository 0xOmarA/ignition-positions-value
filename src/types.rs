use radix_engine_interface::prelude::*;

#[derive(ScryptoSbor, Clone, Debug, PartialEq, Eq)]
pub struct LiquidityReceipt<T>
where
    T: ScryptoSbor,
{
    pub name: String,
    pub lockup_period: String,
    pub pool_address: ComponentAddress,
    pub user_resource_address: ResourceAddress,
    pub user_contribution_amount: Decimal,
    pub user_resource_volatility_classification: Volatility,
    pub protocol_contribution_amount: Decimal,
    pub maturity_date: Instant,
    pub adapter_specific_information: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ScryptoSbor, ManifestSbor)]
pub enum Volatility {
    Volatile,
    NonVolatile,
}

#[derive(Clone, Debug, ScryptoSbor, PartialEq, Eq)]
#[sbor(transparent)]
pub struct AnyValue((ScryptoValue,));

#[allow(dead_code)]
impl AnyValue {
    pub fn from_typed<T>(typed: &T) -> Result<Self, AnyValueError>
    where
        T: ScryptoEncode,
    {
        scrypto_encode(typed)
            .map_err(Into::into)
            .and_then(|value| scrypto_decode(&value).map_err(Into::into))
            .map(|value| Self((value,)))
    }

    pub fn as_typed<T>(&self) -> Result<T, AnyValueError>
    where
        T: ScryptoDecode,
    {
        scrypto_encode(&self.0 .0)
            .map_err(Into::into)
            .and_then(|value| scrypto_decode(&value).map_err(Into::into))
    }
}

#[derive(Clone, Debug)]
pub enum AnyValueError {
    EncodeError(EncodeError),
    DecodeError(DecodeError),
}

impl From<EncodeError> for AnyValueError {
    fn from(value: EncodeError) -> Self {
        Self::EncodeError(value)
    }
}

impl From<DecodeError> for AnyValueError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

#[derive(Debug, ScryptoSbor)]
pub struct CloseLiquidityPositionOutput {
    /* Private fields - these contain buckets which we can't get anything useful
    out of after a transaction is executed */
    resources: (IndexMap<ResourceAddress, Bucket>,),
    others: Vec<Bucket>,
    /* Public fields */
    pub fees: IndexMap<ResourceAddress, Decimal>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ScryptoSbor)]
pub struct Price {
    pub base: ResourceAddress,
    pub quote: ResourceAddress,
    pub price: Decimal,
}

impl Price {
    pub fn exchange(
        &self,
        resource_address: ResourceAddress,
        amount: Decimal,
    ) -> Option<(ResourceAddress, Decimal)> {
        if resource_address == self.base {
            Some((self.quote, self.price.checked_mul(amount)?))
        } else if resource_address == self.quote {
            Some((self.base, amount.checked_div(self.price)?))
        } else {
            None
        }
    }
}
