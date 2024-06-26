#![allow(unused_imports)]
use abstract_interface::{Abstract, AbstractIbc};
use abstract_scripts::ROLLKIT_TESTNET;
use cw_orch::{
    daemon::networks::{neutron::NEUTRON_NETWORK, ARCHWAY_1, JUNO_1, OSMOSIS_1, PHOENIX_1},
    environment::ChainKind,
    prelude::*,
    tokio::runtime::{Handle, Runtime},
};

pub const NETWORK: ChainInfo = ROLLKIT_TESTNET;

/// Script to deploy the IBC modules on a chain.
/// Currently deployed by abstract, so only used on chains where IBC is not present.
fn main() -> cw_orch::anyhow::Result<()> {
    dotenv::dotenv()?;
    env_logger::init();

    let runtime = Runtime::new()?;
    let daemons = vec![
        // get_daemon(JUNO_1, runtime.handle(), None)?,
        // get_daemon(PHOENIX_1, runtime.handle(), None)?,
        // get_daemon(ARCHWAY_1, runtime.handle(), None)?,
        get_daemon(NETWORK, runtime.handle(), None)?,
        // get_daemon(
        //     OSMOSIS_1,
        //     runtime.handle(),
        //     Some(std::env::var("OSMOSIS_MNEMONIC")?),
        // )?,
    ];

    for daemon in daemons {
        deploy_host_and_client(daemon)?;
    }

    Ok(())
}

fn get_daemon(
    chain: ChainInfo,
    handle: &Handle,
    mnemonic: Option<String>,
) -> cw_orch::anyhow::Result<Daemon> {
    let mut builder = DaemonBuilder::default();
    builder.chain(chain).handle(handle);
    if let Some(mnemonic) = mnemonic {
        builder.mnemonic(mnemonic);
    }
    Ok(builder.build()?)
}
pub fn deploy_host_and_client<Chain: CwEnv>(chain: Chain) -> cw_orch::anyhow::Result<()> {
    let abs = Abstract::load_from(chain.clone())?;
    let ibc_infra = AbstractIbc::new(&chain);
    ibc_infra.upload()?;
    ibc_infra.instantiate(&abs, &chain.sender())?;
    ibc_infra.register(&abs.version_control)?;

    abs.version_control.approve_any_abstract_modules()?;

    Ok(())
}
