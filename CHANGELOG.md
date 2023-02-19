## [3.0.1](https://github.com/mobusoperandi/michie/compare/v3.0.0...v3.0.1) (2023-02-19)


### Testing

* add rust-toolchain.toml and update .stderr files ([825f09c](https://github.com/mobusoperandi/michie/commit/825f09c37e330848c6a756895a896f0952534e09))
* satisfy iter_kv_map ([cbdded5](https://github.com/mobusoperandi/michie/commit/cbdded538c4c756aea7483f618bcd66d16a7a4e9))


### Documentation

* fix readme ci badge ([5f122f8](https://github.com/mobusoperandi/michie/commit/5f122f88b0f2a7190b6f73e315e9de67436aa678)), closes [#240](https://github.com/mobusoperandi/michie/issues/240)
* rm "made by mobus operandi" ([6948ba2](https://github.com/mobusoperandi/michie/commit/6948ba2b60c2b51a398832626c6d3c3c883c3f42))
* rm erroneous clone call from "how it works" ([a67994e](https://github.com/mobusoperandi/michie/commit/a67994ebe1fedca2e1da71bf5cb85fc618d3ba4a)), closes [#239](https://github.com/mobusoperandi/michie/issues/239)
* rm unused import in store_init example ([176c1ad](https://github.com/mobusoperandi/michie/commit/176c1ad616acdf3f1667631b972a8a718c6e3fb7)), closes [#227](https://github.com/mobusoperandi/michie/issues/227)


### Build system / dependencies

* rm workaround for https://github.com/rust-lang/cargo/issues/9507 ([25d6fec](https://github.com/mobusoperandi/michie/commit/25d6fec41bc18fafebf47fa46e2a99ba4e4f8c63))
* semantic-release-cargo ([4ad8615](https://github.com/mobusoperandi/michie/commit/4ad8615250a314c28cf93795921c69cfffc3a857))
* upgrade deps ([08ca121](https://github.com/mobusoperandi/michie/commit/08ca12122a353d76b032ae1561e4f8b16ec750c7))

## [3.0.0](https://github.com/mobusoperandi/michie/compare/v2.0.0...v3.0.0) (2022-10-24)


### ⚠ BREAKING CHANGES

* no default store type

Co-authored-by: José Manuel Peña <josemanuelp2@gmail.com>

### Features

* no default store type ([6707a4b](https://github.com/mobusoperandi/michie/commit/6707a4ba153997ca5b3d76ec8abbec83122edb5d))

## [2.0.0](https://github.com/mobusoperandi/michie/compare/v1.1.0...v2.0.0) (2022-10-24)


### ⚠ BREAKING CHANGES

* The `key_type` parameter is removed.

Co-authored-by: Vimal Patel <mailtovimal@gmail.com>
Co-authored-by: José Manuel Peña <josemanuelp2@gmail.com>

### Features

* remove key_type parameter ([56dbf31](https://github.com/mobusoperandi/michie/commit/56dbf319eff35f238c98360ad66e42a77293fc37))


### Testing

* underscore some intentionally unused fns ([3f84c81](https://github.com/mobusoperandi/michie/commit/3f84c819f3d6789e98e27e5bc7ec02ab0d338c2e))
* update expected error messages ([c73608a](https://github.com/mobusoperandi/michie/commit/c73608a60eac99b2c96b7f7902df38ca18bcc5e2))
* update stderr files ([d0c7437](https://github.com/mobusoperandi/michie/commit/d0c74372cc5a70b801244d40b6c878fb817ac09e))


### Build system / dependencies

* do not set RUSTFLAGS in Makefile ([bbae378](https://github.com/mobusoperandi/michie/commit/bbae378c373184075158daddf733892505b5e0b3))
* update sub-dep libc ([131f12f](https://github.com/mobusoperandi/michie/commit/131f12f1266f9c59618f39a16b89824679bf83cf))
* upgrade cargo_toml ([3c99a7f](https://github.com/mobusoperandi/michie/commit/3c99a7fe8c38267a2a24e3e5eaf208503d562c47))
* upgrade cargo-edit and cargo-make ([5d2ea79](https://github.com/mobusoperandi/michie/commit/5d2ea797dde46e1e211d39d400138cb2c5c550c7))
* upgrade jql ([528caae](https://github.com/mobusoperandi/michie/commit/528caae9bdb973ba728cc8f3f365d9d3b45708e2))

## [1.1.0](https://github.com/mobusoperandi/michie/compare/v1.0.0...v1.1.0) (2022-06-26)


### Features

* rename trait parameters ([b7f42dd](https://github.com/mobusoperandi/michie/commit/b7f42ddec61002818fc62c586a811fdf76a25634))

## [1.0.0](https://github.com/mobusoperandi/michie/compare/v0.3.0...v1.0.0) (2022-06-18)


### ⚠ BREAKING CHANGES

* Signatures of `MemoizationStore::insert` and
`MemoizationStore::get` altered.

Co-authored-by: Roland Fredenhagen <dev@modprog.de>

### Features

* return type does not have to be Clone ([044862d](https://github.com/mobusoperandi/michie/commit/044862dbf776c03c697abb1f6d2d8a16cf3e0dbc))

## [0.3.0](https://github.com/mobusoperandi/michie/compare/v0.2.13...v0.3.0) (2022-06-15)


### Features

* better error when fn with default return type ([4ddeb42](https://github.com/mobusoperandi/michie/commit/4ddeb42897320f28f7c4aeb06ae25808137c477e))

## [0.2.13](https://github.com/mobusoperandi/michie/compare/v0.2.12...v0.2.13) (2022-06-09)


### Documentation

* sounds like mickey ([4a85462](https://github.com/mobusoperandi/michie/commit/4a854627191be83b3ae8465e64f6f8c4ab286f05))

## [0.2.12](https://github.com/mobusoperandi/michie/compare/v0.2.11...v0.2.12) (2022-06-08)


### Documentation

* don't use IPA ([12ce350](https://github.com/mobusoperandi/michie/commit/12ce3508beaebfe653ce84ca33e8f05a95116926)), closes [#217](https://github.com/mobusoperandi/michie/issues/217)

## [0.2.11](https://github.com/mobusoperandi/michie/compare/v0.2.10...v0.2.11) (2022-06-07)


### CI

* rename workflow ([fe01348](https://github.com/mobusoperandi/michie/commit/fe0134890035548e91cb6c78baf4d6bacc5f6c9d)), closes [#204](https://github.com/mobusoperandi/michie/issues/204)


### Documentation

* shields.io build badge ([4c0f356](https://github.com/mobusoperandi/michie/commit/4c0f35686ddf362fa3c81bbd2bbb88e7b4fafa98))

## [0.2.10](https://github.com/mobusoperandi/michie/compare/v0.2.9...v0.2.10) (2022-06-07)


### Documentation

* non-features ([802f3fc](https://github.com/mobusoperandi/michie/commit/802f3fc01b9b2c16924c7b20aad61822065c3ac6)), closes [#210](https://github.com/mobusoperandi/michie/issues/210) [#212](https://github.com/mobusoperandi/michie/issues/212)

## [0.2.9](https://github.com/mobusoperandi/michie/compare/v0.2.8...v0.2.9) (2022-06-07)


### Bug fixes

* increase michie-macro available attempts to 90 ([d32756a](https://github.com/mobusoperandi/michie/commit/d32756ae57a30b45a2a097946bdd5fe3e7493b57))

## [0.2.8](https://github.com/mobusoperandi/michie/compare/v0.2.7...v0.2.8) (2022-06-06)


### Documentation

* update section on mobus operandi ([d43f2ad](https://github.com/mobusoperandi/michie/commit/d43f2ade6767187eb15e2176875378bb6d48a068)), closes [#205](https://github.com/mobusoperandi/michie/issues/205)

## [0.2.7](https://github.com/mobusoperandi/michie/compare/v0.2.6...v0.2.7) (2022-06-06)


### Documentation

* no table of contents in table of contents ([b8df712](https://github.com/mobusoperandi/michie/commit/b8df712211fe38ccb71c7010455ae7b025657a7c)), closes [#203](https://github.com/mobusoperandi/michie/issues/203)

## [0.2.6](https://github.com/mobusoperandi/michie/compare/v0.2.5...v0.2.6) (2022-06-05)


### Bug fixes

* obtain michie_macro version with cargo-show ([8b66492](https://github.com/mobusoperandi/michie/commit/8b66492c32d3b0bbeae6b8fe85736250b01654ef))

## [0.2.5](https://github.com/mobusoperandi/michie/compare/v0.2.4...v0.2.5) (2022-06-05)


### Bug fixes

* rename publish task to publish_crates ([b76ef6b](https://github.com/mobusoperandi/michie/commit/b76ef6bc26ee6143c5fde5c8479fe63f68da96ff))

## [0.2.4](https://github.com/mobusoperandi/michie/compare/v0.2.3...v0.2.4) (2022-06-05)


### Bug fixes

* wait for macro crate available on crates.io ([eb62de2](https://github.com/mobusoperandi/michie/commit/eb62de28c708e2c0bf23886afa3ab8b21d1e95c2))

## [0.2.3](https://github.com/mobusoperandi/michie/compare/v0.2.2...v0.2.3) (2022-06-03)


### Bug fixes

* pin macro dep ([35d6fce](https://github.com/mobusoperandi/michie/commit/35d6fcee90dda851d6115af0de80dfc1a2cdaa3b))

## [0.2.2](https://github.com/mobusoperandi/michie/compare/v0.2.1...v0.2.2) (2022-06-02)


### Testing

* all dev_deps are single constraint caret ([0cd4e76](https://github.com/mobusoperandi/michie/commit/0cd4e769b731879d15324eb0b995e51bb7170628))


### Build system / dependencies

* begin bash scripts with `set -euxo pipefail` ([e96cd1e](https://github.com/mobusoperandi/michie/commit/e96cd1e3208770e6a5479b40413249fc531565ad))
* check-in Cargo.lock ([aaced5f](https://github.com/mobusoperandi/michie/commit/aaced5fb87762195bb3e896512af48a661384ae9))
* commit message linting ([81107a2](https://github.com/mobusoperandi/michie/commit/81107a2798762eb0f60a68787ded8da183f36f74)), closes [#152](https://github.com/mobusoperandi/michie/issues/152)
* rm infiltrating .envrc ([35c97f7](https://github.com/mobusoperandi/michie/commit/35c97f7d1434b09a4e050f1e48897df4159e866b))
* shebangs in script tasks ([84875be](https://github.com/mobusoperandi/michie/commit/84875be05f29e1915577abbfbf72706d8de8d060)), closes [#187](https://github.com/mobusoperandi/michie/issues/187)


### Documentation

* explicit guard drop in "how it works" ([e83bff9](https://github.com/mobusoperandi/michie/commit/e83bff98541eb85e7a94c8e487f804b970676536)), closes [#131](https://github.com/mobusoperandi/michie/issues/131)
* fix ci badge ([2d3d0cc](https://github.com/mobusoperandi/michie/commit/2d3d0cc60fff4b3405ed54fb44a9ab4209216086)), closes [#179](https://github.com/mobusoperandi/michie/issues/179)
* fix ci badge in readme ([e56afdf](https://github.com/mobusoperandi/michie/commit/e56afdf7eeaf1544b036e0d7074c5f874b2bc2c9)), closes [#173](https://github.com/mobusoperandi/michie/issues/173)
* key type is not Clone and related improvements ([652c638](https://github.com/mobusoperandi/michie/commit/652c638d82afbf1d10dab6ba067c20b0f774aedf)), closes [#147](https://github.com/mobusoperandi/michie/issues/147)
* rewrite section on store bounds ([a40c3b3](https://github.com/mobusoperandi/michie/commit/a40c3b392f2b04a823cb654fc7ff1fe12b42543b)), closes [#132](https://github.com/mobusoperandi/michie/issues/132)
* table of contents ([b3487ef](https://github.com/mobusoperandi/michie/commit/b3487ef456c1dc9bc5a0616597b5b1b0933d74dd)), closes [#140](https://github.com/mobusoperandi/michie/issues/140)


### Refactoring

* extract store_trait_object ([136f9aa](https://github.com/mobusoperandi/michie/commit/136f9aaddef9def59e4ad07ffc26ade968ab5f81))


### CI

* add restore-keys ([25f82b7](https://github.com/mobusoperandi/michie/commit/25f82b735c57c9cad9eb440f13b2f553f02a0f28)), closes [#177](https://github.com/mobusoperandi/michie/issues/177)
* bypass semantic-release-rust in publishing ([34a9432](https://github.com/mobusoperandi/michie/commit/34a94327765f2fc29b8ba8bc46f8c79c36a2edab)), closes [#117](https://github.com/mobusoperandi/michie/issues/117)
* cache .bin ([48f779f](https://github.com/mobusoperandi/michie/commit/48f779f53c7e96f88d0df9ec16967fa04dd411a9)), closes [#186](https://github.com/mobusoperandi/michie/issues/186)
* correct commit linting condition ([fb6a8c6](https://github.com/mobusoperandi/michie/commit/fb6a8c6f39e40d7a23291742ebeac05cba287953)), closes [#175](https://github.com/mobusoperandi/michie/issues/175)
* rewrite untracked files check ([02e1104](https://github.com/mobusoperandi/michie/commit/02e1104c7e3c32999a84103861f17f7689e4bd7c))
* run on push to any branch ([a7c276a](https://github.com/mobusoperandi/michie/commit/a7c276afbb6ffaf1882f30d73234878c606d5120))
* semantic-release ([aa5e764](https://github.com/mobusoperandi/michie/commit/aa5e764068949a0d02c8a7f6cbd013548c46de91)), closes [#117](https://github.com/mobusoperandi/michie/issues/117)

### Prior to adopting semantic commits

- [83d4bf5](https://github.com/mobusoperandi/michie/commit/83d4bf5abdd5b9ef17584f32274439452da7cb30) gitignore /.bin and check in ci
- [53381e2](https://github.com/mobusoperandi/michie/commit/53381e23fe794471caebad16c84b4e4ea1fc1dca) cargo-run-bin
- [d4fb751](https://github.com/mobusoperandi/michie/commit/d4fb7515c0cb0d66a9e1baf14dd9e3c6b6070322) add cargo-make to CI cache key
- [6cd753f](https://github.com/mobusoperandi/michie/commit/6cd753fcf3203b0a415d072030004235d3f0df7b) rm ~/.cargo/.crates2.json from CI cache key
- [6a886c9](https://github.com/mobusoperandi/michie/commit/6a886c973ab3c37b69c4cb30232374a47be4bcff) actions/cache@v3
- [c17c7ed](https://github.com/mobusoperandi/michie/commit/c17c7ed18baa75de74234d261f4a1ef1223c3b9a) cargo install cargo-make
- [ae6fb14](https://github.com/mobusoperandi/michie/commit/ae6fb14d66024ad6991b1bd2cf3f860756b4e0df) parallel cargo-make tasks
- [2d38992](https://github.com/mobusoperandi/michie/commit/2d3899205f607877d8360920ebcacc3295204f8f) Revert "modular github actions jobs"
- [678ff56](https://github.com/mobusoperandi/michie/commit/678ff569e04ffb5e017b670e711fd40414acdbb4) modular github actions jobs
- [e2fa381](https://github.com/mobusoperandi/michie/commit/e2fa3812611e5ee96a2a1102209e5bf91add424d) rename workflow
- [d8f2f00](https://github.com/mobusoperandi/michie/commit/d8f2f00e7b85bfa87d2a0a94cb77f83468532584) run workflow on PRs against any branch
- [dc76069](https://github.com/mobusoperandi/michie/commit/dc76069ee36512dca6437bea39475459116512f7) rename workflow file
- [118dde1](https://github.com/mobusoperandi/michie/commit/118dde14e7cbfb0526fb720f2ecae70d91eeb03a) remove name for step "rustup update"
- [bd2c9cf](https://github.com/mobusoperandi/michie/commit/bd2c9cfc3b72cd48d3b9b18e52d8e075cc904eca) rename ci job-id
- [25e6592](https://github.com/mobusoperandi/michie/commit/25e6592fc8afea4f9852557666c176e593d44500) rename ci workflow
- [a913d37](https://github.com/mobusoperandi/michie/commit/a913d370efad8a0699ca1c3f8b2968e27263f02b) install cargo-make in github action via binary
- [b33e66d](https://github.com/mobusoperandi/michie/commit/b33e66d0721200b981014e883a71907ab12a284a) ci cache rust on failure
- [c51b3eb](https://github.com/mobusoperandi/michie/commit/c51b3ebf9fc0e2b87f8195b3608b41d614ab2651) pre-commit hook
- [1f75206](https://github.com/mobusoperandi/michie/commit/1f752065f1453e4aacee4db5b269b996e9281f6c) reorder bounds in generic example
- [91ec53a](https://github.com/mobusoperandi/michie/commit/91ec53a867bcc4530badc1844d071c3f8f026c82) improve doc regarding 'static
- [ed0acbb](https://github.com/mobusoperandi/michie/commit/ed0acbbebf1957d6d0f47697cee9d0157e87a89a) rename cache to store in code
- [3338de7](https://github.com/mobusoperandi/michie/commit/3338de7b9bb5edbeb26366725fec5672107eee02) no explicit links for types from std/core
- [442fe8a](https://github.com/mobusoperandi/michie/commit/442fe8a131aaa6219ffee4add5acaa342c903d9a) remove trybuild bug workaround
- [8ceee5d](https://github.com/mobusoperandi/michie/commit/8ceee5d8a67293fcd417739b5aa0a65a54b8eaa7) rewrite "no default key_expr" section
- [ce8ddcf](https://github.com/mobusoperandi/michie/commit/ce8ddcf3df86d43e3c15853bcde253f492942d8f) store type inference
- [438df06](https://github.com/mobusoperandi/michie/commit/438df063ee6e92e6bc3ac7ccfc0354d8726a7a7b) rewrite type requirements section
- [cedaba8](https://github.com/mobusoperandi/michie/commit/cedaba856e3bddcccf1d15fe147e032eb6e7985f) consistently name <K, R> everywhere
- [3597812](https://github.com/mobusoperandi/michie/commit/359781246eb03dcc75756ce55aeeec13c4fdd5cb) rephrase `store_init` section
- [be7f0fd](https://github.com/mobusoperandi/michie/commit/be7f0fd9b88768abe639aa60931d5d219d8b56a0) rewrite `store_type` section
- [adfd853](https://github.com/mobusoperandi/michie/commit/adfd85350669518a258c55f6930bbdefddcd0bb4) rm extra blank line
- [d25bd49](https://github.com/mobusoperandi/michie/commit/d25bd49f66144b9dfcb480a9629db7a1a8476678) move the basic example into the `key_expr` section
- [905c03b](https://github.com/mobusoperandi/michie/commit/905c03bc43d898cd14350a570a6a2faea6138627) rewrite `key_expr` section
- [c1ff2bb](https://github.com/mobusoperandi/michie/commit/c1ff2bb676f812afca7d440820213937ca7a0d9a) move "why no default `key_expr` to later
- [d62694a](https://github.com/mobusoperandi/michie/commit/d62694af6eb28dc71595bf6985413232a9c50b02) move "How it works" to later
- [62346fc](https://github.com/mobusoperandi/michie/commit/62346fc83f6080af3b1d2af43665b430eaaa5940) remove redundant parsing of `ItemFn`
- [2e0352b](https://github.com/mobusoperandi/michie/commit/2e0352bcaf044ff3be3e7e24ba7ad8ce855f6d9f) remove fake attribute workaround
