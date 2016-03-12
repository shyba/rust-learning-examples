require 'ffi'

module Count
    extend FFI::Library
    ffi_lib 'target/release/libembed.so'
    attach_function :process, [], :void
end

Count.process

puts 'Done!'
