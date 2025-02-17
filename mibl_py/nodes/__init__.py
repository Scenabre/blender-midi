import bpy
import os
import importlib
import inspect
from bpy.utils import register_class, unregister_class
from ..node_tree import execute_active_node_tree
from .mi_attr_nodes import NODE_MI_BL_set_attr
from .mi_input_nodes import NODE_MI_BL_value_input


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
    for idx, cls in enumerate(classes):
        print("Register class : ", cls.__name__)
        register_class(cls)

        if hasattr(cls, 'custom_idx'):
            print("Found custom_idx in Class : ", cls.__name__)

    bpy.msgbus.subscribe_rna(
        key=NODE_MI_BL_value_input,
        owner=0,
        args=(),
        notify=execute_active_node_tree,
        options={"PERSISTENT", }
    )

    bpy.msgbus.subscribe_rna(
        key=NODE_MI_BL_set_attr,
        owner=1,
        args=(),
        notify=execute_active_node_tree,
        options={"PERSISTENT", }
    )


def unregister():
    classes = query_all_classes()
    for cls in reversed(classes):
        print("Unregister class : ", cls.__name__)

        if hasattr(cls, 'custom_idx'):
            print("Unregister msgbus")
            bpy.msgbus.clear_by_owner(cls.custom_idx)

        unregister_class(cls)
