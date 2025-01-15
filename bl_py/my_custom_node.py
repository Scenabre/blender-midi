import bpy
import my_rust_lib
from bpy.types import NodeTree, Node, NodeTreeInterfaceSocket, Menu, NODE_MT_add
from nodeitems_utils import NodeCategory, NodeItem, register_node_categories, unregister_node_categories
from bl_ui import node_add_menu

SOCKET_TYPE = 'MyCustomSocketType'


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


# Définir un socket personnalisé pour les entrées et la sortie
class MidiInteractiveSocket(bpy.types.NodeSocket):
    """Custom node socket type"""
    bl_idname = SOCKET_TYPE
    bl_label = 'Custom Socket'

    # Définir les propriétés du socket
    my_custom_property: bpy.props.FloatProperty(name="Custom Property", default=0.0)

    def draw(self, context, layout, node, text):
        if self.is_output or self.is_linked:
            layout.label(text=text)
        else:
            layout.prop(self, "my_custom_property", text=text)

    # Socket color
    @classmethod
    def draw_color_simple(cls):
        return (1.0, 0.4, 0.216, 0.5)


class MidiInteractivePanel(bpy.types.Panel):
    """Creates a Panel in the scene context of the properties editor"""
    bl_label = "Layout Demo"
    bl_idname = "NODE_PT_MidiInteractiveTree"
    bl_space_type = 'NODE_EDITOR'
    bl_region_type = 'UI'
    bl_category = 'Custom'

    @classmethod
    def poll(cls, context):
        return context.space_data.tree_type == 'MidiInteractiveTree'

    def draw(self, context):
        layout = self.layout

        scene = context.scene

        # Create a simple row.
        layout.label(text=" Simple Row:")

        row = layout.row()
        row.prop(scene, "frame_start")
        row.prop(scene, "frame_end")

        # Create an row where the buttons are aligned to each other.
        layout.label(text=" Aligned Row:")

        row = layout.row(align=True)
        row.prop(scene, "frame_start")
        row.prop(scene, "frame_end")

        # Create two columns, by using a split layout.
        split = layout.split()

        # First column
        col = split.column()
        col.label(text="Column One:")
        col.prop(scene, "frame_end")
        col.prop(scene, "frame_start")

        # Second column, aligned
        col = split.column(align=True)
        col.label(text="Column Two:")
        col.prop(scene, "frame_start")
        col.prop(scene, "frame_end")

        # Big render button
        layout.label(text="Big Button:")
        row = layout.row()
        row.scale_y = 3.0
        row.operator("render.render")

        # Different sizes in a row
        layout.label(text="Different button sizes:")
        row = layout.row(align=True)
        row.operator("render.render")

        sub = row.row()
        sub.scale_x = 2.0
        sub.operator("render.render")

        row.operator("render.render")


# Customizable interface properties to generate a socket from.
class MidiInteractiveInterfaceSocket(NodeTreeInterfaceSocket):
    # The type of socket that is generated.
    bl_socket_idname = SOCKET_TYPE

    default_value: bpy.props.FloatProperty(default=1.0, description="Default input value for new sockets")

    def draw(self, context, layout):
        # Display properties of the interface.
        layout.prop(self, "default_value")

    # Set properties of newly created sockets
    def init_socket(self, node, socket, data_path):
        socket.my_custom_property = self.default_value

    # Use an existing socket to initialize the group interface
    def from_socket(self, node, socket):
        # Current value of the socket becomes the default
        self.default_value = socket.my_custom_property


# Mix-in class for all custom nodes in this tree type.
# Defines a poll function to enable instantiation.
class MidiInteractiveTreeNode:
    @classmethod
    def poll(cls, ntree):
        return ntree.bl_idname == 'MidiInteractiveTree'


# Définir un node personnalisé
class MyCustomTestNode(Node):
    '''A custom node'''
    bl_idname = 'MyCustomNodeType'
    bl_label = 'Custom Node'
    bl_icon = 'NODETREE'

    def init(self, context):
        # Ajouter des sockets d'entrée et de sortie
        self.inputs.new('MyCustomSocketType', "Input A")
        self.inputs.new('MyCustomSocketType', "Input B")
        self.outputs.new('MyCustomSocketType', "Output")

    # Copy function to initialize a copied node from an existing one.
    def copy(self, node):
        print("Copying from node ", node)

    # Free function to clean up on removal.
    def free(self):
        print("Removing node ", self, ", Goodbye!")

    def update(self):
        # Mettre à jour le node
        input_a = self.inputs['Input A'].my_custom_property
        input_b = self.inputs['Input B'].my_custom_property
        result = my_rust_lib.sum_float_custom(input_a, input_b)
        print("Node compute : ", result)
        # self.outputs['Output'].my_custom_property = result

    def draw_label(self):
        return "I am a custom node"


class MyNodeCategory(NodeCategory):
    @classmethod
    def poll(cls, context):
        return context.space_data.tree_type == 'MidiInteractiveTree'

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

classes = (
    MidiInteractiveTree,
    MidiInteractiveSocket,
    MidiInteractiveInterfaceSocket,
    MidiInteractivePanel,
    MyCustomTestNode,
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


def menu_func(self, context):
    if context.space_data.tree_type == 'MidiInteractiveTree':
        self.layout.menu("NODE_MT_MidiInteractive_add_all")


def register():
    from bpy.utils import register_class
    for cls in classes:
        register_class(cls)
    NODE_MT_add.prepend(menu_func)

    # bpy.types.NODE_MT_context_menu.append(draw_menu)
    register_node_categories('CUSTOM_NODES', node_categories)


def unregister():
    # bpy.types.NODE_MT_context_menu.remove(draw_menu)
    unregister_node_categories('CUSTOM_NODES')

    from bpy.utils import unregister_class
    for cls in reversed(classes):
        unregister_class(cls)
    NODE_MT_add.remove(menu_func)


if __name__ == "__main__":
    register()
