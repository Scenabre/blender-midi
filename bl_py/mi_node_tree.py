from bpy.types import NodeTree, NodeCategory
from bpy.utils import register_class, unregister_class

# Derived from the NodeTree base type, similar to Menu, Operator, Panel, etc.
class MidiInteractiveTree(NodeTree):
    # Description string
    '''A custom node tree type that will show up in the editor type list'''
    # Optional identifier string. If not explicitly defined, the python class name is used.
    bl_idname = 'MidiInteractiveTree'
    # Label for nice name display
    bl_label = "Midi Interactive Nodes"
    # Icon identifier
    bl_icon = 'LINK_BLEND'


# Mix-in class for all custom nodes in this tree type.
# Defines a poll function to enable instantiation.
# class MidiInteractiveTreeNode:
#     @classmethod
#     def poll(cls, ntree):
#         return ntree.bl_idname == 'MidiInteractiveTree'


class MyNodeCategory(NodeCategory):
    @classmethod
    def poll(cls, context):
        return context.space_data.tree_type == 'MidiInteractiveTree'


classes = (
    MidiInteractiveTree,
    MyNodeCategory
)


# def draw_menu(self, context):
#     print(context.area.ui_type)
#     if context.area.ui_type == 'MidiInteractiveTree':
#         layout = self.layout
#         layout.separator()
#         layout.operator("node.duplicate_move", text="My new context menu item")
def register():
    for cls in classes:
        register_class(cls)


def unregister():
    for cls in reversed(classes):
        unregister_class(cls)


if __name__ == "__main__":
    register()
