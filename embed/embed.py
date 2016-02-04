from ctypes import cdll
from ctypes import CFUNCTYPE

X = []

def increment():
    X.append(1)

def callback():
    with open('/tmp/big_dump', 'a') as appender:
        appender.write(str(X))

CTYPECALLBACK = CFUNCTYPE(None)

lib = cdll.LoadLibrary("target/release/libembed.so")

pycallback = CTYPECALLBACK(callback)
print lib.process(pycallback)
increment()
print len(X)
