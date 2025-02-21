import bpy
from bpy.types import Node
from .. node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_group_output(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_group_output'
    bl_label = 'MI Group Output'

    def init(self, context):
        base_socket_name = 'MI_BL_Channel_In_'
        for chan in range(1, 17, 1):
            socket_name = f'{base_socket_name}{chan}'
            self.inputs.new(
                'SOCKET_MI_BL_Midi_Recipe',
                socket_name,
                use_multi_input=False
            )

    def execute(self):
        mibl_props = bpy.context.scene.mibl
        active_inputs = []
        current_channels = list(mibl_props.mi_midi_channel)

        for idx, input in enumerate(self.inputs):
            if input.is_linked and idx not in current_channels:
                active_inputs.append(idx+1)

        active_inputs.sort()

        while len(active_inputs) < 16:
            active_inputs.append(0)

        mibl_props.mi_midi_channel = active_inputs
        # scene = bpy.context.scene
        # inputs = self.get_linked(self.inputs)
        #
        # if inputs:
        #     inputs[0] = scene.mibl.mi_output_mesg

    def update(self):
        self.execute()
