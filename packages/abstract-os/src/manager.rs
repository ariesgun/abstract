//! # OS Manager
//!
//! `abstract_os::manager` implements the contract interface and state lay-out.
//!
//! ## Description
//!
//! The OS-manager is part of the Core OS contracts along with the `abstract_os::proxy` contract.
//! This contract is responsible for:
//! - Managing modules instantiation and migrations.
//! - Managing permissions.
//! - Upgrading the OS and its modules.
//! - Providing module name to address resolution.
//!
//! **The manager should be set as the contract/CosmWasm admin by default on your modules.**
//! ## Migration
//! Migrating this contract is done by calling `ExecuteMsg::Upgrade` with `abstract::manager` as module.
pub mod state {
    pub use crate::objects::core::OS_ID;
    use cosmwasm_std::Addr;
    use cw_controllers::Admin;
    use cw_storage_plus::{Item, Map};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    pub type Subscribed = bool;

    /// Manager configuration
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
    pub struct Config {
        pub version_control_address: Addr,
        pub module_factory_address: Addr,
        pub subscription_address: Option<Addr>,
    }
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
    pub struct OsInfo {
        pub name: String,
        pub governance_type: String,
        pub chain_id: String,
        pub description: Option<String>,
        pub link: Option<String>,
    }

    /// Subscription status
    pub const STATUS: Item<Subscribed> = Item::new("\u{0}{6}status");
    /// Configuration
    pub const CONFIG: Item<Config> = Item::new("\u{0}{6}config");
    /// Info about the OS
    pub const INFO: Item<OsInfo> = Item::new("\u{0}{4}info");
    /// Contract Admin
    pub const OS_FACTORY: Admin = Admin::new("\u{0}{7}factory");
    /// Root user
    pub const ROOT: Admin = Admin::new("root");
    /// Enabled Abstract modules
    pub const OS_MODULES: Map<&str, Addr> = Map::new("os_modules");
}

use cosmwasm_std::{Binary, Uint64};
use cw2::ContractVersion;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::objects::module::Module;

use self::state::OsInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub os_id: u32,
    pub root_user: String,
    pub version_control_address: String,
    pub module_factory_address: String,
    pub subscription_address: Option<String>,
    pub governance_type: String,
    pub name: String,
    pub description: Option<String>,
    pub link: Option<String>,
}

/// Execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Forward execution message to module
    ExecOnModule { module_id: String, exec_msg: Binary },
    /// Updates the `OS_MODULES` map
    /// Only callable by os factory or root.
    UpdateModuleAddresses {
        to_add: Option<Vec<(String, String)>>,
        to_remove: Option<Vec<String>>,
    },
    /// Create module using module factory, callable by Root
    CreateModule {
        /// Module information.
        module: Module,
        /// Instantiate message used to instantiate the contract.
        init_msg: Option<Binary>,
    },
    /// Registers a module after creation.
    /// Only callable by module factory.
    RegisterModule { module_addr: String, module: Module },
    /// Remove a module
    RemoveModule { module_name: String },
    /// Upgrade the module to a new version
    /// If module is `abstract::manager` then the contract will do a self-migration.
    Upgrade {
        module: Module,
        migrate_msg: Option<Binary>,
    },
    /// Update info
    UpdateInfo {
        name: Option<String>,
        description: Option<String>,
        link: Option<String>,
    },
    /// Sets a new Root
    SetRoot {
        root: String,
        governance_type: Option<String>,
    },
    /// Suspend manager contract
    SuspendOs { new_status: bool },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns [`QueryModuleVersionsResponse`]
    ModuleVersions { names: Vec<String> },
    /// Returns [`QueryModuleAddressesResponse`]
    ModuleAddresses { names: Vec<String> },
    /// Returns [`QueryModuleInfosResponse`]
    ModuleInfos {
        page_token: Option<String>,
        page_size: Option<u8>,
    },
    /// Returns [`QueryConfigResponse`]
    Config {},
    /// Returns [`QueryInfoResponse`]
    Info {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryModuleVersionsResponse {
    pub versions: Vec<ContractVersion>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryModuleAddressesResponse {
    pub modules: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryConfigResponse {
    pub root: String,
    pub version_control_address: String,
    pub module_factory_address: String,
    pub os_id: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryInfoResponse {
    pub info: OsInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ManagerModuleInfo {
    pub name: String,
    pub version: ContractVersion,
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryModuleInfosResponse {
    pub module_infos: Vec<ManagerModuleInfo>,
}
