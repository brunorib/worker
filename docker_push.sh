#!/bin/bash -e

REGISTRY_URL=docker.io
TARGET_IMAGE="${REGISTRY_URL}/${DOCKER_REPO}"
VERSION=$(cat Cargo.toml | grep version | sed -E 's/.*"(.*)"/\1/g')
echo $VERSION
TARGET_IMAGE_VERSIONED="${TARGET_IMAGE}:${VERSION}"

# Push image to docker hub
###################

docker login -u ${DOCKER_HUB_USER} -p ${DOCKER_HUB_PASS}
# update latest version
docker tag ${DOCKER_REPO} ${TARGET_IMAGE_VERSIONED}
docker push ${TARGET_IMAGE_VERSIONED}