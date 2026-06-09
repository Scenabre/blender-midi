import bpy
from bpy.types import Operator, AddonPreferences
from bpy.props import StringProperty, IntProperty, BoolProperty

class MI_BL_AddonPreferences(AddonPreferences):
    # This must match the add-on name, use `__package__`
    # when defining this for add-on extensions or a sub-module of a Python package.
    bl_idname = 'bl_ext.user_default.midi_interactive_bl'

    debug: BoolProperty(
        name="Debug",
        default=False,
    )

    def draw(self, context):
        layout = self.layout
        layout.label(text="Want to print debug messages in console ?")
        layout.prop(self, "debug")
