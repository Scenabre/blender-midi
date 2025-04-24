import bpy
import hashlib
from bpy.types import Node
from .. node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_group_output(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_group_output'
    bl_label = 'MI Group Output'

    _sys_data = None
    _recipe = None
    _recipe_footprint = None

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
        self._sys_data = None
        self._recipe = None
        self._recipe_footprint = None
        collector = []
        mibl_props = bpy.context.scene.mibl

        system_link = self.inputs[0]
        recipe_link = self.inputs[1]
        attr_link = self.inputs[2]

        links = [system_link, recipe_link]

        for idx, link in enumerate(links):
            if link.is_linked:
                linked_node = link.links[0]
                linked_node.from_node.update()
                node_value = linked_node.from_socket.get_value()
                collector.append(node_value)

        if len(collector) == 2:
            self._sys_data = collector[0]
            self._recipe = collector[1]

        if self._recipe is not None:
            node_in = self.id_data.get_node_in()
            recipe_str = ""

            for (idx, ing) in enumerate(self._recipe.ingredients):
                recipe_str += ing.ing_name

            self._recipe_footprint = hashlib.md5(str.encode(recipe_str)).hexdigest()

            # print("Curr str : ", recipe_str)
            # print("Curr footprint : ", self._recipe_footprint)
            # print("Saved footprint : ", mibl_props.mi_recipe_footprint)

            if mibl_props.mi_recipe_footprint != self._recipe_footprint:
                print("Update recipe from python")
                if self._recipe_footprint is not None:
                    mibl_props.mi_recipe.clean_ingredients()

                    for ingredient in self._recipe.ingredients:
                        new_ingredient = mibl_props.mi_recipe.ingredients.add()
                        new_ingredient.ing_name = ingredient.ing_name
                        new_ingredient.midi_in = ingredient.midi_in

                        for midi_arr in new_ingredient.midi_out:
                            new_midi_out = new_ingredient.midi_out.add()
                            new_midi_out.vec_out = midi_arr

                        new_ingredient.opt_val = ingredient.opt_val

                    mibl_props.mi_recipe_need_update = True
                    mibl_props.mi_recipe_footprint = self._recipe_footprint

            if node_in is not None:
                node_in.update()

            if attr_link.is_linked:
                linked_node = attr_link.links[0]
                linked_node.from_node.update()
