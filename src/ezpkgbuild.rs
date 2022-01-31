use std::{
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus},
};

pub struct EZPKGBuild {
    pub name: String,
    pub pkgname: String,
    pub version: i32,
    pub build: Vec<String>,
    pub checksum: String,
    pub install: Vec<String>,
}

impl EZPKGBuild {
    pub fn new(
        name: String,
        pkgname: String,
        version: i32,
        build: Vec<String>,
        checksum: String,
        install: Vec<String>,
    ) -> EZPKGBuild {
        EZPKGBuild {
            name,
            pkgname,
            version,
            build,
            checksum,
            install,
        }
    }

    pub fn build(&self) {
        let mut cwd_buff = PathBuf::from(r"~/.cache/ezpkg/packagesrc/");
        cwd_buff.push(self.pkgname.as_str());

        let cwd = cwd_buff.as_path();

        for command in &self.build {
            let status = EZPKGBuild::execute_command(command.as_str(), cwd)
                .expect("Failed to execute build process");

            assert!(
                status.success(),
                "Building package {} failed.",
                self.pkgname
            );
        }
    }

    fn execute_command(command: &str, cwd: &Path) -> std::io::Result<ExitStatus> {
        Command::new("sh")
            .current_dir(cwd)
            .arg("-c")
            .arg(command)
            .status()
    }
}
