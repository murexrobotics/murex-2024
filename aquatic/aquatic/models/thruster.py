import pyserial
import atexit

THRUSTER_MAX = 255
THRUSTER_NEUTRAL = 127
THRUSTER_MIN = 0

class Thruster:
    def __init__(self, name, address):
        self.name = name
        self.address = address
        self.serial = pyserial.Serial(
            port='/dev/serial0', #TODO: change this to the correct port? unsure.
            baudrate = 115200,
            parity=pyserial.PARITY_NONE,
            stopbits=pyserial.STOPBITS_ONE,
            bytesize=pyserial.EIGHTBITS,
        )
        self.set_throttle(THRUSTER_NEUTRAL)

        atexit.register(self.__deinit__)

    def __deinit__(self):
        self.set_throttle(THRUSTER_NEUTRAL)

    def set_throttle(self, throttle, from_topside=False):

        if from_topside:
            # convert from value between -1 and 1 to 0 and 255
            throttle = (throttle + 1) * THRUSTER_NEUTRAL

        # Ensure throttle is within bounds
        throttle = max(THRUSTER_MIN, min(THRUSTER_MAX, throttle))

        self.throttle = throttle

        # convert data to bytes
        throttle_byte = bytes([throttle])
        address_byte = bytes([self.address])

        # send to thruster
        self.serial.write([address_byte, throttle_byte])

    def get_throttle(self):
        return self.throttle
    
    def telemetry(self):
        return {
            "name": self.name,
            "port": self.port,
            "throttle": self.throttle
        }
