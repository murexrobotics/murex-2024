import json
import atexit
from models.thruster import Thruster
from udpsocket import socket

thrusters = []

def __init__():
    #Initializing thrusters, 0 is a placeholder for addresses
    t1 = Thruster("t1", 0)
    t2 = Thruster("t2", 0)
    t3 = Thruster("t3", 0)
    t4 = Thruster("t4", 0)
    t5 = Thruster("t5", 0)
    t6 = Thruster("t6", 0)

    thrusters.append(t1, t2, t3, t4, t5, t6)

    socket.on_message_recieved += handle_throttle_update

    atexit.register(__deinit__)

def __deinit__():
    socket.on_message_recieved -= handle_throttle_update

def set_throttle(throttle):
    for thruster in thrusters:
        thruster.set_throttle(throttle)

def handle_throttle_update(payload):
    msg = json.loads(payload)
    
    for thruster in thrusters:
        thruster.set_throttle(msg[thruster.name], True)


def telemetry():
    json = {
        "thrusters": [thruster.telemetry() for thruster in thrusters]
    }
    return json
