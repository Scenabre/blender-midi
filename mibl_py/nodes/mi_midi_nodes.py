import bpy
from mibllib import *
from bpy.types import Node
from bpy.props import EnumProperty
from .. node_tree.mi_node_tree import MI_BL_Node

class NODE_MI_BL_MIDI_Params(Node, MI_BL_Node):
    '''MiBl Midi Server Parameters'''
    bl_idname = 'NODE_MI_BL_MIDI_Params'
    bl_label = 'MI Midi Params'
    bl_icon = 'NODETREE'

    channel_enum: EnumProperty(
        name='',
        items=(
            ('1', "Channel 1", ""),
            ('2', "Channel 2", ""),
            ('3', "Channel 3", ""),
            ('4', "Channel 4", ""),
            ('5', "Channel 5", ""),
            ('6', "Channel 6", ""),
            ('7', "Channel 7", ""),
            ('8', "Channel 8", ""),
            ('9', "Channel 9", ""),
            ('10', "Channel 10", ""),
            ('11', "Channel 11", ""),
            ('12', "Channel 12", ""),
            ('13', "Channel 13", ""),
            ('14', "Channel 14", ""),
            ('15', "Channel 15", ""),
            ('16', "Channel 16", ""),
        ),
        default='1'
    )

    def init(self, context):
        self.outputs.new('NodeSocketInt', "Target channel")
        self.outputs.new('NodeSocketVectorInt', "Midi Message In")
        self.outputs.new('NodeSocketInt', "Time Stamp")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        if mibl_props.mi_run_server:
            row.operator("mibl.set_server_state", text="Stop Midi Server")
        else:
            row.operator("mibl.set_server_state", text="Start Midi Server")

        layout.label(text="Server controls :")
        layout.prop(mibl_props, 'mi_use_system_ctlr', text="Use MC system control")

        layout.label(text="Server Channel")
        layout.prop(self, "channel_enum")

    def execute(self):
        midi_channel = self.channel_enum
        bpy.context.scene.mibl.mi_midi_channel = midi_channel
        print(bpy.context.scene.mibl.mi_midi_channel)

    def update(self):
        self.execute()
