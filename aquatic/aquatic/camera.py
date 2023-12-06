camera_angle = 0 

def telemetry():
    json = {
        "angle": camera_angle
    }
    return json