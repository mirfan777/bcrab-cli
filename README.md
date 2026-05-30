# BCRAB CLI - Resource Generator

A powerful Rust CLI tool for generating boilerplate Axum code with complete DDD architecture scaffolding.

## Installation

```bash
cd bcrab-cli
cargo build --release
```

The binary will be available at `target/release/boil.exe` on Windows or `target/release/boil` on Unix.

## Commands

### `boil make resources <Name>`

Generates a complete resource with all boilerplate code following DDD architecture.

**Example:**
```bash
boil make resources Admin
boil make resources Product
boil make resources Order
```

**Generated Files (7 total):**

1. **DTO** - `src/application/dtos/{name}.rs`
   - Serializable request/response structs
   - Includes ID, created_at, updated_at fields
   
2. **Service** - `src/application/services/{name}_service.rs`
   - Business logic layer
   - CRUD methods: create, get_by_id, list, update, delete
   - Uses async/await pattern
   
3. **Entity** - `src/domain/entities/{name}.rs`
   - Core domain model
   - Pure domain object without persistence logic
   
4. **Repository Trait** - `src/domain/repositories/{name}_repository.rs`
   - Async trait defining data access interface
   - Abstraction for persistence concerns
   
5. **Repository Implementation** - `src/infrastructure/database/{name}_repository_impl.rs`
   - PostgreSQL implementation with SQLx
   - Connects to database pool
   
6. **Database Model** - `src/infrastructure/database/{name}_model.rs`
   - SQLx FromRow implementation
   - Maps database rows to domain entities
   
7. **Handlers** - `src/presentation/handlers/{name}_handlers.rs`
   - Axum HTTP handlers for REST endpoints
   - CRUD routes with proper HTTP status codes
   - Router function for composing into main app

### `boil make dto <Name>`

Generates only the DTO file without services, repositories, or handlers.

**Example:**
```bash
boil make dto Admin
boil make dto Comment
```

**Generated File:**
- `src/application/dtos/{name}.rs` - Request and Response DTOs

## Architecture

The generator follows Domain-Driven Design (DDD) architecture:

```
boiled-crab/
├── src/
│   ├── application/          # Application services & DTOs
│   │   ├── dtos/
│   │   │   └── {resource}.rs
│   │   └── services/
│   │       └── {resource}_service.rs
│   ├── domain/               # Domain logic
│   │   ├── entities/
│   │   │   └── {resource}.rs
│   │   ├── repositories/
│   │   │   └── {resource}_repository.rs
│   │   └── errors.rs
│   ├── infrastructure/       # External systems
│   │   └── database/
│   │       ├── {resource}_model.rs
│   │       └── {resource}_repository_impl.rs
│   └── presentation/         # HTTP layer
│       └── handlers/
│           └── {resource}_handlers.rs
```

## Generated Code Patterns

### DTO (Data Transfer Object)

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRequest {
    // TODO: Add request fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminResponse {
    pub id: Uuid,
    pub created_at: String,
    pub updated_at: String,
    // TODO: Add response fields
}
```

### Service

```rust
pub struct AdminService {
    repository: Arc<dyn AdminRepository>,
}

impl AdminService {
    pub async fn create(&self, req: AdminRequest) -> Result<AdminResponse, DomainError> {
        // TODO: Implement
    }
    
    pub async fn get_by_id(&self, id: Uuid) -> Result<AdminResponse, DomainError> {
        // TODO: Implement
    }
    
    pub async fn list(&self) -> Result<Vec<AdminResponse>, DomainError> {
        // TODO: Implement
    }
    
    pub async fn update(&self, id: Uuid, req: AdminRequest) -> Result<AdminResponse, DomainError> {
        // TODO: Implement
    }
    
    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        // TODO: Implement
    }
}
```

### Repository Trait

```rust
#[async_trait]
pub trait AdminRepository: Send + Sync {
    async fn create(&self, entity: Admin) -> Result<Admin, DomainError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Admin, DomainError>;
    async fn list(&self) -> Result<Vec<Admin>, DomainError>;
    async fn update(&self, id: Uuid, entity: Admin) -> Result<Admin, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
```

### Handlers

```rust
pub fn admin_routes() -> Router {
    Router::new()
        .route(
            "/",
            post(create_admin).get(list_admin),
        )
        .route(
            "/:id",
            get(get_admin)
                .put(update_admin)
                .delete(delete_admin),
        )
}
```

## Usage Steps

1. **Generate Resource:**
   ```bash
   cd bcrab-cli
   ./target/release/boil make resources Admin
   ```

2. **Update DTOs** - Add your fields:
   ```rust
   pub struct AdminRequest {
       pub name: String,
       pub email: String,
   }
   ```

3. **Update Entity** - Add domain fields:
   ```rust
   pub struct Admin {
       pub id: Uuid,
       pub name: String,
       pub email: String,
       // ... timestamps
   }
   ```

4. **Implement Service** - Replace `todo!()` with business logic

5. **Implement Repository** - Add database queries

6. **Register Handlers** - Include in your Axum router:
   ```rust
   let app = Router::new()
       .nest("/api/admins", admin_handlers::admin_routes());
   ```

## Module File Updates

The generator automatically updates `mod.rs` files (or equivalent module declaration files):

- `src/application/dtos.rs` - Adds `pub mod {name};`
- `src/application/services.rs` - Adds `mod {name}_service;`
- `src/domain/entities.rs` - Adds `pub mod {name};`
- `src/domain/repositories.rs` - Adds `pub mod {name}_repository;`
- `src/infrastructure/database.rs` - Adds model and impl modules
- `src/presentation/handlers.rs` - Adds `pub mod {name}_handlers;`

## Naming Conventions

The generator uses these naming patterns:

| Type | Format | Example |
|------|--------|---------|
| File names | snake_case | `admin.rs`, `admin_service.rs` |
| Struct names | PascalCase | `Admin`, `AdminService` |
| Function names | snake_case | `create_admin`, `get_admin` |
| Module names | snake_case | `pub mod admin;` |

## Error Handling

If you try to generate a resource that already exists:

```bash
$ boil make resources admin
[ERROR] File Already Exists: src/application/dtos/admin.rs
```

Delete the existing files first or use a different resource name.

## Customization

All generated files include `// TODO:` comments indicating where to add:

- Request/Response field definitions
- Business logic implementation
- Database query implementation
- Validation and error handling
- Proper state injection in handlers

## Development

### Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Lint

```bash
cargo clippy
```

## Troubleshooting

### "File not found" errors

The CLI must be run from the `bcrab-cli` directory. It looks for `../boiled-crab/` relative to the current directory.

### Compilation errors in generated code

Generated code uses dependencies that should be in `boiled-crab/Cargo.toml`:
- `serde` / `serde_json`
- `uuid`
- `chrono`
- `axum`
- `tokio`
- `sqlx`
- `async-trait`

## License

MIT
