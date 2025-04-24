from bpy.types import PropertyGroup
from bpy.props import BoolProperty, IntVectorProperty, IntProperty, CollectionProperty, StringProperty, FloatProperty, PointerProperty

MIDI_IN_SIZE = 8
MIDI_OUT_SIZE = 32


def clean_coll(coll):
    for idx, vec in enumerate(coll):
        coll.remove(idx)


class MI_BL_VecOut(PropertyGroup):
    vec_out: IntVectorProperty(size=MIDI_OUT_SIZE)


class MI_BL_Ingredient(PropertyGroup):
    ing_name: StringProperty()
    midi_in: IntVectorProperty(size=MIDI_IN_SIZE)
    midi_out: CollectionProperty(
        type=MI_BL_VecOut
    )
    opt_val: FloatProperty()

    def clean_midi_out(self):
        for idx, vec in enumerate(self.midi_out):
            self.midi_out.remove(idx)


class MI_BL_Recipe(PropertyGroup):
    ingredients: CollectionProperty(
        type=MI_BL_Ingredient
    )

    def clean_ingredients(self):
        for idx, vec in enumerate(self.ingredients):
            self.ingredients.remove(idx)


class MI_BL_TriggerProp(PropertyGroup):
    idx: IntProperty()
    value: FloatProperty()


class MI_BL_LcdParams(PropertyGroup):
    lcd_num: IntProperty()
    line_num: IntProperty()
    string: StringProperty()


class MI_BL_VPotParams(PropertyGroup):
    vpot_idx: IntProperty()
    vpot_mode: IntProperty()
    vpot_val: IntProperty()


class MI_BL_FaderParams(PropertyGroup):
    fader_num: IntProperty()
    fader_value: FloatProperty()


class MI_BL_ChanBtnParams(PropertyGroup):
    chan_num: IntProperty()
    btn_num: IntProperty()
    btn_status: BoolProperty()


class MI_BL_TimestampParams(PropertyGroup):
    hours: IntProperty()
    minutes: IntProperty()
    seconds: IntProperty()
    frames: IntProperty()


class MI_BL_SysParams(PropertyGroup):
    timestamp: PointerProperty(
        type=MI_BL_TimestampParams
    )
    lcd_vec: CollectionProperty(
        type=MI_BL_LcdParams
    )
    lcd_mesg: StringProperty()
    vpot_vec: CollectionProperty(
        type=MI_BL_VPotParams
    )
    faders: CollectionProperty(
        type=MI_BL_FaderParams
    )
    user_btns: CollectionProperty(
        type=MI_BL_ChanBtnParams
    )
    fps: IntProperty()

    def clean_lcd_vec(self):
        clean_coll(self.lcd_vec)

    def clean_vpot_vec(self):
        clean_coll(self.vpot_vec)

    def clean_faders(self):
        clean_coll(self.faders)

    def clean_user_btns(self):
        clean_coll(self.user_btns)

    def clean_all(self):
        self.clean_lcd_vec()
        self.clean_vpot_vec()
        self.clean_faders()
        self.clean_user_btns()


class PropsMiBl(PropertyGroup):
    mi_run_server: BoolProperty(
        name="mi_run_server",
        description="MIDI Interactive control",
        default=False
    )
    mi_use_system_ctlr: BoolProperty(
        name="mi_use_system_ctlr",
        description="Toggle Mackie Device system control",
        default=True
    )
    mi_trigger: PointerProperty(
        name="mi_trigger",
        description="Trigger from recipe",
        type=MI_BL_TriggerProp,
    )
    mi_trigger_update: BoolProperty(
        name="mi_trigger_update",
        description="A signal to update trigger value in input node"
    )
    mi_recipe: PointerProperty(
        name="mi_recipe",
        type=MI_BL_Recipe
    )
    mi_init: PointerProperty(
        type=MI_BL_SysParams
    )
    mi_recipe_need_update: BoolProperty(
        name="mi_recipe_need_update",
        description="Send recipe update signal to server",
        default=False
    )
    mi_recipe_footprint: StringProperty(
        name="mi_recipe_footprint",
        description="The hash value of the recipe",
        default=""
    )
