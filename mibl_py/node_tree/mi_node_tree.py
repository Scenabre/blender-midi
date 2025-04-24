from bpy.types import NodeTree
from nodeitems_utils import NodeCategory
from . mi_update import execute_active_node_tree

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

    _isMiNodetree = True
    _node_out = None
    _node_in = None

    def get_node_out(self):
        return self._node_out

    def get_node_in(self):
        return self._node_in

    def execute(self, context):
        self._node_out = self.nodes.get("MI Group Output", None)
        self._node_in = self.nodes.get("MI Group Input", None)

        if self._node_out is not None:
            self._node_out.update()

        layer = context.view_layer
        layer.update()


class MI_BL_Node:
    _isMiNode = True
    _index = -1
    _is_sys_node = False
    _is_trigger_node = False

    @classmethod
    def poll(cls, ntree):
        return ntree.bl_idname == TREE_NAME

    @classmethod
    def execute(self):
        pass

    def update(self):
        self.execute()
        # execute_active_node_tree()


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

# def update_prop(dself, context):
#     print("Update prop")
#     execute_active_node_tree()

