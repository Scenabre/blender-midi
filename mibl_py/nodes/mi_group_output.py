import bpy
from bpy.types import Node
from .. node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_group_output(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_group_output'
    bl_label = 'MI Group Output'

    _sys_data = None
    _recipe = None

    def init(self, context):
        self.inputs.new(
            'SOCKET_MI_BL_SystemParamList',
            'MI_BL_Midi_System',
            use_multi_input=False
        )
        self.inputs.new(
            'SOCKET_MI_BL_MidiRecipe',
            'MI_BL_Midi_Recipe',
            use_multi_input=False
        )
        self.inputs.new(
            'SOCKET_MI_BL_AttrLink',
            'MI_BL_Attrs',
            use_multi_input=False
        )

    def get_sys(self):
        return self._sys_data

    def get_recipe(self):
        return self._recipe

    def execute(self):
        # mibl_props = bpy.context.scene.mibl
        # current_triggers = list(mibl_props.mi_trigger_list)
        self._sys_data = None
        self._recipe = None
        collector = []

        system_link = self.inputs[0]
        recipe_link = self.inputs[1]
        attr_link = self.inputs[2]

        links = [system_link, recipe_link]

        for idx, link in enumerate(links):
            if link.is_linked:
                linked_node = link.links[0]
                print("Get value from : ", linked_node.from_node.name)
                linked_node.from_node.update()
                node_value = linked_node.from_socket.get_value()
                collector.append(node_value)

        if len(collector) == 2:
            self._sys_data = collector[0]
            self._recipe = collector[1]

        if attr_link.is_linked:
            linked_node = attr_link.links[0]
            linked_node.from_node.update()

        # for item in collector:
        #     for values in item:
        #         print(list(values))

        # while len(active_inputs) < 16:
        #     active_inputs.append(0)

        # mibl_props.mi_midi_channel = active_inputs
        # scene = bpy.context.scene
        # inputs = self.get_linked(self.inputs)
        #
        # if inputs:
        #     inputs[0] = scene.mibl.mi_output_mesg
