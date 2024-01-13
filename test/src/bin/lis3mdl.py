import time
import adafruit_lis3mdl
import board

i2c = board.I2C()
sensor = adafruit_lis3mdl.LIS3MDL(i2c)

while True:
    mag_x, mag_y, mag_z = sensor.magnetic

    print("X:{0:10.2f}, Y:{1:10.2f}, Z:{2:10.2f} uT".format(mag_x, mag_y, mag_z))
    print("")
    time.sleep(1.0)
