import time
import json
import thrusters, bme680, camera, arm, pixel, i2c
from udpsocket import socket

UPDATE_TIME = 0.01

thrusterslist = []
#Initializing thrusters, 0 is a placeholder for addresses
t1 = thrusters.Thruster("t1", 0)
t2 = thrusters.Thruster("t2", 0)
t3 = thrusters.Thruster("t3", 0)
t4 = thrusters.Thruster("t4", 0)
t5 = thrusters.Thruster("t5", 0)
t6 = thrusters.Thruster("t6", 0)

thrusterslist.append(t1, t2, t3, t4, t5, t6)

telemetrydict = {}

components = [bme680, camera, arm] #can be added to
for thruster in thrusterslist:
    components.append(thruster)

#for getting state info from topside and setting thruster values
def my_event_handler(message):
    msg = json.loads(message.decode())
    for thruster in thrusterslist:
        thruster.set_throttle(msg[thruster.name]["throttle"], True)


def main():
    while True:
        for component in components:
            if (hasattr(component, 'update') and component.isfunction(component.update)):
                component.update()
            if (hasattr(component, 'telemetry') and component.isfunction(component.telemetry)):
                telemetrydict.update(component.telemetry())
        socket.send(telemetrydict)
        time.sleep(UPDATE_TIME)
        socket.on_message_received += my_event_handler