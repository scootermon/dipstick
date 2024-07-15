# Dipstick Protocol

## Conventions

### Entity Selectors

- Should be named `selector` if there's enough context to establish what kind of entity it selects.
- Use the name of the entity kind if more context is needed. For example if the user has to choose between either i2c or spi as the transport, the fields should be named `i2c` and `spi` respectively.

### Services

- Services should be called `{xyz}Service` instead of just `{xyz}`. This is already the case for ambiguous services like `Shadow` and `Stack`, which both contain entity kinds with the same name.
