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

## Ideas

- CAN Frame should store frame kind (remote, error, classic, fd)
- Create tracing spans for each request
- Don't create sensors / attrs / signals on the fly. Force them to be defined up front.
