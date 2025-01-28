from bl_midi_interactive.node_tree.mi_node_tree import TREE_NAME
from bpy.types import Menu
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
        # node_add_menu.add_node_type(layout,
        #                             "MidiInteractiveAttributeStatistic"
        #                             )
        # node_add_menu.add_node_type(layout,
        #                             "MidiInteractiveAttributeDomainSize"
        #                             )
        # layout.separator()
        # node_add_menu.add_node_type(layout,
        #                             "MidiInteractiveBlurAttribute"
        #                             )
        # node_add_menu.add_node_type(layout,
        #                             "MidiInteractiveCaptureAttribute"
        #                             )
        # node_add_menu.add_node_type(layout,
        #                             "MidiInteractiveRemoveAttribute"
        #                             )
        node_add_menu.add_node_type(layout,
                                    "MidiInteractiveStoreNamedAttribute",
                                    search_weight=1.0
                                    )
        node_add_menu.draw_assets_for_catalog(layout,
                                              self.bl_label
                                              )


class NODE_MT_MidiInteractive_add_all(Menu):
    bl_idname = "NODE_MT_MidiInteractive_add_all"
    bl_label = "Utils"

    def draw(self, context):
        layout = self.layout
        layout.menu("NODE_MT_MidiInteractive_MIDI_TEST")
        layout.separator()
        layout.menu("NODE_MT_MidiInteractive_GEO_SHARED")
        layout.separator()
        layout.menu("NODE_MT_category_GEO_UTILITIES_ROTATION")
        layout.menu("NODE_MT_category_utilities_matrix")
        layout.menu("NODE_MT_category_GEO_UTILITIES_MATH")
        layout.menu("NODE_MT_category_GEO_UTILITIES_ROTATION")


# # all categories in a list
# node_categories = [
#     # identifier, label, items list
#     MyNodeCategory('SOMENODES', "Some Nodes", items=[
#         # our basic node
#         NodeItem("MyCustomNodeType"),
#     ]),
# ]


def menu_func(self, context):
    if context.space_data.tree_type == TREE_NAME:
        self.layout.menu("NODE_MT_MidiInteractive_add_all")


# def draw_menu(self, context):
#     print(context.area.ui_type)
#     if context.area.ui_type == 'MidiInteractiveTree':
#         layout = self.layout
#         layout.separator()
#         layout.operator("node.duplicate_move", text="My new context menu item")
