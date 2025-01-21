from bpy.types import PropertyGroup
from bpy.props import BoolProperty, IntVectorProperty


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
        description="MIDI Interactive control ",
        default=False
    )
