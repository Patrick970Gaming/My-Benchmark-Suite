from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives.asymmetric import rsa
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import serialization
import os 
import math
from multiprocessing import Pool
import multiprocessing
import time
from numba import jit

dir_path = os.path.dirname(os.path.realpath(__file__))

private_key = rsa.generate_private_key(
    public_exponent=65537,
    key_size=2048,
    backend=default_backend()
)
public_key = private_key.public_key()

def ReadPrivateKey():
    completeName = os.path.join(dir_path, "private_key.pem")
    with open(completeName, "rb") as key_file:
        private_key = serialization.load_pem_private_key(
            key_file.read(),
            password=None,
            backend=default_backend()
        )
    return private_key
private_key = ReadPrivateKey()

def ReadPublicKey():
    completeName = os.path.join(dir_path, "public_key.pem")   
    with open(completeName, "rb") as key_file:
        public_key = serialization.load_pem_public_key(
            key_file.read(),
            backend=default_backend()
        )
    return public_key
public_key = ReadPublicKey()

def EncryptM(message):
    encrypted = public_key.encrypt(
        message,
        padding.OAEP(
            mgf=padding.MGF1(algorithm=hashes.SHA256()),
            algorithm=hashes.SHA256(),
            label=None
        )
    )
    return encrypted
#encrypted = EncryptM(message)

def DecryptM(encrypted):
    original_message = private_key.decrypt(
        encrypted,
        padding.OAEP(
            mgf=padding.MGF1(algorithm=hashes.SHA256()),
            algorithm=hashes.SHA256(),
            label=None
        )
    )
    return original_message
#original_message = DecryptM(encrypted)

EndNum = 10000
Nums = [""] * EndNum
NumProcess = multiprocessing.cpu_count()
MultiThread = True

def GenerateNums():
    for i in range(EndNum):
        Nums[i] = i + 1

def bench(message):
    message = str(message).encode()
    EMessage = EncryptM(message)
    DMessage = DecryptM(EMessage)
    if (DMessage == message):
        1 + 1 
    else:
        print("Not vaild")

if __name__ == '__main__':
    GenerateNums()
    if (MultiThread):
        print(EndNum)
        print("MultiThreaded for: " + str(NumProcess))
        start_time = time.time()
        with Pool(NumProcess) as p:
            p.map(bench, Nums)
        print(time.time()-start_time)
    else:
        print(EndNum)
        print("Single Threaded")
        start_time = time.time()
        for i in range(len(Nums)):
            bench(Nums[i])
        print(time.time()-start_time)