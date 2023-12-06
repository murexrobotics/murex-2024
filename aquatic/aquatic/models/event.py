# credit: https://www.geeksforgeeks.org/mimicking-events-python/
class Event(object):
    """
    Class that calls handlers when called

    py:class:: Event
    """

    def __init__(self):
        """
        Creates the Event with an empty list of handlers
        """
        self.__eventhandlers = []
 
    def __iadd__(self, handler):
        """"
            Overloads the += operator to add a handler to the event.

            :param handler: The handler to add
        """
        self.__eventhandlers.append(handler)
        return self
 
    def __isub__(self, handler):
        """
            Overloads the -= operator to remove a handler from the event.

            :param handler: The handler to remove
        """
        self.__eventhandlers.remove(handler)
        return self
 
    def __call__(self, *args, **keywargs):
        """
            Call the event handlers with the specified arguments.

            :param args: The arguments to pass to the event handlers
            :param keywargs: The keyword arguments to pass to the event handlers
        """
        for eventhandler in self.__eventhandlers:
            eventhandler(*args, **keywargs)