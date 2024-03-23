# Ignition Position Values Reference Implementation

This repository contains a reference implementation for a Rust crate that finds the value of an Ignition position if it were to be closed now. 

## Running This Example

* Change some of the constants in the [`constants.rs`](./src/constants.rs) with information relevant to the exchange that the position was opened in. More specifically, the `EXCHANGE_ADAPTER_COMPONENT_ADDRESS` and the `EXCHANGE_LIQUIDITY_RECEIPT_RESOURCE_ADDRESS` should be changed. A full list of the addresses can be found in the [Ignition Addresses](#ignition-addresses) section of this document.
* Change the `NonFungibleLocalId` in the [`main.rs`](./src/main.rs#L19) file with the local id of the Ignition liquidity position you wish to get the value of.
* Run the crate through `cargo run`.

## Ignition Addresses

This section contains all of the addresses of all entities created when Ignition was published. 

<details>
    <summary>Addresses</summary>
    
```json
{
    "dapp_definition_account": {
        "kind": "Reference",
        "type_name": "ComponentAddress",
        "value": "account_rdx1cxh9jq27n5vllmsexah8jj3txzue8yu236uekcnfr4hq5ptw8nn7f0"
    },
    "packages": {
        "protocol_entities": {
            "ignition": {
                "kind": "Reference",
                "type_name": "PackageAddress",
                "value": "package_rdx1pksyy7cyun85mgnuqdv4z3wm68d3pkfwzfkqrchhsu358zpjjuv426"
            },
            "simple_oracle": {
                "kind": "Reference",
                "type_name": "PackageAddress",
                "value": "package_rdx1phx0yptt32290n0uym4aqh3zyyup4ykth4enph8a68ggp7c38dqaxw"
            }
        },
        "exchange_adapter_entities": {
            "ociswap_v2": {
                "kind": "Reference",
                "type_name": "PackageAddress",
                "value": "package_rdx1pknh02tzgdjk7fs9nxyckpdkjkz5jhcu87m78vajexurh99dk9yt22"
            },
            "defiplaza_v2": {
                "kind": "Reference",
                "type_name": "PackageAddress",
                "value": "package_rdx1p5q7uhr7kkrtr2ta6xl938txrk8r2cra02cpvf2le548jjrcsfvzkc"
            },
            "caviarnine_v1": {
                "kind": "Reference",
                "type_name": "PackageAddress",
                "value": "package_rdx1p5c0rcv7kwnjlyfpam5qfp0xnknz9rpdy0de7fhxj689mvfxdzj558"
            }
        }
    },
    "components": {
        "protocol_entities": {
            "ignition": {
                "kind": "Reference",
                "type_name": "ComponentAddress",
                "value": "component_rdx1cqplswlzpvw9yx687mcnvjuguy24veqk4c55rscjxl3pll7rxfs2dz"
            },
            "simple_oracle": {
                "kind": "Reference",
                "type_name": "ComponentAddress",
                "value": "component_rdx1cr3psyfptwkktqusfg8ngtupr4wwfg32kz2xvh9tqh4c7pwkvlk2kn"
            }
        },
        "exchange_adapter_entities": {
            "ociswap_v2": {
                "kind": "Reference",
                "type_name": "ComponentAddress",
                "value": "component_rdx1cqrsdg6ag5urfe3av7d6z9q04emgjv726f48uhmzpex54jpwcxasq3"
            },
            "defiplaza_v2": {
                "kind": "Reference",
                "type_name": "ComponentAddress",
                "value": "component_rdx1cr2asvvh7s02l4pzez8szp6kck4f230h8rkxmf56347hwje5gg7vtc"
            },
            "caviarnine_v1": {
                "kind": "Reference",
                "type_name": "ComponentAddress",
                "value": "component_rdx1cpjs0phmgzwmhxel74l256zqdp39d2rfvj6m54e5k758k2vma8grp9"
            }
        }
    },
    "exchange_information": {
        "ociswap_v2": {
            "variant_id": 1,
            "variant_name": "Some",
            "fields": [
                {
                    "blueprint_id": {
                        "package_address": {
                            "kind": "Reference",
                            "type_name": "PackageAddress",
                            "value": "package_rdx1pkrgvskdkglfd2ar4jkpw5r2tsptk85gap4hzr9h3qxw6ca40ts8dt"
                        },
                        "blueprint_name": "PrecisionPool"
                    },
                    "pools": {
                        "bitcoin": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cpgmgrskahkxe4lnpp9s2f5ga0z8jkl7ne8gjmw3fc2224lxq505mr"
                        },
                        "ethereum": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1crahf8qdh8fgm8mvzmq5w832h97q5099svufnqn26ue44fyezn7gnm"
                        },
                        "usdc": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cz8daq5nwmtdju4hj5rxud0ta26wf90sdk5r4nj9fqjcde5eht8p0f"
                        },
                        "usdt": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cz79xc57dpuhzd3wylnc88m3pyvfk7c5e03me2qv7x8wh9t6c3aw4g"
                        }
                    },
                    "liquidity_receipt": {
                        "kind": "Reference",
                        "type_name": "ResourceAddress",
                        "value": "resource_rdx1ngeqqquzmjrd6q6atyawlh7p29jrpshdayw7rklyjw4n5k7ks6plm8"
                    }
                }
            ]
        },
        "defiplaza_v2": {
            "variant_id": 1,
            "variant_name": "Some",
            "fields": [
                {
                    "blueprint_id": {
                        "package_address": {
                            "kind": "Reference",
                            "type_name": "PackageAddress",
                            "value": "package_rdx1p4dhfl7qwthqqu6p2267m5nedlqnzdvfxdl6q7h8g85dflx8n06p93"
                        },
                        "blueprint_name": "PlazaPair"
                    },
                    "pools": {
                        "bitcoin": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1czzqr5m40x3sklwntcmx8uw3ld5nj7marq66nm6erp3prw7rv8zu29"
                        },
                        "ethereum": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cr0nw5ppvryyqcv6thkslcltkw5cm3c2lvm2yr8jhh9rqe76stmars"
                        },
                        "usdc": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1czmha58h7vw0e4qpxz8ga68cq6h5fjm27w2z43r0n6k9x65nvrjp4g"
                        },
                        "usdt": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1crhrzxe6x35hwx3wmnnw0g8qs84p2hle6ud7n2q4ffzp0udluqm8hj"
                        }
                    },
                    "liquidity_receipt": {
                        "kind": "Reference",
                        "type_name": "ResourceAddress",
                        "value": "resource_rdx1ntmgj3amlsrj0qxzqwzlk99d7g0xkzv6mg8vd5egawvgd8nt5ypwa7"
                    }
                }
            ]
        },
        "caviarnine_v1": {
            "variant_id": 1,
            "variant_name": "Some",
            "fields": [
                {
                    "blueprint_id": {
                        "package_address": {
                            "kind": "Reference",
                            "type_name": "PackageAddress",
                            "value": "package_rdx1p4r9rkp0cq67wmlve544zgy0l45mswn6h798qdqm47x4762h383wa3"
                        },
                        "blueprint_name": "QuantaSwap"
                    },
                    "pools": {
                        "bitcoin": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cp9w8443uyz2jtlaxnkcq84q5a5ndqpg05wgckzrnd3lgggpa080ed"
                        },
                        "ethereum": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cpsvw207842gafeyvf6tc0gdnq47u3mn74kvzszqlhc03lrns52v82"
                        },
                        "usdc": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cr6lxkr83gzhmyg4uxg49wkug5s4wwc3c7cgmhxuczxraa09a97wcu"
                        },
                        "usdt": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cqs338cyje65rk44zgmjvvy42qcszrhk9ewznedtkqd8l3crtgnmh5"
                        }
                    },
                    "liquidity_receipt": {
                        "kind": "Reference",
                        "type_name": "ResourceAddress",
                        "value": "resource_rdx1n2uzpxdlg90ajqy9r597xkffeefhacl8hqd6kpvmfmt56wlda0dzk9"
                    }
                }
            ]
        }
    },
    "protocol_configuration": {
        "protocol_resource": {
            "kind": "Reference",
            "type_name": "ResourceAddress",
            "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
        },
        "user_resource_volatility": {
            "bitcoin": {
                "variant_id": 0,
                "variant_name": "Volatile",
                "fields": []
            },
            "ethereum": {
                "variant_id": 0,
                "variant_name": "Volatile",
                "fields": []
            },
            "usdc": {
                "variant_id": 1,
                "variant_name": "NonVolatile",
                "fields": []
            },
            "usdt": {
                "variant_id": 1,
                "variant_name": "NonVolatile",
                "fields": []
            }
        },
        "reward_rates": [
            {
                "key": "23670144",
                "value": "0.125"
            },
            {
                "key": "26300160",
                "value": "0.145"
            },
            {
                "key": "28930176",
                "value": "0.17"
            },
            {
                "key": "31560192",
                "value": "0.2"
            }
        ],
        "allow_opening_liquidity_positions": false,
        "allow_closing_liquidity_positions": false,
        "maximum_allowed_price_staleness_in_seconds": "60",
        "maximum_allowed_price_difference_percentage": "0.05",
        "user_resources": {
            "bitcoin": {
                "kind": "Reference",
                "type_name": "ResourceAddress",
                "value": "resource_rdx1t580qxc7upat7lww4l2c4jckacafjeudxj5wpjrrct0p3e82sq4y75"
            },
            "ethereum": {
                "kind": "Reference",
                "type_name": "ResourceAddress",
                "value": "resource_rdx1th88qcj5syl9ghka2g9l7tw497vy5x6zaatyvgfkwcfe8n9jt2npww"
            },
            "usdc": {
                "kind": "Reference",
                "type_name": "ResourceAddress",
                "value": "resource_rdx1t4upr78guuapv5ept7d7ptekk9mqhy605zgms33mcszen8l9fac8vf"
            },
            "usdt": {
                "kind": "Reference",
                "type_name": "ResourceAddress",
                "value": "resource_rdx1thrvr3xfs2tarm2dl9emvs26vjqxu6mqvfgvqjne940jv0lnrrg7rw"
            }
        },
        "registered_pools": {
            "ociswap_v2": {
                "variant_id": 1,
                "variant_name": "Some",
                "fields": [
                    {
                        "bitcoin": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cpgmgrskahkxe4lnpp9s2f5ga0z8jkl7ne8gjmw3fc2224lxq505mr"
                        },
                        "ethereum": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1crahf8qdh8fgm8mvzmq5w832h97q5099svufnqn26ue44fyezn7gnm"
                        },
                        "usdc": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cz8daq5nwmtdju4hj5rxud0ta26wf90sdk5r4nj9fqjcde5eht8p0f"
                        },
                        "usdt": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cz79xc57dpuhzd3wylnc88m3pyvfk7c5e03me2qv7x8wh9t6c3aw4g"
                        }
                    }
                ]
            },
            "defiplaza_v2": {
                "variant_id": 1,
                "variant_name": "Some",
                "fields": [
                    {
                        "bitcoin": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1czzqr5m40x3sklwntcmx8uw3ld5nj7marq66nm6erp3prw7rv8zu29"
                        },
                        "ethereum": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cr0nw5ppvryyqcv6thkslcltkw5cm3c2lvm2yr8jhh9rqe76stmars"
                        },
                        "usdc": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1czmha58h7vw0e4qpxz8ga68cq6h5fjm27w2z43r0n6k9x65nvrjp4g"
                        },
                        "usdt": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1crhrzxe6x35hwx3wmnnw0g8qs84p2hle6ud7n2q4ffzp0udluqm8hj"
                        }
                    }
                ]
            },
            "caviarnine_v1": {
                "variant_id": 1,
                "variant_name": "Some",
                "fields": [
                    {
                        "bitcoin": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cp9w8443uyz2jtlaxnkcq84q5a5ndqpg05wgckzrnd3lgggpa080ed"
                        },
                        "ethereum": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cpsvw207842gafeyvf6tc0gdnq47u3mn74kvzszqlhc03lrns52v82"
                        },
                        "usdc": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cr6lxkr83gzhmyg4uxg49wkug5s4wwc3c7cgmhxuczxraa09a97wcu"
                        },
                        "usdt": {
                            "kind": "Reference",
                            "type_name": "ComponentAddress",
                            "value": "component_rdx1cqs338cyje65rk44zgmjvvy42qcszrhk9ewznedtkqd8l3crtgnmh5"
                        }
                    }
                ]
            }
        }
    },
    "user_resources": {
        "bitcoin": {
            "kind": "Reference",
            "type_name": "ResourceAddress",
            "value": "resource_rdx1t580qxc7upat7lww4l2c4jckacafjeudxj5wpjrrct0p3e82sq4y75"
        },
        "ethereum": {
            "kind": "Reference",
            "type_name": "ResourceAddress",
            "value": "resource_rdx1th88qcj5syl9ghka2g9l7tw497vy5x6zaatyvgfkwcfe8n9jt2npww"
        },
        "usdc": {
            "kind": "Reference",
            "type_name": "ResourceAddress",
            "value": "resource_rdx1t4upr78guuapv5ept7d7ptekk9mqhy605zgms33mcszen8l9fac8vf"
        },
        "usdt": {
            "kind": "Reference",
            "type_name": "ResourceAddress",
            "value": "resource_rdx1thrvr3xfs2tarm2dl9emvs26vjqxu6mqvfgvqjne940jv0lnrrg7rw"
        }
    },
    "badges": {
        "oracle_manager_badge": {
            "kind": "Reference",
            "type_name": "ResourceAddress",
            "value": "resource_rdx1th3yr5dlydnhw0lfp6r22x5l2fj9lv3t8f0enkp7j5ttnx3e09rhna"
        },
        "protocol_owner_badge": {
            "kind": "Reference",
            "type_name": "ResourceAddress",
            "value": "resource_rdx1t5ezhhs9cnua2thfnknmpj2rysz0rtwpexvjhvylww2ng5h3makwma"
        },
        "protocol_manager_badge": {
            "kind": "Reference",
            "type_name": "ResourceAddress",
            "value": "resource_rdx1t5w3cekqxjcphrvtp8x5rqz55s4qk97ralrtldnlvf3t6nfhq9a4en"
        }
    }
}
```
</details>