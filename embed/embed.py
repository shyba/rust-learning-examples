from ctypes import cdll
from ctypes import CFUNCTYPE
from ctypes import POINTER
from ctypes import c_int

def callback(x):
    print x
    return 1

CTYPECALLBACK = CFUNCTYPE(c_int, c_int)

lib = cdll.LoadLibrary("target/release/libembed.so")

pycallback = CTYPECALLBACK(callback)
dir(lib.process(pycallback))
