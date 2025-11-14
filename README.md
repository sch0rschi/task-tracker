# Simple TODO List

This is a simple TODO list project with a structured build system.

## Features

-   Backend and frontend code generation using OpenAPI.
-   Rust-based backend.
-   Rust (Trunk) frontend application.
-   Swagger UI integration.
-   Docker-based local development environment.

## Build Instructions

To build the entire project:

``` sh
make build
```

## Run Backend

``` sh
make run
```

## Development Mode

Start frontend and backend with live reload:

``` sh
make dev-run
```

## Clean

``` sh
make clean
```

## Project Structure

-   `backend/`       -- Rust backend service
-   `frontend/`      -- Rust/Trunk-based web frontend
-   `openapi/`       -- OpenAPI specification
-   `local-support/` -- Docker-based local environment
-   `swagger/`       -- Static context for swagger

