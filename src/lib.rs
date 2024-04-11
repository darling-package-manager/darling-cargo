use darling_api as darling;

pub struct Cargo;

pub static PACKAGE_MANAGER: Cargo = Cargo;

impl darling::PackageManager for Cargo {
    fn name(&self) -> String {
        "cargo".to_owned()
    }

    fn install(&self, _context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<()> {
        std::process::Command::new("cargo").arg("install").arg(&package.name).spawn()?.wait()?;
        Ok(())
    }

    fn uninstall(&self, _context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<()> {
        std::process::Command::new("cargo").arg("uninstall").arg(&package.name).spawn()?.wait()?;
        Ok(())
    }

    fn get_all_explicit(&self, _context: &darling::Context) -> anyhow::Result<Vec<(String, String)>> {
        let output = String::from_utf8(std::process::Command::new("cargo").arg("install").arg("--list").output()?.stdout)?;
        let installed_crates = output.lines().filter(|line| !line.chars().all(|char| char.is_whitespace()));
        let pattern = regex_macro::regex!(r"^(\S+)\s([^:]+)");
        let crates = installed_crates
            .filter_map(|entry| pattern.captures(entry).map(|capt| (capt[1].to_owned(), capt[2].to_owned())))
            .collect::<Vec<_>>();

        Ok(crates)
    }
}
