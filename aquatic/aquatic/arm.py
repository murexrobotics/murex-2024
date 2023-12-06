servo_position = 0

def telemetry():
    json = {
        "servo_pos": servo_position
    }

    return json