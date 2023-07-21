# REFERENCE = https://learn.adafruit.com/adafruit-mmc5603-triple-axis-magnetometer/python-circuitpython
from busio import I2C
from board import SDA, SCL
from adafruit_mmc56x3 import MMC5603
import time

i2c = I2C(SCL, SDA)
assert(i2c.scan() is not None, "I2C not working")

mmc = MMC5603(i2c)

for _ in range(10):
    mag_x, mag_y, mag_z = mmc.magnetic
    temp = mmc.temperature
    print(f"X: {mag_x} uT   Y: {mag_y} uT   Z: {mag_z} uT   Temp:{temp} C\n")
    time.sleep(1)
