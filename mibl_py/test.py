
import bpy
from your_midi_library import start_midi_server, stop_midi_server  # Replace with your actual MIDI library

# Define custom properties to store the server state and the returned value
bpy.types.Scene.midi_server_running = bpy.props.BoolProperty(
    name="MIDI Server Running",
    default=False
)
bpy.types.Scene.midi_server_value = bpy.props.StringProperty(
    name="MIDI Server Value",
    default=""
)

# Function to start the MIDI server
def start_server(scene):
    if not scene.midi_server_running:
        value = start_midi_server()
        scene.midi_server_value = value
        scene.midi_server_running = True
        print("MIDI Server Started with value:", value)

# Function to stop the MIDI server
def stop_server(scene):
    if scene.midi_server_running:
        stop_midi_server()
        scene.midi_server_running = False
        scene.midi_server_value = ""
        print("MIDI Server Stopped")

# Custom panel in the scene properties
class MIDI_SERVER_PT_panel(bpy.types.Panel):
    bl_label = "MIDI Server Control"
    bl_idname = "MIDI_SERVER_PT_panel"
    bl_space_type = 'PROPERTIES'
    bl_region_type = 'WINDOW'
    bl_context = "scene"

    def draw(self, context):
        layout = self.layout
        scene = context.scene

        row = layout.row()
        row.prop(scene, "midi_server_running", text="Server Running")

        row = layout.row()
        if scene.midi_server_running:
            row.operator("midi.stop_server", text="Stop Server")
        else:
            row.operator("midi.start_server", text="Start Server")

# Operator to start the server
class MIDI_OT_start_server(bpy.types.Operator):
    bl_label = "Start MIDI Server"
    bl_idname = "midi.start_server"

    def execute(self, context):
        start_server(context.scene)
        return {'FINISHED'}

# Operator to stop the server
class MIDI_OT_stop_server(bpy.types.Operator):
    bl_label = "Stop MIDI Server"
    bl_idname = "midi.stop_server"

    def execute(self, context):
        stop_server(context.scene)
        return {'FINISHED'}

# Register the classes
def register():
    bpy.utils.register_class(MIDI_SERVER_PT_panel)
    bpy.utils.register_class(MIDI_OT_start_server)
    bpy.utils.register_class(MIDI_OT_stop_server)
    bpy.types.Scene.midi_server_running = bpy.props.BoolProperty(
        name="MIDI Server Running",
        default=False
    )
    bpy.types.Scene.midi_server_value = bpy.props.StringProperty(
        name="MIDI Server Value",
        default=""
    )

# Unregister the classes
def unregister():
    bpy.utils.unregister_class(MIDI_SERVER_PT_panel)
    bpy.utils.unregister_class(MIDI_OT_start_server)
    bpy.utils.unregister_class(MIDI_OT_stop_server)
    del bpy.types.Scene.midi_server_running
    del bpy.types.Scene.midi_server_value

if __name__ == "__main__":
    register()


(self, context):
        stop_server(context.scene)
        return {'FINISHED'}

# Register the classes
def register():
    bpy.utils.register_class(MIDI_SERVER_PT_panel)
    bpy.utils.register_class(MIDI_OT_start_server)
    bpy.utils.register_class(MIDI_OT_stop_server)
    bpy.types.Scene.midi_server_running = bpy.props.BoolProperty(
        name="MIDI Server Running",
        default=False
    )
    bpy.types.Scene.midi_server_value = bpy.props.StringProperty(
        name="MIDI Server Value",
        default=""
    )

# Unregister the classes
def unregister():
    bpy.utils.unregister_class(MIDI_SERVER_PT_panel)
    bpy.utils.unregister_class(MIDI_OT_start_server)
    bpy.utils.unregister_class(MIDI_OT_stop_server)
    del bpy.types.Scene.midi_server_running
    del bpy.types.Scene.midi_server_value

if __name__ == "__main__":
    register()
