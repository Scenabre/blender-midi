from ..node_tree.mi_node_tree import TREE_NAME
from bpy.types import Panel


class MI_BL_Panel(Panel):
    """MiBl Panel to manage the midi server"""
    bl_label = "MiBl Panel"
    bl_idname = "NODE_PT_" + TREE_NAME
    bl_space_type = 'NODE_EDITOR'
    bl_region_type = 'UI'
    bl_category = 'Custom'

    @classmethod
    def poll(cls, context):
        return context.space_data.tree_type == TREE_NAME

    def draw(self, context):
        layout = self.layout

        row = layout.row()
        row.operator("mibl.set_server_state", text="Start/Stop Midi server")
