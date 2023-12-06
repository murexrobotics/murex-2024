import pygame

pygame.init()
pygame.joystick.init()

Left_Joystick_X = 0
Left_Joystick_Y = 1
Right_Joystick_X = 2
Right_Joystick_Y = 3

GamePad_Right = 14
GamePad_Left = 13
GamePad_Up = 11
GamePad_Down = 12

but_A = 0 
but_B = 1
but_X = 2
but_Y = 3

Cam_Torq = 1
Cam_Vert = 1
Arm_Torq = 1
Claw_Vert = 0.5
# Kept Same Indexes and Values from 2023

controller = pygame.joystick.Joystick(0)

def initial():
    num_controller = pygame.joystick.get_count()
    if num_controller == 1:
        controller.init()
        return("Controller Connected")
    elif num_controller > 1:
        return("Multiple Controllers Detected")
    elif num_controller < 1:
        return("No Controller Connceted")


def controller_input():

   x = controller.get_axis(Right_Joystick_X)
   y = controller.get_axis(Right_Joystick_Y)

   torq = controller.get_axis(Left_Joystick_X)
   z = controller.get_axis(Left_Joystick_Y)

#    if controller.get_button(GamePad_Left):
#         Cam_Torq = -Cam_Torq
#    elif controller.get_button(GamePad_Right):
#         Cam_Torq = Cam_Torq

#    if controller.get_button(but_X):
#        Arm_Torq = -Arm_Torq
#    elif controller.get_button(but_B):
#        Arm_Torq = Arm_Torq

#    if controller.get_button(but_Y):
#        Claw_Vert = -Claw_Vert
#    elif controller.get_button(but_A):
#        Claw_Vert = Claw_Vert 

   return (x, y, torq, z, Cam_Torq, Arm_Torq, Claw_Vert)

def quit():
    controller.quit()

if __name__ == "__main__":
    initial()
    controller_input()
    quit()