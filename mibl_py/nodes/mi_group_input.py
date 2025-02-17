import bpy
from bpy.types import Node
from bpy.props import FloatProperty
from ..node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_group_input(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_group_input'
    bl_label = 'MI Group Input'
    # bl_icon = 'INFO'

    def init(self, context):
        self.outputs.new(
            'NodeSocketVector',
            "MI_BL_Server_In",
            use_multi_input=False
        )

    def execute(self):
        scene = bpy.context.scene
        self.outputs['MI_BL_Server_In'].default_value = scene.mi_input_mesg

    def update(self):
        self.execute()

    def draw_buttons(self, context, layout):
        scene = context.scene
        layout.prop(
            scene,
            "MI_BL_Server_In",
            text="Midi Server In"
        )
