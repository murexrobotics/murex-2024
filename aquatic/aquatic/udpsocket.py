import asyncio
import timeit

from models.socket import UDPSocket

from utils.logger import logger;

WS_IP = "127.0.0.1"
WS_PORT = 5555

# would import this in other modules
socket = UDPSocket(WS_IP, WS_PORT)

async def main():
    await socket.open()

    start = timeit.default_timer()

    for _ in range(100000):
        message = {
            "a": "hello",
            "b": "world"
        }

        socket.send(message)
        
    
    end = timeit.default_timer()
    logger.info(f"Time: {end - start}")

    await asyncio.sleep(10)

    socket.close()

asyncio.run(main())

# if __name__ == "__main__":
#     asyncio.run(main())