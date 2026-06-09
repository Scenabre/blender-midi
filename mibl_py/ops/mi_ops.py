import bpy
import ctypes
from mibllib import MiBlRustProcess
from bpy.types import Operator
from bpy.app import timers
import functools
import threading
import queue
from math import ceil
from .. node_tree.mi_update import execute_active_node_tree
from .. utils.mibl_utils import gen_timestamp
from .. utils.blender_utils import update_count_ev, check_count_ev, clean_count_ev, update_markers, get_area, get_areas, set_persportho, set_prop_layout, set_view_orbit
from mibllib import mibl_get_event_by_index, mibl_get_sys_event_len, mibl_pow

update_func = None
mibl_rs = None
mibl_thread = None
count_ev = {}
update_wait = 0
update_interval = 0.01
timestamp_mode = 0
frame_drop = False
first_press = True


def parse_sys(mibl_props, mibl_rs):
    updates = mibl_props.mi_sys_params.updates
    mi_sys_params = mibl_props.mi_sys_params
    rs_update = []

    if updates.lcd_vec_update:
        print("Update LCD in operator")
        mibl_rs.set_lcd_vec(mi_sys_params.lcd_vec)
        rs_update.extend(0)
    if updates.lcd_mesg_update:
        mibl_rs.set_lcd_string(mi_sys_params.lcd_mesg)
        rs_update.extend(1)
    if updates.vpot_vec_update:
        mibl_rs.set_vpots(mi_sys_params.vpot_vec)
        rs_update.extend(2)
    if updates.faders_update:
        mibl_rs.set_faders(mi_sys_params.faders)
        rs_update.extend(3)
    if updates.user_btns_update:
        mibl_rs.set_chan_btns(mi_sys_params.user_btns)
        rs_update.extend(4)

    if len(rs_update) > 0:
        mibl_rs.set_devicestate_update(rs_update)


def parse_recipe(mibl_props):
    ingredients = []

    for ing in mibl_props.mi_recipe.ingredients:
        rs_in = list(filter((-1).__ne__, list(ing.midi_in)))
        rs_out = []

        for mesg in list(ing.midi_out):
            rs_out.append(list(filter((-1).__ne__, mesg.vec_out)))

        rs_ing = (rs_in, rs_out, ing.opt_val)
        ingredients.append(rs_ing)

    mibl_rs.set_recipe(ingredients)
    mibl_rs.set_recipe_need_update(True)
    mibl_props.mi_recipe.int_update = False


def parse_signals(context, mibl_props, sys_signals, timestamp_mode, curr_frame, frame_drop):
    sys_event_len = mibl_get_sys_event_len()

    for idx, signal in enumerate(sys_signals):
        if signal[0] < (sys_event_len-1):
            sig_event = mibl_get_event_by_index(signal[0])
            update_count_ev(count_ev, sig_event[0], signal[1])

            print("Recieve sys signal : ", list(signal))
            print("Fetching event signal : ", list(sig_event))

            match sig_event[0]:
                case 0x3C:
                    if sig_event[1] == "TRANS_Wheel":
                            accel_x = check_count_ev(count_ev, sig_event[0], signal[1]) - 1
                            accel_calc = int(ceil(0.1*mibl_pow(accel_x, 2)+1))

                            if accel_calc >= 50:
                                clean_count_ev(count_ev, sig_event[0], signal[1])

                            if signal[1] == 1.0:
                                bpy.ops.screen.frame_offset(delta=accel_calc)
                            if signal[1] == -1.0:
                                bpy.ops.screen.frame_offset(delta=-accel_calc)
                    elif sig_event[1] == "FUNC_F7":
                        set_prop_layout(context, 8)
                case 0x5B:  # TRANS_Prev
                    bpy.ops.screen.frame_jump(end=False)
                case 0x5C:  # TRANS_Next
                    bpy.ops.screen.frame_jump(end=True)
                case 0x5D:
                    bpy.ops.screen.animation_cancel(restore_frame=frame_drop)
                case 0x5E:
                    bpy.ops.screen.animation_play()
                case 0x5F:
                    curr_scene = context.window.scene
                    auto_key = curr_scene.tool_settings.use_keyframe_insert_auto
                    curr_scene.tool_settings.use_keyframe_insert_auto = not auto_key
                case 0x35:
                    timestamp_mode ^= 1
                    print(timestamp_mode)
                case 0x54:  # TRANS_Marker
                    update_markers(context, curr_frame)
                case 0x57:  # TRANS_Drop
                    frame_drop = not frame_drop
                case 0x50:
                    bpy.ops.wm.save_mainfile()
                case 0x51:
                    bpy.ops.ed.undo()
                case 0x52:
                    bpy.ops.mibl.set_server_state()
                case 0x60:
                    set_view_orbit(context, 2)
                case 0x61:
                    set_view_orbit(context, 3)
                case 0x62:
                    set_view_orbit(context, 0)
                case 0x63:
                    set_view_orbit(context, 1)
                case 0x65:
                    set_persportho(context)
                case 0x36:
                    set_prop_layout(context, 2)
                case 0x37:
                    set_prop_layout(context, 3)
                case 0x38:
                    set_prop_layout(context, 7)
                case 0x39:
                    set_prop_layout(context, 9)
                case 0x3A:
                    set_prop_layout(context, 15)
                case 0x3B:
                    set_prop_layout(context, 16)
                case 0x3D:
                    set_prop_layout(context, 13)
                case _:
                    print("Event unknown ", list(signal))
        else:
            trigger_idx = signal[0] - (sys_event_len - 1)
            if trigger_idx >= 0 and trigger_idx < len(mibl_props.mi_recipe.ingredients):
                mibl_props.mi_trigger.idx = trigger_idx
                mibl_props.mi_trigger.value = signal[1]
                mibl_props.mi_trigger_update = True


def update_midi_value(context):
    global mibl_rs
    global count_ev
    global update_wait
    global update_interval
    global timestamp_mode
    global frame_drop

    scene = context.scene
    mibl_props = scene.mibl

    sys_signals = mibl_rs.get_triggers()
    fps = scene.render.fps
    curr_frame = abs(scene.frame_current)

    hours, minutes, seconds, frames = gen_timestamp(fps, curr_frame, timestamp_mode)

    mibl_rs.set_fps(fps)
    mibl_rs.set_timestamp(hours, minutes, seconds, frames)

    if mibl_props.mi_sys_params.ext_update:
        parse_sys(mibl_props, mibl_rs)
        mibl_props.mi_sys_params.ext_update = False

    if mibl_props.mi_recipe.ext_update:
        parse_recipe(mibl_props)
        mibl_props.mi_recipe.ext_update = False

    if len(sys_signals) > 0:
        parse_signals(context,
                      mibl_props,
                      sys_signals,
                      timestamp_mode,
                      curr_frame,
                      frame_drop
                      )
    else:
        if update_wait >= 0.3*fps:
            count_ev = {}
            update_wait = 0
        update_wait += 1

    execute_active_node_tree()

    return update_interval


class MI_BL_OT_update_server_state(Operator):
    bl_label = "Start MIDI Server"
    bl_idname = "mibl.set_server_state"

    def execute(self, context):
        global update_func
        global mibl_thread
        global mibl_rs
        global count_ev
        global update_interval
        global first_press

        debug = context.preferences.addons['bl_ext.user_default.midi_interactive_bl'].preferences.debug

        scene = context.scene
        if not scene.mibl.mi_run_server:

            count_ev = {}
            fps = context.scene.render.fps
            update_interval = 1/fps
            first_press = False

            if update_func is None:
                update_func = functools.partial(update_midi_value, context)

            if mibl_rs is None:
                mibl_rs = MiBlRustProcess()

            mibl_rs.set_close_signal(False)
            mibl_rs.set_sysevent(True)
            mibl_thread = threading.Thread(target=mibl_rs.mi_start_server_allow_thread, args=(debug,))

            scene.mibl.mi_run_server = True

            timers.register(
                update_func,
                first_interval=update_interval
            )

            mibl_thread.start()

            self.report({'INFO'}, 'MIDI Server Started')
        else:
            scene.mibl.mi_run_server = False

            if first_press:
                if debug :
                    print("Server doesn't already running")
                self.report({'INFO'}, "Server doesn't already running")
                first_press = False
            else:
                if timers.is_registered(update_func):
                    timers.unregister(update_func)
                else:
                    print("Something is strange, function seems not registered…")

                print(threading.enumerate())

                mibl_rs.set_close_signal(True)
                mibl_thread.join()

                print(threading.enumerate())
                count_ev = {}

                self.report({'INFO'}, 'MIDI Server Stopped')
                if debug :
                    print("MIDI Server Stopped")

        return {'FINISHED'}
