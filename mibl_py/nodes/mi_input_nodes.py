from bpy.types import Node
from bpy.props import FloatProperty
from .. node_tree.mi_node_tree import MI_BL_Node
from .. utils.blender_utils import update_prop, update_all_outputs

class NODE_MI_BL_value_input(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_value_input'
    bl_label = 'MI Value'

    float_output: FloatProperty(name="Value", default=0.0, update=update_prop)

    def init(self, context):
        self.outputs.new('NodeSocketFloat',
                         "Value")

    def get_value(self):
        return self.float_output

    def draw_buttons(self, context, layout):
        layout.prop(self, "float_output")

    def execute(self):
        self.outputs['Value'].default_value = self.float_output
        update_all_outputs(self)


class NODE_MI_BL_object(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_object'
    bl_label = 'MI Object'

    def init(self, context):
        self.inputs.new('NodeSocketObject', "Object")
        self.outputs.new('NodeSocketObject', 'Object')

    def get_value(self):
        return self.inputs[0].default_value

    def execute(self):
        obj = self.inputs[0].default_value
        if obj:
            self.outputs[0].default_value = obj
