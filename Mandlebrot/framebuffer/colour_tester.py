colours = [[255, 255, 255], [251, 243, 5], [255, 100, 3], (221, 9, 7), (242, 8, 132), (71, 0, 165), (0, 0, 211), (2, 171, 234), 
           (31, 183, 20), (0, 100, 18), (86, 44, 5), (144, 113, 58), (192, 192, 192), (128,128,128), (64, 64, 64), (0,0,0)]

colour_ints = list()

print(f"Num Colours: {len(colours)}")
for colour in colours:
    red = colour[0]
    green = colour[1]
    blue = colour[2]

    a = int(f"{'{0:08b}'.format(red)}{'{0:08b}'.format(green)}{'{0:08b}'.format(blue)}", 2)
    print(a)
    colour_ints.append(a)

print(colour_ints)
print(", ".join(colour_ints))