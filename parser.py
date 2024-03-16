# Open the input file in read mode
with open('lies.txt', 'r') as file:
    # Read the content of the file
    content = file.read()

# Replace 'not' and '!' with '-1'
content = content.replace('not', ' ').replace('!', ' ').replace('thecontraryof',' ').replace('theoppositeof',' ')
with open('lies.txt', 'w') as file:
    file.write(content)
# Open the same file in write mode
with open('input.txt', 'w') as file:
    # Write the modified content back to the file
    file.write(content)

with open('lies.txt', 'r') as file:
    # Read the content of the file
    content = file.read()

# Split the content by space to process each word separately
words = content.split()

# Iterate through each word and perform multiplication if it's '-1'
for i in range(len(words)):
    if words[i] == '-1':
        # If the next word exists and it's also '-1', replace the current '-1' with the result of multiplication
        if i+1 < len(words) and words[i+1] == '-1':
            words[i] = str(-1 * -1)
