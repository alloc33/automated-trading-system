use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{
    error::BoxDynError,
    postgres::{PgTypeInfo, PgValueRef},
    Decode, Postgres, Type,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Price(#[serde(with = "rust_decimal::serde::arbitrary_precision")] Decimal);

impl Default for Price {
    fn default() -> Self {
        Self(Decimal::ZERO)
    }
}

impl Price {
    pub fn new(d: Decimal) -> Self {
        Self(d)
    }
}

impl AsRef<Decimal> for Price {
    fn as_ref(&self) -> &Decimal {
        &self.0
    }
}

impl Type<Postgres> for Price {
    fn type_info() -> PgTypeInfo {
        Decimal::type_info()
    }
}

impl<'r> Decode<'r, Postgres> for Price {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Decimal::decode(value).map(Self::new)
    }
}
