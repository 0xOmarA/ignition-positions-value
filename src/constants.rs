use address_macros::*;
use radix_engine_interface::prelude::*;

/// The component address of the Ignition component.
pub const IGNITION_COMPONENT_ADDRESS: ComponentAddress = component_address!(
    "component_rdx1cqplswlzpvw9yx687mcnvjuguy24veqk4c55rscjxl3pll7rxfs2dz"
);

/// The component address of the Ignition oracle.
pub const IGNITION_ORACLE_COMPONENT_ADDRESS: ComponentAddress = component_address!(
    "component_rdx1cr3psyfptwkktqusfg8ngtupr4wwfg32kz2xvh9tqh4c7pwkvlk2kn"
);

/// The component address of the account that holds the protocol owner badge of
/// the Ignition protocol.
pub const PROTOCOL_OWNER_ACCOUNT_COMPONENT_ADDRESS: ComponentAddress = component_address!(
    "account_rdx16ykaehfl0suwzy9tvtlhgds7td8ynwx4jk3q4czaucpf6m4pps9yr4"
);

/// The resource address of the protocol owner badge of the Ignition protocol.
pub const PROTOCOL_OWNER_BADGE_RESOURCE_ADDRESS: ResourceAddress = resource_address!(
    "resource_rdx1t5ezhhs9cnua2thfnknmpj2rysz0rtwpexvjhvylww2ng5h3makwma"
);

/// The address of the adapter of the exchange.
pub const EXCHANGE_ADAPTER_COMPONENT_ADDRESS: ComponentAddress = component_address!(
    "component_rdx1cpjs0phmgzwmhxel74l256zqdp39d2rfvj6m54e5k758k2vma8grp9"
);

/// The address of the liquidity receipt of the exchange.
pub const EXCHANGE_LIQUIDITY_RECEIPT_RESOURCE_ADDRESS: ResourceAddress = resource_address!(
    "resource_rdx1n2uzpxdlg90ajqy9r597xkffeefhacl8hqd6kpvmfmt56wlda0dzk9"
);
