use cargo_toml::{Dependency, Manifest};
use semver::VersionReq;

#[test]
fn all_dev_deps_are_caret_constrained() {
    let manifest = Manifest::from_path("Cargo.toml").unwrap();
    let all_are_caret = manifest
        .dev_dependencies
        .into_iter()
        .map(|(_, dep)| {
            let version = match dep {
                Dependency::Simple(version) => version,
                Dependency::Detailed(detailed) => detailed.version.unwrap(),
            };
            let comparators = VersionReq::parse(&version).unwrap().comparators;
            assert_eq!(comparators.len(), 1);
            comparators[0].op
        })
        .all(|op| op == semver::Op::Caret);
    assert!(all_are_caret);
}
