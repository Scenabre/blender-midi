import mibllib
import threading
import time
import sys

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


def update_loop(thread):
    count = 0
    try:
        while True:
            signal = mibl_rs.get_close_signal()

            if signal:
                print("Getting stop signal !")
                thread.join()
                sys.exit()

            sys_signals = mibl_rs.get_triggers()

            if len(sys_signals) > 0 :
                for idx, signal in enumerate(sys_signals):
                    if signal[0] < len(SYS_EVENT_ARRAY) :
                        print("Sys signal found : ", SYS_EVENT_ARRAY[signal[0]])

            time.sleep(.4)
            count += 1
    except KeyboardInterrupt:
        print("SIGINT RECEIVE")
        mibl_rs.set_close_signal(True)
        thread.join()
        print("Exiting…")


def main():
    global mibl_rs
    global mibl_thread

    mibl_rs = mibllib.MiBlRustProcess()

    mibl_thread = threading.Thread(target=mibl_rs.mi_start_server_allow_thread)
    # mibl_thread.daemon = True

    mibl_thread.start()

    update_loop(mibl_thread)


if __name__ == "__main__":
    main()
