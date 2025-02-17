from bpy.types import Node
from ..node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_Test(Node, MI_BL_Node):
    '''A custom node'''
    bl_idname = 'NODE_MI_BL_Test'
    bl_label = 'Custom Node'
    bl_icon = 'NODETREE'

    def init(self, context):
        self.inputs.new('NodeSocketFloat', 'Input A')
        self.inputs.new('NodeSocketFloat', 'Input B')
        self.outputs.new('NodeSocketFloat', 'Result')
        self.outputs.new('NodeSocketFloat', 'Pass Through A')
        self.outputs.new('NodeSocketFloat', 'Pass Through B')

    def execute(self):
        input_a = self.inputs['Input A'].default_value if not self.inputs['Input A'].is_linked else self.inputs['Input A'].links[0].from_socket.default_value
        input_b = self.inputs['Input B'].default_value if not self.inputs['Input B'].is_linked else self.inputs['Input B'].links[0].from_socket.default_value
        result = input_a * input_b
        self.outputs['Result'].default_value = result
        self.outputs['Pass Through A'].default_value = input_a
        self.outputs['Pass Through B'].default_value = input_b

    def update(self):
        self.execute()
