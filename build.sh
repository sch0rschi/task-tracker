#!/usr/bin/env bash
set -euo pipefail

GENERATOR_VERSION="7.9.0"
GENERATOR_URL="https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli"
GENERATOR_JAR="build/openapi-generator-cli.jar"
OPENAPI_SPEC="openapi/openapi.yaml"
OPENAPI_OUT="target/generated/openapi"

# Ensure target directories exist
mkdir -p "$(dirname "$GENERATOR_JAR")"
mkdir -p "$OPENAPI_OUT"

# Download generator JAR if missing
if [ ! -f "$GENERATOR_JAR" ]; then
  echo "📦 Downloading OpenAPI Generator $GENERATOR_VERSION ..."
  curl -sSL -o "$GENERATOR_JAR" \
    "$GENERATOR_URL/$GENERATOR_VERSION/openapi-generator-cli-$GENERATOR_VERSION.jar"
fi

# Generate Rust server code if missing or outdated
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

# Build project normally
echo "🚀 Running cargo build ..."
cargo build "$@"
