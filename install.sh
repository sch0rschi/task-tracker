#!/usr/bin/env bash
set -euo pipefail

# =========================
# Versions & locations
# =========================
GENERATOR_VERSION="7.16.0"
GENERATOR_URL="https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli"
GENERATOR_JAR="build/openapi-generator-cli.jar"

OPENAPI_SPEC="openapi/openapi.yaml"
OPENAPI_OUT="target/generated/openapi"

SWAGGER_UI_VERSION="5.17.14"
SWAGGER_UI_ZIP="build/swagger-ui.zip"
SWAGGER_UI_URL="https://github.com/swagger-api/swagger-ui/archive/refs/tags/v${SWAGGER_UI_VERSION}.zip"
SWAGGER_UI_DEST="target/static/swagger-ui"

# =========================
# Ensure directories exist
# =========================
mkdir -p "$(dirname "$GENERATOR_JAR")"
mkdir -p "$OPENAPI_OUT"

# =========================
# Download OpenAPI Generator if missing
# =========================
if [ ! -f "$GENERATOR_JAR" ]; then
  echo "🌐 Downloading OpenAPI Generator $GENERATOR_VERSION ..."
  curl -sSL -o "$GENERATOR_JAR" \
    "$GENERATOR_URL/$GENERATOR_VERSION/openapi-generator-cli-$GENERATOR_VERSION.jar"
fi

# =========================
# Generate Rust server code
# =========================
if [ ! -d "$OPENAPI_OUT/src" ] || [ "$OPENAPI_SPEC" -nt "$OPENAPI_OUT" ]; then
  echo "⚙️  Generating Rust server code from $OPENAPI_SPEC ..."
  java -jar "$GENERATOR_JAR" generate \
    -i "$OPENAPI_SPEC" \
    -g rust-server \
    -o "$OPENAPI_OUT" \
    --skip-validate-spec \
    --additional-properties=useSwaggerUI=true
else
  echo "✅ Generated OpenAPI sources are up-to-date."
fi

# =========================
# Download Swagger UI if missing or outdated
# =========================
if [ ! -f "$SWAGGER_UI_ZIP" ]; then
  echo "🌐 Downloading Swagger UI v${SWAGGER_UI_VERSION} ..."
  curl -sSL -o "$SWAGGER_UI_ZIP" "$SWAGGER_UI_URL"
fi

if [ ! -d "$SWAGGER_UI_DEST" ]; then
  mkdir -p "$SWAGGER_UI_DEST"
  unzip -q -j "$SWAGGER_UI_ZIP" "swagger-ui-${SWAGGER_UI_VERSION}/dist/*" -d "$SWAGGER_UI_DEST"
  echo "✅ Swagger UI v${SWAGGER_UI_VERSION} installed to ${SWAGGER_UI_DEST}"

  # =========================
  # Replace Swagger UI index.html with project custom version
  # =========================
  CUSTOM_HTML="swagger/index.html"
  echo "🩹 Replacing Swagger UI index.html with ${CUSTOM_HTML}"
  cp "$CUSTOM_HTML" "$SWAGGER_UI_DEST/index.html"
else
  echo "✅ Swagger UI assets are up-to-date."
fi

# =========================
# Build project
# =========================
echo "🚀 Running cargo build ..."
cargo build "$@"
