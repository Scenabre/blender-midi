import bpy
from bpy.types import Node
from bpy.props import FloatProperty

class NODE_MI_BL_group_input(Node):
    bl_idname = 'NODE_MI_BL_group_input'
    bl_label = 'MI Group Input'
    # bl_icon = 'INFO'

    def init(self, context):
        self.outputs.new(
            'NodeSocketVector',
            "MI_BL_Server_In",
            use_multi_input=False
        )

    def update(self):
        scene = bpy.context.scene
        self.outputs['MI_BL_Server_In'].default_value = scene.mi_input_mesg

    def draw_buttons(self, context, layout):
        scene = context.scene
        layout.prop(
            scene,
            "MI_BL_Server_In",
            text="Midi Server In"
        )


class NODE_MI_BL_value_input(Node):
    bl_idname = 'NODE_MI_BL_value_input'
    bl_label = 'MI Value'

    float_output: FloatProperty(name="Value", default=0.0)

    def init(self, context):
        self.outputs.new('NodeSocketFloat',
                         "Value")

    def draw_buttons(self, context, layout):
        layout.prop(self, "float_output")


class NODE_MI_BL_object(Node):
    bl_idname = 'NODE_MI_BL_object'
    bl_label = 'MI Object'

    def init(self, context):
        self.inputs.new('NodeSocketObject', "Object")
        self.inputs.new('NodeSocketBool', "As Instance")
