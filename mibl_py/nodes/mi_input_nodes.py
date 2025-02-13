from bpy.types import Node
from bpy.props import FloatProperty


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
        self.outputs.new('NodeSocketGeometry', 'Geometry')

    def update(self):
        obj = self.inputs['Object'].default_value
        if obj:
            pass
            # self.outputs['Geometry'].set(obj.data)
