# ===============================
#  Project Build & Development
# ===============================

# --- Tools ---
CURL              := curl -sSL
UNZIP             := unzip -q -j
JAVA              := java -jar
TRUNK             := trunk
CARGO             := cargo
RM                := rm -rf
MKDIR             := mkdir -p
ECHO              := echo
CD                := cd
TOUCH             := touch
CP                := cp
MAKE              := make
DOCKER_COMPOSE_UP := docker compose up -d

# --- Versions & Locations ---
GENERATOR_VERSION     := 7.16.0
GENERATOR_URL         := https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli
GENERATOR_JAR         := build/openapi-generator-cli-$(GENERATOR_VERSION).jar

SWAGGER_UI_VERSION    := 5.17.14
SWAGGER_UI_ZIP        := build/swagger-ui-$(SWAGGER_UI_VERSION).zip
SWAGGER_UI_URL        := https://github.com/swagger-api/swagger-ui/archive/refs/tags/v$(SWAGGER_UI_VERSION).zip
SWAGGER_UI_DEST       := target/static/swagger-ui
CUSTOM_HTML           := swagger/index.html

OPENAPI_SPEC          := openapi/openapi.yaml
OPENAPI_BACKEND_OUT   := target/generated/backend/openapi
OPENAPI_BACKEND_STAMP := $(OPENAPI_BACKEND_OUT)/.stamp
OPENAPI_FRONTEND_OUT  := target/generated/frontend/openapi
OPENAPI_FRONTEND_STAMP:= $(OPENAPI_FRONTEND_OUT)/.stamp

SWAGGER_STAMP         := $(SWAGGER_UI_DEST)/.stamp

BACKEND_DIR           := backend
FRONTEND_DIR          := frontend
FRONTEND_DIST         := target/static/frontend
FRONTEND_STAMP        := $(FRONTEND_DIST)/.stamp

LOCAL_SUPPORT_DIR     := local-support

# --- Default target ---
.DEFAULT_GOAL := build

# =====================
# Setup & Downloads
# =====================

$(GENERATOR_JAR):
	@$(MKDIR) $(dir $@)
	@$(ECHO) "📦 Downloading OpenAPI Generator v$(GENERATOR_VERSION)..."
	@$(CURL) -o $@ "$(GENERATOR_URL)/$(GENERATOR_VERSION)/openapi-generator-cli-$(GENERATOR_VERSION).jar"

$(SWAGGER_UI_ZIP):
	@$(MKDIR) $(dir $@)
	@$(ECHO) "📦 Downloading Swagger UI v$(SWAGGER_UI_VERSION)..."
	@$(CURL) -o $@ "$(SWAGGER_UI_URL)"

# =====================
# OpenAPI Code Generation
# =====================

.PHONY: openapi openapi-backend openapi-frontend

openapi: openapi-backend openapi-frontend

openapi-backend: $(OPENAPI_BACKEND_STAMP)
$(OPENAPI_BACKEND_STAMP): $(GENERATOR_JAR) $(OPENAPI_SPEC)
	@$(MKDIR) $(dir $@)
	@$(ECHO) "🔧 Generating Rust backend server from $(OPENAPI_SPEC)..."
	@$(RM) "$(OPENAPI_BACKEND_OUT)"
	@$(JAVA) $(GENERATOR_JAR) generate \
	    -i "$(OPENAPI_SPEC)" \
	    -g rust-server \
	    -o "$(OPENAPI_BACKEND_OUT)" \
	    --skip-validate-spec \
	    --additional-properties=useSwaggerUI=true >/dev/null
	@$(TOUCH) $@
	@$(ECHO) "✅ Backend OpenAPI stubs generated → $(OPENAPI_BACKEND_OUT)"

openapi-frontend: $(OPENAPI_FRONTEND_STAMP)
$(OPENAPI_FRONTEND_STAMP): $(GENERATOR_JAR) $(OPENAPI_SPEC)
	@$(MKDIR) $(dir $@)
	@$(ECHO) "🔧 Generating Rust API client from $(OPENAPI_SPEC)..."
	@$(RM) "$(OPENAPI_FRONTEND_OUT)"
	@$(JAVA) $(GENERATOR_JAR) generate \
	    -i "$(OPENAPI_SPEC)" \
	    -g rust \
	    -o "$(OPENAPI_FRONTEND_OUT)" \
	    --skip-validate-spec \
	    --additional-properties=packageName=api_client,library=reqwest >/dev/null
	@$(TOUCH) $@
	@$(ECHO) "✅ Frontend OpenAPI client generated → $(OPENAPI_FRONTEND_OUT)"

# =====================
# Swagger UI
# =====================

.PHONY: swagger-ui

swagger-ui: $(SWAGGER_STAMP)
$(SWAGGER_STAMP): $(SWAGGER_UI_ZIP) $(CUSTOM_HTML)
	@$(MKDIR) $(dir $@)
	@$(ECHO) "📦 Installing Swagger UI v$(SWAGGER_UI_VERSION)..."
	@$(RM) "$(SWAGGER_UI_DEST)"
	@$(MKDIR) "$(SWAGGER_UI_DEST)"
	@$(UNZIP) -o "$(SWAGGER_UI_ZIP)" "swagger-ui-$(SWAGGER_UI_VERSION)/dist/*" -d "$(SWAGGER_UI_DEST)" >/dev/null
	@$(CP) "$(CUSTOM_HTML)" "$(SWAGGER_UI_DEST)/index.html"
	@$(TOUCH) $@
	@$(ECHO) "✅ Swagger UI installed → $(SWAGGER_UI_DEST)"

# =====================
# Frontend & Backend
# =====================

.PHONY: frontend backend

frontend: $(FRONTEND_STAMP)
$(FRONTEND_STAMP): $(shell find $(FRONTEND_DIR)/src -type f) $(OPENAPI_SPEC)
	@$(MKDIR) $(FRONTEND_DIST)
	@$(ECHO) "🚀 Building frontend..."
	@$(CD) $(FRONTEND_DIR) && $(TRUNK) build --dist ../$(FRONTEND_DIST)
	@$(TOUCH) $@
	@$(ECHO) "✅ Frontend built → $(FRONTEND_DIST)"

backend:
	@$(ECHO) "🚀 Building backend..."
	@$(CD) $(BACKEND_DIR) && $(CARGO) build

# =====================
# Combined Targets
# =====================

.PHONY: build run clean dev-frontend dev-backend

build: openapi swagger-ui frontend backend
	@$(ECHO) "🎉 Build completed successfully!"

run: build
	@$(ECHO) "🏃 Running backend..."
	@$(CD) $(BACKEND_DIR)/infrastructure && $(CARGO) run

dev-run:
	@$(CD) $(LOCAL_SUPPORT_DIR) && $(DOCKER_COMPOSE_UP)
	@$(MAKE) dev-frontend
	@$(MAKE) dev-backend

# =====================
# Cleanup
# =====================

clean:
	@$(ECHO) "🧹 Cleaning build artifacts..."
	@$(RM) target
	@$(ECHO) "✅ Clean complete!"

# =====================
# Dev Utilities
# =====================

dev-frontend:
	@$(ECHO) "🌐 Starting frontend dev server..."
	@$(CD) $(FRONTEND_DIR) && $(TRUNK) serve --open --port 3000

dev-backend:
	@$(ECHO) "🧩 Running backend in dev mode..."
	@$(CD) $(BACKEND_DIR)/infrastructure && $(CARGO) run
