fn main() {
    println!("Hello, world!");
}


/* print('''
*******************************************************************************
          |                   |                  |                     |
 _________|________________.=""_;=.______________|_____________________|_______
|                   |  ,-"_,=""     `"=.|                  |
|___________________|__"=._o`"-._        `"=.______________|___________________
          |                `"=._o`"=._      _`"=._                     |
 _________|_____________________:=._o "=._."_.-="'"=.__________________|_______
|                   |    __.--" , ; `"=._o." ,-"""-._ ".   |
|___________________|_._"  ,. .` ` `` ,  `"-._"-._   ". '__|___________________
          |           |o`"=._` , "` `; .". ,  "-._"-._; ;              |
 _________|___________| ;`-.o`"=._; ." ` '`."\` . "-._ /_______________|_______
|                   | |o;    `"-.o`"=._``  '` " ,__.--o;   |
|___________________|_| ;     (#) `-.o `"=.`_.--"_o.-; ;___|___________________
____/______/______/___|o;._    "      `".o|o_.--"    ;o;____/______/______/____
/______/______/______/_"=._o--._        ; | ;        ; ;/______/______/______/_
____/______/______/______/__"=._o--._   ;o|o;     _._;o;____/______/______/____
/______/______/______/______/____"=._o._; | ;_.--"o.--"_/______/______/______/_
____/______/______/______/______/_____"=.o|o_.--""___/______/______/______/____
/______/______/______/______/______/______/______/______/______/______/[TomekK]
*******************************************************************************
      ''')
print("Welcome to Treasure Island!")
print("Your mission is to find the treasure.")

direction = input("Do you want to go left or right? ").lower()
if direction == "left":
    print("You travel west and encounter a giant river.")
elif direction == "right":
    print("The ground gives way to a giant sinkhole, unfortunately, you died.")
else:
    print("Please choose a direction.")

if direction == "left":
    swimming = input("Do you want to swim or wait?").lower()
    if swimming == "swim":
        print("You have been eaten by aligators.")
    elif swimming == "wait":
        print("A ferry has arrived to take you across the river safely.")
    else:
        print("Please choose to swim or wait.")

if direction == "left" and swimming == "wait":
    door = input("You stumble upon three colorfull doors. The first door is in a meadow and the color yellow. The second door is red and near a volcano. The third door is blue and near a lake. Which door do you choose?").lower()
    if door == "blue":
        print("As you approach the door. The vegetation you are walking on sinks. You are in a marsh! You struggle to break free from the mud. Your efforts are in vain as the mud grips you from the waist down pulling you under with every muscle movement. You eventually are swallowed whole by the earth.")
    elif door == "red":
        print("You approach the door and you can feel an intense heat growing as you get cloer. You open the door and lava spills forth, dissolving your body within seconds.")
    elif door == "yellow":
        print("You open the door with excitment for what lies beyond. As the door open you gasp in disbelief, before you is a sea of gold for as far as the eye can see. You have found the treasure!")
    else:
        print("Please choose blue, red, or yellow") */