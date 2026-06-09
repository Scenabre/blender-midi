from bpy.props import FloatProperty, FloatVectorProperty, CollectionProperty, PointerProperty
from bpy.types import NodeSocket, NodeTreeInterfaceSocket
from .. props.mi_props import MI_BL_TriggerProp, MI_BL_Ingredient, MI_BL_Recipe, MI_BL_SysParams, MI_BL_LcdParams, MI_BL_VPotParams, clean_coll, MIDI_IN_SIZE, MIDI_OUT_SIZE
from .. utils.mibl_utils import fill_array
# from .. sockets.mi_sockets import SOCKET_SYS_LCD_TYPE, SOCKET_SYS_VPOT_TYPE

SOCKET_ING_TYPE = 'SOCKET_MI_BL_MidiIngredient'
SOCKET_RECIPE_TYPE = 'SOCKET_MI_BL_MidiRecipe'
SOCKET_TRIGGER_TYPE = 'SOCKET_MI_BL_MidiTrigger'
SOCKET_SYS_TYPE = 'SOCKET_MI_BL_SystemParam'
SOCKET_ABS_SYS_TYPE = 'SOCKET_MI_BL_AbstractSystemParam'
SOCKET_SYS_LCD_TYPE = 'SOCKET_MI_BL_LcdSystemParam'
SOCKET_SYS_VPOT_TYPE = 'SOCKET_MI_BL_VPotSystemParam'
SOCKET_ATTR_TYPE = 'SOCKET_MI_BL_AttrLink'
SOCKET_VOID_TYPE = 'SOCKET_MI_BL_Void'


class SOCKET_MI_BL_MidiIngredient(NodeSocket):
    """Just a custom socket type for test"""
    bl_idname = SOCKET_ING_TYPE
    bl_label = 'mi_bl_socket_midi_recipe'

    ingredient: PointerProperty(
        name="mi_midi_ingredient",
        description="Midi Ingredient",
        type=MI_BL_Ingredient,
    )

    def get_value(self):
        return self.ingredient

    def set_name(self, name):
        self.ingredient.ing_name = name

    def set_value(self, midi_in, midi_out, opt_val):
        midi_in_arr = fill_array(midi_in, MIDI_IN_SIZE)
        self.ingredient.midi_in = midi_in_arr

        self.ingredient.clean_midi_out()

        for midi_arr in midi_out:
            new_vec = self.ingredient.midi_out.add()
            filled_midi_arr = fill_array(midi_arr, MIDI_OUT_SIZE)
            new_vec.vec_out = filled_midi_arr

        self.ingredient.opt_val = opt_val

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (1.0, 0.4, 1.0, 1.0)


class SOCKET_INT_MI_BL_MidiIngredient(NodeTreeInterfaceSocket):
    bl_socket_idname = SOCKET_ING_TYPE

    ingredient_default_value: PointerProperty(
        name='mi_midi_default_ingredient',
        type=MI_BL_Ingredient,
    )

    def draw(self, context, layout):
        layout.prop(self, "ingredient_default_value")

    def init_socket(self, node, socket, data_path):
        socket.ingredient.ing_name = self.ingredient_default_value.ing_name
        socket.ingredient.midi_in = self.ingredient_default_value.midi_in
        socket.ingredient.midi_out = self.ingredient_default_value.midi_out
        socket.ingredient.opt_val = self.ingredient_default_value.opt_val

    def from_socket(self, node, socket):
        self.ingredient_default_value = socket.ingredient


class SOCKET_MI_BL_MidiRecipe(NodeSocket):
    bl_idname = SOCKET_RECIPE_TYPE
    bl_label = 'mi_bl_socket_midi_recipe'

    mibl_recipe: PointerProperty(
        type=MI_BL_Recipe,
    )

    def get_value(self):
        return self.mibl_recipe

    def set_value(self, ingredients):
        self.mibl_recipe.clean_ingredients()

        if ingredients is not None:
            self.mibl_recipe.int_update = True

            for ingredient in ingredients:
                new_ingredient = self.mibl_recipe.ingredients.add()
                new_ingredient.ing_name = ingredient.ing_name
                new_ingredient.midi_in = ingredient.midi_in

                for midi_arr in new_ingredient.midi_out:
                    new_midi_out = new_ingredient.midi_out.add()
                    new_midi_out.vec_out = midi_arr

                new_ingredient.opt_val = ingredient.opt_val
        else:
            self.mibl_recipe.int_update = False

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (1.0, 0.4, 0.0, 1.0)


class SOCKET_INT_MI_BL_MidiRecipe(NodeTreeInterfaceSocket):
    bl_socket_idname = SOCKET_RECIPE_TYPE

    mibl_recipe_default_value: PointerProperty(
        type=MI_BL_Recipe,
    )

    def draw(self, context, layout):
        layout.prop(self, "mibl_recipe_default_value")

    def init_socket(self, node, socket, data_path):
        socket.mibl_recipe = self.mibl_recipe_default_value

    def from_socket(self, node, socket):
        self.mibl_recipe_default_value = socket.mibl_recipe


class SOCKET_MI_BL_MidiTrigger(NodeSocket):
    """Custom Midi Trigger Result Socket"""
    bl_idname = SOCKET_TRIGGER_TYPE
    bl_label = 'mi_bl_socket_midi_trigger'

    mibl_trigger: PointerProperty(
        type=MI_BL_TriggerProp
    )

    def get_value(self):
        return self.mibl_trigger

    def set_value(self, idx, value):
        self.mibl_trigger.triggerIdx = idx
        self.mibl_trigger.triggerValue = value

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (1.0, 0.0, 0.4, 1.0)


class SOCKET_INT_MI_BL_MidiTrigger(NodeTreeInterfaceSocket):
    bl_socket_idname = SOCKET_TRIGGER_TYPE

    mibl_trigger_default_value: PointerProperty(
        type=MI_BL_TriggerProp
    )

    def draw(self, context, layout):
        layout.prop(self, "mibl_trigger_default_value")

    def init_socket(self, node, socket, data_path):
        socket.mibl_trigger = self.mibl_trigger_default_value

    def from_socket(self, node, socket):
        self.mibl_trigger_default_value = socket.mibl_trigger


class SOCKET_MI_BL_LcdSystemParam(NodeSocket):
    """Custom Midi LCD System socket"""
    bl_idname = SOCKET_SYS_LCD_TYPE
    bl_label = 'mi_bl_socket_midi_system_lcd'

    mibl_system_params: PointerProperty(
        type=MI_BL_LcdParams
    )

    def get_value(self):
        return self.mibl_system_params

    def set_value(self, lcd_num, line_num, string):
        if lcd_num in range(0, 8):
            self.mibl_system_params.lcd_num = lcd_num
        if line_num in range(0, 2):
            self.mibl_system_params.line_num = line_num
        self.mibl_system_params.string = string

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (0.0, 1.0, 1.0, 1.0)


class SOCKET_INT_MI_BL_LcdSystemParam(NodeTreeInterfaceSocket):
    bl_socket_idname = SOCKET_SYS_LCD_TYPE

    sys_default_value: PointerProperty(
        type=MI_BL_LcdParams
    )

    def draw(self, context, layout):
        layout.prop(self, "sys_default_value")

    def init_socket(self, node, socket, data_path):
        socket.mibl_system_params = self.sys_default_value

    def from_socket(self, node, socket):
        self.sys_default_value = socket.mibl_system_params


class SOCKET_MI_BL_VPotSystemParam(NodeSocket):
    """Custom Midi VPot System socket"""
    bl_idname = SOCKET_SYS_VPOT_TYPE
    bl_label = 'mi_bl_socket_midi_system_vpot'

    mibl_system_params: PointerProperty(
        type=MI_BL_VPotParams
    )

    def get_value(self):
        return self.mibl_system_params

    def set_value(self, vpot_num, vpot_mode, vpot_value):
        if vpot_num in range(0, 8):
            self.mibl_system_params.vpot_idx = vpot_num
        if vpot_mode in range(0, 4):
            self.mibl_system_params.vpot_mode = vpot_mode
        self.mibl_system_params.vpot_val = vpot_value

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (0.0, 1.0, 1.0, 1.0)


class SOCKET_INT_MI_BL_VPotSystemParam(NodeTreeInterfaceSocket):
    bl_socket_idname = SOCKET_SYS_VPOT_TYPE

    sys_default_value: PointerProperty(
        type=MI_BL_VPotParams
    )

    def draw(self, context, layout):
        layout.prop(self, "sys_default_value")

    def init_socket(self, node, socket, data_path):
        socket.mibl_system_params = self.sys_default_value

    def from_socket(self, node, socket):
        self.sys_default_value = socket.mibl_system_params


class SOCKET_MI_BL_AbstractSystemParam(NodeSocket):
    """Custom Midi System socket"""
    bl_idname = SOCKET_ABS_SYS_TYPE
    bl_label = 'mi_bl_socket_midi_abstract_system_param'

    mibl_system_param: PointerProperty(
        type=MI_BL_SysParams
    )

    def get_value(self):
        return self.mibl_system_param

    def set_value(self, sys_param):
        self.mibl_system_param = sys_param

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (0.0, 1.0, 0.0, 1.0)


class SOCKET_MI_BL_SystemParam(NodeSocket):
    """MiBl Midi System socket"""
    bl_idname = SOCKET_SYS_TYPE
    bl_label = 'mi_bl_socket_midi_system'

    mibl_system_param: PointerProperty(
        type=MI_BL_SysParams
    )

    def get_value(self):
        return self.mibl_system_param

    def set_value(self, param_list):
        self.mibl_system_param.clean_all()
        for (value, name) in param_list:
            if name == SOCKET_SYS_LCD_TYPE:
                lcd = value
                new_lcd = self.mibl_system_param.lcd_vec.add()
                new_lcd.lcd_num = lcd.lcd_num
                new_lcd.line_num = lcd.lcd_num
                new_lcd.string = lcd.string
                self.mibl_system_param.updates.lcd_vec_update = True
                self.mibl_system_param.int_update = True
            elif name == SOCKET_SYS_VPOT_TYPE:
                vpot = value
                new_vpot = self.mibl_system_param.vpot_vec.add()
                new_vpot.vpot_idx = vpot.vpot_idx
                new_vpot.vpot_mode = vpot.vpot_mode
                new_vpot.vpot_val = vpot.vpot_val
                self.mibl_system_param.updates.vpot_vec_update = True
                self.mibl_system_param.int_update = True

            # if updates.lcd_mesg_update:
            #     self.mibl_system_param.lcd_mesg = sys_param.lcd_mesg
            #
            # if updates.faders_update:
            #     for fader in sys_param.faders:
            #         new_fader = self.mibl_system_param.faders.add()
            #         new_fader.fader_num = fader.fader_num
            #         new_fader.fader_value = fader.fader_value
            #
            # if updates.user_btns_update:
            #     for btn in sys_param.user_btns:
            #         new_btn = self.mibl_system_param.user_btns.add()
            #         new_btn.chan_num = btn.chan_num
            #         new_btn.btn_num = btn.btn_num
            #         new_btn.btn_status = btn.btn_status

            # self.mibl_system_param.ext_update = sys_param.ext_update
            # self.mibl_system_param.int_update = sys_param.int_update

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (0.0, 1.0, 1.0, 1.0)


class SOCKET_MI_BL_AttrLink(NodeSocket):
    """MiBl attr socket"""
    bl_idname = SOCKET_ATTR_TYPE
    bl_label = 'mi_bl_socket_attr_link'

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (1.0, 1.0, 0.0, 1.0)


class SOCKET_MI_BL_Void(NodeSocket):
    """MiBl void socket"""
    bl_idname = SOCKET_VOID_TYPE
    bl_label = 'mi_bl_socket_void'

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (0.0, 0.0, 0.0, 0.7)
