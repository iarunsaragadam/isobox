#!/bin/bash

# Benchmark script to measure Docker build performance
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}📊 Docker Build Performance Benchmark${NC}"
echo "=================================="

# Function to measure build time
measure_build() {
    local description=$1
    local command=$2
    
    echo -e "${YELLOW}🔨 $description${NC}"
    start_time=$(date +%s)
    
    eval "$command"
    
    end_time=$(date +%s)
    duration=$((end_time - start_time))
    
    echo -e "${GREEN}✅ Completed in ${duration}s${NC}"
    echo ""
    
    return $duration
}

# Clean up any existing images
echo -e "${BLUE}🧹 Cleaning up existing images...${NC}"
docker rmi isobox:latest isobox:deps isobox:rust-builder 2>/dev/null || true
echo ""

# First build (cold start)
echo -e "${BLUE}🔥 First build (cold start)${NC}"
measure_build "Cold build with new multi-stage Dockerfile" "make docker-build"

# Second build (with cache)
echo -e "${BLUE}🔥 Second build (with cache)${NC}"
measure_build "Warm build with cached layers" "make docker-build"

# Third build (only source code change)
echo -e "${BLUE}🔥 Third build (source code change)${NC}"
echo "Simulating source code change..."
touch src/main.rs
measure_build "Build with only source code change" "make docker-build"

echo -e "${GREEN}🎉 Benchmark completed!${NC}"
echo ""
echo -e "${BLUE}💡 Tips for faster builds:${NC}"
echo "  • Use 'make docker-build-deps' to pre-build dependency stage"
echo "  • Use 'make docker-build-rust' to pre-build Rust builder stage"
echo "  • The build script automatically uses existing images as cache"
echo "  • GitHub Actions uses GHA cache for even faster CI builds" 