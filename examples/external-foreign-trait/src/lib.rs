use shared::Tester;
use std::sync::Arc;

#[uniffi::export]
pub fn do_test(tester: Arc<dyn Tester>) -> String {
    tester.test()
}

uniffi::setup_scaffolding!();
