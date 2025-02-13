# from mibllib import mi_start_server, mi_stop_server, mi_get_midi_mesg, mi_set_midi_mesg
from bpy.types import Operator
from bpy.app import timers
import functools
import random

update_func = None


def mi_get_midi_mesg():
    print("Call to mi_get_midi_mesg")
    ran_num = random.randrange(0, 255, 1)
    return (ran_num, ran_num, ran_num)


def mi_start_server():
    print("Call to mi_start_server")


def mi_stop_server():
    print("Call to mi_stop_server")


def update_midi_value(context):
    scene = context.scene
    scene.mibl.mi_input_mesg = mi_get_midi_mesg()
    return 1.0  # update interval 1s


class MI_BL_OT_update_server_state(Operator):
    bl_label = "Start MIDI Server"
    bl_idname = "mibl.set_server_state"

    def execute(self, context):
        global update_func

        scene = context.scene
        if not scene.mibl.mi_run_server:
            update_func = functools.partial(update_midi_value, context)
            mi_start_server()
            scene.mibl.mi_run_server = True
            timers.register(
                update_func,
                first_interval=2.0
            )
            self.report({'INFO'}, 'MIDI Server Started')
            print("MIDI Server Started")
        else:
            mi_stop_server()
            scene.mibl.mi_run_server = False
            scene.mibl.mi_input_mesg = (0, 0, 0)
            scene.mibl.mi_output_mesg = (0, 0, 0)
            if timers.is_registered(update_func):
                timers.unregister(update_func)
            else:
                print("Something is strange, function seems not registered…")

            self.report({'INFO'}, 'MIDI Server Stopped')
            print("MIDI Server Stopped")

        return {'FINISHED'}
