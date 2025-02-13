from ..node_tree.mi_node_tree import TREE_NAME
from bpy.types import Panel


class MI_BL_Panel(Panel):
    """Creates a Panel in the scene context of the properties editor"""
    bl_label = "Layout Demo"
    bl_idname = "NODE_PT_" + TREE_NAME
    bl_space_type = 'NODE_EDITOR'
    bl_region_type = 'UI'
    bl_category = 'Custom'

    @classmethod
    def poll(cls, context):
        return context.space_data.tree_type == TREE_NAME

    def draw(self, context):
        layout = self.layout
        scene = context.scene.mibl

        row = layout.row()
        row.prop(scene, "mi_run_server", text="Server Running")

        row = layout.row()
        if scene.mi_run_server:
            row.operator("mibl.stop_server", text="Stop Server")
        else:
            row.operator("mibl.start_server", text="Start Server")
