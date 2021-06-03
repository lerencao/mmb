#![cfg(test)]

use mmb_lib::core::exchanges::common::{CurrencyPair, ExchangeAccountId};
use mmb_lib::core::lifecycle::launcher::{launch_trading_engine, EngineBuildConfig, InitSettings};
use mmb_lib::core::settings::{AppSettings, BaseStrategySettings};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Default, Clone)]
pub struct TestStrategySettings {}

impl BaseStrategySettings for TestStrategySettings {
    fn exchange_account_id(&self) -> ExchangeAccountId {
        "TestExchange0".parse().expect("for testing")
    }

    fn currency_pair(&self) -> CurrencyPair {
        CurrencyPair::from_codes("base".into(), "quote".into())
    }
}

#[actix_rt::test]
async fn launch_engine() {
    let config = EngineBuildConfig::standard();

    let init_settings = InitSettings::Directly(AppSettings::<TestStrategySettings>::default());
    let engine = launch_trading_engine(&config, init_settings).await;

    let context = engine.context();
    let _ = tokio::spawn(async move {
        sleep(Duration::from_millis(200)).await;
        context
            .application_manager
            .run_graceful_shutdown("test")
            .await;
    });

    engine.run().await;
}
