This is a simple program to test ROCm using a rust program.

It multiplies 2 matric together.
It seems to work OK on smaller matrix, e.g. 10000 x 10000
But it fails on larger matrix: 50000 x 50000

Copy the sysctl config file increase_amd_memory.conf to /etc/modprobe.d

It increases the amount of memory the GPU can use on the 7840HS CPU.
For example, the file configures it to 60GB, which works if the whole system has 64GB of RAM or more.

