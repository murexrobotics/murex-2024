import time
import serial

print("Starting UART")
ser = serial.Serial(
        port='/dev/serial0', #Replace ttyS0 with ttyAM0 for Pi1,Pi2,Pi0
        baudrate = 115200,
        parity=serial.PARITY_NONE,
        stopbits=serial.STOPBITS_ONE,
        bytesize=serial.EIGHTBITS,
)

print("Sending Messages")
messages = [0x00, 0x01, 0x02, 0x03, 0x04]
for msg in messages:
    print(f"<<< {msg}")
    ser.write(b"Hello world")
    time.sleep(1)
    print(ser.read(size=1))
