mod ezpkgbuild;
use ezpkgbuild::EZPKGBuild;

fn main() {
    let pkg = EZPKGBuild::new(
        "EZ Package Manager".to_string(),
        "ezpkg".to_string(),
        1,
        vec![
            "git clone https://github.com/Fullbrik/ezpkg.git".to_string(),
            "cargo build".to_string(),
        ],
        "".to_string(),
        vec!["".to_string()],
    );

    pkg.build();
    //pkg.install();
}
