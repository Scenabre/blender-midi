import bpy
from bpy.types import Node
from .. node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_group_output(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_group_output'
    bl_label = 'MI Group Output'

    def init(self, context):
        self.inputs.new(
            'NodeSocketVector',
            "MI_BL_Server_Out",
            use_multi_input=False
        )

    def execute(self):
        scene = bpy.context.scene
        inputs = self.get_linked(self.inputs)

        if inputs:
            inputs[0] = scene.mibl.mi_output_mesg

    def update(self):
        self.execute()
