# rust-cli-template
[![Build](https://github.com/infra-blocks/rust-cli-template/actions/workflows/build.yml/badge.svg)](https://github.com/infra-blocks/rust-cli-template/actions/workflows/build.yml)
[![Release](https://github.com/infra-blocks/rust-cli-template/actions/workflows/release.yml/badge.svg)](https://github.com/infra-blocks/rust-cli-template/actions/workflows/release.yml)
[![Trigger Update From Template](https://github.com/infra-blocks/rust-cli-template/actions/workflows/trigger-update-from-template.yml/badge.svg)](https://github.com/infra-blocks/rust-cli-template/actions/workflows/trigger-update-from-template.yml)
[![codecov](https://codecov.io/gh/infra-blocks/rust-cli-template/graph/badge.svg?token=X4P607ZTJP)](https://codecov.io/gh/infra-blocks/rust-cli-template)

[//]: # ([![Update From Template]&#40;https://github.com/infra-blocks/rust-cli-template/actions/workflows/update-from-template.yml/badge.svg&#41;]&#40;https://github.com/infra-blocks/rust-cli-template/actions/workflows/update-from-template.yml&#41;)

Template repository for rust CLI utilities. Upon instantiating, go through the following checklist:

- Do a global search & replace for `rust-cli-template` and replace it with the name of your repository
- `cargo build`
- Remove the [trigger update from template workflow](.github/workflows/trigger-update-from-template.yml)
- Replace the `Trigger Update From Template` status badge for the `Update From Template` status badge.
- Describe the package and its usage in this readme.
- Prepare the [changelog](CHANGELOG.md) for the first version of the module that will be released.
- Set up code coverage, overwrite the codecov badge with the specific link for your repository.
