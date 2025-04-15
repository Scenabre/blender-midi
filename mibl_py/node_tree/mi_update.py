import bpy


def execute_active_node_tree():
    for node_tree in bpy.data.node_groups:
        if hasattr(node_tree, '_isMiNodetree'):
            print("Trigger node tree execute")
            node_tree.execute(bpy.context)
