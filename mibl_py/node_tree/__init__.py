import bpy
from bpy.utils import register_class, unregister_class
from .mi_node_tree import TREE_NAME
from bpy.app import handlers
import os
import importlib
import inspect


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


def execute_active_node_tree():
    print("Trigger node tree execute")
    node_editor = next((a for a in bpy.context.screen.areas if a.type == 'NODE_EDITOR'), None)
    if node_editor is None:
        return
    for space in node_editor.spaces:
        node_tree = getattr(space, 'node_tree')
        if (node_tree):
            node_tree.execute(bpy.context)
            break


def register():
    classes = query_all_classes()
    for cls in classes:
        print("Register class : ", cls.__name__)
        register_class(cls)


def unregister():
    classes = query_all_classes()
    for cls in reversed(classes):
        print("Unregister class : ", cls.__name__)
        unregister_class(cls)
