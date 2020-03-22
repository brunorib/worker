#!/bin/bash -e

REGISTRY_URL=docker.io
TARGET_IMAGE="${REGISTRY_URL}/${DOCKER_REPO}"
VERSION=$(cat Cargo.toml | grep version | sed -E 's/.*"(.*)"/\1/g')
echo $VERSION
TARGET_IMAGE_VERSIONED="${TARGET_IMAGE}:${VERSION}"

# Push image to docker hub
###################

docker login -u brunoribca -p a5de0edd-2370-4dd1-9cbe-63f6bf40be0f
# update latest version
docker tag ${DOCKER_REPO} ${TARGET_IMAGE_VERSIONED}
docker push ${TARGET_IMAGE_VERSIONED}