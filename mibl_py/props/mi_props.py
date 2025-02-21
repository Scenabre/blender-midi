from bpy.types import PropertyGroup
from bpy.props import BoolProperty, IntVectorProperty, IntProperty


class PropsMiBl(PropertyGroup):
    mi_output_mesg: IntVectorProperty(
        name="mi_output_mesg",
        description="MIDI Interactive output message",
        default=(0, 0, 0),
        size=3,
        min=0,
        max=255
    )
    mi_input_mesg: IntVectorProperty(
        name="mi_input_mesg",
        description="MIDI Interactive output message",
        default=(0, 0, 0),
        size=3,
        min=0,
        max=255
    )
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
    mi_midi_channel: IntVectorProperty(
        name="mi_midi_channel",
        description="Wait data from this channel",
        min=0,
        max=16,
        soft_min=1,
        soft_max=16,
        size=16,
        default=tuple([0] * 16)
    )
