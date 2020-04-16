import hashlib
from ctypes import cdll
import json
from cffi import FFI
from ctypes import *

# Create hash SHA256 H(seedI + epoch)
def creatHash(seedI, epoch):
    return hashlib.sha256(seedI.encode("utf-8") + epoch.encode("utf-8"))

# Send to python/rust cuckoo and recive back the json filter
def cuckooFilterRustCall(seedI, epoch):
    hashValue = creatHash(seedI,epoch)
    print(hashValue.hexdigest())
    
    
    ffi = FFI()
    ffi.cdef("""
        void createFilter(const char*, uintptr_t);
    """)
    lib = ffi.dlopen("target/release/libarray.so") 
    list_ = ['b55126a39f9b1170a32e6f61e4a694c45235e5ac11c05ecd6ff6395de6a11187']
    out = lib.createFilter(list_, len(list_))
    print(out)
    #Call to the python/rust wrapper
    #lib = cdll.LoadLibrary("target/release/librustLib.so")
    #a = lib.createFilter(hashValue.hexdigest())
    #out = json.load(a)
    #print(out)

def main():
    cuckooFilterRustCall("a", "b")

if __name__ == "__main__":
    main()