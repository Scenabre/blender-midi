import bpy
import random
from bpy.types import Node
from bpy.props import FloatProperty
from .. node_tree.mi_node_tree import MI_BL_Node
from .. node_tree.mi_update import execute_active_node_tree
from .. sockets.mi_sockets import SOCKET_VOID_TYPE


class NODE_MI_BL_group_input(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_group_input'
    bl_label = 'MI Group Input'

    _triggers = []

    def clean_outputs(self, trigger_list):
        if trigger_list is None:
            self.outputs.clear()
            self._triggers.clear()
            return

        if trigger_list is not None:
            for out in self.outputs:
                if out.bl_idname == SOCKET_VOID_TYPE:
                    self.outputs.remove(out)

            trigger_list_ing_name = []

            for trigger in trigger_list:
                trigger_list_ing_name.append(trigger.ing_name)

            for trigger in self._triggers:
                if trigger not in trigger_list_ing_name:
                    if trigger in self.outputs:
                        self.outputs.remove(self.outputs[trigger])
                    self._triggers.remove(trigger)

    def create_outputs(self, trigger_list):
        for trigger in trigger_list:
            socket_name = trigger.ing_name
            if socket_name != '':
                if socket_name not in self._triggers:
                    self.outputs.new(
                        'NodeSocketFloat',
                        socket_name,
                        use_multi_input=False
                    )
                    self._triggers.append(socket_name)

    def init(self, context):
        node_out = self.id_data.get_node_out()
        self._triggers = []

        if node_out is not None:
            recipe = node_out.get_recipe()

            if recipe is not None:
                trigger_list = recipe.ingredients

                if len(trigger_list) == 0:
                    self.clean_outputs(None)
                    self.outputs.new(SOCKET_VOID_TYPE, "")
                    return

                self.clean_outputs(trigger_list)
                self.create_outputs(trigger_list)
                return

        self.clean_outputs(None)
        self.outputs.new(SOCKET_VOID_TYPE, "")

    def execute(self):
        node_out = self.id_data.get_node_out()

        if node_out is not None:
            recipe = node_out.get_recipe()

            if recipe is not None:
                trigger_list = recipe.ingredients

                if len(trigger_list) == 0:
                    self.clean_outputs(None)
                    self.outputs.new(SOCKET_VOID_TYPE, "")
                    return

                self.clean_outputs(trigger_list)
                self.create_outputs(trigger_list)
                return

        self.clean_outputs(None)
        self.outputs.new(SOCKET_VOID_TYPE, "")
