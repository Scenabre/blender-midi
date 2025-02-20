from mibllib import MiBlRustProcess
from bpy.types import Operator
from bpy.app import timers
import functools
import threading
import queue

update_func = None
mibl_rs = None
mibl_thread = None


def update_midi_value(context):
    global mibl_rs
    scene = context.scene
    # scene.mibl.mi_input_mesg = mibl_rs.get_rx_data()
    print("Blender get value from thread :", mibl_rs.get_rx_stamp())
    return 1.0  # update interval 1s


class MI_BL_OT_update_server_state(Operator):
    bl_label = "Start MIDI Server"
    bl_idname = "mibl.set_server_state"

    def execute(self, context):
        global update_func
        global mibl_thread
        global mibl_rs

        scene = context.scene
        if not scene.mibl.mi_run_server:

            if update_func is None:
                update_func = functools.partial(update_midi_value, context)

            if mibl_rs is None:
                mibl_rs = MiBlRustProcess()

            mibl_rs.set_close_signal(False)
            mibl_thread = threading.Thread(target=mibl_rs.mi_start_server_allow_thread)

            scene.mibl.mi_run_server = True

            timers.register(
                update_func,
                first_interval=1.0
            )

            mibl_thread.start()

            self.report({'INFO'}, 'MIDI Server Started')

        else:
            scene.mibl.mi_run_server = False
            scene.mibl.mi_input_mesg = (0, 0, 0)
            scene.mibl.mi_output_mesg = (0, 0, 0)

            if timers.is_registered(update_func):
                timers.unregister(update_func)
            else:
                print("Something is strange, function seems not registered…")

            print(threading.enumerate())

            mibl_rs.set_close_signal(True)
            mibl_thread.join()

            print(threading.enumerate())

            self.report({'INFO'}, 'MIDI Server Stopped')
            print("MIDI Server Stopped")

        return {'FINISHED'}
