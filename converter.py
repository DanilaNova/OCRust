# Converting string with hexadecimal number into a string with binary number
def hextobin(h):
  length = len(h) * 4
  integer = int(h, 16)
  return f'{integer:0>{length}b}'

hexfile = open("font.hex")
output = open("output.txt", "w")

print("Converting...")
for line in hexfile:
  sep = line.find(":") # Index and data separator
  index = int(line[:sep], 16) # Converted index (hex->dec)
  data = hextobin(line[sep+1:-1]) # Converted data (hex->bin)
  # Write index into a file
  output.write(f'{index}:{data}\n')

print("Done.")