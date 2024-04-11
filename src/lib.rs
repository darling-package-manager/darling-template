use darling_api as darling;

// You can rename this safely
pub struct MyPackageManager;

// Do not rename this! it must be `pub static PACKAGE_MANAGER` and impl `darling::PackageManager`.
pub static PACKAGE_MANAGER: MyPackageManager = MyPackageManager;

impl darling::PackageManager for MyPackageManager {
    fn name(&self) -> String {
        unimplemented!()
    }

    fn install(&self, context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<()> {
        unimplemented!();
    }

    fn uninstall(&self, context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<()> {
        unimplemented!();
    }

    fn get_all_explicit(&self, context: &darling::Context) -> anyhow::Result<Vec<(String, String)>> {
        unimplemented!();
    }
}
