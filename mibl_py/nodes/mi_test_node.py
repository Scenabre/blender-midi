# from mibllib import sum_float_custom
from bpy.types import Node


def sum_float_custom(a, b):
    pass


class NODE_MI_BL_Test(Node):
    '''A custom node'''
    bl_idname = 'NODE_MI_BL_Test'
    bl_label = 'Custom Node'
    bl_icon = 'NODETREE'

    def init(self, context):
        self.inputs.new('SOCKET_MI_BL_Test', "Input A")
        self.inputs.new('SOCKET_MI_BL_Test', "Input B")
        self.outputs.new('SOCKET_MI_BL_Test', "Output")

    def copy(self, node):
        print("Copying from node ", node)

    def free(self):
        print("Removing node ", self, ", Goodbye!")

    def update(self):
        input_a = self.inputs['Input A'].my_custom_property
        input_b = self.inputs['Input B'].my_custom_property
        result = sum_float_custom(input_a, input_b)
        print("Node compute : ", result)

    def draw_label(self):
        return "I am a custom node"
