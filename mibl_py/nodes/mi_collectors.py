import bpy
from bpy.types import Node
from bpy.props import PointerProperty
from .. node_tree.mi_node_tree import MI_BL_Node
from .. props.mi_props import MI_BL_SysParams
from .. sockets.mi_sockets import SOCKET_SYS_LCD_TYPE, SOCKET_SYS_VPOT_TYPE


class NODE_MI_BL_MIDI_Cooking_TriggerRecipe(Node, MI_BL_Node):
    '''MiBl Concatenate all ingredients in one meal'''
    bl_idname = 'NODE_MI_BL_MIDI_TriggerCOOK'
    bl_label = 'MI Cooking ingredients'

    def init(self, context):
        ingredients = self.inputs.new('SOCKET_MI_BL_MidiIngredient', "MIDI triggers")
        ingredients.link_limit = 32
        self.outputs.new('SOCKET_MI_BL_MidiRecipe', "MIDI cooking recipe")

    def execute(self):
        self.set_update_state(False)
        values = []

        for link in self.inputs[0].links:
            node_need_update = link.from_node.get_update_state()
            if node_need_update:
                link.from_node.set_update_state(False)
                values.append(link.from_socket.get_value())
                self.set_update_state(True)

        if self.get_update_state():
            self.outputs[0].set_value(values)


class NODE_MI_BL_MIDI_Cooking_SystemRecipe(Node, MI_BL_Node):
    '''MiBl Concatenate all ingredients in one meal'''
    bl_idname = 'NODE_MI_BL_MIDI_SystemCOOK'
    bl_label = 'MI Cooking system recipe'

    def init(self, context):
        params = self.inputs.new('SOCKET_MI_BL_AbstractSystemParam', "MIDI System Params")
        params.link_limit = 32
        self.outputs.new('SOCKET_MI_BL_SystemParam', "MIDI cooking system recipe")

    def execute(self):
        values = []
        for link in self.inputs[0].links:
            node_need_update = link.from_node.get_update_state()
            if node_need_update:
                self.set_update_state(True)

                if link.from_socket.bl_idname == SOCKET_SYS_LCD_TYPE:  # MI_BL_LcdParams
                    value = link.from_socket.get_value()
                    values.append((value, link.from_socket.bl_idname))
                elif link.from_socket.bl_idname == SOCKET_SYS_VPOT_TYPE:  # MI_BL_VPotParams
                    new_vpot = self.sys_param.vpot_vec.add()
                    new_vpot.vpot_idx = value.vpot_idx
                    new_vpot.vpot_mode = value.vpot_mode
                    new_vpot.vpot_val = value.vpot_val
                    self.sys_param.int_update = True
                    self.sys_param.updates.vpot_vec_update = True
                else:
                    print("Unkwnown input")

        if self.get_update_state():
            self.outputs[0].set_value(values)
            link.from_node.set_update_state(False)


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
            link.from_node.update()
