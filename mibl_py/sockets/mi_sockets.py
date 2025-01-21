from bpy.props import FloatProperty
from bpy.types import NodeSocket, NodeTreeInterfaceSocket

SOCKET_TYPE = 'SOCKET_MI_BL_Test'


class SOCKET_MI_BL_Test(NodeSocket):
    """Just a custom socket type for test"""
    bl_idname = SOCKET_TYPE
    bl_label = 'mi_bl_socket_test'

    my_custom_property: FloatProperty(
        name="Custom Property",
        default=0.0
    )

    def draw(self, context, layout, node, text):
        if self.is_output or self.is_linked:
            layout.label(text=text)
        else:
            layout.prop(self, "my_custom_property", text=text)

    # Socket color
    @classmethod
    def draw_color_simple(cls):
        return (1.0, 0.4, 0.216, 0.5)


class SOCKET_INT_MI_BL_Test(NodeTreeInterfaceSocket):
    bl_socket_idname = SOCKET_TYPE

    default_value: FloatProperty(
        default=1.0,
        description="Default input value for new sockets"
    )

    def draw(self, context, layout):
        layout.prop(self, "default_value")

    def init_socket(self, node, socket, data_path):
        socket.my_custom_property = self.default_value

    def from_socket(self, node, socket):
        self.default_value = socket.my_custom_property
