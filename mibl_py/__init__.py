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

def register():
    faulthandler.enable(all_threads=True)
    global update_func

    print("----- Register plugin MiBL -----")
    print(datetime.datetime.now())
    print("-----")

    attr_fn = query_all_modules('register')

    attr_fn['prefs']()

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

    attr_fn['prefs']()

    faulthandler.disable()

    print("---- Unregistering done ! ----")


if __name__ == "__main__":
    register()
