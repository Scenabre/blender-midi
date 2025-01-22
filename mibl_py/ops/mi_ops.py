from mibllib import mi_start_server, mi_stop_server, mi_get_midi_mesg, mi_set_midi_mesg
from bpy.types import Operator
from bpy.app import timers
from bpy.context import scene


def update_midi_value():
    scene.mibl.mi_input_mesg = mi_get_midi_mesg()
    return 1.0  # Update Interval 1s


def start_server(scene):
    if not scene.mibl.mi_run_server:
        mi_start_server()
        scene.mibl.midi_server_running = True
        timers.register(update_midi_value)
        print("MIDI Server Started")


def stop_server(scene):
    if scene.mibl.mi_run_server:
        mi_stop_server()
        scene.mibl.midi_server_running = False
        scene.mibl.midi_server_value = ""
        timers.unregister(update_midi_value)
        print("MIDI Server Stopped")


class MI_BL_OT_start_server(Operator):
    bl_label = "Start MIDI Server"
    bl_idname = "mibl.start_server"

    def execute(self, context):
        start_server(scene)
        return {'FINISHED'}


class MI_BL_OT_stop_server(Operator):
    bl_label = "Stop MIDI Server"
    bl_idname = "mibl.stop_server"

    def execute(self, context):
        stop_server(scene)
        return {'FINISHED'}
