bl_info = {
    "name": "My Custom Node Addon",
    "blender": (4, 4, 0),
    "category": "Node",
}

import bpy
import subprocess
import sys
import os
from . import my_custom_node

# Chemin vers le fichier .whl
wheel_path = os.path.join(os.path.dirname(__file__), "wheels", "my_rust_lib-0.1.0-cp311-cp311-manylinux_2_34_x86_64.whl")


# Installer le module my_rust_lib
def install_my_rust_lib():
    subprocess.check_call([sys.executable, "-m", "pip", "install", "--force-reinstall", wheel_path])


def menu_func(self, context):
    self.layout.operator(my_custom_node.MyCustomTestNode.bl_idname)


def register():
    install_my_rust_lib()
    my_custom_node.register()


def unregister():
    my_custom_node.unregister()


if __name__ == "__main__":
    register()
