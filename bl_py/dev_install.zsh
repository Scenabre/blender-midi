#!/bin/zsh

addon_id=$(grep -m 1 id blender_manifest.toml | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
addon_version=$(grep -m 1 version blender_manifest.toml | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')

echo "Removed already installed extension"
$1/blender --command extension remove $addon_id
echo "Building new extension"
$1/blender --command extension build
echo "Installing new extension"
$1/blender --command extension install-file -r user_default $addon_id-$addon_version.zip
