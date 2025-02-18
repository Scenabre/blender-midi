from mibllib import *
from bpy.types import Node
from bpy.props import FloatProperty
from .. node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_MATH_add(Node, MI_BL_Node):
    '''MiBl Math Add Node'''
    bl_idname = 'NODE_MI_BL_MATH_add'
    bl_label = 'MI Math Add'
    bl_icon = 'NODETREE'

    float_input1: FloatProperty(name="Input A", default=0.0)
    float_input2: FloatProperty(name="Input B", default=0.0)
    float_output: FloatProperty(name="Output", default=0.0)

    def init(self, context):
        self.inputs.new('NodeSocketFloat', "Input A")
        self.inputs.new('NodeSocketFloat', "Input B")
        self.outputs.new('NodeSocketFloat', "Output")

    def copy(self, node):
        print("Copying from node ", node)

    def free(self):
        print("Removing node ", self, ", Goodbye!")

    def execute(self):
        result = mibl_add(self.float_input1, self.float_input2)
        self.float_output = result
        print("Node compute : ", result)

    def update(self):
        self.execute()
        
    def draw_label(self):
        return "Mi Math Node"
