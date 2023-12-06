import board
import adafruit_bme680

i2c = board.I2C()
sensor = adafruit_bme680.Adafruit_BME680_I2C(i2c)
sensor.seaLevelhPa = 1013.25 # from last year's codebase

def telemetry():
    json = { 
        "temperature": sensor.temperature,
        "humidity": sensor.humidity,
        "pressure": sensor.pressure,
        "gas": sensor.gas,
        "altitude": sensor.altitude
    }

    return json