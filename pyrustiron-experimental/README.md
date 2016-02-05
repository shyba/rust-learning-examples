PyRustIron
=============

Just an insane idea for learning ctypes, Rust and Iron.

Warning: This insanity has a memory leak, but I am unable to tell why yet.

Current implementation handles arount 30k/sec HTTP requests calling a python callback that tells the current time.

Running
=========

```
cargo build --release
$ python embed.py
$ curl http://localhost:3000/
Fri, 05 Feb 2016 02:16:24 +0000%
```

AB Benchmark
============
```
Server Software:        
Server Hostname:        localhost
Server Port:            3000

Document Path:          /
Document Length:        31 bytes

Concurrency Level:      1000
Time taken for tests:   20.453 seconds
Complete requests:      600000
Failed requests:        0
Total transferred:      79800000 bytes
HTML transferred:       18600000 bytes
Requests per second:    29336.00 [#/sec] (mean)
Time per request:       34.088 [ms] (mean)
Time per request:       0.034 [ms] (mean, across all concurrent requests)
Transfer rate:          3810.24 [Kbytes/sec] received
```
