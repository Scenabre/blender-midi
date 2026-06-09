def update_markers(context, frame):
    markers = context.scene.timeline_markers
    marker_not_found = True

    for marker in markers:
        if marker.frame == frame:
            marker_not_found = False
            markers.remove(marker)
            break

    if marker_not_found:
        marker_name = 'F_' + str(frame)
        markers.new(marker_name, frame=frame)


def update_count_ev(count_ev, ev, value):
    key = (ev, value)
    if key not in count_ev:
        if len(count_ev) >= 1:
            count_ev.clear()
        count_ev[key] = 1
    else:
        count_ev[key] += 1


def check_count_ev(count_ev, ev, value):
    key = (ev, value)
    if key not in count_ev:
        return 0
    else:
        return count_ev[key]


def clean_count_ev(count_ev, ev, value):
    key = (ev, value)
    del count_ev[key]


def get_areas(context, area_name):
    areas = []
    for area in context.screen.areas:
        if area.type == area_name:
            areas.append(area)
    return areas


def get_area(context, area_name):
    return get_areas(context, area_name)[0]


def set_persportho(context):
    areas = get_areas(context, 'VIEW_3D')
    for area in areas:
        with context.temp_override(area=area, region=area.regions[-1]):
            bpy.ops.view3d.view_persportho()


def set_view_orbit(context, orbit_dir):
    orbits_dir = {
        0: 'ORBITLEFT',
        1: 'ORBITRIGHT',
        2: 'ORBITUP',
        3: 'ORBITDOWN'
    }

    if orbit_dir in orbits_dir:
        area = get_area(context, 'VIEW_3D')
        with context.temp_override(area=area, region=area.regions[-1]):
            bpy.ops.view3d.view_orbit(type=orbits_dir[orbit_dir])


def set_prop_layout(context, tab_num):
    prop_name = {
        0: "TOOL",
        1: "SCENE",
        2: "RENDER",
        3: "OUTPUT",
        4: "VIEW_LAYER",
        5: "WORLD",
        6: "COLLECTION",
        7: "OBJECT",
        8: "CONSTRAINT",
        9: "MODIFIER",
        10: "DATA",
        11: "BONE",
        12: "BONE_CONSTRAINT",
        13: "MATERIAL",
        14: "TEXTURE",
        15: "PARTICLES",
        16: "PHYSICS",
        17: "SHADERFX",
    }
    area = get_area(context, 'PROPERTIES')
    area.spaces[0].context = prop_name[tab_num]

def update_all_inputs(self):
    for input in self.inputs:
        if input.is_linked:
            for link in input.links:
                link.from_node.execute()

def update_all_outputs(self):
    for output in self.outputs:
        if output.is_linked:
            for link in output.links:
                link.to_node.execute()

def update_prop(self,context):
    self.execute()

