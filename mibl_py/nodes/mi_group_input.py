import bpy
import random
from bpy.types import Node
from bpy.props import FloatProperty
from .. node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_group_input(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_group_input'
    bl_label = 'MI Group Input'
    # bl_icon = 'INFO'

    def init(self, context):
        # channel_list = list(bpy.context.scene.mibl.mi_midi_channel)
        base_socket_name = 'MI_BL_Channel_Out_'
        for chan in range(1, 17, 1):
            socket_name = f'{base_socket_name}{chan}'
            self.outputs.new(
                'SOCKET_MI_BL_Midi_Recipe',
                socket_name,
                use_multi_input=False
            )

    def execute(self):
        pass

        # scene = bpy.context.scene
        # self.outputs['MI_BL_Server_In'].default_value = scene.mibl.mi_input_mesg

    def update(self):
        self.execute()
