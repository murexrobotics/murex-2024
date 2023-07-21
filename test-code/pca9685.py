from busio import I2C
from board import SDA, SCL
from adafruit_pca9685 import PCA9685
from adafruit_motor.servo import Servo
import time

i2c = I2C(SCL, SDA)
assert(i2c.scan() is not None, "I2C not working")

pca = PCA9685(i2c)
pca.frequency = 50
CHANNEL = 0 # Change to whatever channel is necessary

servo = Servo(pca.channels[CHANNEL])

servo.angle = 0
time.sleep(1)
servo.angle = 90
time.sleep(1)
servo.angle = 180
time.sleep(1)

pca.deinit()
