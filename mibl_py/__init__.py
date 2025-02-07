bl_info = {
    "name": "Midi Interactive",
    "blender": (4, 4, 0),
    "category": "Node",
}

import bpy
import subprocess
import sys
import os
import pkgutil
import importlib


def install_mibllib():
    if not check_if_exists("mibllib"):
        whl_name = "mibllib-0.1.0-cp311-cp311-manylinux_2_34_x86_64.whl"
        whl_path = os.path.join(os.path.dirname(__file__), "wheels", whl_name)

        pip_cmd = [sys.executable,
                   "-m",
                   "pip",
                   "install",
                   whl_path
                   ]

        subprocess.check_call(pip_cmd)


def check_if_exists(name) -> bool:
    try:
        importlib.util.find_spec(name)
    except Exception as e:
        print("Exception occured when searching for module {} : {}"
              .format(name, e))
        raise e
    else:
        if importlib.util.find_spec(name) is None:
            print("Module {} not found".format(name))
            return False
        print("Module found")
        return True


def query_all_modules(attr):
    current_dir = os.path.dirname(__file__)

    for _, module_name, is_pkg in pkgutil.iter_modules([current_dir]):
        if is_pkg:
            module = importlib.import_module(f"{__name__}.{module_name}")
            if hasattr(module, attr):
                print("Query module : ", module)
                module.register()


# def menu_func(self, context):
#     self.layout.operator(my_custom_node.MyCustomTestNode.bl_idname)


def register():
    # install_mibllib() # Test all the code then uncomment !
    query_all_modules('register')


def unregister():
    query_all_modules('unregister')


if __name__ == "__main__":
    register()
