from bpy.types import Node
from bpy.props import FloatProperty
from .. node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_value_input(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_value_input'
    bl_label = 'MI Value'
    _index = 0

    float_output: FloatProperty(name="Value", default=0.0)

    def init(self, context):
        self.outputs.new('NodeSocketFloat',
                         "Value")

    def draw_buttons(self, context, layout):
        layout.prop(self, "float_output")

    def execute(self):
        self.outputs['Value'].default_value = self.float_output

    def update(self):
        self.execute()


class NODE_MI_BL_object(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_object'
    bl_label = 'MI Object'

    def init(self, context):
        self.inputs.new('NodeSocketObject', "Object")
        self.outputs.new('NodeSocketObject', 'Object')

    def execute(self):
        obj = self.inputs['Object'].default_value
        if obj:
            self.outputs['Object'].default_value = obj

    def update(self):
        self.execute()
