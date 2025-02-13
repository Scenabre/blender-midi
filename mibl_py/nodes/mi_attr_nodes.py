import bpy
from bpy.types import Node
from bpy.props import StringProperty, EnumProperty


class NODE_MI_BL_set_attr(Node):
    bl_idname = 'MidiInteractiveStoreNamedAttribute'
    bl_label = 'MI Store Named Attribute'

    attribute_name: StringProperty(name='',
                                   default="custom_attr"
                                   )
    attribute_domain: EnumProperty(
        name='',
        items=(
            ('POINT', "Point", ""),
            ('EDGE', "Edge", ""),
            ('FACE', "Face", ""),
            ('CORNER', "Corner", ""),
            ('INSTANCE', "Instance", ""),
        ),
        default='POINT'
    )
    attribute_type: EnumProperty(
        name="",
        items=(
            ('FLOAT', "Float", ""),
            ('INT', "Integer", ""),
            ('FLOAT_VECTOR', "Float Vector", ""),
            ('FLOAT_COLOR', "Float Color", ""),
            ('BYTE_COLOR', "Byte Color", ""),
            ('STRING', "String", ""),
            ('BOOLEAN', "Boolean", ""),
        ),
        default='FLOAT'
    )

    attr_handlers: {
                'FLOAT':
                    ('value',
                     lambda:
                         [self.inputs['Value'].default_value] * len(attr.data)
                     ),
                'INT':
                    ('value',
                     lambda:
                         [self.inputs['Value'].default_value] * len(attr.data)
                     ),
                'FLOAT_VECTOR':
                    ('vector',
                     lambda:
                         [self.inputs['Value'].default_value[i] for i in range(3)] * (len(attr.data) // 3)
                     ),
                'FLOAT_COLOR':
                    ('color',
                     lambda:
                         [self.inputs['Value'].default_value[i] for i in range(4)] * (len(attr.data) // 4)
                     ),
                'BYTE_COLOR':
                    ('color',
                     lambda:
                         [int(self.inputs['Value'].default_value[i] * 255) for i in range(4)] * (len(attr.data) // 4)
                     ),
                'STRING':
                    ('string',
                     lambda:
                         [self.inputs['Value'].default_value] * len(attr.data)
                    ),
                'BOOLEAN':
                    ('boolean',
                     lambda:
                         [self.inputs['Value'].default_value] * len(attr.data)
                    ),
            }

    def init(self, context):
        socket_mapping = {
            'FLOAT': 'NodeSocketFloat',
            'INT': 'NodeSocketFloat',
            'FLOAT_VECTOR': 'NodeSocketVector',
            'FLOAT_COLOR': 'NodeSocketColor',
            'BYTE_COLOR': 'NodeSocketColor',
            'STRING': 'NodeSocketString',
            'BOOLEAN': 'NodeSocketBool',
        }
        socket_type = socket_mapping.get(self.attribute_type)

        self.inputs.new('NodeSocketGeometry', "Geometry")

        if socket_type:
            self.inputs.new(socket_type, "Value")

    def draw_buttons(self, context, layout):
        layout.prop(self, "attribute_name")
        layout.prop(self, "attribute_type")
        layout.prop(self, "attribute_domain")

    def update(self):
        obj_data = self.inputs["Geometry"].get("Geometry")

        if obj_data:
            domain = self.attribute_domain.lower()
            attr_type = self.attribute_type

            if self.attribute_name in obj_data.attributes:
                attr = obj_data.attributes[self.attribute_name]
            else:
                attr = obj_data.attributes.new(
                    self.attribute_name,
                    attr_type,
                    domain
                )

            handler = self.attr_handlers.get(attr_type)
            if handler:
                data_type, prepare_data = handler
                attr_data = prepare_data()
                attr.data.foreach_set(data_type, attr_data)
