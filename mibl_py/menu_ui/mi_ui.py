from .. node_tree.mi_node_tree import TREE_NAME
from bpy.types import Menu
from bl_ui import node_add_menu
from nodeitems_utils import NodeCategory, NodeItem


class MI_BL_NodeCategory(NodeCategory):
    @classmethod
    def poll(cls, context):
        return context.space_data.tree_type == TREE_NAME


# all categories in a list
node_categories = [
    MI_BL_NodeCategory('INPUT', "Input", items=[
        NodeItem("NODE_MI_BL_group_input"),
        NodeItem("NODE_MI_BL_value_input"),
        NodeItem("NODE_MI_BL_object"),
    ]),
    MI_BL_NodeCategory('MIDI_PARAMS', "Midi Utils", items=[
        NodeItem("NODE_MI_BL_MIDI_Params"),
        NodeItem("NODE_MI_BL_MIDI_LCD"),
        NodeItem("NODE_MI_BL_MIDI_TriggerCOOK"),
        NodeItem("NODE_MI_BL_MIDI_SystemCOOK"),
        NodeItem("NODE_MI_BL_MIDI_Trigger_Note"),
        NodeItem("NODE_MI_BL_MIDI_Trigger_Fader"),
        NodeItem("NODE_MI_BL_MIDI_Trigger_Pan"),
    ]),
    MI_BL_NodeCategory('OUTPUT', "Output", items=[
        NodeItem("NODE_MI_BL_group_output"),
    ]),
    MI_BL_NodeCategory('GEOSHARED', "Geo Shared", items=[
        NodeItem("NODE_MI_BL_MATH_add"),
        NodeItem("MidiInteractiveStoreNamedAttribute"),
    ]),
    MI_BL_NodeCategory('TEST', "Testing", items=[
        NodeItem("NODE_MI_BL_Test"),
    ]),
]


# def draw_menu(self, context):
#     print(context.area.ui_type)
#     if context.area.ui_type == 'MidiInteractiveTree':
#         layout = self.layout
#         layout.separator()
#         layout.operator("node.duplicate_move", text="My new context menu item")
