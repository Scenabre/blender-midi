import bpy
from mibllib import *
from bpy.types import Node
from bpy.props import EnumProperty, FloatProperty, IntProperty
from .. node_tree.mi_node_tree import MI_BL_Node

class NODE_MI_BL_MIDI_Params(Node, MI_BL_Node):
    '''MiBl Midi Server Parameters'''
    bl_idname = 'NODE_MI_BL_MIDI_Params'
    bl_label = 'MI Midi Params'
    bl_icon = 'NODETREE'

    channel_enum: EnumProperty(
        name='',
        items=(
            ('1', "Channel 1", ""),
            ('2', "Channel 2", ""),
            ('3', "Channel 3", ""),
            ('4', "Channel 4", ""),
            ('5', "Channel 5", ""),
            ('6', "Channel 6", ""),
            ('7', "Channel 7", ""),
            ('8', "Channel 8", ""),
            ('9', "Channel 9", ""),
            ('10', "Channel 10", ""),
            ('11', "Channel 11", ""),
            ('12', "Channel 12", ""),
            ('13', "Channel 13", ""),
            ('14', "Channel 14", ""),
            ('15', "Channel 15", ""),
            ('16', "Channel 16", ""),
        ),
        default='1'
    )

    def init(self, context):
        self.outputs.new('NodeSocketInt', "Target channel")
        self.outputs.new('NodeSocketVectorInt', "Midi Message In")
        self.outputs.new('NodeSocketInt', "Time Stamp")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        if mibl_props.mi_run_server:
            row.operator("mibl.set_server_state", text="Stop Midi Server")
        else:
            row.operator("mibl.set_server_state", text="Start Midi Server")

        layout.label(text="Server controls :")
        layout.prop(mibl_props, 'mi_use_system_ctlr', text="Use MC system control")

        layout.label(text="Server Channel")
        layout.prop(self, "channel_enum")

    def execute(self):
        midi_channel = self.channel_enum
        bpy.context.scene.mibl.mi_midi_channel.append(midi_channel)
        print(bpy.context.scene.mibl.mi_midi_channel)

    def update(self):
        self.execute()


class NODE_MI_BL_MIDI_Cooking_Recipe(Node, MI_BL_Node):
    '''MiBl Concatenate all ingredients in one meal'''
    bl_idname = 'NODE_MI_BL_MIDI_COOK'
    bl_label = 'MI Cooking ingredients'

    def init(self, context):
        ingredients = self.inputs.new('SOCKET_MI_BL_Midi_Recipe', "MIDI Recipes")
        ingredients.link_limit = 32
        self.outputs.new('SOCKET_MI_BL_Midi_Recipe', "MIDI cooking recipe")

    def execute(self):
        pass

    def update(self):
        self.execute()



class NODE_MI_BL_MIDI_LCD(Node, MI_BL_Node):
    '''MiBl Display String into LCD'''
    bl_idname = 'NODE_MI_BL_MIDI_LCD'
    bl_label = 'MI String to LCD'

    fader_num: IntProperty(
        name='LCD #',
        min=1,
        max=8,
        soft_min=1,
        soft_max=8,
        default=1
    )

    line_num: IntProperty(
        name='Line #',
        min=1,
        max=2,
        soft_min=1,
        soft_max=2,
        default=1
    )

    def init(self, context):
        self.inputs.new('NodeSocketString', "Text")
        self.outputs.new('SOCKET_MI_BL_Midi_Recipe', "System Recipe")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        layout.prop(self, "fader_num")
        layout.prop(self, "line_num")

    def execute(self):
        pass

    def update(self):
        self.execute()


class NODE_MI_BL_MIDI_Trigger_Note(Node, MI_BL_Node):
    '''MiBl Note Trigger'''
    bl_idname = 'NODE_MI_BL_MIDI_Trigger_Note'
    bl_label = 'MI Note Trigger'

    trigger_type: EnumProperty(
        name='',
        items=(
            ('0', "Name", ""),
            ('1', "Octave", ""),
            ('2', "Velocity", ""),
            ('3', "All", ""),
        ),
        default='3'
    )

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
        min=-1,
        max=8,
        soft_min=-1,
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
        self.outputs.new('SOCKET_MI_BL_Midi_Recipe', "Trigger Recipe")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        layout.label(text="Trigger type :")
        layout.prop(self, "trigger_type")

        match self.trigger_type:
            case '0':
                layout.prop(self, "note_name")
            case '1':
                layout.prop(self, "octave_num")
            case '2':
                layout.prop(self, "vel")
            case '3':
                layout.prop(self, "note_name")
                layout.prop(self, "octave_num")
                layout.prop(self, "vel")
            case _:
                layout.prop(self, "note_name")

    def execute(self):
        pass

    def update(self):
        self.execute()


class NODE_MI_BL_MIDI_Trigger_Fader(Node, MI_BL_Node):
    '''MiBl Fadder Trigger'''
    bl_idname = 'NODE_MI_BL_MIDI_Trigger_Fader'
    bl_label = 'MI Fader Trigger'

    trigger_type: EnumProperty(
        name='',
        items=(
            ('0', "Fader #", ""),
            ('1', "Fader Value", ""),
            ('2', "All", ""),
        ),
        default='2'
    )

    fader_num: IntProperty(
        name='Fader #',
        min=1,
        max=9,
        soft_min=1,
        soft_max=9,
        default=1
    )

    fader_value: FloatProperty(
        name='Fader Value',
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
        self.outputs.new('SOCKET_MI_BL_Midi_Recipe', "Trigger Recipe")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        layout.label(text="Trigger type :")
        layout.prop(self, "trigger_type")

        match self.trigger_type:
            case '0':
                layout.prop(self, "fader_num")
            case '1':
                layout.prop(self, "fader_value")
            case '2':
                layout.prop(self, "fader_num")
                layout.prop(self, "fader_value")
            case _:
                layout.prop(self, "fader_num")

    def execute(self):
        pass

    def update(self):
        self.execute()


class NODE_MI_BL_MIDI_Trigger_Pan(Node, MI_BL_Node):
    '''MiBl Fadder Trigger'''
    bl_idname = 'NODE_MI_BL_MIDI_Trigger_Pan'
    bl_label = 'MI Pan Trigger'

    trigger_type: EnumProperty(
        name='',
        items=(
            ('0', "Pan #", ""),
            ('1', "Value", ""),
            ('2', "All", ""),
        ),
        default='2'
    )

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

    def init(self, context):
        self.outputs.new('SOCKET_MI_BL_Midi_Recipe', "Trigger Recipe")

    def draw_buttons(self, context, layout):
        mibl_props = context.scene.mibl
        row = layout.row()

        layout.label(text="Trigger type :")
        layout.prop(self, "trigger_type")

        match self.trigger_type:
            case '0':
                layout.prop(self, "pan_num")
            case '1':
                if self.pan_value > 0.0:
                    layout.label(text="Pan left value :")
                elif self.pan_value > 0.0:
                    layout.label(text="Pan right value :")
                else:
                    layout.label(text="No panning")
                layout.prop(self, "pan_value")
            case '2':
                layout.prop(self, "pan_num")
                if self.pan_value > 0.0:
                    layout.label(text="Pan left value :")
                elif self.pan_value > 0.0:
                    layout.label(text="Pan right value :")
                else:
                    layout.label(text="No panning")
                layout.prop(self, "pan_value")
            case _:
                layout.prop(self, "pan_num")

    def execute(self):
        pass

    def update(self):
        self.execute()
