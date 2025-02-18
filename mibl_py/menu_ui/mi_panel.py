from .. node_tree.mi_node_tree import TREE_NAME
from bpy.types import Panel


class MI_BL_Panel(Panel):
    """MiBl Panel to manage the midi server"""
    bl_label = "MiBl Panel"
    bl_idname = "NODE_PT_" + TREE_NAME
    bl_space_type = 'NODE_EDITOR'
    bl_region_type = 'UI'
    bl_category = 'MiBl'

    @classmethod
    def poll(cls, context):
        return context.space_data.tree_type == TREE_NAME

    def draw(self, context):
        layout = self.layout
        mibl_props = context.scene.mibl

        row = layout.row()

        if mibl_props.mi_run_server:
            row.operator("mibl.set_server_state", text="Stop Midi Server")
        else:
            row.operator("mibl.set_server_state", text="Start Midi Server")

        layout.label(text="Server controls :")
        layout.prop(mibl_props, 'mi_use_system_ctlr', text="Use MC system control")
