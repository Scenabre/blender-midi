from mibllib import *
from bpy.types import Node
from bpy.props import EnumProperty, FloatProperty, IntProperty, BoolProperty
from .. node_tree.mi_node_tree import MI_BL_Node
from .. utils.mibl_utils import generate_midi_note_bang, get_midi_note_num
from .. sockets.mi_sockets import SOCKET_ING_TYPE, SOCKET_ATTR_TYPE, SOCKET_RECIPE_TYPE, SOCKET_TRIGGER_TYPE
from .. node_tree.mi_update import execute_active_node_tree


class NODE_MI_BL_MIDI_Trigger_Note(Node, MI_BL_Node):
    '''MiBl Note Trigger'''
    bl_idname = 'NODE_MI_BL_MIDI_Trigger_Note'
    bl_label = 'MI Note Trigger'

    _is_trigger_node = True

    note_name: EnumProperty(
        name='Note name',
        items=(
            ('0', "A", ""),
            ('1', "A#/Bb", ""),
            ('2', "B", ""),
            ('3', "C", ""),
            ('4', "C#/Db", ""),
            ('5', "D", ""),
            ('6', "D#/Eb", ""),
            ('7', "E", ""),
            ('8', "F", ""),
            ('9', "F#/Gb", ""),
            ('10', "G", ""),
            ('11', "G#/Ab", ""),
        ),
        default='0'
    )

    octave_num: IntProperty(
        name='Octave #',
        min=0,
        max=8,
        soft_min=0,
        soft_max=8,
        default=4
    )

    vel: FloatProperty(
        name='Velocity',
        min=0.0,
        max=1.0,
        soft_min=0.0,
        soft_max=1.0,
        step=1,
        subtype='FACTOR',
        precision=2,
        default=0.5
    )

    def init(self, context):
        self.outputs.new(SOCKET_ING_TYPE, "Trigger")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        layout.prop(self, "note_name")
        layout.prop(self, "octave_num")
        layout.prop(self, "vel")

    def execute(self):
        self.set_update_state(True)
        trigger_name = f"{self.bl_rna.properties['note_name'].enum_items.get(self.note_name).name}{self.octave_num} CLICK"
        midi_note = get_midi_note_num(int(self.note_name), self.octave_num)
        midi_in = generate_midi_note_bang(midi_note, self.vel)
        self.outputs[0].set_name(trigger_name)
        self.outputs[0].set_value(midi_in, [], 1.0)


class NODE_MI_BL_MIDI_Trigger_SpecialNote(Node, MI_BL_Node):
    '''MiBl Note Trigger'''
    bl_idname = 'NODE_MI_BL_MIDI_Trigger_SpecialNote'
    bl_label = 'MI Special Note Trigger'

    _is_trigger_node = True

    note_name: EnumProperty(
        name='Note name',
        items=(
            ('VPOT', "VPot Click", ""),
            ('FADER', "Fader Touch", ""),
        ),
        default='VPOT'
    )

    channel_num: EnumProperty(
        name='',
        items=(
            ('0', "Channel #1", ""),
            ('1', "Channel #2", ""),
            ('2', "Channel #3", ""),
            ('3', "Channel #4", ""),
            ('4', "Channel #5", ""),
            ('5', "Channel #6", ""),
            ('6', "Channel #7", ""),
            ('7', "Channel #8", ""),
        ),
        default='0'
    )

    btn_state: EnumProperty(
        name='Btn state',
        items=(
            ('ON', "On", ""),
            ('OFF', "Off", ""),
        ),
        default='ON'
    )

    def init(self, context):
        self.outputs.new(SOCKET_ING_TYPE, "Trigger")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        layout.prop(self, "note_name")
        layout.prop(self, "channel_num")

        if self.note_name == 'FADER':
            layout.prop(self, "btn_state")

    def execute(self):
        self.set_update_state(True)
        note = 0x20
        midi_in = []
        trigger_name = ""
        channel_name = str(int(self.channel_num)+1)

        match self.note_name:
            case "VPOT":
                note = 0x20+int(self.channel_num)
                midi_in = generate_midi_note_bang(note, 0)
                trigger_name = f"PAN #{channel_name} CLICK"
            case "FADER":
                note = 0x68+int(self.channel_num)

                match self.btn_state:
                    case "ON":
                        midi_in = [0x90, note, 0x00]
                    case "OFF":
                        midi_in = [0x80, note, 0x00]

                trigger_name = f"FADER #{channel_name} {self.btn_state}"

        self.outputs[0].set_name(trigger_name)
        self.outputs[0].set_value(midi_in, [], 1.0)


class NODE_MI_BL_MIDI_Trigger_Fader(Node, MI_BL_Node):
    '''MiBl Fadder Trigger'''
    bl_idname = 'NODE_MI_BL_MIDI_Trigger_Fader'
    bl_label = 'MI Fader Trigger'

    _is_trigger_node = True

    fader_num: IntProperty(
        name='Fader #',
        min=1,
        max=9,
        soft_min=1,
        soft_max=9,
        default=1
    )

    def init(self, context):
        self.outputs.new(SOCKET_ING_TYPE, "Trigger")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        layout.prop(self, "fader_num")

    def execute(self):
        self.set_update_state(True)
        midi_fader_num = 0xE0 + (self.fader_num - 1)
        trigger_name = f"FADER #{str(self.fader_num)} SLIDE"
        self.outputs[0].set_name(trigger_name)
        self.outputs[0].set_value([midi_fader_num, 0x00, 0x00], [], 0.0)


class NODE_MI_BL_MIDI_Trigger_Pan(Node, MI_BL_Node):
    '''MiBl Fadder Trigger'''
    bl_idname = 'NODE_MI_BL_MIDI_Trigger_Pan'
    bl_label = 'MI Pan Trigger'

    _is_trigger_node = True

    pan_num: IntProperty(
        name='Pan #',
        min=1,
        max=8,
        soft_min=1,
        soft_max=8,
        default=1
    )

    pan_value: FloatProperty(
        name='Pan Value',
        min=-1.0,
        max=1.0,
        soft_min=-1.0,
        soft_max=1.0,
        step=1,
        subtype='FACTOR',
        precision=2,
        default=0.0
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
        self.outputs.new(SOCKET_ING_TYPE, "Trigger")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        layout.prop(self, "pan_num")

        if self.pan_value < 0.0:
            layout.label(text="Pan left value :")
        elif self.pan_value > 0.0:
            layout.label(text="Pan right value :")
        else:
            layout.label(text="No panning")

        layout.prop(self, "pan_value")

    def execute(self):
        self.set_update_state(True)
        midi_pan_num = 0x10 + (self.pan_num - 1)
        midi_pan_value = 0x01

        if self.pan_value < 0.0:
            midi_pan_value = 0x41

        trigger_name = f"PAN #{str(self.pan_num)}"

        self.outputs[0].set_name(trigger_name)
        self.outputs[0].set_value([0xB0, midi_pan_num, midi_pan_value], [], 0.0)


class NODE_MI_BL_MIDI_Trigger_User_Buttons(Node, MI_BL_Node):
    '''MiBl Trigger Buttons in the user space'''
    bl_idname = 'NODE_MI_BL_MIDI_Trigger_Usr_Btns'
    bl_label = 'MI User Button Trigger'

    _is_trigger_node = True

    def prop_changed(self, context):
        self.execute()
        execute_active_node_tree()

    def set_btn_enum(self, context):
        items = ()
        if self.channel_num == '8':
            items = (
                ('Flip', "Flip", ""),
            )
        else:
            items = (
                ('Rec', "Rec", ""),
                ('Solo', "Solo", ""),
                ('Mute', "Mute", ""),
                ('Select', "Select", ""),
            )

        return items

    channel_num: EnumProperty(
        name='',
        items=(
            ('0', "Channel #1", ""),
            ('1', "Channel #2", ""),
            ('2', "Channel #3", ""),
            ('3', "Channel #4", ""),
            ('4', "Channel #5", ""),
            ('5', "Channel #6", ""),
            ('6', "Channel #7", ""),
            ('7', "Channel #8", ""),
            ('8', "Main", ""),
        ),
        update=prop_changed,
        default='0'
    )

    btn_name: EnumProperty(
        name='Button',
        items=set_btn_enum,
    )

    btn_state: EnumProperty(
        name='State',
        items=(
            ('On', "On", ""),
            ('Off', "Off", ""),
        ),
        default='On'
    )

    def init(self, context):
        self.outputs.new(SOCKET_ING_TYPE, "Trigger")

    def draw_buttons(self, context, layout):
        row = layout.row()

        layout.label(text="Channel num :")
        layout.prop(self, "channel_num")

        layout.prop(self, "btn_name")

    def execute(self):
        self.set_update_state(True)
        note = 0x00

        match self.btn_name:
            case 'Rec':
                note = 0x00
            case 'Solo':
                note = 0x08
            case 'Mute':
                note = 0x10
            case 'Select':
                note = 0x18
            case 'Flip':
                note = 0x32

        channel_name = str(int(self.channel_num)+1)
        trigger_name = f"{self.btn_name} #{channel_name} CLICK"
        midi_in = generate_midi_note_bang(note+int(self.channel_num), 0)

        self.outputs[0].set_name(trigger_name)
        self.outputs[0].set_value(midi_in, [], 1.0)
