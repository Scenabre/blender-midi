from bpy import context as C
from bpy.types import Node


class NODE_MI_BL_group_output(Node):
    bl_idname = 'NODE_MI_BL_group_output'
    bl_label = 'MI Group Output'
    bl_icon = 'INFO'

    def init(self, context):
        self.inputs.new(
            'NodeSocketVector',
            "MI_BL_Server_Out",
            use_multi_input=False
        )

    def update(self):
        scene = C.scene
        self.inputs['MI_BL_Server_Out'].default_value = scene.mi_output_mesg

    def draw_buttons(self, context, layout):
        scene = C.scene
        layout.prop(
            scene,
            "MI_BL_Server_Out",
            text="Midi Server Out"
        )
