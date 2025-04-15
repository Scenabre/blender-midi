#!/bin/zsh

if [ $# -gt 2 ]; then
  echo "Usage : $0 blender_install_dir [-f]"
  exit
fi

cd mibl_py

addon_id=$(grep -m 1 id blender_manifest.toml | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
addon_version=$(grep -m 1 version blender_manifest.toml | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')

if [ $2 = "-f" ]; then
  source /home/lynerlok/.pyenv/versions/py_env/bin/activate
  cd ../mibl_rs
  maturin build
  cp target/wheels/mibllib-0.1.0-cp311-cp311-manylinux_2_34_x86_64.whl ../mibl_py/wheels/
  deactivate
  cd ../mibl_py
fi

echo "Removed already installed extension"
$1/blender --command extension remove $addon_id
echo "Building new extension"
$1/blender --command extension build
echo "Installing new extension"
$1/blender --command extension install-file -r user_default $addon_id-$addon_version.zip
