from busio import I2C
from board import SDA0, SCL0
import adafruit_ads1x15.ads1015 as ADS
import adafruit_ads1x15.analog_in as AnalogIn
import time

i2c = I2C(SCL0, SDA0)
assert(i2c.scan() is not None, "I2C not working")

ads = ADS.ADS1015(i2c)
channel = AnalogIn(ads, ADS.P0) # Change to any analog input 0-3

for _ in range(10):
    print(f"Voltage: {channel.voltage}")
    time.sleep(1)
