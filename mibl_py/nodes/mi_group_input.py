import bpy
from bpy.types import Node


class NODE_MI_BL_group_input(Node):
    bl_idname = 'MidiInteractiveGroupInput'
    bl_label = 'MI Group Input'
    bl_icon = 'INFO'

    def init(self, context):
        self.outputs.new('NodeSocketString', "MIDI Server Value")

    def update(self):
        scene = bpy.context.scene
        self.outputs['MIDI Server Value'].default_value = scene.mi_output_mesg

    def draw_buttons(self, context, layout):
        scene = context.scene
        layout.prop(scene, "midi_server_value", text="MIDI Value")
