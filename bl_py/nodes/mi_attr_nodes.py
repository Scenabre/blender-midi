import bpy
from bpy.types import Node
from bpy.props import StringProperty, EnumProperty, FloatProperty

# Define the custom node
class MidiInteractive_set_attr(Node):
    bl_idname = 'CustomNodeType'
    bl_label = 'Custom Node'

    # Define properties for the node
    attribute_name: StringProperty(name="Attribute Name", default="custom_attr")
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
        # Create input sockets based on the attribute type
        if self.attribute_type in ['FLOAT', 'INT']:
            self.inputs.new('NodeSocketFloat', "Value")
        elif self.attribute_type == 'FLOAT_VECTOR':
            self.inputs.new('NodeSocketVector', "Value")
        elif self.attribute_type == 'FLOAT_COLOR':
            self.inputs.new('NodeSocketColor', "Value")
        elif self.attribute_type == 'BYTE_COLOR':
            self.inputs.new('NodeSocketColor', "Value")
        elif self.attribute_type == 'STRING':
            self.inputs.new('NodeSocketString', "Value")
        elif self.attribute_type == 'BOOLEAN':
            self.inputs.new('NodeSocketBool', "Value")

    def update(self):
        # Get the object associated with the node tree
        obj = bpy.context.object

        if obj and obj.type == 'MESH':
            # Get the attribute domain and type
            domain = self.attribute_domain.lower()
            attr_type = self.attribute_type

            # Create or update the named attribute
            if self.attribute_name in obj.data.attributes:
                attr = obj.data.attributes[self.attribute_name]
            else:
                attr = obj.data.attributes.new(self.attribute_name, attr_type, domain)

            # Set the attribute value (for demonstration, setting all values to the input value)
            if attr_type in ['FLOAT', 'INT']:
                input_value = self.inputs['Value'].default_value
                attr_data = [input_value] * len(attr.data)
                attr.data.foreach_set('value', attr_data)
            elif attr_type == 'FLOAT_VECTOR':
                input_value = self.inputs['Value'].default_value
                attr_data = [input_value[0], input_value[1], input_value[2]] * (len(attr.data) // 3)
                attr.data.foreach_set('vector', attr_data)
            elif attr_type == 'FLOAT_COLOR':
                input_value = self.inputs['Value'].default_value
                attr_data = [input_value[0], input_value[1], input_value[2], input_value[3]] * (len(attr.data) // 4)
                attr.data.foreach_set('color', attr_data)
            elif attr_type == 'BYTE_COLOR':
                input_value = self.inputs['Value'].default_value
                attr_data = [int(input_value[0] * 255), int(input_value[1] * 255), int(input_value[2] * 255), int(input_value[3] * 255)] * (len(attr.data) // 4)
                attr.data.foreach_set('color', attr_data)
            elif attr_type == 'STRING':
                input_value = self.inputs['Value'].default_value
                attr_data = [input_value] * len(attr.data)
                attr.data.foreach_set('string', attr_data)
            elif attr_type == 'BOOLEAN':
                input_value = self.inputs['Value'].default_value
                attr_data = [input_value] * len(attr.data)
                attr.data.foreach_set('boolean', attr_data)

# Register the custom node
def register():
    bpy.utils.register_class(MidiInteractive_set_attr)

def unregister():
    bpy.utils.unregister_class(MidiInteractive_set_attr)

if __name__ == "__main__":
    register()
