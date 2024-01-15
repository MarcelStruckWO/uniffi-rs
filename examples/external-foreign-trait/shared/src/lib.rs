#[uniffi::export(with_foreign)]
pub trait Tester: Send + Sync {
    fn test(&self) -> String;
}

uniffi::setup_scaffolding!();
