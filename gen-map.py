import random
import json
import sys

# alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=~!@#$%^&*()_+[{]};:{/|?<>,."
alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ "

def genChar():
    return random.choice(alphabet)

def genWord():
    word = ""
    for i in range(random.randint(16,128)):
        word += genChar()
    return word

def genCharMap(char, size):
    result = []
    sizePow = 8
    if size == "XS":
        sizePow = 4
    if size == "L":
        sizePow = 16
    for i in range(2**sizePow):
        result.append(genWord())
    return result

def genMap(size):
    theMap = {}
    for char in alphabet:
        theMap[char] = genCharMap(char, size)
    return theMap
            

args = sys.argv[1:]
size = "S"
if len(args) > 0:
    size = args[0]

filename = "map-small.json"
content = genMap(size)
file = open(filename,"wt")
file.write(
    json.dumps(content,indent="    ")
)
file.close()

print("Map generated successfully at "+filename+"!")