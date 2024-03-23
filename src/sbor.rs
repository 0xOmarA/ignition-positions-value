use radix_engine_interface::prelude::*;

pub fn manifest_value_from_scrypto_value<T: ScryptoEncode>(
    typed: &T,
) -> ManifestValue {
    let scrypto_value =
        scrypto_decode::<ScryptoValue>(&scrypto_encode(typed).unwrap())
            .unwrap();

    match scrypto_value {
        Value::Bool { value } => Value::Bool { value },
        Value::I8 { value } => Value::I8 { value },
        Value::I16 { value } => Value::I16 { value },
        Value::I32 { value } => Value::I32 { value },
        Value::I64 { value } => Value::I64 { value },
        Value::I128 { value } => Value::I128 { value },
        Value::U8 { value } => Value::U8 { value },
        Value::U16 { value } => Value::U16 { value },
        Value::U32 { value } => Value::U32 { value },
        Value::U64 { value } => Value::U64 { value },
        Value::U128 { value } => Value::U128 { value },
        Value::String { value } => Value::String { value },
        Value::Enum {
            discriminator,
            fields,
        } => Value::Enum {
            discriminator,
            fields: fields
                .iter()
                .map(manifest_value_from_scrypto_value)
                .collect(),
        },
        Value::Array {
            element_value_kind,
            elements,
        } => Value::Array {
            element_value_kind: manifest_value_kind_from_scrypto_value_kind(
                element_value_kind,
            ),
            elements: elements
                .iter()
                .map(manifest_value_from_scrypto_value)
                .collect(),
        },
        Value::Tuple { fields } => Value::Tuple {
            fields: fields
                .iter()
                .map(manifest_value_from_scrypto_value)
                .collect(),
        },
        Value::Map {
            key_value_kind,
            value_value_kind,
            entries,
        } => Value::Map {
            key_value_kind: manifest_value_kind_from_scrypto_value_kind(
                key_value_kind,
            ),
            value_value_kind: manifest_value_kind_from_scrypto_value_kind(
                value_value_kind,
            ),
            entries: entries
                .iter()
                .map(|(k, v)| {
                    (
                        manifest_value_from_scrypto_value(k),
                        manifest_value_from_scrypto_value(v),
                    )
                })
                .collect(),
        },
        Value::Custom {
            value: ScryptoCustomValue::Reference(reference),
        } => ManifestValue::Custom {
            value: ManifestCustomValue::Address(ManifestAddress::Static(
                *reference.as_node_id(),
            )),
        },
        Value::Custom {
            value: ScryptoCustomValue::Own(..),
        } => unreachable!(),
        Value::Custom {
            value: ScryptoCustomValue::Decimal(decimal),
        } => ManifestValue::Custom {
            value: ManifestCustomValue::Decimal(ManifestDecimal(
                decimal.0.to_le_bytes(),
            )),
        },
        Value::Custom {
            value: ScryptoCustomValue::PreciseDecimal(decimal),
        } => ManifestValue::Custom {
            value: ManifestCustomValue::PreciseDecimal(ManifestPreciseDecimal(
                decimal.0.to_le_bytes(),
            )),
        },
        Value::Custom {
            value: ScryptoCustomValue::NonFungibleLocalId(local_id),
        } => match local_id {
            NonFungibleLocalId::String(string) => Value::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(
                    ManifestNonFungibleLocalId::String(
                        string.value().to_owned(),
                    ),
                ),
            },
            NonFungibleLocalId::Integer(integer) => Value::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(
                    ManifestNonFungibleLocalId::Integer(integer.value()),
                ),
            },
            NonFungibleLocalId::Bytes(bytes) => Value::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(
                    ManifestNonFungibleLocalId::Bytes(bytes.value().to_vec()),
                ),
            },
            NonFungibleLocalId::RUID(ruid) => Value::Custom {
                value: ManifestCustomValue::NonFungibleLocalId(
                    ManifestNonFungibleLocalId::RUID(*ruid.value()),
                ),
            },
        },
    }
}

pub fn manifest_value_kind_from_scrypto_value_kind(
    scrypto_value_kind: ScryptoValueKind,
) -> ManifestValueKind {
    match scrypto_value_kind {
        ValueKind::Bool => ValueKind::Bool,
        ValueKind::I8 => ValueKind::I8,
        ValueKind::I16 => ValueKind::I16,
        ValueKind::I32 => ValueKind::I32,
        ValueKind::I64 => ValueKind::I64,
        ValueKind::I128 => ValueKind::I128,
        ValueKind::U8 => ValueKind::U8,
        ValueKind::U16 => ValueKind::U16,
        ValueKind::U32 => ValueKind::U32,
        ValueKind::U64 => ValueKind::U64,
        ValueKind::U128 => ValueKind::U128,
        ValueKind::String => ValueKind::String,
        ValueKind::Enum => ValueKind::Enum,
        ValueKind::Array => ValueKind::Array,
        ValueKind::Tuple => ValueKind::Tuple,
        ValueKind::Map => ValueKind::Map,
        ValueKind::Custom(ScryptoCustomValueKind::Decimal) => {
            ValueKind::Custom(ManifestCustomValueKind::Decimal)
        }
        ValueKind::Custom(ScryptoCustomValueKind::PreciseDecimal) => {
            ValueKind::Custom(ManifestCustomValueKind::PreciseDecimal)
        }
        ValueKind::Custom(ScryptoCustomValueKind::NonFungibleLocalId) => {
            ValueKind::Custom(ManifestCustomValueKind::NonFungibleLocalId)
        }
        ValueKind::Custom(ScryptoCustomValueKind::Reference) => {
            ValueKind::Custom(ManifestCustomValueKind::Address)
        }
        ValueKind::Custom(ScryptoCustomValueKind::Own) => unreachable!(),
    }
}
