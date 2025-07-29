#!/bin/bash

# Build script for isobox with optimized caching
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
IMAGE_NAME="isobox"
TAG=${1:-latest}
CACHE_FROM=""

echo -e "${BLUE}🏗️  Building isobox with optimized caching...${NC}"

# Check if we have a previous build to use as cache
if docker images | grep -q "${IMAGE_NAME}:${TAG}"; then
    echo -e "${YELLOW}📦 Found existing image, using as cache source...${NC}"
    CACHE_FROM="--cache-from ${IMAGE_NAME}:${TAG}"
fi

# Build with BuildKit for better caching
export DOCKER_BUILDKIT=1

echo -e "${BLUE}🔨 Building multi-stage image...${NC}"

# Build the image with optimized caching
docker build \
    --progress=plain \
    --tag "${IMAGE_NAME}:${TAG}" \
    --tag "${IMAGE_NAME}:cache" \
    ${CACHE_FROM} \
    --build-arg BUILDKIT_INLINE_CACHE=1 \
    .

echo -e "${GREEN}✅ Build completed successfully!${NC}"
echo -e "${BLUE}📊 Image info:${NC}"
docker images "${IMAGE_NAME}:${TAG}" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedAt}}"

# Show build cache usage
echo -e "${BLUE}💾 Cache layers:${NC}"
docker history "${IMAGE_NAME}:${TAG}" --format "table {{.CreatedBy}}\t{{.Size}}" | head -10

echo -e "${GREEN}🚀 Ready to run: docker run -p 8000:8000 -v /var/run/docker.sock:/var/run/docker.sock -v /tmp:/tmp ${IMAGE_NAME}:${TAG}${NC}" 