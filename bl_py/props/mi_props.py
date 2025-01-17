from bpy.types import Scene
from bpy.props import BoolProperty, StringProperty

Scene.mi_run_server = BoolProperty(
    name="mi_bool_running",
    default=False
)

Scene.mi_output_mesg = StringProperty(
    name="mi_output",
    default=""
)

Scene.mi_input_mesg = StringProperty(
    name="mi_input",
    default=""
)
