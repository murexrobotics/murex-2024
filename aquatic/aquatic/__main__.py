import time
import thrusters, bme680, camera, arm, pixel, i2c
import udpsocket #unsure

websocket = udpsocket.socket
telemetrydict = {}
components = [thrusters, bme680, camera, arm] #can be added to

def main():
    while True:
        for component in components:
            telemetrydict.update(component.telemetry())
        websocket.send(telemetrydict)
        time.sleep(0.01)
        '''
        TODO
        - Initialize thrusters
        - Receive information from topside
        - Update the state
        '''