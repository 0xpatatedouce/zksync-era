use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use zksync_utils::ZeroPrefixHexSerde;

use crate::{ethabi, Address, EIP712TypedStructure, StructBuilder, H256, U256};

/// `Execute` transaction executes a previously deployed smart contract in the L2 rollup.
#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Execute {
    pub contract_address: Address,

    #[serde(with = "ZeroPrefixHexSerde")]
    pub calldata: Vec<u8>,

    pub value: U256,

    /// Factory dependencies: list of contract bytecodes associated with the deploy transaction.
    #[serde(default)]
    pub factory_deps: Vec<Vec<u8>>,
}

impl std::fmt::Debug for Execute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let factory_deps = format!("<{} factory deps>", self.factory_deps.len());
        f.debug_struct("Execute")
            .field("contract_address", &self.contract_address)
            .field("calldata", &hex::encode(&self.calldata))
            .field("value", &self.value)
            .field("factory_deps", &factory_deps)
            .finish()
    }
}

impl EIP712TypedStructure for Execute {
    const TYPE_NAME: &'static str = "Transaction";

    fn build_structure<BUILDER: StructBuilder>(&self, builder: &mut BUILDER) {
        builder.add_member("to", &U256::from(self.contract_address.as_bytes()));
        builder.add_member("value", &self.value);
        builder.add_member("data", &self.calldata.as_slice());
        // Factory deps are not included into the transaction signature, since they are parsed from the
        // transaction metadata.
        // Note that for the deploy transactions all the dependencies are implicitly included into the `calldataHash`
        // field, because the deps are referenced in the bytecode of the "main" contract bytecode.
    }
}

impl Execute {
    pub fn calldata(&self) -> &[u8] {
        &self.calldata
    }

    /// Prepares calldata to invoke deployer contract.
    /// This method encodes parameters for the `create` method.
    pub fn encode_deploy_params_create(
        salt: H256,
        contract_hash: H256,
        constructor_input: Vec<u8>,
    ) -> Vec<u8> {
        // TODO (SMA-1608): We should not re-implement the ABI parts in different places, instead have the ABI available
        //  from the `zksync_contracts` crate.
        static FUNCTION_SIGNATURE: Lazy<[u8; 4]> = Lazy::new(|| {
            ethabi::short_signature(
                "create",
                &[
                    ethabi::ParamType::FixedBytes(32),
                    ethabi::ParamType::FixedBytes(32),
                    ethabi::ParamType::Bytes,
                ],
            )
        });
        let params = ethabi::encode(&[
            ethabi::Token::FixedBytes(salt.as_bytes().to_vec()),
            ethabi::Token::FixedBytes(contract_hash.as_bytes().to_vec()),
            ethabi::Token::Bytes(constructor_input),
        ]);

        FUNCTION_SIGNATURE.iter().copied().chain(params).collect()
    }
}
