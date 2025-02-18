import bpy


def execute_active_node_tree():
    print("Trigger node tree execute")
    node_editor = next((a for a in bpy.context.screen.areas if a.type == 'NODE_EDITOR'), None)
    if node_editor is None:
        return
    for space in node_editor.spaces:
        node_tree = getattr(space, 'node_tree')
        if (node_tree):
            node_tree.execute(bpy.context)
            break
