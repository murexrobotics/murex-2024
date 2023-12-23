import time
import thrusters, bme680, camera, arm, pixel, i2c
from udpsocket import socket

UPDATE_TIME = 0.01

telemetrydict = {}
components = [thrusters, bme680, camera, arm] #can be added to

def main():
    for component in components:
        if (hasattr(component, '__init__') and component.isfunction(component.__init__)):
                component.__init__()

    while True:
        for component in components:
            if (hasattr(component, 'telemetry') and component.isfunction(component.telemetry)):
                telemetrydict.update(component.telemetry())
        socket.send(telemetrydict)
        time.sleep(UPDATE_TIME)
