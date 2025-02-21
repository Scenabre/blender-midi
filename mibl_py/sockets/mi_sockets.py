from bpy.props import FloatProperty
from bpy.types import NodeSocket, NodeTreeInterfaceSocket

SOCKET_TYPE = 'SOCKET_MI_BL_Midi_Recipe'


class SOCKET_MI_BL_MidiRecipe(NodeSocket):
    """Just a custom socket type for test"""
    bl_idname = SOCKET_TYPE
    bl_label = 'mi_bl_socket_midi_recipe'

    mibl_float: FloatProperty(
        name="mibl_float",
        default=0.0
    )

    def draw(self, context, layout, node, text):
        layout.label(text=text)

    @classmethod
    def draw_color_simple(cls):
        return (1.0, 0.4, 0.216, 0.5)


class SOCKET_INT_MI_BL_Midirecipe(NodeTreeInterfaceSocket):
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
