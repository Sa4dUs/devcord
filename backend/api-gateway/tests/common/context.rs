use api_gateway::config::{self, Config};
use tokio::sync::OnceCell;

use crate::common::helpers::launch_instance;

struct TestContext {
    config: Config,
}

static TEST_CONTEXT: OnceCell<TestContext> = OnceCell::const_new();

async fn tcx() -> &'static TestContext {
    TEST_CONTEXT
        .get_or_init(|| async {
            let config = config::load().expect("Cannot load config");
            launch_instance(3001).await;
            launch_instance(3002).await;
            TestContext { config }
        })
        .await
}
