from bpy.types import Scene, Operator
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


class OBJECT_OT_property_example(Operator):
    bl_idname = "object.property_example"
    bl_label = "Property Example"
    bl_options = {'REGISTER', 'UNDO'}

    my_float: bpy.props.FloatProperty(name="Some Floating Point")
    my_bool: bpy.props.BoolProperty(name="Toggle Option")
    my_string: bpy.props.StringProperty(name="String Value")

    def execute(self, context):
        self.report(
            {'INFO'}, "F: {:.2f}  B: {:s}  S: {!r}".format(
                self.my_float, self.my_bool, self.my_string,
            )
        )
        print('My float:', self.my_float)
        print('My bool:', self.my_bool)
        print('My string:', self.my_string)
        return {'FINISHED'}
