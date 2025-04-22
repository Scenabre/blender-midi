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
import datetime
import faulthandler
from bpy.app import timers
from bpy.types import AddonPreferences

# class ModuleAPrefs(AddonPreferences):
#     bl_idname = __name__
#     # Put your module A preferences here


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
        print("MiBl python module already installed")
        return True


def query_all_modules(attr):
    current_dir = os.path.dirname(__file__)
    attr_fn = {}

    for _, module_name, is_pkg in pkgutil.iter_modules([current_dir]):
        if is_pkg:
            module = importlib.import_module(f"{__name__}.{module_name}")
            if hasattr(module, attr):
                print("Query module : ", module)
                module_fn = getattr(module, attr)
                attr_fn[module_name] = module_fn

    return attr_fn

# def menu_func(self, context):
#     self.layout.operator(my_custom_node.MyCustomTestNode.bl_idname)


def register():
    faulthandler.enable(all_threads=True)
    global update_func

    print("----- Register plugin MiBL -----")
    print(datetime.datetime.now())
    print("-----")
    install_mibllib()  # Test all the code then uncomment !
    attr_fn = query_all_modules('register')

    attr_fn['node_tree']()
    attr_fn['ops']()
    attr_fn['props']()
    attr_fn['sockets']()
    attr_fn['nodes']()
    attr_fn['menu_ui']()

    print("---- Registering done ! ----")


def unregister():
    global update_func

    print("----- Unregister plugin MiBL -----")
    print(datetime.datetime.now())
    print("-----")
    attr_fn = query_all_modules('unregister')

    attr_fn['nodes']()
    attr_fn['sockets']()
    attr_fn['props']()
    attr_fn['ops']()
    attr_fn['menu_ui']()
    attr_fn['node_tree']()

    faulthandler.disable()

    print("---- Unregistering done ! ----")


if __name__ == "__main__":
    register()
