import serial

MASCP_FORMAT = "[{address}, {thrust_magnitude}]"
THRUSTER_MAX = 255
THRUSTER_NEUTRAL = 127
THRUSTER_MIN = 0

thrusters = []

class Thruster:
    def __init__(self, name, address):
        self.name = name
        self.address = address
        self.set_throttle(THRUSTER_NEUTRAL)

    def __del__(self):
        self.set_throttle(THRUSTER_NEUTRAL)

    def update(self):
        pass

    def set_throttle(self, throttle, from_topside=False):

        if from_topside:
            # convert from value between -1 and 1 to 0 and 255
            throttle = (throttle + 1) * THRUSTER_NEUTRAL

        # Ensure throttle is within bounds
        throttle = max(THRUSTER_MIN, min(THRUSTER_MAX, throttle))

        self.throttle = throttle
        MASCP_FORMAT.format(address=self.address, thrust_magnitude=self.throttle)

    def get_throttle(self):
        return self.throttle
    
    def telemetry(self):
        return {
            "name": self.name,
            "port": self.port,
            "throttle": self.throttle
        }

def update():
    for thruster in thrusters:
        thruster.update()

def telemetry():
    json = {
        "thrusters": {}
    }

    for thruster in thrusters:
        json["thrusters"][thruster.name] = thruster.telemetry()

    return json