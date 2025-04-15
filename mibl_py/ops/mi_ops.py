import bpy
from mibllib import MiBlRustProcess
from bpy.types import Operator
from bpy.app import timers
import functools
import threading
import queue
from .. node_tree.mi_update import execute_active_node_tree
    
update_func = None
mibl_rs = None
mibl_thread = None

SYS_EVENT_ARRAY = [
    "EA_Track",
    "EA_PAN",
    "EA_EQ",
    "EA_Send",
    "EA_Plug_in",
    "EA_Inst",
    "DISP_Name_Value",
    "DISP_SMPTE_Beats",
    "VIEW_Global",
    "VIEW_Midi_Tracks",
    "VIEW_Inputs",
    "VIEW_Audio_Tracks",
    "VIEW_Audios_Inst",
    "VIEW_Aux",
    "VIEW_Buses",
    "VIEW_Outputs",
    "VIEW_User",
    "FUNC_F1",
    "FUNC_F2",
    "FUNC_F3",
    "FUNC_F4",
    "FUNC_F5",
    "FUNC_F6",
    "FUNC_F7",
    "FUNC_F8",
    "MOD_Shift",
    "MOD_Option",
    "MOD_Ctrl",
    "MOD_Alt",
    "AUTO_Read_OFF",
    "AUTO_Write",
    "AUTO_Trim",
    "AUTO_Touch",
    "AUTO_Latch",
    "AUTO_Group",
    "UTILS_Save",
    "UTILS_Undo",
    "UTILS_Cancel",
    "UTILS_Enter",
    "TRANS_Marker",
    "TRANS_Nudge",
    "TRANS_Cycle",
    "TRANS_Drop",
    "TRANS_Replace",
    "TRANS_Click",
    "TRANS_Solo",
    "TRANS_Prev",
    "TRANS_Next",
    "TRANS_Stop",
    "TRANS_Play",
    "TRANS_Rec",
    "SWITCH_Flip",
    "SWITCH_Fader_Bank_Prev",
    "SWITCH_Fader_Bank_Next",
    "SWITCH_Channel_Prev",
    "SWITCH_Channel_Next",
    "PAD_Up",
    "PAD_Down",
    "PAD_Left",
    "PAD_Right",
    "PAD_Zoom",
    "TRANS_Scrub",
    "TRANS_Wheel",
    "LED_SMPTE",
    "LED_Beats",
    "LED_Solo",
]


def gen_timestamp(fps, curr_frame):
    hours = int(curr_frame / (3600*fps))
    minutes = int(curr_frame / (60*fps) % 60)
    seconds = int(curr_frame / fps % 60)
    frames = curr_frame % fps

    return [hours, minutes, seconds, frames]


def update_midi_value(context):
    global mibl_rs
    scene = context.scene

    sys_signals = mibl_rs.get_triggers()
    fps = scene.render.fps
    curr_frame = scene.frame_current

    hours, minutes, seconds, frames = gen_timestamp(fps, curr_frame)
    print(f"Timestamp : {hours}:{minutes}:{seconds}.{frames}")

    mibl_rs.set_fps(fps)
    mibl_rs.set_timestamp(hours, minutes, seconds, frames)

    if len(sys_signals) > 0:
        for idx, signal in enumerate(sys_signals):
            print("Sys signal found : ", signal[sys_signals[1]])
            print("Sys signal value found : ", signal[sys_signals[2]])

    execute_active_node_tree()


    # TRANSPORT
    # bpy.ops.screen.keyframe_jump(next=False)
    # bpy.ops.screen.keyframe_jump(next=True)
    # bpy.ops.screen.frame_jump(end=True)
    # bpy.ops.screen.frame_jump(end=False)
    # bpy.ops.screen.animation_play()
    # bpy.ops.screen.animation_cancel(restore_frame=True)
    # bpy.ops.screen.frame_offset(delta=1)
    # bpy.ops.screen.frame_offset(delta=-1)
    #
    # UTILS
    # bpy.ops.wm.save_mainfile()
    # bpy.ops.ed.undo()

    return 0.2  # update interval 1s


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
