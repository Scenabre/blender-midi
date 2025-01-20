from mibllib import sum_float_custom
from nodeitems_utils import NodeCategory, NodeItem, register_node_categories, unregister_node_categories

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
        result = sum_float_custom(input_a, input_b)
        print("Node compute : ", result)
        # self.outputs['Output'].my_custom_property = result

    def draw_label(self):
        return "I am a custom node"
