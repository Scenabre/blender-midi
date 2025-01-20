from bpy.utils import register_class, unregister_class
from bpy.types import NODE_MT_add

import os
import importlib
import inspect

from mi_ui import menu_func


def query_all_classes():
    current_dir = os.path.dirname(__file__)

    classes = []

    for filename in os.listdir(current_dir):
        if filename.endswith('.py') and filename != '__init__.py':
            module_name = f"{__name__}.{filename[:-3]}"
            module = importlib.import_module(module_name)

            for name, obj in inspect.getmembers(module, inspect.isclass):
                if obj.__module__ == module_name:
                    classes.append(obj)

    return classes


def register():
    classes = query_all_classes()
    for cls in classes:
        register_class(cls)
    NODE_MT_add.prepend(menu_func)


def unregister():
    classes = query_all_classes()
    for cls in reversed(classes):
        unregister_class(cls)
    NODE_MT_add.remove(menu_func)
