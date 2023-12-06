import asyncio
from models.event import Event
from utils.logger import logger;
import json

class UDPSocket:
    """
    A class that represents a UDP Socket.

    py:class:: UDPSocket(ip: str, port: int)
        py:property:: ip: str
        py:property:: port: int
        py:property:: transport: Transport
        py:property:: protocol: UDPClientProtocol
        py:property:: on_message_recieved: Event
        py:classmethod:: open()
        py:classmethod:: close()
        py:classmethod:: send(msg: object)
    """

    class UDPClientProtocol:
        def __init__(self, on_message_recieved: Event):
            """
            Creates a new UDPClientProtocol.

            :param on_message_recieved: The event to fire when a message is recieved.
            :type on_message_recieved: Event
            """
            self.on_message_recieved = on_message_recieved
            self.transport = None

        def connection_made(self, transport):
            """
            Definition for when the connection is made.

            :param transport: The transport.
            """
            self.transport = transport
            logger.info("UDP Socket connection made.")

        def datagram_received(self, data, addr):
            """
            Definition for when a datagram message is recieved.

            :param data: The data recieved.
            :param addr: The address the data was recieved from. I have no clue why this is needed.
            """
            msg = json.loads(data.decode())
            logger.info(f"Message recieved: {data}")
            self.on_message_recieved(msg)

        def error_received(self, exc):
            """
            Definition for when an error is recieved.

            :param exc: The error recieved.
            """
            logger.error(f"UDP Socket error recieved: {exc}")

        def connection_lost(self, exc):
            """
            Definition for when the connection is lost.

            :param exc: The exception that caused the connection to be lost.
            """
            logger.warning(f"UDP Socket connection lost. {exc}")

    def __init__(self, ip: str, port: int):
        """
        Creates a new UDPSocket.

        :param ip: The IP to connect to.
        :type ip: str
        :param port: The port to connect to.
        :type port: int
        """
        self.ip = ip
        self.port = port
        self.on_message_recieved = Event()
        self.transport = None
        self.protocol = None

    async def __enter__(self):
        """
        Opens the UDP Socket and returns itself.

        :returns: The UDPSocket.
        :rtype: UDPSocket
        """
        await self.open()
        return self
    
    def __exit__(self, *args):
        """
        Closes the UDP Socket.

        :param exc_type: The type of exception.
        :param exc_value: The exception value.
        :param traceback: The traceback.
        """
        self.close()

    async def open(self):
        """
        Opens the UDP Socket and attempts to connect to the specified IP and port.

        :async:
        :raises Exception: If the UDP Socket is already open.
        """
        if self.transport is not None and not self.transport.is_closing():
            logger.error("Attempted to open UDP Socket when it was already open.")
            raise Exception("Attempted to open UDP Socket when it was already open.")
        
        loop = asyncio.get_running_loop()

        self.transport, self.protocol = await loop.create_datagram_endpoint(
            lambda: self.UDPClientProtocol(self.on_message_recieved),
            remote_addr=(self.ip, self.port))
        
        logger.info(f"UDP Socket started on {self.ip}:{self.port}")

    def close(self):
        """
        Asynchronously closes the UDP Socket.

        :raises Exception: If the UDP Socket is not open.
        """
        if self.transport is None or self.transport.is_closing():
            logger.error("Attempted to close UDP Socket when it was not open.")
            raise Exception("Attempted to close UDP Socket when it was not open.")
        
        self.transport.close()
        self.transport = None
        self.protocol = None
        logger.info("UDP Socket closed.")

    def send(self, msg: dict):
        """
        Asynchronously sends a message over the UDP Socket.

        :param msg: The message to send.
        :type msg: object
        :raises Exception: If the UDP Socket is not open.
        """
        if self.transport is None or self.transport.is_closing():
            logger.error("Attempted to send a message over the UDP Socket when it was not open.")
            raise Exception("Attempted to send a message over the UDP Socket when it was not open.")
        
        js = json.dumps(msg).encode()
        self.transport.sendto(js)