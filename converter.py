def hextobin(h):
  length = len(h) * 4
  integer = int(h, 16)
  return f'{integer:0>{length}b}'

hexfile = open("font.hex")
output = open("output.txt", "w")
print("Converting...")
for i in range(100):
  index = hexfile.read(4)
  hexfile.read(1)
  data = hexfile.readline()[:-1]
  index = int(index, 16)
  data = hextobin(data)
  if(index == 48):
    print(data)
  output.write(str(index) + ":\n")
  for j in range(16):
    stringLen = int(len(data) / 16)
    print(stringLen)
    output.write(str(data)[j*stringLen:(j+1)*stringLen] + "\n")
  output.write("\n")