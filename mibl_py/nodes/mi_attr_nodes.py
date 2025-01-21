import bpy
from bpy.types import Node
from bpy.props import StringProperty, EnumProperty


class NODE_MI_BL_set_attr(Node):
    bl_idname = 'MidiInteractiveStoreNamedAttribute'
    bl_label = 'MI Store Named Attribute'

    attribute_name: StringProperty(name="Attribute Name",
                                   default="custom_attr"
                                   )
    attribute_domain: EnumProperty(
        name="Attribute Domain",
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
        name="Attribute Type",
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

    def init(self, context):
        # Mapping of attribute types to socket types
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

        if socket_type:
            self.inputs.new(socket_type, "Value")

    def update(self):
        # Get the object associated with the node tree
        obj = bpy.context.object

        if obj and obj.type == 'MESH':
            domain = self.attribute_domain.lower()
            attr_type = self.attribute_type

            if self.attribute_name in obj.data.attributes:
                attr = obj.data.attributes[self.attribute_name]
            else:
                attr = obj.data.attributes.new(
                    self.attribute_name,
                    attr_type,
                    domain
                )

            attr_handlers = {
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

            handler = attr_handlers.get(attr_type)
            if handler:
                data_type, prepare_data = handler
                attr_data = prepare_data()
                attr.data.foreach_set(data_type, attr_data)
