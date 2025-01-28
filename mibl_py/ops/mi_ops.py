# from mibllib import mi_start_server, mi_stop_server, mi_get_midi_mesg, mi_set_midi_mesg
from bpy.types import Operator
from bpy.app import timers
from bpy import context as C


def mi_get_midi_mesg():
    print("Call to mi_get_midi_mesg")


def mi_start_server():
    print("Call to mi_start_server")


def mi_stop_server():
    print("Call to mi_stop_server")


def update_midi_value():
    C.scene.mibl.mi_input_mesg = mi_get_midi_mesg()
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
        start_server(C.scene)
        return {'FINISHED'}


class MI_BL_OT_stop_server(Operator):
    bl_label = "Stop MIDI Server"
    bl_idname = "mibl.stop_server"

    def execute(self, context):
        stop_server(C.scene)
        return {'FINISHED'}
