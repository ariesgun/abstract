use crate::msg::*;
use abstract_core::app::MigrateMsg;
use abstract_interface::AppDeployer;
use cw_orch::interface;
use cw_orch::prelude::*;

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Template<Chain>;

impl<Chain: CwEnv> Uploadable for Template<Chain> {
    fn wasm(&self) -> cw_orch::prelude::WasmPath {
        ArtifactsDir::env().find_wasm_path(&self.id()).unwrap()
    }

    fn wrapper(
        &self,
    ) -> Box<dyn cw_orch::prelude::MockContract<cosmwasm_std::Empty, cosmwasm_std::Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            )
            .with_reply(crate::contract::reply)
            .with_migrate(crate::contract::migrate),
        )
    }
}

// Custom Abstract deployer trait, TODO: fix this
// impl<Chain: CwEnv> AppDeployer<Chain> for Template<Chain> {}
