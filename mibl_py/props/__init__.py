from bpy.utils import register_class, unregister_class
from bpy.types import Scene
from bpy.props import PointerProperty
from .mi_props import PropsMiBl


def register():
    register_class(PropsMiBl)
    Scene.mibl = PointerProperty(type=PropsMiBl)


def unregister():
    del Scene.mibl
    unregister_class(PropsMiBl)
