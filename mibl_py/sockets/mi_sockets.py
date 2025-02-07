from bpy.props import FloatProperty
from bpy.types import NodeSocket, NodeTreeInterfaceSocket

SOCKET_TYPE = 'SOCKET_MI_BL_Test'


class SOCKET_MI_BL_Test(NodeSocket):
    """Just a custom socket type for test"""
    bl_idname = SOCKET_TYPE
    bl_label = 'mi_bl_socket_test'

    mibl_float: FloatProperty(
        name="mibl_float",
        default=0.0
    )

    def draw(self, context, layout, node, text):
        if self.is_output or self.is_linked:
            layout.label(text=text)
        else:
            layout.prop(self, "mibl_float", text=text)

    @classmethod
    def draw_color_simple(cls):
        return (1.0, 0.4, 0.216, 0.5)


class SOCKET_INT_MI_BL_Test(NodeTreeInterfaceSocket):
    bl_socket_idname = SOCKET_TYPE

    default_value: FloatProperty(
        default=1.0,
        description="Default socket value"
    )

    def draw(self, context, layout):
        layout.prop(self, "default_value")

    def init_socket(self, node, socket, data_path):
        socket.mibl_float = self.default_value

    def from_socket(self, node, socket):
        self.default_value = socket.mibl_float
