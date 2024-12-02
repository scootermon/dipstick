# Dipstick

## Design Philosophy

### Entities

Entities are the fundamental building block behind Dipstick.
Every entity consists of the following parts:

- "meta"
- "spec" which contains the config of the entity.
- "status"

The contents of "spec" and "status" depend on the entity kind.

### Packages

APIs are structured into packages.
Packages contain gRPC services and entity kinds.

### Naming

`dp` is used as the abbreviation for Dipstick.

## Ideas

- CAN Frame should store frame kind (remote, error, classic, fd)
- Create tracing spans for each request
- Don't create sensors / attrs / signals on the fly. Force them to be defined up front.

## Install on Raspberry Pi

Lazy notes for Simon.

```bash
# build for aarch64
cargo deb -p dipstick --cargo-build zigbuild --target=aarch64-unknown-linux-gnu

# run in ssh
sudo apt install ./dipstick_1.0.0-1_arm64.deb && rm ./*.deb
```
