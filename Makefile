# --- Tools ---
CURL := curl -sSL
UNZIP := unzip -q -j
JAVA := java -jar
TRUNK := trunk
CARGO := cargo

# --- Versions & locations ---
GENERATOR_VERSION := 7.16.0
GENERATOR_URL := https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli
GENERATOR_JAR := build/openapi-generator-cli-${GENERATOR_VERSION}.jar

OPENAPI_SPEC := openapi/openapi.yaml
OPENAPI_OUT_BACKEND := backend/target/generated/openapi
OPENAPI_OUT_FRONTEND := frontend/target/generated/openapi

SWAGGER_UI_VERSION := 5.17.14
SWAGGER_UI_ZIP := build/swagger-ui-${SWAGGER_UI_VERSION}.zip
SWAGGER_UI_URL := https://github.com/swagger-api/swagger-ui/archive/refs/tags/v${SWAGGER_UI_VERSION}.zip
SWAGGER_UI_DEST := backend/target/static/swagger-ui
CUSTOM_HTML := swagger/index.html

FRONTEND_DIR := frontend
BACKEND_DIR := backend
FRONTEND_DIST := $(BACKEND_DIR)/target/static/frontend

# --- Default target ---
.DEFAULT_GOAL := build

#====================
# Setup and Downloads
#====================

$(GENERATOR_JAR):
	@mkdir -p $(dir $(GENERATOR_JAR))
	@$(CURL) -o $(GENERATOR_JAR) \
		"$(GENERATOR_URL)/$(GENERATOR_VERSION)/openapi-generator-cli-$(GENERATOR_VERSION).jar"

$(SWAGGER_UI_ZIP):
	@mkdir -p $(dir $(SWAGGER_UI_ZIP))
	@$(CURL) -o $(SWAGGER_UI_ZIP) "$(SWAGGER_UI_URL)"

#================
# Code Generation
#================

generate-openapi-backend: $(GENERATOR_JAR)
	@if [ ! -d "$(OPENAPI_OUT_BACKEND)/src" ] || \
	    [ "$(OPENAPI_SPEC)" -nt "$(OPENAPI_OUT_BACKEND)" ] || \
	    [ ! -f "$(OPENAPI_OUT_BACKEND)/.generator_version" ] || \
	    [ "$$(cat $(OPENAPI_OUT_BACKEND)/.generator_version)" != "$(GENERATOR_VERSION)" ]; then \
	    echo "Generating Rust server stubs from $(OPENAPI_SPEC)..."; \
	    mkdir -p "$(OPENAPI_OUT_BACKEND)"; \
	    $(JAVA) $(GENERATOR_JAR) generate \
	        -i "$(OPENAPI_SPEC)" \
	        -g rust-server \
	        -o "$(OPENAPI_OUT_BACKEND)" \
	        --skip-validate-spec \
	        --additional-properties=useSwaggerUI=true >/dev/null; \
	    echo "$(GENERATOR_VERSION)" > "$(OPENAPI_OUT_BACKEND)/.generator_version"; \
	    echo "OpenAPI code regenerated (v$(GENERATOR_VERSION))"; \
	else \
	    echo "OpenAPI sources are up-to-date (no changes)."; \
	fi

generate-openapi-frontend:
	$(JAVA) -jar $(GENERATOR_JAR) generate \
	    -i $(OPENAPI_SPEC) \
	    -g rust \
	    -o "$(OPENAPI_OUT_FRONTEND)" \
	    --skip-validate-spec \
	    --additional-properties=packageName=api_client,library=reqwest


swagger-ui: $(SWAGGER_UI_ZIP)
	@if [ -f "$(SWAGGER_UI_DEST)/.version" ] && \
	    [ "$$(cat $(SWAGGER_UI_DEST)/.version)" = "$(SWAGGER_UI_VERSION)" ]; then \
	    echo "Swagger UI v$(SWAGGER_UI_VERSION) already installed."; \
	else \
	    echo "Installing/Updating Swagger UI to v$(SWAGGER_UI_VERSION)..."; \
	    rm -rf "$(SWAGGER_UI_DEST)"; \
	    mkdir -p "$(SWAGGER_UI_DEST)"; \
	    $(UNZIP) -o "$(SWAGGER_UI_ZIP)" "swagger-ui-$(SWAGGER_UI_VERSION)/dist/*" -d "$(SWAGGER_UI_DEST)" >/dev/null; \
	    cp "$(CUSTOM_HTML)" "$(SWAGGER_UI_DEST)/index.html"; \
	    echo "$(SWAGGER_UI_VERSION)" > "$(SWAGGER_UI_DEST)/.version"; \
	    echo "Swagger UI v$(SWAGGER_UI_VERSION) installed at $(SWAGGER_UI_DEST)"; \
	fi


#===================
# Frontend & Backend
#===================

FRONTEND_SRC := $(shell find $(FRONTEND_DIR)/src -type f)

$(FRONTEND_DIST)/index.html: $(FRONTEND_SRC)
	cd $(FRONTEND_DIR) && $(TRUNK) build --release --dist ../$(FRONTEND_DIST)

frontend: $(FRONTEND_DIST)/index.html
	@mkdir -p $(FRONTEND_DIST)
	cd $(FRONTEND_DIR) && $(TRUNK) build --release --dist ../$(FRONTEND_DIST)
	@if [ -f "$(FRONTEND_DIST)/index.html" ]; then \
	    echo "Frontend built successfully at $(FRONTEND_DIST)"; \
	else \
	    echo "ERROR: Frontend build failed (index.html not found in $(FRONTEND_DIST))"; \
	    exit 1; \
	fi

backend:
	cd $(BACKEND_DIR) && $(CARGO) build --release

#=======================
# Combined Build Targets
#=======================

build: generate-openapi-backend generate-openapi-frontend swagger-ui frontend backend
	cargo build --release

run:
	cd $(BACKEND_DIR) && $(CARGO) run

#==============
# Clean Targets
#==============

clean:
	rm -rf target
	rm -rf backend/target
	rm -rf frontend/target

#==============
# Dev Utilities
#==============

dev-frontend:
	cd $(FRONTEND_DIR) && $(TRUNK) serve --open --port 3000

dev-backend:
	cd $(BACKEND_DIR) && $(CARGO) run
