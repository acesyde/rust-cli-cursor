# Rust CLI Code Style Guide

Please follow these guidelines when writing Rust code for the CLI project.

## Directory Structure

- `src/main.rs`: The main entry point of the application.

- `src/commands/`: Contains all command implementations.
- `src/commands/mod.rs`: The main module file that declares all commands.
- `src/commands/<command_name>.rs`: Each command should have its own file.

- `src/models.rs`: Contains all data models used in the CLI.

- `src/services/`: Contains all service implementations.
- `src/services/mod.rs`: The main module file that declares all services.
- `src/services/<service_name>.rs`: Each service should have its own file.

## Naming Conventions

- Use `snake_case` for file names and module names.
- Use `PascalCase` for struct and enum names.
- Use `snake_case` for function and variable names.
- Use `UPPER_SNAKE_CASE` for constants and static variables.
- Use `lowercase` for command names (e.g., `create_user`, `delete_user`).
- Use `lowercase` for command aliases (e.g., `cu`, `du`).
- Use `lowercase` for command flags (e.g., `--force`, `--verbose`).
- Use `lowercase` for command arguments (e.g., `username`, `password`).
- Use `lowercase` for command options (e.g., `--json`, `--yaml`).
- Use `lowercase` for command subcommands (e.g., `list`, `show`).
- Use `lowercase` for command subcommand arguments (e.g., `user_id`, `group_id`).
- Use `lowercase` for command subcommand options (e.g., `--all`, `--active`).
- Use `lowercase` for command subcommand flags (e.g., `-a`, `-v`).
- Use `lowercase` for command subcommand aliases (e.g., `l`, `s`).

## Unit Tests

- Use `#[cfg(test)]` to mark test modules.
- Use `#[test]` to mark test functions.
- Use `assert_eq!` and `assert_ne!` for equality assertions.
- Use `assert!(condition)` for boolean assertions.
- Use `assert!(result.is_ok())` for result assertions.

Use the Given/When/Then format for writing tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Given
        let input = "example input";

        // When
        let result = example_function(input);

        // Then
        assert_eq!(result, "expected output");
    }
}
```
