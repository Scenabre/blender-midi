from bpy.utils import register_class, unregister_class
from bpy.types import NODE_MT_add
import nodeitems_utils

import os
import importlib
import inspect

from .mi_ui import node_categories


def query_all_classes():
    current_dir = os.path.dirname(__file__)

    classes = []

    for filename in os.listdir(current_dir):
        if filename.endswith('.py') and filename != '__init__.py':
            module_name = f"{__name__}.{filename[:-3]}"
            module = importlib.import_module(module_name)

            for name, obj in inspect.getmembers(module, inspect.isclass):
                print("Query class : ", name)
                if obj.__module__ == module_name and hasattr(obj, 'bl_idname'):
                    classes.append(obj)

    return classes


def register():
    classes = query_all_classes()
    for cls in classes:
        print("Register class : ", cls.__name__)
        register_class(cls)

    nodeitems_utils.register_node_categories('MIBL_NODES', node_categories)


def unregister():
    classes = query_all_classes()
    nodeitems_utils.unregister_node_categories('MIBL_NODES')

    for cls in reversed(classes):
        print("Unregister class : ", cls.__name__)
        unregister_class(cls)
