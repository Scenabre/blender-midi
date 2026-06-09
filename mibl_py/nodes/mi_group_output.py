import bpy
import hashlib
from bpy.types import Node
from .. node_tree.mi_node_tree import MI_BL_Node


class NODE_MI_BL_group_output(Node, MI_BL_Node):
    bl_idname = 'NODE_MI_BL_group_output'
    bl_label = 'MI Group Output'

    sys_data = None
    recipe = None
    recipe_footprint = None
    sys_footprint = None

    def init(self, context):
        self.inputs.new(
            'SOCKET_MI_BL_SystemParam',
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
        return self.sys_data

    def get_recipe(self):
        return self.recipe

    def parse_sys(self, mibl_props):
        if self.sys_data.int_update:
            mibl_props.mi_sys_params.clean_all()
            updates = self.sys_data.updates

            if updates.lcd_vec_update:
                print("Update LCD in output node")
                for lcd in self.sys_data.lcd_vec:
                    new_lcd = mibl_props.mi_sys_params.lcd_vec.add()
                    new_lcd.lcd_num = lcd.lcd_num
                    new_lcd.line_num = lcd.lcd_num
                    new_lcd.string = lcd.string

            if updates.lcd_mesg_update:
                mibl_props.mi_sys_params.lcd_mesg = self.sys_data.lcd_mesg

            if updates.vpot_vec_update:
                for vpot in self.sys_data.vpot_vec:
                    new_vpot = mibl_props.mi_sys_params.vpot_vec.add()
                    new_vpot.vpot_idx = vpot.vpot_idx
                    new_vpot.vpot_mode = vpot.vpot_mode
                    new_vpot.vpot_val = vpot.vpot_val

            if updates.faders_update:
                for fader in self.sys_data.faders:
                    new_fader = mibl_props.mi_sys_params.faders.add()
                    new_fader.fader_num = fader.fader_num
                    new_fader.fader_value = fader.fader_value

            if updates.user_btns_update:
                for btn in self.sys_data.user_btns:
                    new_btn = mibl_props.mi_sys_params.user_btns.add()
                    new_btn.chan_num = btn.chan_num
                    new_btn.btn_num = btn.btn_num
                    new_btn.btn_status = btn.btn_status

            mibl_props.mi_sys_params.ext_update = True
            mibl_props.mi_sys_params.int_update = False
            self.sys_data.int_update = False

    def parse_recipe(self, mibl_props):
        node_in = self.id_data.get_node_in()

        mibl_props.mi_recipe.clean_ingredients()

        for ingredient in self.recipe.ingredients:
            new_ingredient = mibl_props.mi_recipe.ingredients.add()
            new_ingredient.ing_name = ingredient.ing_name
            new_ingredient.midi_in = ingredient.midi_in

            for midi_arr in new_ingredient.midi_out:
                new_midi_out = new_ingredient.midi_out.add()
                new_midi_out.vec_out = midi_arr

            new_ingredient.opt_val = ingredient.opt_val

        mibl_props.mi_recipe.ext_update = True

        if node_in is not None:
            node_in.update()

    def parse_link(self, node_link):
        data = None
        if node_link.is_linked:
            linked_node = node_link.links[0]
            linked_node.from_node.update()

            if linked_node.from_node.get_update_state():
                node_value = linked_node.from_socket.get_value()
                data = node_value
                linked_node.from_node.set_update_state(False)

        return data

    def execute(self):
        self.sys_data = None
        self.recipe = None
        self.recipe_footprint = None
        self.sys_footprint = None

        mibl_props = bpy.context.scene.mibl

        system_link = self.inputs[0]
        recipe_link = self.inputs[1]
        attr_link = self.inputs[2]

        self.sys_data = self.parse_link(system_link)
        self.recipe = self.parse_link(recipe_link)

        if self.sys_data is not None:
            self.parse_sys(mibl_props)

        if self.recipe is not None:
            self.parse_recipe(mibl_props)

        if attr_link.is_linked:
            linked_node = attr_link.links[0]
            linked_node.from_node.update()
