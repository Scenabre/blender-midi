from bpy.types import NodeTree
from bpy.props import BoolProperty
from nodeitems_utils import NodeCategory
from . mi_update import execute_active_node_tree

TREE_NAME = 'MidiInteractiveTree'


# Derived from the NodeTree base type, similar to Menu, Operator, Panel, etc.
class MI_BL_NodeTree(NodeTree):
    '''A custom node tree type that will show up in the editor type list'''
    bl_idname = TREE_NAME
    bl_label = "Midi Interactive Nodes"
    bl_icon = 'LINK_BLEND'

    _isMiNodetree = True
    node_out = None
    node_in = None

    def get_node_out(self):
        return self.node_out

    def get_node_in(self):
        return self.node_in

    def execute(self, context):
        self.node_out = self.nodes.get("MI Group Output", None)
        self.node_in = self.nodes.get("MI Group Input", None)

        if self.node_out is not None:
            self.node_out.update()

        layer = context.view_layer
        layer.update()


class MI_BL_Node:
    _isMiNode = True
    _index = -1
    _is_sys_node = False
    _is_trigger_node = False

    need_update: BoolProperty()

    @classmethod
    def poll(cls, ntree):
        return ntree.bl_idname == TREE_NAME

    @classmethod
    def execute(self):
        pass

    def get_update_state(self):
        return self.need_update

    def set_update_state(self, update):
        self.need_update = update

    def update(self):
        self.execute()


class MI_BL_NodeCategory(NodeCategory):
    @classmethod
    def poll(cls, context):
        return context.space_data.tree_type == TREE_NAME
