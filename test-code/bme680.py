from busio import I2C
from board import SDA, SCL
from adafruit_bme680 import Adafruit_BME680_I2C
import time

i2c = I2C(SCL, SDA)
assert(i2c.scan() is not None, "I2C not working")

bme = Adafruit_BME680_I2C(i2c)

for _ in range(10):
    print(f"Temperature: {bme.temperature}")
    print(f"Humidity: {bme.humidity}")
    print(f"Pressure: {bme.pressure}")
    print(f"Gas: {bme.gas}")
    print(f"Altitude: {bme.altitude}\n")
    time.sleep(1)



