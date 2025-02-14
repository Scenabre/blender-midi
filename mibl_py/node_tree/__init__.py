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


def force_update_node_tree(node_tree):
    for node in node_tree.nodes:
        node.update()


def on_depsgraph_update(scene, depsgraph):
    for update in depsgraph.updates:
        if isinstance(update.id, bpy.types.NodeTree) and update.id.bl_idname == TREE_NAME:
            force_update_node_tree(update.id)


def register():
    classes = query_all_classes()
    for cls in classes:
        print("Register class : ", cls.__name__)
        register_class(cls)

    handlers.depsgraph_update_post.append(on_depsgraph_update)


def unregister():
    classes = query_all_classes()
    for cls in reversed(classes):
        print("Unregister class : ", cls.__name__)
        unregister_class(cls)

    handlers.depsgraph_update_post.remove(on_depsgraph_update)
