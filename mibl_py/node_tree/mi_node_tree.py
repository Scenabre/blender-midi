from bpy.types import NodeTree
from nodeitems_utils import NodeCategory

TREE_NAME = 'MidiInteractiveTree'


# Derived from the NodeTree base type, similar to Menu, Operator, Panel, etc.
class MI_BL_NodeTree(NodeTree):
    # Description string
    '''A custom node tree type that will show up in the editor type list'''
    # Optional identifier string. If not explicitly defined, the python class name is used.
    bl_idname = TREE_NAME
    # Label for nice name display
    bl_label = "Midi Interactive Nodes"
    # Icon identifier
    bl_icon = 'LINK_BLEND'


class MI_BL_NodeTreeNode:
    @classmethod
    def poll(cls, ntree):
        return ntree.bl_idname == TREE_NAME


class MI_BL_NodeCategory(NodeCategory):
    @classmethod
    def poll(cls, context):
        return context.space_data.tree_type == TREE_NAME

# def draw_menu(self, context):
#     print(context.area.ui_type)
#     if context.area.ui_type == TREE_NAME:
#         layout = self.layout
#         layout.separator()
#         layout.operator("node.duplicate_move",
#                          text="My new context menu item")
