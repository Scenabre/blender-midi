import os
import importlib
import inspect
from bpy.utils import register_class, unregister_class
from bpy.types import Scene
from bpy.props import PointerProperty
from .mi_props import MI_BL_Ingredient, MI_BL_Recipe, MI_BL_TriggerProp, PropsMiBl, MI_BL_VecOut, MI_BL_LcdParams, MI_BL_VPotParams, MI_BL_FaderParams, MI_BL_ChanBtnParams, MI_BL_TimestampParams, MI_BL_SysParams


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
    register_class(MI_BL_VecOut)
    register_class(MI_BL_Ingredient)
    register_class(MI_BL_Recipe)
    register_class(MI_BL_TriggerProp)
    register_class(MI_BL_LcdParams)
    register_class(MI_BL_VPotParams)
    register_class(MI_BL_FaderParams)
    register_class(MI_BL_ChanBtnParams)
    register_class(MI_BL_TimestampParams)
    register_class(MI_BL_SysParams)
    register_class(PropsMiBl)

    Scene.mibl = PointerProperty(type=PropsMiBl)


def unregister():
    del Scene.mibl

    unregister_class(PropsMiBl)
    unregister_class(MI_BL_TriggerProp)
    unregister_class(MI_BL_Recipe)
    unregister_class(MI_BL_Ingredient)
    unregister_class(MI_BL_VecOut)
    unregister_class(MI_BL_SysParams)
    unregister_class(MI_BL_TimestampParams)
    unregister_class(MI_BL_ChanBtnParams)
    unregister_class(MI_BL_FaderParams)
    unregister_class(MI_BL_VPotParams)
    unregister_class(MI_BL_LcdParams)
