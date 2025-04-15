import bpy
from bpy.types import Node
from .. node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_MIDI_Cooking_TriggerRecipe(Node, MI_BL_Node):
    '''MiBl Concatenate all ingredients in one meal'''
    bl_idname = 'NODE_MI_BL_MIDI_TriggerCOOK'
    bl_label = 'MI Cooking ingredients'

    def init(self, context):
        ingredients = self.inputs.new('SOCKET_MI_BL_MidiIngredient', "MIDI triggers")
        ingredients.link_limit = 32
        self.outputs.new('SOCKET_MI_BL_MidiRecipe', "MIDI cooking recipe")

    def execute(self):
        values = []

        for link in self.inputs[0].links:
            link.from_node.update()
            values.append(link.from_socket.get_value())

        self.outputs[0].set_value(values)


class NODE_MI_BL_MIDI_Cooking_SystemRecipe(Node, MI_BL_Node):
    '''MiBl Concatenate all ingredients in one meal'''
    bl_idname = 'NODE_MI_BL_MIDI_SystemCOOK'
    bl_label = 'MI Cooking system recipe'

    def init(self, context):
        ingredients = self.inputs.new('SOCKET_MI_BL_SystemParam', "MIDI System Params")
        ingredients.link_limit = 32
        self.outputs.new('SOCKET_MI_BL_SystemParamList', "MIDI cooking system recipe")

    def execute(self):
        values = []

        for link in self.inputs[0].links:
            link.from_node.update()
            values.append(link.from_socket.get_value())

        self.outputs[0].set_value(values)


class NODE_MI_BL_MIDI_CollectAttrs(Node, MI_BL_Node):
    '''MiBl collect all attr node for node_tree evaluation'''
    bl_idname = 'NODE_MI_BL_MIDI_CollectAttrs'
    bl_label = 'MI Collect node attr'

    def init(self, context):
        attr_in = self.inputs.new('SOCKET_MI_BL_AttrLink', "Attr")
        attr_in.link_limit = 32
        self.outputs.new('SOCKET_MI_BL_AttrLink', "Attr")

    def execute(self):
        for link in self.inputs["Attr"].links:
            print("Update attr node : ", link.from_node.name)
            link.from_node.update()
