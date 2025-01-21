from bpy import context as C
from bpy.types import Node


class NODE_MI_BL_group_input(Node):
    bl_idname = 'NODE_MI_BL_group_input'
    bl_label = 'MI Group Input'
    bl_icon = 'INFO'

    def init(self, context):
        self.outputs.new(
            'NodeSocketVector',
            "MI_BL_Server_In",
            use_multi_input=False
        )

    def update(self):
        scene = C.scene
        self.outputs['MI_BL_Server_In'].default_value = scene.mi_input_mesg

    def draw_buttons(self, context, layout):
        scene = C.scene
        layout.prop(
            scene,
            "MI_BL_Server_In",
            text="Midi Server In"
        )
