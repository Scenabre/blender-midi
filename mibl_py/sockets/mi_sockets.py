from bpy.types import NodeSocket, NodeTreeInterfaceSocket

SOCKET_TYPE = 'MyCustomSocketType'


# Définir un socket personnalisé pour les entrées et la sortie
class MidiInteractiveSocket(NodeSocket):
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
