from mibllib import *
from bpy.types import Node
from bpy.props import FloatProperty
from .. node_tree.mi_node_tree import MI_BL_Node
from .. utils.blender_utils import update_all_outputs


class NODE_MI_BL_MATH_add(Node, MI_BL_Node):
    '''MiBl Math Add Node'''
    bl_idname = 'NODE_MI_BL_MATH_add'
    bl_label = 'MI Math Add'
    bl_icon = 'NODETREE'

    def init(self, context):
        self.inputs.new('NodeSocketFloat', "Input A")
        self.inputs.new('NodeSocketFloat', "Input B")
        self.outputs.new('NodeSocketFloat', "Output")

    def copy(self, node):
        print("Copying from node ", node)

    def free(self):
        print("Removing node ", self, ", Goodbye!")

    def execute(self):
        a = 0.0
        b = 0.0

        if self.inputs["Input A"].is_linked:
            a = self.inputs["Input A"].links[0].from_socket.default_value
        else:
            a = self.inputs["Input A"].default_value

        if self.inputs["Input B"].is_linked:
            b = self.inputs["Input B"].links[0].from_socket.default_value
        else:
            b = self.inputs["Input B"].default_value

        result = mibl_add(a,b)

        self.outputs['Output'].default_value = result
        update_all_outputs(self)

    def draw_label(self):
        return "Mi Math Node Add"


class NODE_MI_BL_MATH_multiply(Node, MI_BL_Node):
    '''MiBl Math Multiply Node'''
    bl_idname = 'NODE_MI_BL_MATH_multiply'
    bl_label = 'MI Math Multiply'
    bl_icon = 'NODETREE'

    def init(self, context):
        self.inputs.new('NodeSocketFloat', "A")
        self.inputs.new('NodeSocketFloat', "B")
        self.outputs.new('NodeSocketFloat', "Output")

    def copy(self, node):
        print("Copying from node ", node)

    def free(self):
        print("Removing node ", self, ", Goodbye!")

    def execute(self):
        if len(self.inputs) == 2:
            a = 0.0
            b = 0.0
            if self.inputs[0].is_linked:
                a = self.inputs[0].links[0].from_socket.default_value
            else:
                a = self.inputs[0].default_value

            if self.inputs[1].is_linked:
                b = self.inputs[1].links[0].from_socket.default_value
            else:
                b = self.inputs[1].default_value

            result = mibl_multiply(a,b)

            self.outputs['Output'].default_value = result
            update_all_outputs(self)

    def draw_label(self):
        return "Mi Math Node Multiply"
