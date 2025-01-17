from bpy.types import Menu, NODE_MT_add
from bpy.utils import register_class, unregister_class
from bl_ui import node_add_menu


class NODE_MT_MidiInteractive_MIDI_TEST(Menu):
    bl_idname = "NODE_MT_MidiInteractive_MIDI_TEST"
    bl_label = "Test"

    def draw(self, _context):
        layout = self.layout
        node_add_menu.add_node_type(layout, "MyCustomNodeType")


class NODE_MT_MidiInteractive_GEO_SHARED(Menu):
    bl_idname = "NODE_MT_MidiInteractive_GEO_SHARED"
    bl_label = "Shared Nodes"

    def draw(self, _context):
        layout = self.layout
        node_add_menu.add_node_type(layout, "MidiInteractiveAttributeStatistic")
        node_add_menu.add_node_type(layout, "MidiInteractiveAttributeDomainSize")
        layout.separator()
        node_add_menu.add_node_type(layout, "MidiInteractiveBlurAttribute")
        node_add_menu.add_node_type(layout, "MidiInteractiveCaptureAttribute")
        node_add_menu.add_node_type(layout, "MidiInteractiveRemoveAttribute")
        node_add_menu.add_node_type(layout, "MidiInteractiveStoreNamedAttribute", search_weight=1.0)
        node_add_menu.draw_assets_for_catalog(layout, self.bl_label)


class NODE_MT_MidiInteractive_add_all(Menu):
    bl_idname = "NODE_MT_MidiInteractive_add_all"
    bl_label = "Utils"

    def draw(self, context):
        layout = self.layout
        layout.menu("NODE_MT_MidiInteractive_MIDI_TEST")
        layout.separator()
        layout.menu("NODE_MT_MidiInteractive_GEO_SHARED")


# all categories in a list
node_categories = [
    # identifier, label, items list
    MyNodeCategory('SOMENODES', "Some Nodes", items=[
        # our basic node
        NodeItem("MyCustomNodeType"),
    ]),
]


def menu_func(self, context):
    if context.space_data.tree_type == 'MidiInteractiveTree':
        self.layout.menu("NODE_MT_MidiInteractive_add_all")


classes = (
    NODE_MT_MidiInteractive_add_all,
    NODE_MT_MidiInteractive_GEO_SHARED,
    NODE_MT_MidiInteractive_MIDI_TEST,
)


# def draw_menu(self, context):
#     print(context.area.ui_type)
#     if context.area.ui_type == 'MidiInteractiveTree':
#         layout = self.layout
#         layout.separator()
#         layout.operator("node.duplicate_move", text="My new context menu item")
def register():
    from bpy.utils import register_class
    for cls in classes:
        register_class(cls)
    NODE_MT_add.prepend(menu_func)

    # bpy.types.NODE_MT_context_menu.append(draw_menu)


def unregister():
    # bpy.types.NODE_MT_context_menu.remove(draw_menu)

    from bpy.utils import unregister_class
    for cls in reversed(classes):
        unregister_class(cls)
    NODE_MT_add.remove(menu_func)


if __name__ == "__main__":
    register()
