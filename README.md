# Enums & union types across languages

The same two ideas in six languages:

- **`Status`** - a plain enum (`pending`, `running`, `succeeded`, `failed`).
- **`Media`** - a tagged union with three variants (`text`, `image`, `video`), plus a `describe` function that dispatches over them.

Each example prints the same output.

## Excalidraw link

[https://excalidraw.com/#json=gT-eLu8sfVfv6tCByDpMu,gZRzQuLVk7lSEmSJ25Io9g](https://excalidraw.com/#json=gT-eLu8sfVfv6tCByDpMu,gZRzQuLVk7lSEmSJ25Io9g)

## Running

| Language   | Command                                      |
| ---------- | -------------------------------------------- |
| C          | `cc 0-C/main.c -o /tmp/demo && /tmp/demo`    |
| Go         | `cd 1-Go && go run main.go`                  |
| Python     | `python3 2-Python/main.py`                   |
| TypeScript | `node 3-Typescript/main.ts`                  |
| Zig        | `zig run 4-Zig/main.zig`                      |
| Rust       | `rustc 5-Rust/main.rs -o /tmp/demo && /tmp/demo` |

TypeScript runs directly on Node ≥ 22 (native type stripping); no build step.
