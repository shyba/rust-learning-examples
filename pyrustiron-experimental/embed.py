from ctypes import cdll
from ctypes import CFUNCTYPE
from ctypes import POINTER
from ctypes import c_char_p
from time import gmtime, strftime

def callback(body):
    return strftime("%a, %d %b %Y %H:%M:%S +0000", gmtime()) + body

CTYPECALLBACK = CFUNCTYPE(c_char_p, c_char_p)

lib = cdll.LoadLibrary("target/release/libpyrustiron.so")

pycallback = CTYPECALLBACK(callback)
lib.listen_with_callback(pycallback)
