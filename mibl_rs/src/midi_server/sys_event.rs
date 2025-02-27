use crate::midi_server::container::Event;

pub const SYS_EVENT_ARRAY: [(u8, &str); 66] = [
    (0x28, "EA_Track"),
    (0x2A, "EA_PAN"),
    (0x2C, "EA_EQ"),
    (0x29, "EA_Send"),
    (0x2B, "EA_Plug_in"),
    (0x2D, "EA_Inst"),
    (0x34, "DISP_Name_Value"),
    (0x35, "DISP_SMPTE_Beats"),
    (0x33, "VIEW_Global"),
    (0x3E, "VIEW_Midi_Tracks"),
    (0x3F, "VIEW_Inputs"),
    (0x40, "VIEW_Audio_Tracks"),
    (0x41, "VIEW_Audios_Inst"),
    (0x42, "VIEW_Aux"),
    (0x43, "VIEW_Buses"),
    (0x44, "VIEW_Outputs"),
    (0x45, "VIEW_User"),
    (0x36, "FUNC_F1"),
    (0x37, "FUNC_F2"),
    (0x38, "FUNC_F3"),
    (0x39, "FUNC_F4"),
    (0x3A, "FUNC_F5"),
    (0x3B, "FUNC_F6"),
    (0x3C, "FUNC_F7"),
    (0x3D, "FUNC_F8"),
    (0x46, "MOD_Shift"),
    (0x47, "MOD_Option"),
    (0x48, "MOD_Ctrl"),
    (0x49, "MOD_Alt"),
    (0x4A, "AUTO_Read_OFF"),
    (0x4B, "AUTO_Write"),
    (0x4C, "AUTO_Trim"),
    (0x4D, "AUTO_Touch"),
    (0x4E, "AUTO_Latch"),
    (0x4F, "AUTO_Group"),
    (0x50, "UTILS_Save"),
    (0x51, "UTILS_Undo"),
    (0x52, "UTILS_Cancel"),
    (0x53, "UTILS_Enter"),
    (0x54, "TRANS_Marker"),
    (0x55, "TRANS_Nudge"),
    (0x56, "TRANS_Cycle"),
    (0x57, "TRANS_Drop"),
    (0x58, "TRANS_Replace"),
    (0x59, "TRANS_Click"),
    (0x5A, "TRANS_Solo"),
    (0x5B, "TRANS_Prev"),
    (0x5C, "TRANS_Next"),
    (0x5D, "TRANS_Stop"),
    (0x5E, "TRANS_Play"),
    (0x5F, "TRANS_Rec"),
    (0x32, "SWITCH_Flip"),
    (0x2E, "SWITCH_Fader_Bank_Prev"),
    (0x2F, "SWITCH_Fader_Bank_Next"),
    (0x30, "SWITCH_Channel_Prev"),
    (0x31, "SWITCH_Channel_Next"),
    (0x60, "PAD_Up"),
    (0x61, "PAD_Down"),
    (0x62, "PAD_Left"),
    (0x63, "PAD_Right"),
    (0x64, "PAD_Zoom"),
    (0x65, "TRANS_Scrub"),
    (0x3C, "TRANS_Wheel"),
    (0x71, "LED_SMPTE"),
    (0x72, "LED_Beats"),
    (0x73, "LED_Solo"),
];

//pub const

// ** LED **
// SMPTE Led :	0x71
//BEATS Led : 0x72
//RUDE SOLO Led :	0x73

// ** Transport **
// Marker : 0054
// Nudge : 0055
// Cycle : 0056
// Drop : 00
// Replace : 0058
// Click : 0059
// Solo : 005A
// Play : 005E
// Stop : 005D
// Rec : 005F
// Prev : 005B
// Next : 005C
//
// ** Utility **
// Save : 0050
// Undo : 0051
// Cancel : 0052
// Enter : 0053
//
// ** Function **
// F1 : 36
// F2 : 37
// F3 : 38
// F4 : 39
// F5 : 3A
// F6 : 3B
// F7 : 3C
// F8 : 3D
//
// ** Modify **
// Shift : 0046
// Option : 0047
// Control : 0048
// Alt : 0049
//
// ** Automation **
// Read/OFF : 004A
// Write : 004B
// Trim : 004C
// Touch : 004D
// Latch : 004E
// Group : 004F
//
// ** Encoder Assign **
// Track : 0028
// Pan/Surround : 002A
// EQ : 002C
// Send : 0029
// Plug-In : 002B
// Inst : 002
//
// ** Display **
// Name/Value : 0034
// SMPTE/BEATS : 0035
//
// ** Views **
// Global view : 0033
// Midi Tracks : 003E
// Inputs : 003F
// Audio Tracks : 0040
// Audio Inst : 0041
// Aux : 0042
// Buses : 0043
// Outputs : 0044
// User : 0045
//
// ** Switch **
// Fader Bank Prev : 002E
// Fader Bank Next : 002F
// Channel Prev : 0030
// Channel Next : 0031
//
// ** Directional Pad/ZOOM :
// Left : 0062
// Right : 0063
// Up : 0060
// Down : 0061
// Zoom : 0064
//
// ** Wheel CC **
// Wheel CCW : 003C 0041
// Wheel CW : 003C 0001
// Scrub : 0065
