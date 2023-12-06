thrusters = []

#Temporary until thruster code is written:
for thruster in thrusters:
    thruster.throttle = 0

def telemetry():
    json = {
        "thrusters": {}
    }

    for thruster in thrusters:
        json["thrusters"][thruster.name] = thruster.throttle

    return json