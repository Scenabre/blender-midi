from mibllib import *
from bpy.types import Node
from bpy.props import EnumProperty, FloatProperty, IntProperty, BoolProperty
from .. node_tree.mi_node_tree import MI_BL_Node
from .. sockets.mi_sockets import SOCKET_SYS_LCD_TYPE, SOCKET_SYS_VPOT_TYPE


class NODE_MI_BL_MIDI_LCD(Node, MI_BL_Node):
    '''MiBl Display String into LCD'''
    bl_idname = 'NODE_MI_BL_MIDI_LCD'
    bl_label = 'MI String to LCD'

    _is_sys_node = True

    lcd_num: IntProperty(
        name='LCD #',
        min=1,
        max=8,
        soft_min=1,
        soft_max=8,
        default=1,
    )

    line_num: IntProperty(
        name='Line #',
        min=1,
        max=2,
        soft_min=1,
        soft_max=2,
        default=1,
    )

    def init(self, context):
        self.inputs.new('NodeSocketString', "Text")
        self.outputs.new(SOCKET_SYS_LCD_TYPE, "System Parameter")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        layout.prop(self, "lcd_num")
        layout.prop(self, "line_num")

    def execute(self):
        self.set_update_state(True)
        self.outputs[0].set_value(self.lcd_num, self.line_num, self.inputs[0].default_value)


class NODE_MI_BL_MIDI_VPOT(Node, MI_BL_Node):
    '''MiBl Set Vpot mode'''
    bl_idname = 'NODE_MI_BL_MIDI_VPOT'
    bl_label = 'MI Set Vpot Mode'

    _is_sys_node = True

    pan_num: IntProperty(
        name='Pan #',
        min=1,
        max=8,
        soft_min=1,
        soft_max=8,
        default=1
    )

    pan_mode: EnumProperty(
        name='Pan mode',
        items=(
            ('NORMAL', "INCREASE", ""),
            ('JAUGE', "JAUGE", ""),
            ('FILL', "FILL", ""),
            ('CENTER', "CENTER", ""),
        ),
        default='NORMAL'
    )

    def init(self, context):
        self.outputs.new(SOCKET_SYS_VPOT_TYPE, "Trigger")

    def draw_buttons(self, context, layout):
        layout.prop(self, "pan_num")
        layout.prop(self, "pan_mode")

    def execute(self):
        self.set_update_state(True)
        self.outputs[0].set_value(self.pan_num, self.pan_mode, 0)
