import bpy
import my_rust_lib
from bpy.types import NodeTree, Node, NodeSocket, NodeTreeInterfaceSocket

# Derived from the NodeTree base type, similar to Menu, Operator, Panel, etc.
class MyCustomTree(NodeTree):
    # Description string
    '''A custom node tree type that will show up in the editor type list'''
    # Optional identifier string. If not explicitly defined, the python class name is used.
    bl_idname = 'CustomTreeType'
    # Label for nice name display
    bl_label = "Custom Node Tree"
    # Icon identifier
    bl_icon = 'NODETREE'

# Définir un socket personnalisé pour les entrées et la sortie
class MyCustomSocket(bpy.types.NodeSocket):
    """Custom node socket type"""
    bl_idname = 'MyCustomSocketType'
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

# Customizable interface properties to generate a socket from.
class MyCustomInterfaceSocket(NodeTreeInterfaceSocket):
    # The type of socket that is generated.
    bl_socket_idname = 'MyCustomSocketType'

    default_value: bpy.props.FloatProperty(default=1.0, description="Default input value for new sockets",)

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
class MyCustomTreeNode:
    @classmethod
    def poll(cls, ntree):
        return ntree.bl_idname == 'GeometryNodeTree'

# Définir un node personnalisé
class MyCustomNode(Node):
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
        print("Node compute : ",result)
        # self.outputs['Output'].my_custom_property = result

    def draw_label(self):
        return "I am a custom node"

import nodeitems_utils
from nodeitems_utils import NodeCategory, NodeItem

class MyNodeCategory(NodeCategory):
    @classmethod
    def poll(cls, context):
        print(context.space_data.tree_type)
        return context.space_data.tree_type == 'GeometryNodeTree'


# all categories in a list
node_categories = [
    # identifier, label, items list
    MyNodeCategory('SOMENODES', "Some Nodes", items=[
        # our basic node
        NodeItem("MyCustomNodeType"),
    ]),
]

classes = (
    MyCustomSocket,
    MyCustomInterfaceSocket,
    MyCustomNode,
)

import bpy

def draw_menu(self, context):
    print(context.area.ui_type)
    if context.area.ui_type == 'GeometryNodeTree':
        layout = self.layout
        layout.separator()
        layout.operator("node.duplicate_move", text="My new context menu item")

def register():
    from bpy.utils import register_class
    for cls in classes:
        register_class(cls)

    bpy.types.NODE_MT_context_menu.append(draw_menu)
    # nodeitems_utils.register_node_categories('CUSTOM_NODES', node_categories)


def unregister():
    bpy.types.NODE_MT_context_menu.remove(draw_menu)
    # nodeitems_utils.unregister_node_categories('CUSTOM_NODES')

    from bpy.utils import unregister_class
    for cls in reversed(classes):
        unregister_class(cls)


if __name__ == "__main__":
    register()
