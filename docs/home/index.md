#### The Flow of compilation:


`hello.c` -> Pre-processor(cpp) -> `hello.i` -> Compiler(cc1) -> `hello.s` -> Assembler(as) -> `hello.o` & `printf.o` -> Linker(ld) -> hello(binary executable)

Preprocessing Phase: the cpp modifies the original C program of lines beginning with a `##`.
For example having `##include <stdio.h>` will make the cpp insert the contents of the header file into the program. The result is another c program, usually with a `.i` extension.

Compilation Phase: The compiler (cc1) translates the text file `hello.i` into the text file `hello.s` which is an assembly-language program.

Assembly Phase: THe assembler translates `hello.s` into machine language instructions, and packages them into a relocatable object program, and stores the result in the object file `hello.o`.


Linker Phase: The linker merges these functions into the `hello.o` program. In the example, you could see it inserted `hello.o` and `printf.o` as both are needed in order for the program to work. These are combined into one working standalone executable.

---

#### Hardware Organization of a system

##### Buses

Buses are all over the system, and they are a collection of electrical conduits.
They carry bytes of information back and forth between components(hence the 'bus').
Buses are typically desgined to carry fixed-size chunks of bytes called 'words'. These are usually either 32 bits or 64 bits.

##### I/O Devices

Each I/O device is connected to the I/O bus by either a controller or an adapter.
Controllers are chipsets in the device itself or on the system's main PCB(Printed Circuit Board), often called the motherboard.
An adapter is a card that plugs into a slot on the motherboard.

However they both provide the same functionality, moving information from the I/O bus and an I/O device.

##### Main Memory

Logically, memory is a linear array of bytes, each with its own unique address(or array index!) starting at zero.


##### Memory and Clocking

Storage devices are controlled by a single clock,  a periodic signal that determines when new values are to be loaded into the devices.

Clocked registers(usually just called registers) store invdividual bits or words, the clock signal controls the loading of the register with the value at its input.

Random Access Memories store multiple words, using an address to select which word should be read or written. Examples of random access memories include The virutal memory system of a processor, where a combination of hardware and OS software make it appear to a procesoor that it can access any word within a large address space.
Another example is the register file, where register identifiers server as the addresses. In x86-64, the register holds 16 registers, `%rax` to `r15`.


The meaning of 'register' differs between speaking about hardware and machine-level programming.

In machine-level programming, register represent a small collection of addressable words in the CPU.
In hardware, a register is directly connected to the rest of the circuit by its input and output wires.

When distinction is needed, the two classes of registers are called 'program registers' and 'hardware registers' respectively.



##### Processor

This is the engine that executes instructions stored in main memory.

At its core it has a word-size storage device, called the 'program counter'(PC).

At any point in time, the PC points at some machine-language instruction in main memory.

It repeatedly finishes an instruction, then updates the PC; which then returns a new instruction.

The ALU computes new data and address values.

All the instructions are extremely basic, They revolve around main memory, the register file and the arithmetic logic unit(ALU)

Here are some examples of some operations the CPU might carry out at the request of an instruction:

Load: Copy a byte or word from main memory into a register, overwriting the previous content.

Store: Copy a byte or word from a register to a location in main memory, overwriting the previous contents of that location.

Operate: Copy the contents of two registers to the ALU, perform an arithmetic operation on the two words, then store the result in a register; overwriting previous content.

Jump: Extract a word from the instruction itself and copy that word into the program counter, overwriting the previous value of the PC.


### Control Unit

TODO Update here



### Cache

As processor speeds have advanced, and are considerably faster than disk or main memory; there needed to be a solution.

However, you couldn't build even moderately sized storage that would be fast enough, so caches were created.

These are usually tiny, in comparison to the disk or RAM. And they are temporary staging areas for information that the processor is likely to need in the near future.


A L1 cache on the processor chip holds tens of thousands of bytes and can be accessed nearly as fast as the register file.

A larger L2 cache with hundreds of thousands to millions of bytes is connected to the processor by a special bus. It is orders of magnitiude slower than the L1 cache, but still much faster than main memory.

L1 and L2 caches are static random access memory(SRAM).

Some newer systems have an L3 cache also.


The register file(stored in the Processor chip) is the fastest memory, and it has to be, as it is needed for all operations. Sometimes known as L0 cache.

### Maths Notation

`w` is the length of a set/array/list.

The `∑` is Sigma and means Sum.

The number on top of Sigma is the length.
The number below is the iterator variable? like i=0 at the bottom and it goes up through a bit vector.
				.
Equals sign with a `.` above it:`=` means 'nearly equal to'.

A complement of a set `A` refers to elements NOT in `A`.

A variable with an arrow above it means it is a vector - `v⃗`.

An apostrophe just after a variable means it is the complement of a set - `A′`

Square brackets, as in `[π] = 3` denote the floor function, rounding down.

They are also half versions of this, which each one having the right angle at the top or bottom, depending on where it is being rounded to.

Double greater/less than symbols mean much greater/less than.

### Context Switching

The procress of executing multiple processes concurrently; switching between execution of processes, is called context switchting.

The context, or the 'state' is contains information like the contents of the PC, the register file and the contents of main memory.

When the OS decides to to transfer control from its current process to a different one, it performs a *context switch* by saving the context of the current one, and restoring the context of the new process. Then restarting execution on the PC.

This transition is handled the kernel.

You can call functions of the kernal with system calls. or 'syscalls'.




### Threads

A process can consist of multiple execution units called *threads*.

These are extremely important in certain programs where concurrency is essential, but communication between threads/elements of the program is also essential; such as a network server.

Threads are typically more efficent than processes.

Multi-threading is one way to make programs run faster when multiple processors are available.


### Concurrency and Parallelism

Concurrency - general concept of a system with multiple, simulataenous activites.


Parallelism - The use of concurrency to make the system run faster.


##### Thread level abstraction

A typical multi-core processor is a chip that has four CPU cores, each with their own L1 and L2 caches, and with each L! cache split into two parts: one to hold fetched instructions and one to hold data.

The cores usually share a L3 cache, which serves between the main memory and the L2 & L1 caches of each core.


Superscalar Procssors are processors that can sustain execution rates faster than 1 instruction per cycle.

###### Hyperthreading

Hyperthreading(sometimes called simultaneous multi-threading) is where a single CPU can execute multiple flows of control.

IT has multiple copies of of some of the CPU hardware, such as PCs and register files, while only have one copy of other bits, like the units that do floating-point arithmetic.

A conventional processor usually needs around 20,000 clock cycles to shift between threads, hyperthreaded processors do it on a cycle-by-cycle basis.

So you can have a 4 core processor running two threads on each core, so 8 threads in parallel.

### Single Instruction, Multiple Data (SIMD) Parallelism

Many modern processors have special hardware that allows a single instruction to cause multiple operations to be performed in parallel. This is SIMD.

For example, recent generations have instructions that can add 8 pairs of single precision point float numbers in parallel.

### Virtual Address Space

Virtual Memory is an abstraction that provides each process with the illusion that it has exclusive use of the main memory.

Each process has the same view of memory, known as its virtual address space.


At the top is *Program Code and Data*. Code begins at the same fixed address for all processes, followed by data locations that correspond to global C variables. The code and data areas are initialized directly from the contents of an executable object file, such as the `hello` executable.

Then there is the *Heap*, the run-time heap. Unlike program code and data, which are fixed in size once the program begins, the heap expands and contracts dynamically at run time as a result of calls to allocate/free data such as `malloc` and `free`.

*Shared Libraries* this contains code and data for shared libraries, such as the C standard library and the math library.

The *Stack*: This sits at the top of the user's virtual address space; the *user stack* that the compiler uses to implement function calls. Like the heap, the stack grows and contracts dynamically in accordance with what the program needs, during execution.

*Kernel Virtual Memory*: The top region of the address space is reserved for the kernel. Applications are not allowed to read or write the contents of this rea or to directly call functions defined in the kernel code. Instead, they must invoke the kernel to do these operations.

### Pipelining

Pipelining is a method of how processors can achieve higher performance.
An instruction is passed through a sequence of stages, each performing one small portion of the required operations(e.g fetching the instruction from memory, determining the instruction type, reading from memory, doing an arithmetic operation, writing to memory, and then updating the program counter.)

It achieves high performance by overlapping the steps of successive operations.
It does this by dertermining the sequence of instructions to be executed well ahead of time, so the pipeline is is kept full of instructions to be executed.

When the machine encounters a conditional jump(referred to as a branch), it cannot determine which way the program will go, until it has evaluated the condition.

Processors do employ 'branch prediction logic' to try to guess whether the branch will be followed or not. Implementations of this can be Never Taken(NT), backward taken, forward not taken (BTFNT). This is effectively predicts the next value of the PC.

Modern microporcessor designs aim to be above 90%, so then the pipeline will always be full of instructions.

Mispredicting can be costly, causing up to 30 clock cycles of waste, and such impact program performance.

We must implement this knowledge in the writing of our programs, the less conditionals the faster our program can run. We can look into assigning two variables, then performing a oneline if check, and re-assign a variable, and return that variable.

This is conditional transfer of data, rather than conditional transfer of control.

This can be done neatly in C: `v = test-expr ? then-expr : else-expr;`

Good example of pipelining on p. 461

#### Speculative Execution

This is almost a sequel to branch prediction, where the processor will begin fetching and decoding instructions from where it predicts the branch will go; and begins executing these before the branch has actually been evaluated.

#### Retirement Unit

There is a retirement unit which writes the results of speculatively executed microoperations into the user-visible registers and removes said mirco operations from the buffer. The retirement unit can retire three microoperations per clock cycle.

#### Inter-Execution Unit Communication

Any updates to the program registers occur only as instructions are being retired, and this happens after the processor can be certain any branches leading to this instruction have been correctly predicted. Communication between execution units is performed directly with each other.

The most common mechanism for controlling the communication of operands among the EUs is called register renaming. When an instruction that updates register *r* is decoded, a tag *t* is generated, giving a unique identifier to the result of the operation. An entry *(r,t)* is added to a table maintaining the associataion between program register *r* and tag *t* for an operation that will update this register. When a subsequent operation using register *r* as an operand is decoded, the operation sent to the EU with the tag *t* as the source for the operand value. By this mechanism, values can be forwarded directly from one operation to another, rather than performing reads and writes over and over.



#### Loop Unrolling

Loop unrolling is a method to reduce the number of iterations for a loop by increasing the number of elements computed on each iteration.

It can improve performance in two ways:

- It reduces the number of operations that do not contribute directly to the program result.

- It It exposes ways in which we can further transform the code to reduce the number of operations in critical paths of the overall computation.



#### Paralell Execution

For machines with complex instructions such as x86, instructions can be divided up into seperate operations. For example, to do `addq %rax,8(%rdx)` you need to load a value from memory, add the loaded value to the value in register `%rax` and then store that result back into memory.

In order to speed up these computations, these are sent to the Execution Unit(EU) where there are multiple sub-units(called functional units), all of which can perform different operations. Therefore, we can send an instruction which needs 3 operations to complete, and these 3 operations will be performed in paralell!

An Intel Core i7 Haswell has eight functional units. Each do Integer arithmetic, float arithmetic, storage, store address computation, loading and load address computation.


#### Dependencies and Hazards

When there are dependicies between sucessive instructions we can use technique(s) to speed up these processes.

A **data dependency**  is where sucessive instructions rely on the result of another instruction in order to perform the task.

A **control dependency** is where the outcome of a conditional test determines whether an instruction will be executed.

Dependencies that can cause an erroneous computation by the pipeline, they are called hazards. Like dependicies, we have a data and a control hazard.

There is a large problem in CPU design in this respect, as control dependencies must wait for the write-back stage of is dependent process, or it will read the old value from the register file.
#### Data Hazards

A common technique to avoid data hazards is *stalling*, where the processor holds back one or more instructions until the hazard condition no longer holds.
It does via detecting for hazards, likely by looking at writes and reads to the same register in the same cycle; and then inserting `nop` instructions instead of a write-back or read or whatever of an instruction execute pattern. This allow data to be written and then read post-write.

Another tecnique to avoid data hazards can be done by *forwarding*; instead of writing the value from the instruction to the register, it simply passes it to the dependent instruction AS the source operand, skipping the writing and reading part.

A **load/use** where one instruction reads a value from memory and the next instruction needs that value as a source operand in the next cycle. We can't forward here, as in order to do so the value would need to be forwarded backwards in time.This requires a combination of stalling and forwarding to avert this hazard.This is called a **load interlock**.


#### Control Hazards

Control hazards arise when the processor cannot reliably determine the address of the next instruction based on the current instruction in the fetch stage.

#### Computational Pipelines

In contemporary logic design we measure circuit delays in units of picoseconds (usually abbreivated 'ps'), or 10<sup>-12<sup> seconds.

Balancing delays from a pipelined system is done via circuit retiming. Retiming changes the state representation for a system without changing its logical behaviour.

#### Exception Handling

Activites in a processor can sometimes lead to exceptional control flow, where the normal chain of program execution gets broken.

Exceptions can be generated internally, ie inside the program; or externally by an outside signal(like `SIGINT`).

In a pipelined system, it is possible to have multiple exceptions generated in the same clock cycle, from different instructions. The processor must have methods of which to decide what to priotize.
A general rule for dealing with this problem is to put priority on the exception triggered by the instruction furthest along the pipeline.

Another problem is when an instruction is fetched, begins execution then causes an exception, and later is canceled due to a mispredicted branch.

These problems are usually handlid by merging the handling into the pipeline structure. By including a status code into registers, the status field can indicate the nature of the exception and this propagates through the pipeline until reaching the write-back stage.By carrying this status of exception through the pipeline, no other changes to the programmer-visible state are able to occur, which could cause further harm to the system.



### SIMD

Single Instruction Multiple Data

This allows a processor to perform the same instruction across a number of data values, **in paralell**.

This have progressed into extensions & revisions with SSE(Streaming SIMD Extensions) and AVX(Advanced Vector Extensios.)

The AVX floating-point architecture has 16 YMM registers, named `%ymm0`-`%ymm15`. Each register is 32 bytes!
These, like standard registers, can have their lower half bits accessed via changing the `y` to an `x`, i.e `%xmm14`.

### Program Performance

Optimization is done via either more efficent source code, or a more aggressive and effective optimizing compiler.

#### Function Calls
Some compilers will optimize function calls with inline substiution, which is inserting the body of that function inline where the function call would be in address space. This way you don't have to deal with pushing and popping stacks, address math etc. On gcc, this can be turned on with either `-finline` or optimization levels <= `-01g`.
Compilers usually do not know if a function call affects global program state, it usually assumes the worst case scenario and leaves function calls intact.

#### Measuring Program Performance

*Cycles Per Element* or CPE, measures program performance. It is appropriately used in programs that perform a repetitive computation, such as computing elements in a matrix product.

Measuring per element is more useful than measuring per iteration as you can use techniques such as loop unrolling to reduce overall computation time at expensive of per-iteration performance.

##### Code Motion

'Code Motion' is a class of optimization where a specific computation is identified as being performed mulitple times and getting the same result(such as in a for loop). We can therefore move that computation to an earlier section of code so it does not get evaluated as often.

##### Uneeded Memory References

Programs are often bottlenecked by uneeded rewriting to memory for a variable. Simply computing a value before, and then passing it to a function or loop;  then doing computations can save massive amount of CPE.





## Storage Technologies

A **memory system** is a hierarchy of storage devices with different capactiies costs and access times.

CPU registers hold the most frequently used data.

Small cache memories nearby to the the CPU act as staging areas for a  subset of the data and instruction stored in main memory.

The main memory(commonly referred to as RAM) stages data from the large slow disks(HDDs or SSDs).

Large disks store most data and can be used as staging data from networked devices/storage.

Which storage device(s) are used to access and store data are key to efficency of a program.

Retreiving data from a CPU register can take 0 cycles, where as retreiving data from a cache can take from 4 to 75. RAM can take hundreds, from a disk, **millions** of cycles.

**Locality** this is a measure of where data used in the program is stored. If most data that is accessed is stored in CPU/cache registers, the program will have good locality.Thus meaning performance scales with locality.

The principle of locality enables computer designers to speed up main memory accesses by introducing small fast memories knwon as *cache memories* that hold blocks of the most recently referenced instructions and data items.




### Random Access Memory

Random Access Memory comes in two varieties - static(SRAM) and dynamic (DRAM).

SRAM is used for cache memories, both on and off the CPU chip.

DRAM is used for main memory plus the frame buffer of a graphics system.

#### Static RAM

SRAM stores each bit in a bistable memory cell. Each cell is implemented with a six-transistor circuit.
The circuit can stay indefinitely in either of two different voltage configurations, or *states*.
Any other state will be unstable.

The circuit could go into a *metastable* state, which is where it is is a state of between 0 and 1. However the smallest distrubance would push it one way or the other.

#### Dynamic RAM

DRAM stores each bit as charge on a capacitor.

Unlike SRAM, a DRAM memory cell is very sensitive to any distrubance. When the capacitor voltage is disturbed, the it will never recover.

The sensors in digital cameras and camcorders are essentially arrays of DRAM cells.

Various sources of leakage current cause a DRAm cell to lose its charge within a time period of around 10 to 100 milliseconds.

Some systems use error-correcting codes, where the words are encoded using a few more bits, like a 64-bit word encoded using 72 bits.


###### Conventional DRAMs

The cells (bits) in a DRAM chip are partitioned into *d supercells*, each consisting of *w* DRAM cells.
A *d* x *w* DRAM stores a total of *d w* bits of information.

The supercells are organized as a rectangular array with *r* rows and *c* columns, where *rc* = *d*.

Each supercell has the address of the form (*i, j*) where *i* denotes the row and denotes the column. Information flows in and out of a DRAM chip via external connectors called *pins*. Each pin carries a 1 bit signal.

There are data pins that carry data, addr pins that carry supercell addresses and more...

Each DRAM chip is connected to some circuitry, known as the *memory controller* that can transfer *w* bits at a time to and from each DRAM chip.

To read the contents of supercell (*i, j*) the memory controller send the row address *i* to the DRAM, followed the the column address *j*. The DRAM responds by sending the contents of supercell (*i, j*) back to the controller.

The row address *i* is called a a RAS (row access strobe) request. The clumn address *j* is called a *CAS* (*column acess strobe) request*.

###### Enchanced DRAMs

New kinds of DRAM memories appear on the market regularly as manafacturs attempt to keep up with rishing CPU and GPU speeds.

Each are based on the basic DRAM cells.

- **Fast Page Mode DRAM (FPM DRAM)** A conventional DRAM copies an entire row of supercells into its internal row buffer, uses one and then discards the rest. FPM DRAM imporves this by allowing consectiuve access to the same row to be served directly from the row buffer. To read supercells from the same row of an FPM DRAM, the memory controller sends an intial RAS/CAS request, followed by three CAS requests. The inital RAS/CAS request copies row *i* into the row buffer and returns the supercell addressed by the CAS. The next three supercells are served directly from the row buffer, and thus are returned more quickly than the inital supercell.

- **Extended data out DRAM (EDO DRAM)** An enhanced form of FPM DRAM that allows indivual CAS signals to be spaced closer together in time.

- **Synchronous DRAM (SDRAM)** Converntional, FPM and EDO DRAMs are asynchronous in the sense that they communicate with the memory controller using a set of explicit control signals. SDRAM replcaes these control signals with the risign edges of the same external clock signal that drives the memory controller. With this, SDRAM can output the contents of its supercells at a faster rate than its asynchronous counterparts.

- **Double Data-Rate Synchronous (DDR SDRAM)** DDR SDRAM is an enhancement of SDRAM that doubles the speed of the DRAM by using both clock edges as control signals.

- **Video RAM (VRAM)** used in frame buffers of graphics systems. VRAM is similar to FPM DRAM. Two major differences are that VRAM output is produced by shifting the entire contents of the internal buffer in seuquence and VRAM allows concurrent reads and writes to the memory. Thus so, the system can be painting the screen with the pixels in the frame buffer(reads) while concurrently writing new values for the next update (writes).

###### Nonvolatile Memory

DRAMs and SRAMs are volatile in the sense that they lose their information if the voltage is turned off.

Nonvolatile memories keep their information even when powered off.

Nonvolatile memories are usually referred to as ROMs (Read Only Memory) even though some ROMs can be written to as well as read. ROMs are distinguised by how many times they can be reprogrammed(written to) and how you can repromme them.


- **Programmable ROM (PROM)** can be programmed exactly once. PROMs have a (sort of) fuse with each memory cell that can be blown once by zapping it with a high current.

- **Erasable Programmable ROM (EPROM)** has a transparent(quartz) window that allows light through to reach storage cells. The EPROM cells are cleared to zeros by shining UV light through the window. Programming an EPROM is done by using a special device, which can write ones into it. An EPROM ca be erased and reprogrammed on the order of 1,000 times. An **electrically erasable PROM (EEPROM)**  is similar to an EPROM but does not require a physically seperate programming device, and can be reprogrammed in-place via printed circuit cards. An EEPROM can be reprogrammed on the order of 10<sup>5<sup> times before it wears out.

- **Flash memory** us a subtype of nonvolatile memory, based on EEPROMs that has become an important storage technology. They are used as the storage in SSDs, in small devices. Programs stored in ROM devices are often referred to as *firmware*. When a computer system is powered on, it runs firmware stored in a  ROM. This could be your PC's BIOS (Basic Input Output System), a bootloader and more.

##### Accessing Main Memory

Data flows between the processor and the DRAM main memory over shared electrical conduits called *buses*. Each transfer of data between the CPU and memory is done with a series of steps called a *bus transaction*.

A *read transaction* transfer data from main memory to the CPU.

A *write transaction* transfers data from the CPU for the main memory.

A *bus* is a collection of parallel wires that carry address, data and control signals.

Depending on the design, data and address signals can share the same set of wires or use different sets.
The control wires carry signlas that synchronize the transaction and identify what kind of transaction is being performed.

For example, Is this transaction of interest to the main memory, or to some other I/O device such as a disk controller? Is the transaction a read or a write? Is the information on the bus an address or a data item?

The I/O bridge translates the electrical signals of the system bus into the eelctrical signals of the memory bus.

So what happens when the CPU performs a load operation such as
```
movq A, %rax
```
where the contents of address A are loaded into register `%rax`. Circuitry on the CPU chip called the *bus interface* intiates a read transaction onn the bus.

The read transaction consists of three steps:

- First, the CPU places the address A on the system bus. The I/O bridge passes the signal along to the memory bus.

- The main memory senses the address signal on the memory bus, reads the address from the memory bus and then fetches the data from the DRAM, and writes the data to the memory bus. The I/O bridge translate the memory bus signal into a system bus signal and passes it along to the system bus.

- Finally, the CPU senses the data on the system bus, reads the data from the bus and copies the data to register `%rax`

When the CPU performs a store operation such as

```
movq %rax,A
```
where the contents of register `%rax` are written to address A, the CPU intitiates a write transaction:

- The CPU places the address on the system bus.

-  The memory reads the address from the memory bus and waits for the data to arrive.

- The main memory reads the data from the memory bus and stores the bits in the DRAM.


#### Memory Modules

DRAM chips are packaged in *memory modules* that plug into expansion slots on the motherboard.

i7 Systems use the 240-pin *dual inline memory module* (DIMM) which transfers data to and from the memory controller in 64-bit chunks.

To retreive the word at memory address *A*, the memory controller converts *A* to a supercell address(*i, j*) and sends it to the memory module, which then broadcasts *i* and *j* to each DRAM. In response, each DRAM outputs the 8-bit contents of its (*i, j*) supercell. Circuitry in the module collects these outputs and forms them into a 64 bit word, which it then returnrs to the memory controller.

Main memory can be aggregated by connecting multiple memory modules to the memory controller. kill


### Disk Storage

Disks are storage devices that hold enormous amounts of data. It can hold much more data than RAM based memory. However, this means it takes far longer to read and write data to and from the disk.

#### Disk Gemoetry

Disks are constructed from *platters*. Each platter consists of two sides, or *surfaces* coated with magnetic recording material.

A rortating spindle in the center of the platter spins the platter at a fixed rotational rate typically between 5,400 and 15,0000 *revoloutions per minute* (RPM).

Each surface has consists of a copllection of concentric rings called *tracks*. Each track is partitioned into a collection of *sectors*. Each sector contains an equal number of data bits encoded in the magnetic material on the sector.

Sectors are seperated by gaps, where no data bits are stored. Gaps store formatting bits that identify sectors.

A disk consists of one or more platters stacked on top of each other.

The entire assembly is often referred to as a **disk drive**.

Disk manafacturers desbribe thge geometry of multiple-platter drives in terms of *cylinders*, where a cylinder is the collection of tracks on all the surfaces that are equidistant from the center of the spindle.

#### Disk Capacity

The maximum number of bits that can be recorded by a disk is known as its *maximum capacity* or simply *capacity*.
Disk capacity is determined by the following technology factors:

- Recording Density: (bits/in). The number of bits that can be squeezed into a 1-inch segment of a track.

- Track Density (tracks/in). The number of tracks that can be squeezed into a 1-inch segment of the radius extending from the center of the platter.

- Areal Density (bits/in<sup>2</sup>) The product of the recording density and the track density.

Historically, as areal densities have increased, the gaps between sectors became unacceptably large.

Thus, modern high-capacity disks use a technique known as **multiple zone recording** where the set of cylinders is partitioned into disjoint subsets known as recording zones. Each zone consists of a contiguous collection of cylinders.

Each track in each cylinder in a zone has the same number of sectors which is determined by the number of secotrs that can be pakced into the innermost track of the zone.

The capacity of a disk is as per the following formula:

Capacity = (no. of bytes / sector) * (average no. of sectors / track) * (no. of surfaces / platter) * (no. of platters / disk)

##### Formatted Disk Capacity

Before a disk can be used to store data, it must be *formatted* by the disk controller.

This process invovles filling in the gaps between sectors with information that identifies the sectors.
Identifying any clinders with surface defects and taking them out of action, setting aside a set of cylinders in each zone as spares that can be used if one or more cylinders in that zone goes bad during the lifetime of the disk.

The *formatted capacity* quoted by disk manufactures is less than the maxiumum capacity because of the existence of these spare cylinders.

#### Disk Operation

Disks read and write bits stored on the magnetic surface using a *read/write head* connected to the end of an actuator arm.

By moving the arm back and forth along its radial axis, the drive can position the head over any track on the surface.

This mechnaical motion is known as a **seek**.

Once the head is positioned over the desired track, as each bit on the track passes underneath, the head can either sense the value of the bit(read the bit) or alter the value of the bit(write the bit).

Disks with multiple platters have a seperate read/write head for each surface. The heads are lined up vertically and move in unison.

The read/write head at the end of the arms flies (literally) on a thin cushion of air over the disk surface at a high of about 0.1 microns and a speed of about 80 km/h.

Due to the velocity and speed of the heads, no dust or other objects can get into the container as it will destroy much data. Therefore, disks are kept in airtight packages.

Disks read and write in sector size blocks.

The *access time* for a sector has three main components: *seek time, rotational latentency* and *transfer time*:


- *Seek Time.* To read the contents of a traget secotr, the arm positions the head over the track that contains the target sector. The time required to move the arm is called the *seek time*.The seek time depends on the previous position of the head and the speed that the arm moves across the surface.

- *Rotational Latency.* Once the head is in position over the track, the drive waits for the first bit of the target sector to pass under the head. The performance of this step depends on both the position of the surface when the head arrives at the target track and the rotational speed of the disk. The max rorational latencty can be calculated ass if the head just misses the target secotr and waits for the disk to make a full rotation.

- *Transfer Time* When the first bit of the target sector is under the head, the drive can begin to read or write the contents of the sector. The transfer time for one sector depends on the rotational speed and the number of sectors per track.



#### Logical Disk Blocks

Disks gave complex geometries with multiple surfaces and different recording zones on those surfaces. To abstract this complexity from the operating system, modern disks present a simpler view of their geometry:  a seqeunce of *B* sector-size *logical blocks*, numbered from 0,1 ... *b* - 1.

There is a small device (the *disk controller*) maintains mapping between logical block numbers and physical disk sectors.
Firmware on the controller performs a fast table lookup that translates the logical block number into a (*surface, track, sector*) triple that uniquely identifies the corresponding physical sector.

Hardware on the controller interprets this triple to move the heads to the appropiate cylinder, waits for the sector to pass under the head, gathers up the bits sensed by the head into a small memory buffer on the controller and copies them into main memory.




#### Connecting I/O Devices

Graphics cards, monitors, mice and keyboard all use an *I/O bus*. I/O buses are designed to be general purpose and to be independent of the underlying CPU.

Although the I/O bus structure is slower than the system and memory buses, it can accommdoate a wide variety of 3rd party IO devices, such as:

A *host bus adapter* that connects one or more disks to the I/O bus using a commuinication protocol defined by a particular *host bus interface*. The two most popular interfaces for disks are SCSCI and SATA. A SCSI host bus adapter (often called a SCSI controller) can support multiple disk drives unlike SATA, which can only do one.

The CPU issues commands to I/O devices using a technique called *memory-mapped I/O*. In a system with memory-mapped I/O a block of address in the address space is reserved for communicating with I/O devices.

In a system with memory mapped I/O a block of addresses in the address space is reserved for communicating with I/O devices.
Each of these addresses is known as an I/O port. Each device is associated with/mapped to one or more ports when it is attached to the bus.

If a disk controller was mapped to port `0xa0`, the CPU would intiate a disk read by executing three store instructions to address `0xa0`

- The first of these intructions sends a command word that tells the disk to intiate a read, along with other parameters such as whether to interrupt the CPU when the read is finished.

- The second instruction indicates the logical block number that should be read.

- The third instruction indicates the main memory address where the contents of the disk sector should be stored.

After it issues the request the CPU will typically do other work while the disk is performing the read. A 1GHz processor with a 1 ns clock cycle can potentially execute 16 million instructions in the 16 ms it takes to read the disk, so it would be wasteful to wait this time idle.

After the disk controller receives the read command from the CPU, it translates the logical block number to a sector address, reads the contents of the sector and transfers the contents directly to main memory, without any intervention from the CPU.

This method of performing a read/write withouot involvement of the CPU is called *Direct Memory Access* (DMA). Transfer of data is known as a DMA *transfer*.

After the DMA transfer is complete, the disk controller notifies the CPU by sending an interrupt signal to the CPU.




#### Peripheral Component Interconnect (PCI)

In the PCI model, each device in the system shares the bus and only one device at a time can access these wires.

In modern systems, the shared PCI bus has been replaced by a PCI express (PCIe) bus which is a set of high-speed serial, point-to-point links connected by switches.

A PCIe bus with a max throughput of 16 GB/s is an order of magnitude faster than a PCI bus with a max throughput of 533 MB/s.


#### Solid State Disks

SSDs consist of one or more flash memory chips and a *flash translation layer*.

A flash translation layer is a hardware/firmware device that does the same thing as a disk controller, translating requests for logical blocks into access of the physical device.


A Flash memory consists of a sequence of blocks, where each block consists of pages.

Common page sizes are between 2K-8K, with 128-256 pages per block.

Data are read and written in units of pages.

SSDs have a number of advantages over rotating disks:

- Built of semiconductor memory

- No moving parts

- faster random access times

- Use less power

However, because flash blocks do wear out after repeated writes, SSDs have the potential to wear out as well.

Wear-leveling logic in the flash translation layer attempts to maximize the lifetime of each block by spreading erasures evenly across all blocks.

### Locality

Well-written programs exhibit good *locality*.


Meaning, they tend to reference data that are near other recently referenced data items or that were recently referenced themselves.

Locality is tyoucally described as having two distinct forms: *temporal locality* and *spatial locality*.


Good temporal locality means a memory location that is referenced once is likely to be referenced again multiple times in the near future.

Good spatial locality means that if a memory location is referenced once, the program is likely to reference a nearby memory location in the near future.

Obviously, programs with good locality will run faster than programs with poor locality.

A function that visits each element of a vector sequentially has a *stride-1 reference pattern* also referred to as *sequential reference patterns*.

Visiting every *k*th element of a contiguous vector is called a *stride-k reference pattern*.

### Memory Hierarchy

In computer systems, there is a hierarchy of memory, starting from smaller and faster memory and as we move down the hierarchy, memory gets larger but slower.

At `L0` we have the CPU registers, which obviously are tiny, 32/64 bits!

At `L1-3` we have the caches, which are each slower than another going down from L1.

At `L4` we have DRAM, which is still relatively fast, but much slower than caches.

At `L5` We have local disks, which has huge storage, but is very slow.

At `L6` we have remote storage, web servers, distributed file systems etc.

There also exist tape drives, which are very slow but extremely cheap per byte.

Each level in the hierarchy caches data objects from the next lower level.

The storage at level *k* + 1 is partitioned into contiguous chunks of data objects called *blocks*. Each block has a unique address or name that distinguishes it from other *blocks*.

Storage at level *k* is partitioned into a smaller set of blocks of the same size of the blocks at level *k* + 1.
Data is always copied back and forth between level *k* and *k* + 1 in block-size *transfer units*.

It is important to realize that while block size is fixed between any particular pair of adjacent levels in the hierarchy. For example, transfers between L1 and L0 typically use word-size blocks. Transfers between L2 and L1(L3 and L2, L4 and L3) typically use blocks of tens of bytes.While the further down the size increases.

##### Cache Hits

When a program needs a data object *d* from level *k* + 1, it looks for *d* in one of the blocks in *k*. If *d* happens to be in *k*, then we have a *cache hit*. The program reads *d* directly from level *k* which is obviously faster than reading from level *k* + 1.

##### Cache Misses

If the data object *d* is not cached at level *k* then we have a *cache miss*. When there is a miss the cache at level *k* fetches the block containg *d* from the cache at level *k* + 1, possibily overwriting an existing block if the level *k* cache is already full.

The process of overwriting an existing block is known as *replacing* or *evicting* a block.

The block that is evicted is often called the *victim block*.

The decision of which block to replace is governed by the cache's *replacement policy*.

A cache with a *random replacement policy* would choose a random victim block.

A cache with a *least recently used* (LRU) replacement policiy would choose the block that was last accessed the furthest in the past.

After the cache at level *k* has fetched the block from the level below, the program can then read *d* from level *k* as before.

###### Kinds Of Cache Misses

There are different kinds of cache misses:

- If the cache at level *k* is empty, than any access of any data object will miss. An empty cache is sometimes called a *cold cache*. This is called a *compulsory miss* or a *cold miss*

Whenever there is a cache miss, the cache at level *k* must implement a *placement policy*  that determines where to place the block that it has retreived from level *k* + 1.
The most flexible placement policiy is to allow any blocvk from level *k* + 1 to be stored in any block at level *k*.

For caches high in the memory hierarchy this is too expensive to implement because randomly placed blocks are hard to locate.

Thus, hardware usually implements a simpler placement policiy that restricts a block at level *k* + 1 to a small subset of the blocks at level *k*.

However, these restrictive placement policies lead to a type of miss knwon as a *conflict miss*, in which the cache is large enough to hold the referenced data objects, but because they map to the same cache block, the cache keeps missing.

Programs often run as a sequence of phases (e.g loops) where each phase accesses some reasonably constant set of cache blocks. For example, a nested for loop might access the elements of the same array over and over again. This set of blocks is called the *working set* of the phase.

When the size of the working set exceeds the size of the cache, the cache will experience *capacity misses*, meaning the cache is too small to handle this particular working set.


##### Cache Management

At each level in the memory hierarchy, some form of logic must manage the cache. Something has to partition cache storage into blocks, transfer blocks etc.

This logic can be hardware, software or a comibation.

For example, the compiler manages the register file, it decides when to issue loads when there are misses, and determines which register to store the data in.

The caches at levels are managed entirely by hardware logic built into the caches.

In a system with virtual memory, the DRAM main memory serves as a cache for data blocks stored on disk, and is managed by a combination of operating system software and address translation hardware on the CPU.

##### Generic Cache Memory Organization

In a computer system where each memory address has *m* bits that form *M = 2*<sup>*m*</sup> unique addresses.

A cache for such a machine is organized as an array of *S = 2*<sup>*s*</sup> *cache sets*. Each set consists of *E cache lines*.

Each line consists of a data *block* of *B = 2*<sup>b</sup> bytes, a *valid bit* that indicates whether or not the line contains meaningful information and *t = m - (b + s) tagh bits* (a subset of the bits from the current block's memory address) that uniquely identifies the block stored in the cache line.

In general, a cache's organization can be charectized by the tuple (*S, E, B, m*).

The size (or capacity) of a cache, *C*, is stated in terms of the aggregate size of all the blocks. The tag bits and valid bit are not included. Thus, *C = S x E x B*.

When the CPU is instruycted by a load instruction to read a word from address *A* of main meory, it sends address *A* to the cache. If the cache is holding a copy of the word at address *A*, it sends the word immediately back to the CPU. So how does the cache know whether it contains a copy of the word at address *A*? The cache is organized so that it can find the requested word by simply inspecting the bits of the address, similar to functionality of a hash table with a very basic hash function.

#### Direct-Mapped Caches

Caches are grouped into different classes based on *E*(number of cache lines per set). A cache with exactly one line per set (*E* = 1) is known as a *direct-mapped* cache.

For example, say we have a system with a CPU, register file, an L1 cache and a main memory. When the CPU executes an instruction that reads a memory word *w*, it requests a the word from the L1 cache. If the L1 cache has a cached copy of *w*, then we have an L1 cache hit and the cache quickly extracts *w* and returns it to the CPU.

Otherwise, we have a cache miss and the CPU must wait while the L1 cache requests a copy of the block containg *w* from the main memory. When the requested block finally arrives from memory, the L1 cache stores the block in one of its cache lines, extracts word *w* from the stored block and returns it to the CPU.

The process that a cache goes through of determining whether a request is a hit or a miss and then extracting the requested word consists of three steps:

1. Set Selection

2. Line Matching

3. Word Extraction

##### Set Selection in Direct-Mapped Caches

In this step the cache extracts the set index bits from the middle of the address for *w*.

These bits are interpreted as an unsigned integer that corresponds to a set number.(similar to an index when accessing an array).

##### Line Matching in Direct-Mapped Caches

Now we have selected a set in the previous step, now we need to determine if a copy of the word *w* is stored in one of the cache lines contained in the set. In a direct-mapped cache this is easy and fast since there is exactly one line per set.
We use the valid bit and the tags to determine if we have a cache hit or a cache miss.

##### Word Selection in Direct-Mapped Caches

Once we have a hit, we knwo that *w* is somewhere in the block.

This last step determines where the desired word starts in the block. The block offset bits provide us with the offset of the first byte in the desired word.

Similar to the view of a cache as an array of lines, we can think of a block as an array of bytes, and the byte offset as an index into that array.


##### Line Replacement on Misses in Direct-Mapped Caches

If the cache misses, then it needs to retreive the requested block from the next level in the memory hierarchy and store the new block in one of the cache lines of the set indicated by the set index bits.

If the set is full of valid cache lines, then one of the existing lines must be evicted.

For a **Direct-Mapped Cache**, where each set contains exactly one line, the replacement policiy is simple, the current line is replaced by ther newly fetched line.

#### Direct-Mapped Cache in Action

The mechanisms that a caches uses to select sets, and identify lines are very simple, as hardware must be able to perform them in a few nanoseconds.

For example, let us consider a direct-mapped cache described by
```
(S, E, B, m) = (4, 1, 2, 4)
```
The cache has four sets, one line per set, 2 byttes per block and 4-bit addresses.(This is unrealistic, but useful as an example)

Here are some interesting things about this enurmated space:

- The concatenation of the tag and index bits uniequely identifies each block in memory. Block 0 consist of address 0 and 1, Block 1 1 of addresses 2 and 3 and so on...

- Since there is eight memory blocks but only four cache sets, multiple blocks map to the same cache set(have the same index).

- Blocks that map to the same cacahe set are uniquely identified by the tag bit.

##### Conflict Misses in Direct-Mapped Caches

Conflict misses in direct-mapped caches typically occur when programs access arrays whose sizes are a power of 2.

Say we had a function which took two float arrays, both 8 elements. And if our cache was 32 bytes, for each iteration of a loop over each, we would cause a cache miss, and load in 4 elements of each array, twice, per iteration. Good spatial locality does not always mean effiency.

A cache repeatedly loading and evicting the same sets of cache blocks is knwon as *thrashing*.

Luckily, thrashing is easy to fix once you recognize what is going on. One easy fix is top put *B* bytes of padding at the end of each array. This way, the arrays wll map to different sets, elimating the need to load and reload over and over.

###### Why Index with the middle bits?

Caches use the middle bits for the set index, instead of the high order bits as is used in most other scenerioas/data. If we used highorder bits, and mapped over an array sequentially, then the cache can only hold a block-sized chunk of the array at any point in time. Contrast this with middle-bit indexing, adjacent blocks always map to different cache sets, so the cache can hold an entire cache-sized chunk of the array.

##### Set Associative Caches

The problem with conflict misses in direct-mapped caches stem from the constraint that each set has exactly one line.

A *set associative cache* relaxes this constraint so that each set holds more than one cache line.

Set selection is identical to a direct-mapped cache with the set index bits indeitfying the set.

###### Line Matching and Word Selection in Set Associative Caches

Line matching is more involved in a set associative cache than in a direct-mapped cache because it must check the tags and valid bits of multiple lines in order to determine if the requested word is in the set. A convential memory is an array of values that takes an address as input and returns the value stored at that address.

An *associative memory* is an array of key value pairs that takes as input the key and returns from one of the key value pairs that matches the input key.

So, we can think of each set in a set associative cache as a small associative memory where the keys are the concatenation of the tag and valid bits, and the values are the contents of a block.

###### Line Replacement on Misses in Set Associative Caches

If the word requested by the CPU is not stored in any of the lines in the set then we have a cache miss and the cache must  fetch the block that contains the word from memory. However, once the cache has retreived the block, which line should it replace?

If there is an empty line, than obviously pick that. If not, we must choose a nonempty line and hope the CPU does not <F16>reference the replaced line anytime soon.
This will of course depend on your CPU's Line Replacement Policy.


#### Fully Associative Caches

A *fully associative cache* consists of a single set that contains all of the cache lines.
Set Selection is a non-sequitur, as there is only one set. There are no index bits in the address, only a tag and a block offset.

Line matching and word selection ina fully associtive cache work the same as with a set associative cache, the difference is mainly a question of scale.
Because of cache circuitry, it is difficult and expensive to builkd an associative cache that is both large and fast. As a result, fully associative caches are only appropiate for small caches, such as the translation lookaside buffers (TLBs) in virtual memory systems that cache page table entries.

#### Cache & Writes

As we have seen cache read operations are fairly straightforward, look for the word in the cache, if foudn return it, if not retreive the block containg the word from the next level down in the memory hierarchy and store the block in some cache line, then return it the desired word.

Writes are more complicated.

Say we write a word *w* that is already cache( a *write hit*). After the cache updates its copy of *w* what does it do about updating the copy of *w* lower down in the memory hierarchy?

One simple approach is called a *write-through* which is to immediately write *w*'s cache block to the next lower level. This has problems with causes bus traffic with each write.

Another approach is *write-back*, which defers the update as long as possible by writing the updated block to the next lower level only when it is evicted from the cache by the replacement algorithim.
Because of locality, write-back can significantly reduce the amount of bus traffic but has the disadvantage of additional complexity.

The cache must maintain and additional *dirty bit* for each cache line that indicates whether or not the cache block has been modified.

Another issue is dealing with write misses. One approach known as *write-allocate* loads the corresponding block from the next lower level into the cache and then updates the cache block.

Write-allocate tries to epxloit spatial locality if writes, but it has the disadvantage that every miss results in a block transfer from the next lower level to the cache. The alternative, *no-write-allocate* bypasses the cache and writes the word directly to the next lower level.

Write thrgouh caches are typically no-write-allocate.

Write-back caches are typically write-allocate.

As a rule, caches at lower levels of the memory hierarchy are more likely to use write-back instead of write-through because of the larger transfer times.

But as logic densities increse, the increased complexity of write-back is becoming less of an impemdiment and we are seeing write-back caches at all levels of modern systems. So this assumption(rule) meets current trends.We can develop our programs at a high level to exhibit good spatial and temporal locality rather than trying to optimize for a particular memory system.


#### Anatomy of a Real Cache Hierarchy

A cache that holds only instructions is called an *i-cache*. They are typically read-only.

A cache that holds only program data is called a  *d-cache*.

A cache that holds both is called a *unified cache*.

Modern Processors inlcude seperate i-caches and d-caches. This way, a processor can read an instruction and a data word at the same time.
The two caches are optimized to different access patterns and can have different block sizes, associativies, and capacities.
Also, seperate caches ensure that data accesses do not create conflict misses with instruction accesses and vice versa.


##### Cache Performance Metrics

*Miss rate* The fraction of memory references during the execution of a program that miss. Computed as `no. of misses / no. of references`.

*Hit rate* The fraction of memory references that hit. Computed as `1 - miss rate`.

*Hit time* The time to deliever a word in the cache to the CPU. Time for set selection, line identification and word selection. Hit time is on the order of several clock cycles for L1 caches.

*Miss Penalty* Any additional required because of a miss. The penalty for L1 misses served from L2 is on the order of 10 clock cycles.

#### Factors on Cache Performance

Cache size also has a demonstrable impact on performance. A larger cache size will tend to increase the hit rate, however it is always harder to make large memories run faster.

This is why L1 is always smaller than a L2 cache, which is smaller than an L3.

Larger blocks can help increase the hit rate by exploiting any spatial locality that might exist in a program. However, larger blocks imply a smaller number of cache lines, which can hurt the hit rate in programs with more temporal locality than spatial locality. Larger blocks cause larger transfer times.

The impact of the cache lines per set is key. The advantage of higher associativity is that it decreases the vulnerability of the cache to thrashing due to conflict misses.

However, higher associativity is expensive to implement and hard to make fast; requires more tag bits per line, additional LRU state bits per line and additional control logic.

Higher associativity can increase hit time because of the increased complexity and it can also increase the miss penalty because of the increased complexity of choosing a victim line.

The choice of associativity boils down to a trade-off between hit time and the miss penalty.

Usually, higher-performance systems that pushed clocks rates opted for smaller associativity for L1 caches.

Impact of the wirte strategy that is used impacts performance. Write-through caches are simpler to implement and can use a *write buffer* that works independently of the cache to update memory. Read missess are less expensive than write misses as they do not trigger a memory write.

Write-back caches do however result in fewere transfers, allowing more bandwith to memory for I/O devices that perform DMA(Direct Memory Access).

Caches further down the hierarchy are more likely to use write-back than write-through.

#### Cache, Lines, Sets & Blocks Recap

- A *block* is a fixed-size packet of information that moves back and forth between a cache and main memory (or a lower-level cache).

- A *line* is a container in a cache that stores a block, as well as other information such as the valid bit and the tag bits.

- A *set* is a collection of one or more lines. Sets in direct-mapped caches consists of a single line. Sets in set associative and fully associative caches consist of multiple lines.




#### Writing Cache-Friendly Code

As we know, programs with better locality will tend to have lower miss rates, and lower miss rates will run faster than programs with higher miss rates.
To do this we can:

1. *Make the common case go fast* Programs often spend most of their time in a few core functions. These functions often spend most of their time in a few loops. So focus on the inner loops of the core functions and ignore the rest.

2. *Minimize the number of caches misses in each inner loop.* All other things being equal, such as the total number of loads and stores, loops with better miss rates will run faster.


### The Impact of Caches on Program Performance

A nice visualization of cache performance is done via a **memory mountain**: ![An Example of a Memory Mountain](http://csapp.cs.cmu.edu/public/images/mountain2e-labeled.gif)

We can think of a 'stride' as a memory access of a certain block of memory such as
```c
int32[] array;
for (int i = 0; i < N; i++) {
	printf("%d", array[i]);
}

```
Here we are accessing in 4-byte/32-bit chunks.

On each of the L2, L3 and main memory ridges there is a slope spatial locality that falls downhill as the stridge increases and spatial locality decreases.

Notice there is more than an order of magnitude between the highest peak of the L1 ridge and the lowest point of the main memory ridge.



#### Using Blocking to increase temporal locality

Blocking is used to increase temporal locality of inner loops.

The idea is to organize data structures into large chinks called *blocks*. (A *block* here refers to an application-level chunk of data, not to a cache block.)
The program is struyctured so that it loads a chunk into the L1 cache, does all the reads and writes that it needs to on that chunk, discards that chunk & loads in the next and so on.

Unlike loop transformations/adjustments for improving spatial locality, blocking makes code harder to read and to understand. For this reason, it is best suited for optimizing compilers or very commonly used library routines/functions.


## Linking

Linking is the process of collecting and combining various pieces of code and data into a single file that can be *loaded*(copied) into memory and executed.

Linking can be performed at:

- *compile time* when the source code is translated into machine code.

- *load time* when the program is loaded into memory and executed by the *loader*.

- even at *run time* by application programs.

The process of *linking* used to be performed manually, but is now done automatically by *linkers*.

Linkers are very useful as they enable *seperate compilation*, instead of one monolithic binary, you can decompose it into smaller more manageable modules.

### Compiler Drivers

Most compilation systems provide a *compiler driver* that invokes the language preprocessor, compiler, assembler and linker as needed on behalf of the user. For example, to build a program using the GNU compilation system(based):
```bash
$ gcc -0g -o a.out main.c sum.c
```
The driver first runes the C preprocessor (cpp) which translates the C source `main.c` into an ASCII intermediate file `main.i`.

Then the driver runes the C compiler (cc1) which translates `main.i` into an ASCII assembly-language file `main.s`.

Then, the driver runes the assembler (as) which translates `main.s` into a binary *relocatable object file* `main.o` The driver will then go through the same process to generate `sum.o`. Finally, it runs the linker program `ld` which combines `main.o` and `sum.o` along with the neccessary system object files.

To run our program, we type its name into a shell:
```bash
$ ./a.out
```
This invokes a function in the operating system called the *loader* which copies the code and data in the executable file `a.out` into memory and then transfers control to the beginning of the program.

### Static Linking

*Static Linkers* such as the Linux `LD` program take as input a collection of relocatble object files and command-line arguments and generate as output a fully linked executable object file that can be loaded and run. The input relocatable object file consist of various code and data sections where each section is a contiguous sequence of bytes.

To build the executable the linker must perform two main tasks:

1. Symbol Resolution: Object file define and reference symbols where each symbol correpsonds to a function a global variable or a static variable(i.e any C variable declared with the `static` attribute).

2. Relocation: Compulers and assemblers generate code and data sections that start at address 0. The linker *relocates* these sections by associating a memory location with each symbol definition, and then modifying all references to those symbols so they point to the new memory location. The linker blindly performs these relocations using detailed instructions, generated by ther assembler, called *relocation entries*.

Object files are merely collections of blocks of cytes, some contain program code, others contain data structures that guide the linker and loader. A linker concatenates blocks together, decides on run-time locations for the concatenated blocks, and modifies various locations within the code and data blocks.  

### Object Files

Object Files come in three forms:

- *Relocatable Object File*: Contains binary code and data in a form that can be combined with other relocatable object files at compile time to create an executable object file.

- *Executable object File*: Contains binary code and data ina  form that can be directly into memory and executed.

- *Shared Object File*: A special type of relcatble object file that can be loaded into memory and linked dynamically, at either load time or run time.

Compilers & Assemblers generate relocatable object files (including shared object files).
Linkers generate executable object files.

### Relocatable Object Files

![ELF Relocatable Object File Section Table](https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpeople.cs.pitt.edu%2F~xianeizhang%2Fnotes%2Flink_img%2Freloc_obj.png&f=1&nofb=1)

The above picture shows the format of a typical ELF relocatable object file. The *ELF header* begins witha  16-byte sequence that describes the word size and byte ordering of the system that generated the file.
The rest of the ELF header contains information that allows a linker to parse and interpret the object file. This incluides the size of the ELF header, the object file type (relocatable, executable or shared), the CPU architecture(e.g x86-64)

The locations and sizes of the various sections are described by the *section header table* which contains a fixed-size entry for each section in the object file.

Sandwiched between the ELF header and the section header table are the sections themselves. A typical ELF relocatable object file contains the following sections:

- `.text` The machine code of the compiled program.

- `.rodata` Read-only data such as the format strings in `printf` statements and jump tables for switch statements.

- `.data` *Initalized* global and static C variables. Local C variables are maintained at run time on the stack and do *not* appear in either the `.data` or `.bss` sections.

- `.bss` *Uninitalized* global and static C variables, along with any global or static variables that are initalized to zero. This section occupies no actual space in the object file, it is merely a placeholder. Object file format distinguish between initalized and uninitalized variables for space effiency, uninitalized do not need to occupy any actual disk space in the object file. At run time, these variables are allocated in memory with an initial value of zero.
The reason for using 'bss' is as it stands for 'block started by symbol' directive from the IBM 704 assembly language.

- `.symtab` A *symbol table* with information about functions and global variables that are defined and referenced in the program. A common misconception is that a program must be compiled with the `-g` to get symbol table information.Every relocatable object file has a symbol table in `.symtab`(unless the progreammer has spefically removed it with the `strip` command). However, inlike the symbol table inside a compiler, the `.symtab` symbol does not contain entries for local variables.

- `.rel.text` A list of locations in the `.text` that will need to be modified when the linker combines this object file with others. Usually, any instruction that calls an external function or references a global variable **will** need to be modified. Relocation information is not needed in executable object files, and is usually omitted unless the user explicitly instruts the linker to include it.

- `.rel.data` Relocation information for any global variables that are referenced or defined by the module.

- `.debug` A debugging symbol table with entries for local variables and typedefs defined in the program, global variables defined and referenced in th eprogram and the original C source file. Only present if the compiler driver is invoked with the `-g` flag.

- `.line` A mapping between line numbers in the C source and machine code instructions in the `.text` section. Only present when `-g` flag is used.

- `.strtab`  A string table for the symbol tables in the `.symtab` and `.debug` sections and for the section names in the section headers. A string table is a sequence of null terminated character strings.

### Symbols and Symbol Tables

In the context of a linker, there are three types of symbols:

Each Relocatable object module *m* has a symbol table that contains information about the symbols that are defined and referenced by *m*.

- *Global Symbols* that are defined by that are defined by module *m* and that can be referenced by other modules. Global linker symbols correspond to *nonstatic* C functions and global variables.

- Global symbols that are referenced by module *m* but defined by some other module. Such symbols are called externals and correpsond to nonstatic C functions and global variables that are defined in other modules.

- *Local Symbols* that are defined and referenced exclusively by module *m*. These correspond to static C function and global variables that are defined with the `static` attribute. These symbols are visisble anywhere within module *m* but cannot be referenced by other modules.

Local linker symbols are not the same as local program variables.

The symbols table in `.symtab` does not contain any symbols that correspond to local nonstatic program variables. These are managed at runtime, and as such not of interest to the linker.

Local procedure variables that are defined with the C `static` attribute are not managed on the stack. Instead, the compiler allocates space in `.data` or `.bss` for each definition and creates a local linker symbol in the symbol table with a unique name, for example:
```c

int f() {
	static int x = 0;
	return x;
}

int g() {
	static int x = 1;
	return x;
}

```
Here, the compiler exports a pair of local linker symbols with different names to the assembler, so it may use `x.1` for the definition in function f and `x.2` for the definition in function g.
Symbol tables are built by assemblers, using symbols exported by the compiler into the `.s` file.

An ELF symbol table is contained in the `.symtab` section.
It contains an array of entries, below is the structure of an entry expressed in a C struct:
```c
typedef struct {
    int name;        /* string table offset */
    long value;       /* section offset, or VM address */
    long size;        /* object size in bytes */
    char type:4,     /* data, func, section, or src file name (4 bits) */
	 binding:4;  /* local or global (4 bits) */
    char reserved;   /* unused */  
    char section;    /* section header index, ABS, UNDEF, */
                     /* or COMMON  */  
} Elf64_Symbol;
```
The `name` is a byte offset into the string table that points to the null-terminated string name of the symbol.

The `value` is the symbol's address.
For relocatable object modules, the `value` is an offset from the beginning of the section where the object is defined.
For executable object, files the `value` is an absoloute run-time address.

The `size` is the size (in bytes) of the object.

The `type` is usually either `data` or `function`.

The symbol table can also contain entries for the individual sections and for the path name of the original source file. So there are distinct types for these objects as well.

There are distinct types for these objects also.
The `binding` field indicates whether the symbol is local or global.

Eacyh symbol is assigned to some section of the object file, denoted by the `section` field which is an index into the section header table.

##### Pseudosections

There are three special pseudosections which don't have entries in the section header table:

- ABS is for symbols that should not be relocated. (ABSoloute)

- UNDEF is for undefined symbols, referenced in this object module but defined elsewhere.

- COMMON is for unintialized data objects that are not yet allocated.

For COMMON symbols, the `value` field gives the alignment requirement, and `size` gives the minimum size.

These pseudosections exist **only** in relocatable object files.

The distinction between COMMON and `.bss` is subtle. Modern versions of *GCC* assign symbols in relocatable object files to COMMON and `.bss` using the following convention:

- COMMON	Uninitalized global variables.

- `.bss`	Uninitalized static variables and global or static variables that are initalized to zero.

The GNU `readelf` program is useful for viewing the contents of object files.

#### Symbol Resoloution

*Symbol here meaning the name of a function or variable.*

The linker resolves symbol references by associating each reference with exactly one symbol definition from the symbol tables of its input relocatable object files.

The compiler allows only one definition of each local symbol per module.

It also ensures that static local variables, which get local linker symbols, have unique names.

Global symbols is more difficult however. When the compiler encounters a symbol that is not defined in the current module, it assumes that it is defined in some other module, generates a linker symbol table entry and leaves it for the linker to handle.

If the linker is unable to find a definition for the referenced symbol in any of its input modules, it will error and terminate(usually something akin to `undefined reference to 'foo'`).

Global symbols could also be defined multiple times in different object modules with the exact same name. The linker must either flag an error or choose one of the definitions and discard the rest.

The approach of Linux systems involves cooperation from the compiler, assembler & linker, which can introduce some strange bugs.

At compile time, the compiler exports each global symbol to the assembler as either a *strong symbol*, stored in `.bss` and `.data` or a *weak symbol* stored in `.COMMON`.

Functions and initlizated global variables get strong symbols.

Uninitalized global variables get weak symbols.

Using this system of weak and strong symbols, linux linkers use the following rules for dealing with duplicate symbol names:

1. Multiple strong symbols with the same name are not allowed.

2. Given a strong symbol and multiple weak symbols with the same name, choose the strong symbol.

3. Given multiple weak symbols with the same name, choose any of the weak symbols.

#### Linking with Static Libraries

So far we have assumed that the linker reads a collection of relocatable object files and links them together into an output ELF.

Compilation systems provide a mechanism for packaging related object modules into a single file called a *static library*, which can then be supplied into the linker.

When the linker builds the output executable, it only copies the object modules that are referenced.

This is extremely useful, as instead of lib devs having to choose between creating a monolith lib, wasting masses of memory and disk space in uneeded functions and variables, or a seperate object module for each function, we can link to only what we need.

On linux systems, static libraries are stored on disk in a particular file format known as an *archive*. An archive is a collection of concatenated relocatable object files, with a header that describes the size and location of each member object file. Archive files are denoted with a `.a` suffix.

While static libraries are useful, they can be confusing due to the way the Linux linker uses them to resolve external reference.

During the symbol resoloution phase, the linker scans the relocatable object files and archives left to right in the same sequential order they appear on the compiler driver's command line.
The driver also automatically translates any `.c` files on the command line to `.o` files.

During this scan, the linker maintains a set *E* of relocatable object files that will be merged to form the executable, a set *U* of unresolved symbols and a set *D* of symbols that have been defined in previous input files.

- For each input file *f* on the command line, the linker determines if *f* is an object file or an archive. If *f* is an object file, the linker adds *f* to *E*, updates *U* and *D* to reflect the symbol definitions and references in *f* and proceeds to the next input file.

- If *f* is an archive, the linker attempts to match the unresolved symbols in *U* against the symbols defined by members of the archive. If some archive member *m* defines a symbol that resolves a reference in *U*, then *m* is added to *E* and the linker updates *U* and *D* to reflect the symbol definitions and references in *m*. This process iterates over the member object files in the archive until a fixed point is reached where *U* and *D* no longer change. At this point any member object files not contained in *E* are simply discarded and the linker proceeds to the next input file.

- If *U* is nonempty when the linker finishes scanning the input files on the command line, it prints an error and terminates. Otherwise, it merges and relocates the object files in *E* to build the ouptut executable file.

The general rule for libraries is to place them at the end of the command line.

### Relocation

*note the linker will use ASLR when assigning run-time address to the stack, shared library and heap segments.*

Once the linker has completed symbol resoloution and associated each symbol reference in code with exactly one symbol definition.

At this point, the linker knows the exact sizes of the code and data sections in its input object modules.

It can now begin the relocation step, where it merges the input modules and assigns run-time addresses to each symbol.

Relocation consists of two steps:

1. *Relocating sections and symbol definitions*. In this step, the linker merges all sections of the same type into a new aggregate section fo the same type. For example, the `.data` sections from the input modules are all merged into one section that will become the `.data` section for the ouput executable object.

2. *Relocating symbol references within sections*. The linker modifies every symbol reference in the bodies of the code and data sections so that they point ot the correct run-time addresses. The linker relies on data structures in the relocatable object modules known as relocation entries.


When an assembler generates an object module, it does not know where the code and data will ultimately be stored in memory, nor the location of any externally defined functions or global variables that are referenced by the module.

So, whenever the assembler encounters a reference to an object whose ultimate location is unknown, it generates a *relocation entry* that tells the linker how to modify the reference when it merges the object file into an executable.

Relocation entries for code are placed in `.rel.text` and for data in `.rel.data`.

Below is the format of an ELF relocation entry.

```c

typedef struct {
	long offset; /* Offset of the reference to relocate */
	long type:32, /* Relocation type */
		symbol:32; /* Symbol table index */
	long addend; /* Constant part of relocation expression */
} Elf64_Rela;

```
`offset` is the section offset of the reference that will need to modified.

`symbol` identifies the symbol that the modified reference should point to.

`type` tells the linker how to moidify the new reference.

`addend` is a signed constant that is used by some type of relocations to bias the value of the modified reference.

ELF defines 32 different relocation types. However, I will only mention the two most basic relocation types:

`R_X64_64_PC32` Relocate a reference that uses a 32-bit PC-relative address. When the CPU executes an instruction using PC-relative addressing, it forms the *effective address* (e.g the target of `call`) by adding the 32-bit value encoded in the instruction to the current run-time value of the PC, which is `$eip` or `$rip`.

`R_X86_64_32` Relocate a reference that uses 32-bit absolute address. With absolute addressing, the CPU directly uses the 32-bit value encoded in the instruction as the effective address.

These two relocation types support the x86-64 *small code model* which assumes that the total size of the code and data in the executable object is smaller than 2GB, and thus can be accessed at run-time using 32-bit PC-relative addresses. This is the default for GCC. Programs larger than 2GB can be compiled with `-mcmodel=medium` or large flags, for medium and large code model respectively.

#### Relocating PC-Relative References

Take this ection of a main binary:

``` asm

	sub $0x8, %rsp
	mov $0x2, %esi
	...

	callq 13 <main+0x13>	sum()
     f: R_X86_64_PC32 sum-0x4	Relocation entry

```

The call instruction begins at section offset `0xe8` followed by a placeholder for the 32-bit PC-Relative reference to the target `sum`.
The corresponding relocation entry `r` consists of four fields:

```python

r.offset = 0xf
r.symbol = sum
r.type = R_X86_64_PC32
r.addend = -4

```

These fields tell the linker to modify the 32-bit PC-Relative reference starting at offset `0xf` so that it will point to the `sum` routine at run time.

Prior to these calculations, the linker would have already chosen run-time addresses for the sections.

For this example, we assume that the linker has already chosen run-time addresses for each section, (denoted `ADDR(s)`) and each symbol (denoted `ADDR(r.symbol)`).

So if `.text` is located at `0x4004d0`, and the the r.symbol of sum at `sum` at `0x4004e8`.

the linker then computes the run-time address of the reference:
```python
refaddr = ADDR(s) + r.offset
	= 0x4004d0 + 0xf
	= 0x4004df
```

It then updates the reference so that it will point to the `sum` routine at run time:

```c

*refptr = (unsigned) (ADDR(r.symbol + r.addend - refaddr)
	= (unsigned) (0x4004e8	    + (-4)	- 0x4004df)
	= (unsigned) (0x5)
```

In the resulting executable object file, the `call` instruction has the following relocated form:

```asm

4004de:    e8 05 00 00 00		callq 4004e8 <sum>	sum()

```

The call instruction is located at address `0x4004de`. When the CPU executes the `call` instruction the PC has a value of `0x4004e3` which is the address of the instruction immediately following the `call` instruction.

To execute the call instruction, the CPU performs the following steps:

1. Push PC onto stack

2. PC <- PC + `0x5` = `0x4004e3` + `0x5` = `0x4004e8`

Thus, the next instruction is the first inside of the `sum` function/routine. Nice!


### Relocating Absoloute References

This is much easier than relative relocations.

Here we can skip calculating run-time address (`refaddr`) and minusing this from the `ADDR(r.symbol)`.

`ADDR(r.symbol)` = `ADDR(array)` = `0x601018`

```python

*refptr = (unsigned) (ADDR(r.symbol) + r.addend)
	= (unsigned) (0x601018 + 0)
	= (unsigned) (0x601018)

```

### Exeutable Object Files

The format of an executable object file is similar to that of a relocatable object file.

The ELF header desribes the overall format of the file.

The `.text`, `.rodata` and `.data` sections are similar to those in a relocatable object file, except that these sections have been relocated to their eventual run-time memory addresses.

The `init` sections defines a small function, called `_init`, that will be called by the programs initialization code. Obviously as the executable is relocated (*fully linked*) it needs no `.rel` sections.

ELF executables are designed to be easy to load into memory, with contigous chunks of the exeutable file mapped to contiguous memory segments. This mapping is described by the *program memory table*.

### Loading Exeutable Object Files

when running a executable object file, we can type its name into a shell as such:
```sh
$ ./prog
```
Since prog does not correspond to a built-in shell command, the shell assumes that `prog` is an exectuable object file, which it runs by invoking the `loader`. Any linux program can invoke the loader by calling the `execve` function.

The loader copies the code and data in the executable objet file from disk into memory, then runs the program by jumping to its first instruction or *entry point*.

This process of copying the program into memory and then running it is known as *loading*.

On Linux x86-64 systems, the code segment starts at address `0x400000`, followed by the data segment.

The run-time *heap* follows the data segment and grows upward via calls to the `malloc` library.

This is followed by a region that is reserved for shared modules.

The user stack starts below the largest legal user address (2<sup>48</sup> - 1) and grows down. The region above the stack, starting at address 2<sup>48</sup>, is reserved for code and data in the kernel(the memory resident part of the operating system).

There is usually a gap between the code and data segments due to alignment requiremen on the `.data` segment.

Even though the locations of these regions change each time the program is run (ASLR) their relative positions stay the same.

When the loader runs, it creates a memory image, guided by the program header table, it copies chunks of the executable object file into the code and data  segments.

Next, the loader jumps to the program's entry point which is always the address of the `_start` function. This function os defined in the system object file `crt1.o` and is the same for all C programs. The `_start` function calls the *system startup function*, `__libc_start_main`, as defined in `libc.so`. It initializes the execution enviroment calls the user-level `main` function, handles its return values and if necessary returns control to the kernel.



### Dynamic Linking

Lets say we have an example program, `prog21`.

When the loader loads and runs the executable `prog21`, it loads the partially linked executable. Next it notices that `prog21` contains a `.interp` section, which contains the path name of the dynamic linker, which is itself a shared object (`ld-linux.so` on linux systems). Instead of passing control to the application, i the loader loads and runs the dynamic linker.

The dynamic linker then finishes the linking task by relocating the text and data of the `.so` files, then relocates any references in `prog21` to symbols defined by shared objects libs.

Finally, the dynamic linker passes control to the application. From this point on, the locations of the shared libraries are fixed and do not change during execution of the program.

### Loading and Linking Shared Libraries from Applications

So far we have just noted how an application to request the dynamic linker to load and link shared libraries when an application is loaded, just before it executes.

However, an application can requiest the dynamic linker to load and and link arbitrary shared libraries while the application is running, without having to link in the applications against those libraries at compile time.

### Position-Independent Code (PIC)

A key purpose of shared libraries is to allow multiple running processes to share the same library code in memory and thus save memory resources.

So how is this implemented?

Code that can be loaded witout needing any relocations is knwon as *position-independent code (PIC)*. GNU compilation systems can generate PIC with the `-fpic` flag to GCC. Shared libraries must be compiled with this option.

On x86-64 systems, references to symbols in the same executable object module require no special treatment to be PIC. These refences can be com,piled using PC-relative addressing and relocated by the static linker when it builds the object file.

However, refernces to external proceudres and global variables defined by shared modules require some special techniques.

#### PIC Data References

Compiler generate PIC references to global variables by expoiting the fact that no matter where we load an object module in memory, the data segment is always the same distance from the code segment. So, the *distance* between any instruction in the code segment and any variable in the data segment is a run-time constant, independent of the absoloute locations of the code and data segments in memory.
Compilers that want to generate PIC references to globa variables do this by creating the *global offset table (GOT)* at the beginning of the data segment.

The GOIT contains an 8-byte entry for each global data object (proceudre or variable) that is referenced by the object module. The compiler also generates a relocation record for each entry in the GOT.

This may be used in a code segment such as:

```x86

addvec:
	mov 0x2008b9(%rip), %rax	# %rax=*GOT[3]=&addcnt
	addl $0x1, (%rax)		# addcnt++

```

The code in `addvec` refences the global variable `addcnt` via the known fixed distance at run-time - `0x2008b9`, then adds 1 to the variable via the pointer dereferenced in `(%rax)`.

Whats key to note is that the offset in the PC-relative reference to `GOT[3]` is a run-time constant.


#### PIC Function Calls

If a program calls a function that is defined by a shared library, the compiler has no way of predicting the run-time address of the function, since the shared module that defines it could be loaded anywhere at run time.

A 'normal' approach here would be to generate a relocation record for the reference, which the dynamic linker could then resolve when the the program was loaded.

However this would not be PIC as it would require the linker to modify the code segment of the calling module. GNU compilation systems (such as GCC) solve this by using a technique called *lazy binding*, which defers the binding of each procedure address until the **first time a procedure is called**.

This works because a typical program will call only a handful of the hundreds or thousands of functions exported by a shared library such as `libc.so`.
By deferring the relocations until used, the dynamic linker can avoid hundreds or thousands of relocations at load time.

Lazy binding is implemented via intercation between the Global Offset Table (GOT) and the Procedure Linkage Table (PLT). If an object module calls any functions that are defined in shared libraries then it has its own GOT & PLT.

The GOT is part of the data segment.

The GOT is an array of 8-byte address entries. When used in conjunction with the PLT. `GOT[0]` and `GOT[1]` contain information that the dynamic linker uses when it resolves function addresses.

The PLT is part of the code segment.

The PLT is an array of 16-byte code entries.
PLT[0] is a special entry that jumps into the dynamic linker.
Each shared library function called by the executable has its own PLT entry. each entry is repsonsible for invokiung a specific function. One of the key entries will be the system startup function - `__libc_start_main`.


For example, say we want to resolve the run-time address of function `addvec` the first time it is called:

1. Instead of directly calling `addvec`, the program calls into `PLT[2]` which is the PLT entry for `addvec`.

2. The first PLT instruction does an indirect jump through `GOT[4]`. Since each GOT entry intially points to the second instruction in its corresponding PLT entry, the indirect jump simply transfers control back to the next instruction in `PLT[2]`.

3. After pushing an ID for `addvec` (0x1) onto the stack, `PLT[2]` jumps to `PLT[0]`.

4. `PLT[0]` pushes an argument for the dynamic linker indirectly through `GOT[1]` and then jumps into the dynamic linker indirectly through `GOT[2]`. The dynamic linker uses the two stack entries to determine run-time location of `addvec`, overwrites `GOT[4]` with this address and passes control to `addvec`.

For any subsequent invocations of `addvec`:

1. Control passes to `PLT[2]` as before.

2. However, the indirect jump `GOT[4]` transfer control directly to `addvec`.


### Library Interpositioning

Linux linkers support a technique called *library positioning* that allows you to intercept calls to shared library functions to execute your own code instead.

Using interpositioning, you could trace the amount of time a library function is called, validate and trace its input and output values or even replcaec it with a completely different implementation.

For example, create a wrapper function whose prototype is the exact same as the target function, using some interpositioning mechanic, make the system run your wrapper function insstead. You can then execute some arbritary code, then call the actual function and return its return value(s).

You could implement this with including a header file, which uses preprocessor diretctives to define `malloc` as `mymalloc`, such as:

`mymalloc.h`

```c

#define malloc(size) mymalloc(size)
```

Now, if we have
```c
#include <stdlib.h>
#include <mymalloc.h>
```

calls to `malloc` will be redirected to the `mymalloc()` function defined in `mymalloc.h`.

Compile time interpositioning requires access to a program's source files. Link-time interpositioning requires access to its relocatable object files. However, there is a mechnaism for interpositining at runetime that requires access only to the executable object file. This mechanism is based on the dynamic linker's `LD_PRELOAD` enviroment variable.

If this variable is set to a listof shared library pathnames, (seperated by commas or spaces) then when you load and execute a program, the dynmic linker will search the `LD_PRELOAD` libraries first, before any other shared librares when resolving undefined references.


### Exceptions

Exceptions are a form of exceptional control flow that are implemented partly by hardware and partly by operating system.

An *exception* is an abrupt change in the control flow in response to some change in processor state.

This change in state is known as an event.

When the processor detects that an even thas occurred, it makes an indirect procedure call (the exception), through a jump table called an *exception table* to an operating system subroutine(the *exception handler*) that is designed to process this particular kind of event.

When the exception handler finishes processing, one of three things happens:

- The handler returns control to the current instruction(the instruction that was executing when the event occurred).

- The handler returns control to the instruction after the current instruction.

- The handler aborts the interrupted program.

Each type of possible exception in a system is assigned a unique nonnegative integer **exception number**.

At system boot time the operating system allocates and initalizes a jump table called an *exception table* so that entry *k* contains the address of the handler for exception *k*.

At runtime, the processor detects that an event has occurred and determines the corresponding exception number *k*. the processor then triggers the exception by making an indirect procedure call, through entry *k* of the exception table, to the corresponding handler.

The exception number is an index into the exception tabl, whose starting address is contained in a sepcial CPU register called the *exception table base register*.

An exception is similar to a procedure call, but has some important differences:

- As with a procedure call, the processor pushes a return address on the stack before branching to the handler. However, depending on the class of exception the return address is either the current instruction or the next instruction.

- The processor pushes some additional processor state onto the stack that will be needed to restart the interrupted program, such as on a x86-64 system, pushing the FLAGS register, containg current condition codes and other context information.

- When control is being transferred from user program to the kernel, all of these items are pushed onto the kernel's stack instead of the user's stack.

Exceptions can be divided into four classes:


#### Interrupts

Interrupts occur asynchronously as a result of signals from I/O devices that are external to the processor. They are async as they arenot caused by the execution of any instruction.

Return to the next instruction.

The remaining classes of exceptions (traps, faults, aborts) occur synchronously as a result of executing the current instruction. We refer to this instruction as the *faulting instruction.*

#### Traps

Traps are intentional exceptions that occur as a result of executing an instruction. Same as interrupts, Trap handlers return control to the next instruction.

The most important use of traps is to provide a procedure-like interface gbetween user programs and the kernel known as a *system call*.

#### Faults

Faults result from error conditions that a handler might be able to correct.

If the fault handler is able to correct the error condition it returns control back to the faulting instruction, re-executing it.

If it is not able to correct it, the handler returns to an `abort` routine.

A classic example of a fault is the page fault exception, which occurs when an instruction references a virtual address whose corresponding page is not resident in memory and therefore must be retreived from disk.


#### Aborts

Aborts result from unrecoverable fatal errors, typicall hardware errors such as parity errors that occur when DRAM or SRAM bits are corrupted.

### System Calls

C programs can invoke any system call directly using the `syscall` function. However, mostly you use wrapper functions in the C standard library.

All arguments to linux system calls are passed through general-purpose registers rather than the stack. By convention, register `%rax` contains the syscall number with up to six arguments in `%rdi`, `%rsi`, `%rdx`, `%r10`, `%r8` and `%r9`. On return from the system call registers `%rcx` and `%r11` are destroyed and `%rax` contains the return value. A negative return value -4095 and -1 indicates an error corresponding to negative `errno`.

Processors and Address Space are presented to programs as if they have complete control. Below is the address space of a process.

![Process Address Space](https://i.imgur.com/uiGj4vp.png)

Processors use a *mode bit* mechanism to know if they are in kernel mode or user mode. If the mode bit is set, kernel mode is enabled, and a process running in kernel mode can execute instruction and access any memory location in the system.

Linux the `/proc` filesystem that allows user mode processes to access the contents kernel data structures. The `/proc` filesystem exports the contents of many kernel data strucutres as a hierarchy of text file that can be read by user programs.


The kernel maintains a *context* for each process. The context is the state that the kernel needs to restart a preempted(preempted meaining 'suspended') process.

At certain points during the execution of a process the kernel can decide to preempt the current process and restart a previously preempted proces. This decision is known as scheduling and is handled by code in the kernel, known as the *schedueler*.

When the kernel selects a new process to run, it preempts the current process and tranfsers control to the new process using a mechnaism called a *context switch*.

#### Reaping Child Processes

When a process terminates for any reason, the kernel does not remove it from the system immediately, instead the process is kept around in a terminated state until it is *reaped* by the parent.

When the parent reaps the terminated child, the kernel passes the child's exit status to the parent and then discards the terminated process, at which point it ceases to exist.

A terminated process that has not yet been reaped is called a *zombie*.

When a parent process terminates, the kernel arranges the `init` process to become the adpoted parent of any orphaned children. If there is any zombie children the kernel will reap them.

The `init` process has a PID of 1, and is created by the kernel during system startup and is the ancestor of every process.

##### wait and waitpid

A process can wait for children to terminate via the `waitpid` function, which, by default (when `options` = 0) `waitpid` suspends execution of the calling process until a children process in its *wait set* terminates.

The members of the waitset is dertermined by the `pid` argument:


- If `pid` > 0, the wait set is a singleton child process whose process ID is equal to `pid`.

- If `pid` = -1, the wait set consists of all the parent's child processes.

There are various `options` constants, which are useful and can be found on the detailed man page [here](https://pubs.opengroup.org/onlinepubs/9699919799/functions/waitpid.html)

##### Sleep function

`sleep` retuns zero if the requested amount of time has elasped and the number of seconds left to sleep otherwise. The latter is only possible if it returns prematurely because of being interrupted by a signal such as SIGCONT.

#### Loading and Running Processes

We can load and run executable object files with `execve`, with the specified args:
```c
#include <unistd.h>

int execve(const char *filename, char *const argv[],
           char *const envp[]);
```

`execve` only returns  if there is an error.

 We can use `setenv`, `getenv` and `unsetenv` to perform operations on enviroment variables.

### Signals

A *signal* is a small message that notifies a process that an event of some type has occurred in the system. There are many types, which you can find via `man 7 signal`.

Sending a signal - The kernel sends(delivers) a signal to the destination process by updating some state in the context of the destination process.#

Receiving a signal - A destination process receives a signal when it is forced by the kernel to react in some way to the delivery of the signal. The process can either ignore the signal, terminate or *catch* the signal by executing a user-level function called a *signal handler*.

A signal that has been sent but not yet received is called a *pending signal*.If a process has a pending signal of type *k*, then any subsequent signals of type *k* sent to that process are not queued, they are just discarded.

A process can selectively *block* the receipt of certain signals. When a signal is blocked, it can be delivered, but the resulting pending signal will not be received until the porcess unblocks the signal.

Unix shells use the abstraction of a *job* to represent the processes that are created as a result of evaluting a single command line.
At any point, there is at most one foreground job and zero or more background jobs.

For example typing:

```bash
$ ls | sort
```
creates a foreground job consisting of two processes connected by a Unix pipe, one running `ls`, one running `sort`.
The shell creates a seperate process group for each job.

#### Receiving Signals

When the kernel switches a process *p* from kernel mode to user mode(e.g returning from a syscall or context switch) it checks the set of unblocked pending signals for *p*. If this et is empty (the usual case),  then the kernel passes to control to the next instruction in the logical flow of *p*. However, if the set is nonempty then tyhe kernel chooses some signal *k* in the set (usually the smallest k) and forces *p* to receive signal *k*. The receipt of the signal triggers some action by the process.Once the process completes the action, the control passes back to the next instruction in the logical control flow of *p*.

Each signal type has a predefined default action, which is one the following:

- The process terminates

- The process terminates and dumps core.

- The stops (susspends) until restarted by a SIGCONT signal.

- The process ignores the signal.

#### Blocking and Unblocking Signals

Linux provides implicit and explicit mechnaisms for blocking signals:

- *Implicit blocking mechanism* By default the kernel blocks any pending signals of the type currently being processed by a handler.

- *Explicit blocking mechanism* Applications can block and unblock selected signals using the `sigprocmask` function and its helpers. The `sigprocmask` function chantges the set of currently block signals.



#### Nonlocal Jumps

C provides a form a user-level exceptional control flow called a *nonlocal jump*, that transfers control from one function to another currently executing function without having to go through without having to go through the normal call and return sequence. Nonlocal jumps are provided by the `setjmp` and `longjmp` functions.o

refer to manpages.

#### Hiding variable and function names with static in C

C programmers use `static` to hide variable and function declarations inside modules, much the same as using `public` and `private` declarations in Java and C++. In C, source files play the role of modules. Any global variable or function declared with the static attribute is private to that module, therefore by default any global var/function declared without `static` is public and can be accessed by any other module.

## Virtual Memory

Modern systems provide an abstraction of main memory - *virtual memory*.

It provides:

1. Uses main memory efficently by treating it as a cache for an address space stored on disk, keeping only active areas in main memory and transferring data back and forth between disk and memory as needed.

2. Simplifies memory management for each process by providing a clean address space per-process.

3. Protects the address space of each process from corruption by other processes.

### Physical and Virtual Addressing

The main memory of a computer system is organised as an array of *M* contiguous byte-size cells. Each byte has a unique *physical address* (*PA*).

The most simple and natural way to access memory would be to use physical addresses.

This is what early PCs used, and some systems like digital signal processors embedded MCUs and more.

Modern processors use *virtual addressing*.

With virtual addressing the CPU acceses main memory by generating a *virtual address* (*VA*) which is then converted to the appropiate physical address before being sent to main memory to retreive. This process of conversion is *address translation*.

Dedicated hardware on the CPU chip called the *memory management unit* (*MMU*) translates virtual addresses using a lookup table stored in main memory, the contents of which are managed by the operating system.

### Address Spaces

An *address space* is an ordered set of nonnegative integer addresses.

If the integers in the address space are consecutive then it is an *linear address space*. In a system with virtual memory, the CPU generates virtual addresses from an address space of *N = 2<sup>n</sup>* addresses called the the *virtual address space*.
```python
{0,1,2,...,N - 1}
```

Naturally, the size of an address space is charectized by the number of bits that are needed to represent the largest address.
A system also has a *physical address space* that corresponds to the *M* bytes of physical memory in the system:
```python
{0,1,2,...,M - 1}
```

An address space is important as it makes a distinction between data objects(bytes) and their attributes(addresses).
Each memory address is an index into the array of contiguous bytes that is the address space.

As with any other cache in the memory hierarchy the data on the disk is partioned into blocks that serve as the transfer units 

VM systems handle this by partioning the virtual memory into fixed-size blocks, *virtual pages (VPs)*. Each virtual page is *P = 2<sup>p</sup> bytes in size.

Similarly, physical memory is partitioned into physical pages, also *P* pages in size.

The set of virtual pages is partitioned into three disjoint subsets:

- *Unallocated*. Pages that have not yet been allocated (or created) by the VM system. Unallocated blocks do not have any data associated with the, and thus do not occupy any space on disk.

- *Cached*. Allocated pages that are currently cached in physical memory.

- *Uncached*. Allocated pages that are not cached in physical memory.

### DRAM Cache Organization


For the purpose of this paragraph, *SRAM Cache* will denote the L1, L2 and L3 caches. And *DRAM Cache* denotes the VM system's cache that caches virtual pages in main memory.

As DRAM caches are lower in the memory hierarchy, the impact of a cache miss will be much higher as each operation costs more - *O(n<sup>2</sup>)* .

DRAM cache misses are served from disk, while SRAM cache misses are usually served from DRAM-based main memory. Because of the large miss penalty and the expense of accessing the first byte, virtual pages tend to be large typically ranging from 4KB to 2MB, DRAM caches also tend to be fully associative due to the large miss penalty that is any virtual page can placed in any physical page.Operating systems use sophisticated replacement algorithims for DRAM caches than the hardware does for SRAM caches.

### Page Tables

As with any cache the VM system must have a way to determine if a virtual page is cached somewhere in DRAM and which physical page it is cached in.

If there is a miss the system must determine where the virtual page is stored in disk, select a victim page in physical memory and copy the virtual page from disk to DRAM, replacing the victim page.

These capabilities are provided by a combination of software address translation hardware in the MMU(memory management unit) and a data strcture in physical memory called a *page table* that maps physical<->virtual addresses via Page Table Entries (PTEs).

The OS is responsible for maintaining the page table contents and trasnferring pages back and forth.

Each page in the virtual address space has a PTE at a fixed offset in the page table. PTEs usually consist of a *valid bit* and an *n*-bit address field. The valid bit indicates whether the virtual page is currently cached in DRAM. If the valid bit is set, the address field indicates the start of the corresponding physical address in DRAM where the virtual page is cached. 

If the valid bit is not set then a null address indicates that the virtual page has not yet been allocated, otherwise the address points to the start of the virtual page on the disk.

### Page Faults

A DRAM cache miss is known as a *page fault*.

The CPU has referenced a word in VP 3 which is not cached in DRAM. The address translation hardware reads PTE 3 from memory and infers from the valid bit that VP 3 is not cached and triggers a page fault exception. 

The page fault handler in the kernel selects a victim page which if modified copies the Victim page back to disk. In either case the klernel modifies the page table entry for the victim page to reflect the fact that it is no longer cached in main memory.

Next the kernel copies VP 3 from disk to PP 3 (physical page 3) in memory, updates PTE 3 and then returns.
When the handler returns it restarts the faulting instruction which resends the faulting virtual address to the address translation hardware, but now VP 3 is cached in main memory and the page hit is handled normally.

In virtual memory parlance, blocks are known as *pages*, the activity of transferring a page between disk and memory is known as *swapping out* or *paging*. Pages are swapped in (paged in) from disk to DRAM and swapped out for the opoosite.

The stratergy of waiting until the last moment to swap in a page when a miss occurs is known as *demand paging*.


### Locality (but for memory management)

Given the large miss penalities, memory is hard to manage efficently. To enhance performance use of **locality** is crucial. 

Although the number of distinct pages that programs reference during an entire run might exceed the total size of physical memory, the principle of locality promises that at any point in time they will tend to work on a smaller set of *active pages* known as the *working set* or *resident set*.

After an intial overhead where the working set is paged into memory subsequent references to the working set result in hits with no additional disk traffic.

If the the working set exceeds the size of physical memory then the program can start *thrashing* wherer pages are continously swapped in and out over and over.

Note: multiple virtual pages can map to 1 physical page.

### The use of virtual memory as a tool for memory management

VM simplifies linking and loading, the sharing of code and data, and allocating memory to applications.

- *Simplifying Linking*. A seperate address space allows each process to use some basic format for its memory image, regardless of actual position in memory beyond the abstraction of VM. For example, in the ELF format, code **always** starts at `0x400000`. The data segment follows the code segment after some alignment gap. This allows compilation and linking to be far easier and far more portable.

- *Simplifying Loading*. VM makes it easy to load executable and shared object files into memory. To load the `.text` and `.data` sections of an object file into a newly created process the linux loader allocates virtual pages for the code and data segments marks them as invalid (i.e not cached) and points their PTEs (Page Table Entries) to the appropiate locations in the object file. The loader never actually copies any data from disk-> memory. The data is paged in automatically and on demand by the VM system the first time each page is referenced. The notion of mapping a set of contiguous virtual pages to an arbitrary location in an arbitrary file is known as *memory mapping*. We can do this in linux in application level mapping via `mmap`.

- *Simplifying Sharing*. Seperate address spaces provide the operating system with a consistent mechanism for managing sharing between user processes and the operating system itself.Each process has its own private code, data, heap and stack areas not shared with any process. However sometimes the operating system may want different processes to map virtual pages to the same physical page, such as different programs making call to libc `printf`, in dynlinked programs.

- *Simplifying memory allocation*. When a program requests additional heap space the operating system allocates an appropiate number - *k*  of contiguous virtual memory pages and maps them to *k* arbitrary physical pages located anywhere in physical memory. There is no need for physical memory pages to be contignous and are mostly not.

### VM for Memory Protection  

Virtual memory spaces make it easy to isolate private memories of different processes. Making it harder to view memory of other processes etc.

The address translation mechanism can be extended in a natural way to provide even finer access control. Since the address translation hardware reads a PTE each time the CPU generates an address. We can control the contents of a virtual page by adding some additional permission bits to the PTE.

### Address Translation

Address translation is a mapping between elements of an *N-*element virtual address space (VAS) and an *M-*element physical address space (PAS).

A control register in the CPU, the *page table base register (PTBR)* points to the current page table which contains an *n*-bit virtual address is composed of two components - a *p-bit virtual page offset (VPO)* and an *(n - p)-bit virtual page number (VPN)*. The MMU uses the VPN to select the appropriate PTE.

Since the physical and virtual pages are both *p* bytes, the *physical page offset (PPO)* is identitical to the VPO.

### Integrating Caches and Virtual Memory

In any system using both virtual memory and SRAM caches there is the issue of whether to use virtual or physical addresses to access the SRAM cache.

With physical addressing it is straightforward for multple processes to have blocks in the cache at the same time and to share blocks from the same virtual pages. And, the cache does not have to deal with protection issues because access rights are chechked as part of the address translation process.

### Speeding up address translation with a TLB

Each time the CPU generates a virtual address the MMu must refer to a PTE in order to translate the virtual address into a physical address. This could cause an additional fetch from memory, depending on what level in memory hierarcvhy the needed block is in. If the PTE is cached in L1the cost goes as low as a handful of cycles, some systems try to eliminate this cost via including a small cache of PTEs in the MMU called a *translation lookaside buffer*. 

A TLB is a small, virtually addressed cache where each line holds a block consisting of a single PTE. A TLB usually has a high degree of associativity.

The index and tag fields that are used for set selection and line matching are extracted from the virtual page number (VPN) in the virtual address. If the TLB has *T = 2*<sup>*t*</sup> sets then the TLB *index (TLBI)* consists of the *t* least signifcant bits of the VPN and the TLB *tag (TLBT)* consits of the remaining bits in the VPN.

note: check figure 9.15/6

### Multi-Level Page Tables

If we have a 32-bit address space, 4KB pages and a 4-byte PTE then we need a 4MB page table resident in memory at all times. This is compounded for 64 bit address spaces.

A common approach for compacting the page table is to use a hierarchy of page tables instead.

Say we have a 32-bit virtual address space partitioned into 4 KB pages with page table entries (PTE) of 4-bytes each. Suppose also that at this point in time the virutal address space as the following form: The first 2K pages of memory are allocated for code and data, the next 6K pages are unallocated, th next 1,023 pages are also unallocated and the next page is allocated for the user stack

Each PTE in the level 1 table is responsible for mapping a 4 MB chunk of the virtual address space where each chunk consists of 1,024 contiguous pages.Given that the address space is 4 GB, only 1,024 PTEs are needed to cover the entire address space.

If every page in chunk *i* is unallocated then level 1 PTE *i* is null. If at least one page in chunk *i* is allocated, then level 1 PTE *i* points to the base of the level 2 page table.

Each PTE in a level 2 page table is responsible for mapping a 4 KB page of virtual memory. Notice that with 4-byte PTEs, each level 1 and level 2 page table is 4 kilobytes, same size as a page!

How does this reduce memory?

- If a PTE in the level 1 table is null then the corresponding level 2 page table does not even have to exist.

- Only the level 1 page table needs to be in main memory at all times, the level 2 page table(s) can be created and paged in & out as and when needed.


### Putting it together: End-to-End Address Translation

This will be an example of an end-to-end address translation on a small system with a TLB and L1 d-cache.
We will make the following assumptions:

- The memory is byte addressable.

- Memory accesses are to *1-byte words* (not 4-byte words).

- Virtual addresses are 14 bits wide (*n = 14*).

- Physical addresses are 12 bits wide (*m = 12*).

- The page size is 64 bytes (*P = 64*).

- The TLB is 4-way set associative with 16 total entries.

- The L1 d-cache is physically addressed and direct mapped, with a 4-byte line size and 16 total sets.

Here each page is 2<sup>6</sup> = 64 bytes, the low order 6 bits of the virtual address serve as the VPO and PPO respectively.

The high-order 8 bits of the virtual address serve as the VPN.

The high-order 6 bits of the physical address server as the PPN.

The TLB is virtually addressed using the bits of the VPN. Since the TLB has four sets, the low-order bits of the VPN serve as the set index. The remaining 6 high-order bits serve as the tag that distinguishes the different VPNs that might map to the same TLB set.

The page table is a single-level design with a total of 2<sup>8</sup> = 256 page table entries (PTEs). 


The direct-mapped cache is addressed by the fields in the physical address. Since each block is 4 bytes, the low-order 2 bits of the physical address serve as the block offset(CO). Since there are 16 sets, the next 4 bits serve as the set index and the remaining 6 bits serve as the tag.

Now lets say the CPU executes a load instruction that reads the byte at address `0x03d4`. To begin the MMU extracts the VPN (`0x0F`) from the virtual address and checks with the TLB to see if it has a cached copy of PTE `0x0F` from some previous reference. The TLB extracts the TLB index `0x03` and the TLB tag `0x3` from the VPN hits on a valid match in the second entry of set `0x3` and returns the cached PPN `0x0D` to the MMU.

If the TLB had missed, then the MMU would need to fetch the PTE from main memory.

It now can form the address by concatenating the PPN (`0x0D`) from the PTE with the VPO (`0x14`) from the virtual address, forming the physical address `0x354`. (concatenate here means prepend the binary values of the PPN to the VPO).

Next, the MMU sends the physical address to the cache which extracts the cache offset CO (`0x0`), the set index CI (`0x5`) and the cache tag CT (`0x0D`) from the physical address. 

Since the tag in set `0x5` matches CT, the cache detects a hit, reads out the data byte (`0x36`) at offset CO and returns it to the MMU, which then passes it back to the CPU. 

Other paths are also possible. For example, if the TLB misses then the MMu must fetch the PPn from a PTE in the page table. If the resulting PTE is invalid then there is a page fault and the kernel must page in the appriopriate page and rerun the instruction. Another possibility is that the PTE is valid but the necessary memory block misses in the cache.


### Case Study: The Intel Core i7/Linux Memory System

The diagram below gives highlights:

![core i7 memory system](https://i.imgur.com/PeuNeUG.png)

The processor package (chip) includes four cores, a large L3 cache shared by all of the cores and a DDR3 memory controller.

Each core contains a hierarchy of TLBs, a hierarchy of data and instruction caches, and a set of fast point-to-point links based on the QuickPath technology for communicating directly with other cores and the external I/O bridge.

The TLBs are virtually addressed and 4-way set associative. 

The L1, L2 and L3 caches are physically addressed witha  block size of 64 bytes.

L1 and L2 are 8-way set associative and L3 is 16-way set associative. 

The page size can be configured at startup time to be either 4 KB or 4 MB. Linux uses 4 KB pages.

#### Core i7 Address Translation

![](https://i.imgur.com/17GMTpc.png)

When a Linux process is running, the page tables associated with allocated pages are all memory resident as we know, although core i7 architecture allows pages to be swapped in and out.

The CR3 control register (bottom left) contains the physical address of the beginning of the level 1 (L1) page table. The value of CR3 is part of each process context and is restored during each context switch.

![](https://i.imgur.com/F92Chjh.png)

Above is the format of level 1,2 and 3 page table entries. When *P = 1* (which is always the case in Linux) the address field contains a 40-bit physical page number pointing to start of the appriopriate page table.

The PTE has three permission bits that control access to the page.

The *R/W* bit determines whether a page's contents are read/write or read-only. The *U/S* bit determines whether the page can be accessed in user mode and protects code and data in the os kernel from user programs.

The *XD* (Execute Disable) bit, introduced in 64-bit systems, can be used to disable instruction fetches from invdividual memory pages. Useful in buffer-overflow mitigations.

As the MMU translates each virtual address, it also updates two other bits that can be used by the kernel's page fault handler. The MMU sets the *A* bit which is known as a *reference bit* each time a page is acessed. The MMU sets the *D* bit or *dirty bit* each time th page is written to. A page that has been modified is sometime called a *dirty page*. The dirty bit tells the kenrel whether or not it must write back a victim page before it copies in a replacement page.

![](https://i.imgur.com/1uBTUER.png)

Above shows hows the Core i7 MMU uses the four levels of page tables to translate a virtual address to a physical address. The 36-bit VPN is partitioned into four 9-bit chunks each of which is used as an offset into page table. The CR3 register contains the physical address of the L1 page table. VPN 1 provides an offset to an L1 PTE, which contains the base address of the L2 page table. VPN 2 provides an offset to an L2 PTE and so on.

### Linux Virtual Memory System

As we know Linux has different memory address space for each process, different kernel stack, page tables, and other data structures.

![](https://i.imgur.com/3mUFeC9.png)

Above is the complete virtual memory of a linux process.

The kernel virtual memory contains the code and data structures in the kernel. Some regions of the kernel virtual memory are mapped to physical pages that are shared by all processes, such as kernel code and global data structures.

Linux also maps a set of contiguous virtual pages (equal to the amount of DRAM in the system) to the corresponding set of contiguous physical pages. THis provides the kernel with a convenient way to access any specific location in physical memory, such as when it needs to access page tables or perform memory-mapped I/O operations on devices that are ampped to particular physical memory locations.

### Linux Virtual Memory Areas

Linux organizes virtual memory as a collection of *areas* (also called *segments*). An area is a contiguous chunk of existing (allocated) virtual memory whose pages are related in some way.

For example, the code segment, data segment, heap, shared library segment and user stack are all distinct areas.

Each existing virtual page is contained in some area, and any virtual page that is not part of an area does not exist and cannot be referenced by the process.

Areas allow address spaces to have gaps.

The kernel does not keep track of virtual pages that do not exist, and such pages do not consume any sadditional resources in memory, on disk or in the kernel itself.

![](https://i.imgur.com/JHYoeKm.png)

The above image highlights the kernel data structures that keep trackl of the virtual memory areas in a process. THe kernel maintains a distinct task structyure (`task_struct` in the source code) for each process in the system.

 The elements of the task structure either contain or point to all of the information that the kernel needs to run the process (e.g the PID, pointer to user stack, name of executable object file, program counter).

One of the entries in the task structure points to an `mm_struct` that characterizes the current state of the virtual memory. Two fields of interest are `pgd` which points to the base of the level 1 table, and `mmap` which points to a list of `vm_area_struct` (area structs) each of which characterizes an area of the current virtual address space. When the kernel runs this process, it stores `pgd` in the CR3 control register.

An area struct cotains the following fields(some more not mentioned maybe):

- `fvm_start`. Points to the beginning of the area.

- `vm_end`. Points to the end of the area.

- `vm_prot`. Desribes the read/write permissions for all pages in the area.

- `vn_flags`. Describes various things about the pages in the area.

- `vm_next`. Points to the next area struct in the list.


### Linux Page Fault Exception Handling

Suppose the MMU triggers a page fault while trying to translate some virtual address *A*. An exception transfers control to the kernel's page fault handler, which then performs the following steps:

1. Is virtual address *A* legal? Does it lie within an area defined by some area struct? To check this, the fault handler searches the list of area structs `vm_start` and `vm_end` to check if it lies within bounds of any struct. If it doesn't then the fault handler triggers a segmentation fault.(Note that iterating over area structs lineraly would be inefficent, so Linux imposes a tree on the list and performs a search on the tree) This situation is labled '1' in the below image.

2. Is the attempted memory access legal? Does the process have permissions to read/write/execute in this area? For example, a page fault resulting from a store instruction attempting to write to a read-only page of the code segment.

3. At this point, the kernel knows that the page fault resulted from a legal operation on a legal virtual address. It handles the fault by selecting a vctim page, swapping out the victim page if it is dirt, swapping in the new page and updating the page table. When the page fault handler returns the CPU restarts the faulting instruction.

### Memory Mapping

Linux intializes the contents of a virtual memory area by associating it with an *object* on disk a process known as *memory mapping*. 

Areas can be ampped to one of two types of objects:

1. *Regular file in the Linux file system*. An area can be mapped to a contiguous section of a regular disk file such as an executable object file. The file section is divided into page-size pieces with each piece containg the intial contents of a virtual page. Because of demand paging, none of these virtual pages are actually swapped into memory until the CPU first *touches* the page(i.e issues a virtual address that falls within that page's region of the address space).

2. *Anonymous File*. An area can also be mapped to an anonymous file created by the kernel containing all binary zeros. Pages in areas that are mapped to anonymous files are sometimes called *demand-zero pages*.

Once a virtual page is initalized, it is swapped back and forth between a special *swap file*(also known as *swap space* or the *swap area*) maintained by the kernel. At any point in time, the swap space bounds the total amount of vbirtual pages that can be allocated by the currently running processes.

Memory mapping is a way of integreting the virtual memory system into the conventional file system to provide a simple and efficent way to load programs and data into memory.

Process abstraction promises to provide each process with its own private virtual address space that is protected by unexpected writes and reads from other processes. However, some areas of memory need the identical read-only sections of code, such as libc, for example `printf`. It would be wasteful to duplicate a copy of the code for it; for each process.

Memory mapping provides a clean mechanims to do this.

An object can be mapped into an area of virtual memory as either a *shared object* or a *private object*.

If a process maps a shared object into an area of its virtual address space, then any writes the process makes to that area are visible to any other processes that have also mapped the shared object into their virtual memory(and also the original object on disk).

Change made to an area mapped to a private object are NOT visible to other processes, and any writes are **not** reflected back to the object on disk.


Private objects are mapped into virtual memory using *copy-on-write*. 

For each process that maps the private object, the page table entries for the corresponding private area are flagged as read-only and the area struct is flagged as *private copy-on-write*.

When the fault handler notices that the exception was caused by the process trying to write to a page in a private copy-on-write area, it creates a new copy of the page in physical memory updates the page table to point to the new copy and then restores write permissions to the page. 



#### `fork`

When the `fork` function is called by the *current process*, the kernel creates various data structures for the *new process* and assigns it a unique PID. To create the virtual memory for the new process, it creates exact copies of the current process's `mm_struct`, area structs and page tables. It flags each page in both processes as read-only and flags each area struct in both processes as private copy-on-write.

#### `execve`

Now we can look in depth to how the `execve` function really loads and executes programs. Suppose the program running in the current process makes the following call:
```c
execve("a.out", NULL, NULL);
```

Now the `execve` function loads and runs the `a.out` exectuable object file by the following steps:

1. *Delete existing user areas*. Delete the existing area structs in the user portion of the current process's virtual address.

2. *Map private areas*. Create new area structs for the code, data, bss and stack areas of the new program. All of these new areas are private copy-on-write. The bss is demand-zero mapped to an anonymous file whose size is contained in `a.out`. The stack and heap area are also demand-zero.

3. *Map shared areas*. If the `a.out` program was linked with shared objects such as the standard C library `libc.so`, then these objects are dynamically linked into the program, then mapped into the shared region of the user's virtual address space.

4. *Set the Program Counter*. The last thing `execve` does is set the PC in the current process's context to the entry point in the code area.

#### `mmap`

Linux processes can use the `mmap` function to create new areas of virtual memory and to map objects into these areas.

```c
void* mmap(void* start, size_t length, int prot, int flags, int fd, off_t offset);
```

The `mmap` function asks the kernel to create a new virtual memory area, prefereably one that starts at the address specified in the `start` argument.

The `prot` argument contains bits that describe the access permissions of the newly mapped virtual memory area.

The rest of the arguments define other properties you want for the new memory area.

### Dynamic Memory Allocation

It is certainly possible to use the low-level `mmap` and `munmap` functions to create and delete areas of virtual memory. Programmers typically find it more conveinent and portable to use a *dynamic memory allocator* when they need to acquire additional virtual memory at runtime.

A dynamic memory allocator maintains the process's heap.

For each process the kernel maintains a variable `brk` that points to the top of the heap.

An allocator maintains the heap as a collection of various-size *blocks*. Each block is a contiguous chunk of virtual memory that is either *allocated* or *free*. An allocated block is a contiguous chunk of virtual memory that has been explicitly reserved for use by the application.

Allocators usually conform to two styles:

- *Explicit Allocators* require the application to explicityl free any allocated blocks. For example the C stdlib provides the `malloc` package which is an explicit allocator. You allocate via calling `malloc` and free via calling `free`.

- *Implicit Allocators* require the allocator to detect when an allocated block is no longer being used by the program and then free the block. IMplicit allocators are known as **garbage collectors** and the process of freeing is known as *garbage collection*.



### `malloc` and `free` functions

`malloc` will allocate extra space automatically if needed to fufill alignment requirements.

Applications that want dynamic memory can use `calloc` a wrapper around the `malloc` function that intializes the memory to zero.

Applications that want to change the size of a previously allocated block can use the `realloc` function.

Dynamic memory allocators such as `malloc` can allocate or deallocate heap memory explicitly by using the `mmap` and `munmap` functions or they can use the `sbrk`.

```c
#include <unistd.h>

void* sbrk(intptr_t incr);
```
The `sbrk` function grows or shrinks the heap by adding `incr` to the kernel's `brk` pointer. If successful it returns the old value of `brk`, otherwise it returns -1 and sets `errno` to ENOMEM.

If `incr` is zero, then `sbrk` returns the current value of `brk`.

Calling `incr` with a negative `incr` is legal, but tricky as the return value (old value of `brk`) points to `abs(incr)` bytes past the new top of the heap.

Programs free allocated heap blocks by calling the `free` function.

```c
#include <stdlib.h>

void free(void* ptr)
```
The `ptr` argument must point to the beginning of an allocated block that was obtained from `malloc`, `calloc`, or `realloc`, (or wrappers?). If not then the behaviour of free is undefined (UB!), and since it returns nothing, this gives no indication that anything is wrong!

### Allocator requirements and Goals

Explicit allocators must operate within constraints:

- *Handling arbitrary request sequences*. An application can make an arbitrary sequence of allocate and free requests. The allocator must not make any assumptions about the order of such requests. And cannot assume that each allocate will be matched by a free and vice versa.

- *Making immediate responses to requests*. The allocator must respond immediately to allocate requests. Thus, the allocator is not allowed to reorder or buffer requests for performance or other reasons.

- *Using only th heap*. 

- *aligning blocks*. The allocator must align blocks in such a way that they can hold any data object.

- *Not modifying allocated blocks*. Allocators can only manipulate or change free blocks, they cannot modify or move blocks once they are allocated.


#### *Goal 1: Maximizing Throughput*

Given some sequence of *n* allocate and free requests:

*R<sub>0</sub>,R<sub>1</sub>,...,R<sub>k</sub>,...,R<sub>*n*-1</sub>*

We need to maximize the allocator's throughput of allocate & free requests.


#### *Goal 2: Maximizing memory utilization*

There are a number of ways to characterize how efficently an allocator uses the heap. One metric of measuring this, is *peak utilization*. If an application requests a block *p*  then the resulting allocated block a *payload* of *p* bytes. 

After request *R<sub>k</sub>* has completed, let the *aggregate payload*, denoted *P<sub>k</sub>* be the sum of the payloads of the currently allocated blocks, and let *H<sub>k</sub>* denote the current size of the heap.

Then, the peak utilization over the first *k* + 1 requests, denoted by *U<sub>k</sub>* is given by

*U<sub>k</sub>* = max<sub>*i<=k*</sub> P<sub>*i*</sub> / H<sub>*k*</sub>

The objective of the allocator then is to maximize the peak utilization *U<sub>n-1</sub> over the entire sequence.

### Fragmentation

The primary cause of poor heap utilization is **fragmentation**, which occurs when otherwise unused memory is not available to satisfy allocate requests.

There are two forms of fragmentation, internal and external.

*Internal fragmentation* occurs when an allocated block is large than the payload. It is simple to quanity, as it is just the sum of the difference between the sizes of the allocated blocks and their payloads.

*External fragmentatio* occurs when there *is* enough aggregate free memory to satisfy an allocate request but not single free block is large enough to handle the request.

External fragmentation is much more difficult to quanity than internal fragmentation because it depends not only on the pattern of previous requests and the allocator implementation but also the pattern of future requests.
If all the future allocate are four words or less, than there is no external fragmentation. But if there is any for more than four words, there IS external fragmentation.

### Implicit Free Lists

Any pratical allocator needs some data structure that allows it to distinguish block boundaries and to distinguish between allocated and free blocks. Most allocators embed this in the blocks themselves.

Often this will be implemented as a header, containing size and an allocated/free bit. `malloc` calls will return a pointer to the start of the payload. The end is padding to fit alighment constraints.

This organization is an *implicit free list* because the free blocks are linked implicitly by the size fields in the headers. The allocator can indirectly traverse the entire set of free blocks by traversing all of the blocks in the heap.

### Placing Allocated Blocks

When an application requests a block of *k* bytes, the allocator searches the free list for a free block that is large enough to hold the requested block. The manner of this is decided by *placement policy*.

Some examples are first fit, next fit and best fit.

*First fit* searches the free list from the beginning and chooses the first free block that fits. *Next fit*is the same as the first fit, but starts each search where the last ended. *Best fit* examines every free block and chooses the free block with the smallest size that fits.

### Splitting Free Blocks

Once the allocator has located a free block that fits, it must make another decision abouty how much of the free block to allocate.

One option is the entire free block, but that introduces internal fragmentation. If the placment policy tends to produce good fits then some additional fragmentation may be acceptable.
If the fit is not good the allocator will usually opt to split the free block into two parts.

What happens if the allocator is unable to find a fit for the requested block? One option is to try to create some larger free blocks by merging (coalescing) free blocks that are adjacent in memory.

### Coalescing Free Blocks

When the allocator frees an allocated block there might be other free blocks that are adjacent to ther newly freed block. Such adjacent free blocks can cause *false fragmentation*. Which is where there is a lot of available free memory chopped into small unusable free blocks.

To combat false fragmentation the allocator must merge adjacent free blocks in a process known as *coalescing*. This raises an important policy decision about when to perform coalescing. The allocator can opt for *immediate coalescing* by merging any adjacent blocks each time a block is freed. This can cause a form of thrashing where a block is repeatedly coalesced and then split soon thereafter.

It can alternatively opt for *deferred coalescing* by waiting to coalesce free blocks at some later time. For example, the allocator might defer coalescing until some allocation request fails, and then scan the entire heap coalescing all free blocks.

### Coalescing with Boundary Tags

Lets look at how an allocator implements coalescing.

We will refer to the block we want to free as the *current block*. Then, coalescing the next free block is straightforward and efficient; the header of the current block points to the heaer of the next block, which can then be checkd if it is free. If so, its size is added to the size of the current header and the blocks are coalesced in constant time.

There is a problem wheb walking the implicit free list, as you in order to coalesce the previous block, you have to remember the location of the previous block until we reach the current block.
Meaning, each call to `free` would require time where *h* is the size of the heap O(*H*).

*Boundary Tags* allows for constant time coalescing of the previous block. The idea, is to add a *footer*(this is the boundary tag) at the end of each block, where the footer is a replica of the header.

If each block includes this footer, then the allocator can determine the starting location and status of the previous block by insepecting its footer, which is always one word away from the start of the current block.

This can reduce large memory overheads, as you are doubling 'metadata' size by replicating the header information at the top and bottom of each block, but the size field in the footer of the previous block is only needed if the porevious block is *free*. If we were to store the allocated/free bit of the previous block in one of the excess low-order bits of the current block then the allocated blocks would not need footers and we could use that extra space for payload. Note that free blocks would still need foooters.

### Implementaing a simple allocator

Here we will implement a simple allocator, based on an implicit free list with immediate boundary-tag coalescing.

#### General Allocator Design

`mem_init` models the virtual memory available to the heap as a large double-word aligned array of bytes. The bytes between `mem_heap` and `mem_brk` represent allocated virtual memory. The bytes following `mem_brk` represent unallocated virtual memory. The allocator requests additional heap memory by calling the `mem_sbrk` function which has the same interface as standard `sbrk`.

```c
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>

#define MAX_HEAP (20*(1<<20)) /* 20 MB */




/* Private global variables */

static char *mem_heap; /* Points to first byte of heap */
static char *mem_brk; /* Points to the last byte of heap plus 1  */
static char *mem_max_addr; /* Max legal heap addr plus 1 */

/* Initalize the memory system model */
void mem_init(void)
{
	mem_heap = (char *)malloc(MAX_HEAP);	
	mem_brk = (char *)mem_heap;
	mem_max_addr = (char *)(mem_heap + MAX_HEAP);
}

/*
 * mem_sbrk is a simple model of the sbrk function.
 * Extends the heap by incr bytes and returns the start address of the new area. 
 * In this model, the heap cannot be shrunk
 */
void *mem_sbrk(int incr)
{
	char *old_brk = mem_brk;
	if ( (incr <0) || (mem_brk + incr) > mem_max_addr) {
		errno = ENOMEM;
		fprintf(stderr, "ERROR: mem_sbrk failed. Ran out of memory. \n");
		return (void *)-1;
	}
	mem_brk += incr;
	return (void *)old_brk;

}

```

The allocator uses the block format with a min. block size of 16 bytes, with the free list organized as an implicit free list. The first word is an unsued padding word aligned to a double-word boundary. The padding is followed by a special *prologue block* which is an 8-byte allocated block consisting of only a header and a footer. The prologue block is created during initialization and is never freed. Following the prologue block are zero or more regular blocks that are created by calls to `malloc` or `free`. The heap always ends with a special *epilogue block* which is a zero-size allocated block that consists of only a header. The prologue and epilogue blocks are tricks that elimate the edge conditions during coalescing. 

The allocator uses a single private (`static`) global variable (`heap_listp`) that always points to the prologue block.

#### Basic Constants and Macros for Manipulating the Free List

```c
/* Basic constants and macros */
#define WSIZE 4 /* Word and header/footer size (bytes) */
#define DSIZE 8 /* Double word size (bytes) */
#define CHUNKSIZE (1<<12) /* Extend heap by this amount (bytes) */

#define MAX(x, y) ((x) > (y)? (x) : (y))
```

Above shows some basic constants and macros to be used throughout the allocator code.

The first intialize some basic size constants, for a word and double word; and the size of the intial free block and the default size for exapnding the heap - `CHUNKSIZE`.

Manipulating the headers and footers in the free list can be troublsome as it demands extensive use of casting and pointer arithmetic.  So we define a set of macros to help with this.

```c
/* Pack a size and and allocated bit into a word */
#define PACK(size, alloc)  ((size) | (alloc))

/* Read and write a word at address p */
#define GET(p)		(*unsigned int *)(p))
#define PUT(p, val)	(*(unsigned int*)(P) = (val))

/* Read the size and allocated fields from address p */
#define GET_SIZE(p)	(GET(p) & ~0x7)
#define GET_ALLOC(p)	(GET(p) & 0x1)

/* Given block ptr bp, compute address of its header and footer */
#define HDRP(bp)	((char *)(bp) - WSIZE)
#define FTRP(bp)	((char *)(bp) + GET_SIZE(HDRP(bp)) - DSIZE)

/* Given block ptr bp, compute address of next and previous blocks */
#define NEXT_BLKP(bp)	((char *)(bp) + GET_SIZE((char*)(bp) - WSIZE))
#define PREV_BLKP(bp)	((char *)(bp) - GET_SIZE((char*)(bp) - DSIZE))
```
The `PACK` macro combines a size and an allocate bit and returns a value that can be stored in a header or footer.

The `GET_SIZE` and `GET_ALLOC` macros return the size and allocated bit, respectively from a header or footer at address `p`. The remiaining macros operate on *block pointers* (`bp`).

The `HDRP` and `FTRP` macros returns pointers to the block header and footer. The `NEXT_BLKP` and `PREV_BLKP` return block pointers of the next and previous blocks.


#### Creating the Intial Free List

Before calling our `malloc` and `free` implementations, we must init the heap by calling `mm_init`, which returns four words from memory and intializes them to create the empty free list. It then calls the `extend_heap` function, which extends the heap by `CHUNKSIZE` bytes and creates the intial free block.


`mm_init`:
```c
int mm_init(void) 
{
	void* heap_listp = NULL;	
	/* Create the intial, empty heap */
	if ((heap_listp = mem_sbrk(4*WSIZE)) == (void*)-1)
		return -1;
	PUT(heap_listp, 0); /* Alignment padding */
	PUT(heap_listp + (1*WSIZE), PACK(DSIZE, 1)); /* Prologue header */
	PUT(heap_listp + (2*WSIZE), PACK(DSIZE, 1)); /* Prologue header */
	PUT(heap_listp + (3*WSIZE), PACK(0, 1)); /* Prologue header */
	heap_listp += (2*WSIZE);

	/* Extend the empty heap with a free block of CHUNKSIZE bytes */
	if (extend_heap(CHUNKSIZE/WSIZE) == NULL)
		return -1;
	return 0;

}
```

Which calls `extend_heap`:
```c
static void* extend_heap(size_t words)
{
	char* bp;
	size_t size;

	/* Allocate an even number of words to maintain alignment */
	size = (words % 2) ? (words + 1) * WSIZE : words * WSIZE;
	if ((long)(bp = mem_sbrk(size)) == -1)
			return NULL;
	/* Initalize free block header/footer and the epilogue header */
	PUT(HDRP(bp), PACK(size, 0)); /* Free block header */
	PUT(FTRP(bp), PACK(size, 0)); /* Free block footer */
	PUT(HDRP(NEXT_BLKP(bp)), PACK(0 , 1)); /* New epilogue header */

	/* Coalesce if the previous block was free */
	return coalesce(bp);
}
```
To maintain alignment, the `extend_heap` rounds up the requested size to the nearest two words multiple.

### Explicit Free Lists

The implicit free list provides a simple way to introduce some basic allocator concepts. However, because block allocation time is linear in the total number of heap blocks, the implicit free list is not appriopiate for a general-purpose allocator.

A better appproach is to organize the free blocks into some form of explicit data structure.

Since we do not need the body of a free block, the pointers that implement the data structure can be stored within the bodies of the free blocks. For example, the heap can be organized as a doubly linked free list including a `pred`(predecessor) and `succ`(successor) pointer in each block.

Using a doubly linked list instead of an implicit free list reduces the first-fit allocation time from linear in the total number of blocks to linera in the number of *free* blocks. However, the time to free a block can be either linear or constant , depending on the policy we choose for ordering the blocks in the free list.

One approach is to maintain the list in *last-in first-out* (LIFO) order by inserting newly freed blocks at the beginning of the list. With a LIFO ordering and a first-fit placement policy, the allocator inspects the most recently used blocks first. In this case, freeing a block can be performed in constant time. If boundary tags are used then coalescing can also be performed in constant time.

Another approach is to maintain the list in *address order* where the address of each block in the list is less than the address of its successor.  In this case, freeing a block requires a linerar-time search to locate the appriopiate predecessor. The LIFO-ordered first fit apporoaching the utilization of best fit.

A disadvantage of explicit lists in general is that free blocks must be large enough to contain all of the necessary pointers, as well as the header and possibly a footer. This results in a larger minimum block size and increases the potential for internal fragmentation.

### Segregated Free Lists

An allocator that uses a single linked list of blocks requires time linear int he number of free blocks to allocate a block. (where *n* is the number of free blocks *O(n)*.)

A popular approach for reducing the allocation time is *segregated storage* where you maintain multiple free lists where each list holds blocks that are roughly the same size. The general idea is to partition the set of all possible block sizes into equivalence classes called *size classes*. We could define size classes by partioning block sizes by powers of 2, or assign small blocks to their own size classes and parition large blocks by powers of 2.

The allocator maintains an array of free lists with one free list per size class, ordered by increasing size.

When an allocator needs a block of size *n*, it searches the appropiate free list. If it cannot find a block that fits, it searches the next list and so on.

#### Simple Segregated Storage

With simple segregated storage, the free list for each size class contains same-size blocks, each the size of the largest element of the size class.If some size class is defined as {17-32} then the free list for that class consists entirely of blocks of size 32.

To allocate a block of some givern size we chgeck the appriopiate free list, we allocate the first block in its entirety. Free blocks are never split to satisfy allocation requests. If the list is empty the allocator requests a fixed-size chunk of additional memory from the operating system , divies the chunk into equal-szie and links the blocks together to formt he new free list. To free a block the allocator simply inserts the block at the front of the appriopiate free list.

#### Segregated Fits

With this approach, the allocator maintains an array of free lists. Each free list in associated with a size class and organised as some kind of explicit or implicit list.

To allocate a block, we determine the size class of the request and do a first-fit serach of the appriopiate free list for the block that fits. If we find one, we (optionally) split it and insert the fragment in the appropriate free list. If we cannot fin a block that fits, then we search the free list for the next larger size class, repeating until a hit. If none of the free lists yiled a block that fits, we request additional heap memory from the OS, and allocate the block out of this new heap memory, and place the remainder in an appriopiate free list.

This is a popular choice with allocators such as the GNU `malloc` package in libc.

#### Buddy Systems

A *buddy system* is a special case of segregated fits where each size class is a power of 2.

The basic idea is that given a heap of 2<sup>*m*</sup> words, we maintain a seperate free list for each block size 2<sup>*k*</sup>, where 0 <= *k* <= *m*. Requested block sizes are rounded to the nearest power of 2.

To allocate a block of size 2<sup>*k*</sup> we find the first available block of size 2<sup>*j*</sup> such that l <= j <= m. If *j* = *k*, then we are done. Otherwise, we recursively split the block in half until *j* = *k*. As we perform this splitting, each remaining half (knwon as a *buddy*) is placed on the appropriate free list. To free a block of size 2<sup>*k*</sup>, we continue coalescing with the free buddies. When we encounter an allocated buddy, we stop the coalescing. 

A key fact about buddy systems is that, given the address and size of a block, it is easy to compute the address of its buddy.For example, a block of size 32 bytes with address `xxx...x00000` has its buddy at address `xxx...x10000`. In other words, tha ddress of a block and its buddy differ in exactly one bit position.

An advantage of the buddy system allocator is its fast searching and coalescing. 

The major disadvantage is that the power-of-2 requirement on the block size can cause significant internal fragmentation. However, for certain application-specific workloads where the block sizes are known in advance to be powers of 2, buddy system allocators have a certain appeal.

### Garbage Collection

A *garbage collector* is dynamic storage allocator that autmatically frees allocated blocks that are no longer needed by the program. The process of automatically reclaming allocated heap storage is known as *garbage collection*. 

#### Garbage Collector Basics 

A garbage collector view memory as a directed *reachability graph* of the form shown below.
![](https://i.imgur.com/fhNYH0z.png)

The nodes of the graph are paritioned into a set of *root nodes* and a set of *heap nodes*. Each heap node corresponds to an allocated block in the heap. A direct edge *p* -> *q* means that some location in block *p* points to some location in block *q*. Root nodes correspond locations not in the heap that contain pointers into the heap. These locations can be registers, variables on the stack, or global variables in the r/w area of virtual memory.

We say that node *p* is reachable if there exists a directed path from any root node to *p*. The role of the garbage collector is to maintain some representation of the reachability graph and periodically reclaim the unreachable nodes by freeing them and returning them to the free list.


#### Mark&Sweep Garbage Collectors

A Mark&Sweep  garbage collector consists of a *mark phase*, which marks all reachable and allocated descendents of the root nodes, followed by a *sweep phase*, which free each unmarked allocated block. Typically, one of the spare low-order bits in the block header is used to indicate whether a block is marked or not.


### Common Memory-Related Bugs in C Programs

#### Derefencing Bad Pointers

```c
scanf("%d, &val);
```
This is a common use to read an input to a variable from stdin. You may make an error and not use the address, instead passing `val` by value:
```c
scanf("%d", val);
```
Here, `scanf` will attempt to read the value of `val` as an address and write the input to the bytes passed by val. Likely causing a segfault.


#### Reading Uninitalized Memory

While bss memory locations (such as uninitalized global C variables) are always initalized to zeros by the loader, this is not true for heap memory. A common error is to assume that heap memory is initilized to zero:
```c

int i, j;

int *y = (int *)malloc(n * sizeof(int));

for (i = 0; i < n; i++)
	for (j = 0;j < n; j++)
		y[i] += A[i][j] * x[j];
return y;
```
In this example the programmer has incorrectly assumed that vector *y* has been initialized to zero. A correct implementation would explicitly zero y[i] or use `calloc`.

## System-Level I/O

### Unix I/O

A Linux *file* is a sequence of *m* bytes:

B<sup>0</sup>,B<sup>1</sup>,...,B<sup>k</sup>,...,B<sup>m - 1</sup>

All I/O devices such as networks, disks and terminals are modeled as files, and all input and output is performed by reading and writing the appropriate file.

This mapping of devices to file allows the Linux kernel to export a simple, low-level application interface known as *Unix I/O* that enables all input and output to be performed in a uniform and consistent way:

- **Opening Files**. An application annouces its intention to access an I/O device by asking the kernel to **open** the corresponding file. The kernel returns a nonnegative integer, called a *descriptor*, that identifies the file in all subsequent operations on the file. The kernel keeps track of all information about the open file. The application only keeps track of the descriptor. Each process created by a Linux shell begins with three open files `stdin` (descriptor 0), `stdout` (descriptor 1) and `stderr` (descriptor 2).

- **Changing the current file position**. The kernel maintains a file position *k*, intially 0, for each open file. The file position is a byte offset from the beginning of a file. An application can set the current file position *k* by performing a *seek* operation.
- **Reading and Writing Files**. A *read* operation copies *n* > 0 bytes from a file to memory, starting at the current file position *k* and then incrementing *k* by *n*. Given a file with a size of *m* bytes, performing a read operation when *k* >= *m* triggers a condition known as *end-of-file*(EOF), which can be detected by the application. There is no explicit 'EOF character' at the end of file. Similarly, a *wrute* operation copies *n* > 0 bytes from memory to a file, starting at the current file position *k* and then updating *k*.

- **Closing Files**. When an application has finished accessing a file, it informs the kernel by asking it to *close* the file. The kernel responds by freeing the data structures it created when the file was opened and restoring the descriptor to a pool of available descriptors. When a process terminates for any reason, the kernel closes all open files and frees their memory resources.

### Files

Each Linux file has a *type* that indicates its role in the system:

- A *regular file* contains arbitrary data. Applications will often distinguish between text files (files containing only ASCII or unicode characters/bytes) and binary files, which is everything else.

- A *directory* is a file consisting of an array of *links* where each links maps a *filename* to a file. Each directory contains at least `.` (a link to the directory itself) and `..` which is a link to the parent directory.

- A *socket* is a file that is used to communicate with another process across a network.

Other file types include *named pipes*, *symbolic links*, and *character* and *block devices*.

The Linux kernel organizes all files in a single directory hierarchy anchored by the root directory - `/`. 

As a part of its context, each process has a *current working directory* that identifies its current location in the directory hierarchy. You can change the shell's current working directory with `cd`.

### Opening and Closing Files

A process opens an existing file or creates a new file by calling `open`:
```c
#include <sys/types.h>
#include <sys/stat.h>
#include <fnctl.h>

int open(char* filename, int flags, mode_t mode);
```
`open` returns new file descriptor if OK, `-1` on error.

The `open` function converts a `filename` to a file descriptor (often abbreivated to `fd` as a var name for the ret value of `open`) and returns the descriptor number.

The descriptor returned is always the smallest descriptor that is not currently open in the process.

The `flags` argument indicates how the process intends to access the file:

- `O_RDONLY` - Reading Only.
- `O_WRONLY` - Writing Only.
- `O_RDWR` - Reading and writing


`flags` can also be be ORed with one or more bit masks that include additional instructions for writing:

- `O_CREAT`. If the file doesn't exist, then create a *truncated* (empty) version of it.
- `O_TRUNC`. If the file already exists, then truncate it.
- `O_APPEND`. Before the write operation set the file position(*k* from earlier) to the end of the file.

The `mode` argument specifies the access permission bits of new files. Using the Bit masks and ORing them allows multiple settings to be defined at once.

Below are the access permission bits symbolic names as defined in `sys/stat.h`:

![](https://i.imgur.com/7DLGPB1.png)

As part of its context, each process has a `umask` that is set by calling the `umask` function.
When a process creates a new file by calling `open`, with some `mode` argument, then the access permission bits of the file are set to `mode & ~umask`

A process closes an open file by calling the `close` function:

```c
#include <unistd.h>

int close(int fd);
```
### Reading and Writing Files

Applications perform input and output by calling the `read` and `write` functions:

```c
#include <unistd.h>

ssize_t read(int fd, void* buf, size_t n);

ssize_t write(int fd, const void* buf, size_t n);
```

The `read` function copies at most `n` bytes from the current file position of descriptor `fd` to memory location `buf`. A return value of -1 indicates an error, a return value of 0 indicates EOF. 

The `write` function copies at most `n` bytes from memory location `buf` to to the current file position of descriptor `fd`. (You can edit file position via the `lseek` function)

In some situations, `read` and `write` transfer fewer bytes than the application requests. Such *short counts* do **not** indicate an error. They can occur for a number of reasons:

- *Encountering EOF on reads*. Say we are reading a 70 byte file in 50-byte chunks. The first `read` returns as normal, reading 50 bytes into `buf`. The next can only read 20 bytes, so does so into buf. Only on the next read after, it reads 0 and returns EOF.

- *Reading text lines from a terminal*. If the open file is associated with a terminal (i.e keyboard or display) then each `read` will transfer one text line at a time, returning a short count equal to the size of the text line.

- *Reading and writing network sockets*. If the open file corresponds to a network sockets, then internal buffering constraints and long network delays can cause `read` and `write` to return short counts. Short counts can also occur on pipes.
 
 In practice, you will never occur short counts when you read from disk except on EOF, and you will never encounter short counts when you *write* to disk files.However, if you want to build robust(reliable) network applcaitions such as Web servers, then you must deal with short counts by reapeatedly calling `read` and `write` until all requested bytes have been transferred.

### Reading File Metadata

An application can retreive information about a file by calling the `stat` and `fstat` functions:

```c
#include <unistd.h>
#include <sys/stat.h>

int stat(const char* filename, struct stat *buf);
int fstat(int fd, struct stat *buf);
```

The `stat` function takes as input a filename and fills in the members of a `stat` structure.

`st->st_size` contains the file size in bytes.

`st->st_mode` contains the file permission bits and the file type.

Linux defines macro predicates in `sys/stat.h` for determining the file type from the `st_mode` member

- `S_ISREG(m)`. Is this regular file?
- `S_ISDIR(m)`. Is this a directory file?
- `S_ISSOCK(m)`. Is this a network socket?

### Reading Directory Contents

Aplications can read the contents of a directory with the `readdir` family of functions:

```c
#include <sys/types.h>
#include <dirent.h>

DIR *opendir(const char* name);
```

The `opendir` function takes a pathname and returns pointer to a *directory stream*. A stream is an abstraction for an ordered list of items, in this case a list of directory entries.
```c
#include <dirent.h>

struct dirent* readdir(DIR* dirp);
```

Each call to `readdir` returns a pointer to the next directory entry in the stream `dirp` or NULL if there are no more entries. Each direcotry entry is a strucutre of the form:
```c
struct dirent {
	ino_t d_ino;	 /* inode number */
	char d_name[256];/* Filename     */
};
```

Some versions of Linux include other structure members.

On error, `readdir` returns NULL and sets `errno`. The only way to distinguish an error from the end-of-stream condition is to check if `errno` has been modified since the call to `readdir`.
```
#include <dirent.h>

int closedir(DIR* dirp);
```

The `closedir` functions closes the stream and frees up any of its resources. 

### Sharing Files

The kernel represents open fioles using three related data structures:

- *Descriptor table*. Each process has its own descriptor table, whose entries are indexed by the process's open file descriptors. Each open descriptor points to an entry in the *file table*.

- *File table*. The set of open files is represented by a file table that is shared by all processes. Each file table entry consists of: the current file position, a reference count of the number of descriptor entries that currently point to it, and a pointer to an entry in the *v-node table*. Closing a descriptor decrements the reference count in the associated file table entry. The kernel will not delete the file table entry until its reference count is zero.

- *v-node table*. Like the file table, the v-node table is shared by all processes. Each entry contains most of the information in the `stat` structure including the `st_mode` and `st_size` members.  

### I/O Redirection

Linux shells provide *I/O redirection* operators that allow uysers to associate standard input and output with disk files. For example, typing:
```bash
$ ls > foo.txt
```
causes the shell to load and execute the `ls` program with stdout redirected to disk file `foo.txt`.

How does I/O redirection work? One way is using the `dup2` function:
```c
#include <unistd.h>

int dup2(int oldfd, int newfd);
```
The `dup2` function copies descriptor table entry `oldfd` to descriptor table entry `newfd`, overwriting the previous contents of it.

The `FILE` data type as defined in `stdio.h` is an abstraction for a file descriptor and a stream buffer. The purpose of a stream buffer is to minimize the number of expensive Linux I?O system calls. For example, if we are calling `getc` to return a single char, from a file - we will have to perform a `read` syscall for each char. Instead we can just grab one char from the stream buffer, after reading many more bytes into it from the disk file.



## Network Programming

To a host, a network is just another I/O device that serves as a source and sink for data.

Linux provides the `htonl` function and `ntohl` function which convert 32-bit integers from host byte order to network byte order and vice versa, as network byte order is big-endian.

IP addresses are typically presented to humans in a form known as *dotted-decimal notation* where each byte is represented by its decimal and seperated from the other bytes by a period. For example, `128.2.194.242` is the dotted-decimal representation of the address `0x8002c2f2`. 

You can convert back and forht between IP address and dotted-decimal strings using the functions `inet_pton` and `inet_ntop`:
```c
#include <arpa/inet.h>

int inet_pton(AF_INET, const char* src, void* dst);

const char* inet_ntop(AF_INET, const void* src, char* dst, socklen_t size);
```
Here the 'n' stands for *network* and the 'p' stands for *presentation*.

They can manipulate either 32-bit IPv4 addresses (`AF_INET`) or 128-bit IPv6 addresses (`AF_INET6`).

`inet_pton` converts a dotted-decimal string (`src`) to a binary IP address in network byte order (`dst`).

### The Sockets Interface

*note the `_in` suffix is short for internet, not input.*

```c
/* IP socket address structure */
struct sockaddr_in {
	uint16_t	sin_family;  /* Protocol family (always AF_INET)  */
	uint16_t	sin_port;    /* Port number in network byte order */
	struct in_addr  sin_addr;    /* Protocol family (always AF_INET)  */
	unsigned char   sin_zero[8]; /* Pad to sizeof(struct sockaddr)    */
};

/* Generic socket address structure (for connect, bind and accept) */
struct sockaddr {
	uint16_t  sa_family; /* Protocol family */
	char      sa_data[14];
};
```

Internet socket address are stored in 16-byte structures having the type `sockaddr_in`.

For internet applications, the `sin_family` field is `AF_INET` the `sin_port` field is a 16-bit port number and the `sin_addr` field contains a 32-bit IP address address.

The IP address and port number are always stored in network (big-endian) byte order.

#### `socket`

Clients and servers use the `socket` function to create a *socket descriptor*:

```c
#include <sys/types.h>
#include <sys/socket.h>

int socket(int domain, int type, int protocol);
```
If we wanted the socket to be the end point for a connection then we could call `socket` with the following hardcoded arguments:
```c
clientfd = socket(AF_INET, SOCK_STREAM, 0);
```
Where AF_INET indicates that we are using 32-bit IP addresses and `SOCK_STREAM` indicates that the socket will be an endpoint for a connection. However, the best practice is to use the `getaddrinfo` with the `socket` function.

The `clientfd` descriptor return by `socket` is only partially opened and cannot yet be used for reading and writing. Howe finish opening the socket depends on whether we are a client or a server.

#### `connect`

A client establishes a connection with a server by calling the `connect` function:
```c
#include <sys/socket.h>

int connect(int clientfd, const struct sockaddr* addr, socklen_t addrlen);
```

The `connect` function attempts to establish an Internet connection with the server at socket address `addr` where `addrlen` is `sizeof(sock_in)`. The `connect` function blocks until the connection is either successfully established or an error occurs. If successful the `clientfd` descriptor is now ready for reading and writing and the reuslting connection is characterized by the socket pair
```c
(x:y, addr.sin_addr:addr.sin_port)
```
where `x` is the client's IP address and `y` is the ephermal port that uniquely identifies the client process on the client host.

#### `bind`, `listen` and `accept`

The remaining sockets fucntions - `bind`, `listen` and `accept` are used by servers to establish connections with clients.

```c
#include <sys/socket.h>

int bind(int sockfd, const struct sockaddr* addr, socklen_t addrlen);

int listen(int sockfd, int backlog);

int accept(int listenfd, struct sockaddr* addr, int* addrlen);
```

The `bind` function asks the kernel to associate the server's socket address in `adddr` with the socket descriptor `sockfd`. The `addrlen` argument is  `sizeof(sockaddr_in)`.

Clients are active entities that initiate connection requests. Servers are passive entities that wait for connection.

By default the kernel assumes that a descriptor by the `socket` function corresponds to an *active socket* that will live on the client end of a connection.

A server calls the `listen` function to tell the kernel that the descriptor will be used by a server instead of a client.

The `listen` function converts `sockfd` from an active socket ot a *listening socket* that can accept connection requests that the kernel should queue up before it starts to refuse requests. 

Servers wait for connection requests from clients by calling the `accept` function.

The `accept` function watis for a connection request from a client to arrive on the listening descriptor `listenfd` then fills in the client's socket address in `addr` and returns a *connected descriptor* that can be used to communicate with client using Unix I/O functions.

### Host and Service Conversion

`getaddrinfo` and `getnameinfo` for converting back and forth between binary socket address structures and the string representations, host addresses, service names and port numbers.

#### `getaddrinfo`

The `getaddrinfo` function converts string representations of hostnames, host addresses, service names and port numbers into socket address structures. It is the modern replacement for the obsolete `gethostbyname` and `getservbyname` fucntions.

```c
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>

int getaddrinfo(const char* host, const char* service, const struct addrinfo* hints, struct addrinfo **result);

void freeaddrinfo(struct addrinfo* result);

const char* gai_strerror(int errcode);
```
Given `host` and `service`, (the two components of a socket address) `getaddrinfo` returns a `result` that points to a linked list of `addrinfo` structures each of which points to a socket address sstructure that corresponds to `host` and `service`. See below for the returned data structure in diagram form.

![](https://i.imgur.com/NzFgGAV.png)

After the client receives this list, it walks the list, trying each socket address until the calls to `socket` and `connect` succeed and the connection is established.

Similarly, a server tries each socket address on the list until the calls to `socket` and `bind` succeed, and the descriptor is bound to a valid socket address.

In order to prevent memory leaks, the application must eventually free the list by calling `freeaddrinfo`. If `getaddrinfo` returns a nonzero error code, we can call `gai_strerror` which will convert the code to a message string.

The `host` argument to `getaddrinfo` can be either a domain name or a numeric address such as a dotted-decimal IP address.

The `service` argument can be either a service name such as `http` or a decimal port number. If we are not interested in converting the hostname to an address we can set `host` to NULL. The same holds for `service`, but at least one must be specified.
 
The optional `hints` argument is an `addrinfo` structure() that provides finer control over the list of of socket addresses that `getaddrinfo` returns. When passed as a `hints` argument, only `ai_family`, `ai_socktype`, `ai_protocol` and `ai_flags` fields can be set. In pracetice you will use `memset` and zero the struct, and then set a few selected fields:

- By default, `getaddrinfo` can return both IPv4 and IPv6 socket addresses. Setting `ai_family` restricts the list to either v4 or v6 via the `AF_INET` & `AF_INET6` macros(?).

- By default, for each unqiue address associated with host, the `getaddrinfo` function can return up to three `addrinfo` structures, each with a different `ai_socktype` field: one for connections, one for datagrams and one for row sockets. Setting `ai_socktype` to `SOCK_STREAM` restricts the list to at most one `addrinfo` structure for each unique address, one whose socket address can be used as the end point of a connection.

- The `ai_flags` field is a bit mask that modifies default behaviour. As usual, we can create custom settings via ORing what we want.

	- `AI_ADDRCONFIG` - This asks `getaddrinfo` to return IPv4 addresses only if the local host is configured for IPv4, same for IPv6.
	- `AI_CANONNAME` - By default this field is NULL. If this flag is set, then it tells `getaddrinfo` to point the `ai_canonname` field in the first `addrinfo` structure in the list to the canonical(official) name of `host`.
	- `AI_NUMERICSERV` - By default `service` can be a service name or a port number. This forces the `service` argument to be a port number.
	- `AI_PASSIVE` - By default, `getaddrinfo` returns socket addresses that can be used by clients as active sockets in calls to `connect`. This flag instructs it to return socket addresses that can be used by servers as listening sockets. In this case, the `host` argument should be NULL. The address field in the resulting socket address structure(s) will be the *wildcard address*, which tells the kernel that this server will accept requests to any of the IP addresses for this host.

#### `getnameinfo`

The `getnameinfo` funciton is the inverse of `getaddrinfo`. It converts a socket address structure to the corresponding host and service name strings. It is the modern replacement for the obsolete `gethostbyaddr` and `getservbyport` functions.

```c
#include <sys/socket.h>
#include <netdb.h>

int getnameinfo(const struct sockaddr* sa, socketlen_t salen, char* host, size_t hostlen, char* service, size_t servlen, int flags);
```
The arguments are easy to understand, simply refer to above, and infer from context.

The `flags` argument is a bit mask that modifies the default behaviour:

- `NI_NUMERICHOST` - By default `getnameinfo` tries to return a domain in `host`. Setting this flag will cause it to return a numeric address string instead.
- `NI_NUMERICSERV` - By defualt `getnameinfo` will look int `/etc/services` and if possible, return a serivce name insead of a port number. Setting this flag forces it to skip the lookup and simply return the port number.

The `getaddrinfo` functions and the sockets inferface are often wrapped in helper functions via `open_clientfd` and `open_listenfd`

## Concurrent Programming

### Concurrent Programming with I/O Multiplexing

The `pselect`, `select` and `FD_...` macros in the `sys/select.h` header file define certain functions for observing file descriptors.

```c
#include <sys/select.h>

int pselect(int nfds, fd_set *restrict readfds,
           fd_set *restrict writefds, fd_set *restrict errorfds,
           const struct timespec *restrict timeout,
           const sigset_t *restrict sigmask);

int select(int nfds, fd_set *restrict readfds,
           fd_set *restrict writefds, fd_set *restrict errorfds,
           struct timeval *restrict timeout);

void FD_CLR(int fd, fd_set *fdset);

int FD_ISSET(int fd, fd_set *fdset);

void FD_SET(int fd, fd_set *fdset);

void FD_ZERO(fd_set *fdset);
```

The `select` function manipulates sets of type `fd_set`, which are known as *descriptor stes*. Logically we can think of a descriptor set as a bit vector of size n.

Each bit<sub>k</sub> corresponds to descriptor *k*. Descriptor *k* is a membor of the descriptor set if and only if b<sub>*k*</sub> = 1. 

You are only allowed to do three things with descriptor sets:

1. Allocate them
2. Assign one variable of this type to another
3. Modify and inspect ehm using the `FD_ZERO`, `FD_SET`, `FD_CLR` and `FD_ISSET` macros.

The `select` function takes inputs, we will look at two in particular - a descriptor set `fdset` called the *read set* and the carinality (`n`) of the read set. The `select` function blocks until at least one descriptor in the read set is ready for reading.

A descriptor *k* *for reading* only if a request to read 1 byte from that descriptor would not block.

As a side effect `select` modifies the `fd_set` pointed to by argument `fdset` to indicate a subset of the read set called *ready set*, consisting of the descriptors in the read set that are ready for reading.

### Concurrent Programming with Threads

A *thread* is a logical flow that runs in the context of a process. Modern systems allow us to write programs that have multiple threads running concurrently in a single process. The threads are scheduled atuomatically by the kernel. Each threads has its own *thread context* including a unique integer *thread ID (TID)*, stack, stack pointer, program counter, general-purpose registers and condition codes.

#### Thread Execution Model

Consider the example for multiple threads below.

![](https://i.imgur.com/FFcuL3h.png)

Each process begins life as a single thread called the *main thread*. At some point the main thread creates a *peer thread*, and from this point in time the two threads run concurrently. 

Eventually, control passes to the peer thread via a context switch either because the main thread executes a slow syscall such as `read` or `sleep` or because it is interrupted by the system's interval timer. The peer thread executes for a while before control passes back to the main thread, and so on.

### Posix Threads

Posix Threads (Pthreads) is a standard maniuplating threads from C programs.

```c
#include <pthread.h>

typedef void*(func)(void*);

int pthread_create(pthread_t* tid, pthread_attr_t* attr, func* f, void* arg);

pthread_t pthread_self(void);

void pthread_exit(void* thread_return);

int pthread_cancel(pthread_t tid);

int pthread_join(pthread_t tid, void** thread_return);
```

The `pthread_create` function creates a new thread and runs the *thread routine* `f` in the context of the new thread and with an input argument of `arg`. The `attr` argument can be used to change the default argument can be used to change the default attributes of the newly created thread.

When `pthread_create` returns argument `tid` contains the ID of the newly created thread. The new thread can determine its own thread ID by calling the `pthread_self` function.

A thread terminates in one of the following ways:

- A thread terminates *implicitly* when its top-level threads routine returns.
- The terminates *explicitly* by calling the `pthread_exit` function. If the main thread calls `pthread_exit`, it waits for all other peer threads to terminate and then terminates the main thread and the entire process with a return value of `thread_return`.
- Another peer thread either calls the Linux `exit` function, terminating self and and all threads associated with the process. 
- Another peer thread terminates the current thread by calling the `pthread_cancel` with the ID of the current thread.

Threads wait for other threads to terminate by calling the `pthread_join` function. The `pthread_join` function blocks until `tid` terminates, assigns the generic `(void*)` pointer returned by the thread routine to the location pointed to by `thread_return` and reaps any memory resources held by the terminated thread.

At any point in time, a thread is *joinable* or *detached*. A joinable thread can be reaped and killed by other threads. Its memory resources are not freed until it is reaped by another thread.

A detached thread cannot be reaped or killed by other threads. Its memory reources are freed automatically by the system when it terminates.

`pthread_once` allows you to intialize the state associated with a routine.

Variables are shared cross-threads only if multiple threads reference some instance of the variable.

A pool of concurrent threads run in the context of a process. 

#### Mapping Variables to Memory

Variables in threaded C memory are mapped to virtual memory according to their storage classes:

- *Global variables*. A *global variable* is any variable declared outside of a function. At any time the read/write area of virtual memory contains exactly one instance of each global variable that can be referenced by any thread.

- *Local automatic variable*. A *local automatic variable* is one that is declared inside a function without the `static` attribute. At run time, each thread's stack contains its own instances of any local automatic variables. This is true even if multiple threads execute the same thread routine.

- *Local static variables*. A *local static variable* is one that is declared inside a function with the `static` attribute. As with global variables, the read/write area of virtual memory contains exactly one instance of each local static variable declared in a program.

#### Synchronizing Threads with Semaphores

Shared variables can be highly useful, especially for performance-critical programs, however thye introduce a nasty class of new bugs - *synchronization errors*. 

Consider the following x86 code, which when implemented on a processor will cause synchronization errors(note we are considering the proceesor here to be a single-core for the sake of simplicitly):

```nasm
	movq (%rdi), %rcx
	testq %rcx, %rcx
	jle .L2
	movl $), %eax
.L3:
	movq cnt(%rip), %rdx
	addq %eax
	movq %eax, cnt(%rip)
	addq $1, %rax
	cmpq %rcx, %rax
	jne .L3
L2:
```

Here we can see that if two threads ran this code at similar times, there is a likely chance one thread will execute `movq cnt(%rip)` and load the variable into its stack, while another thread could be executing its `movq` just after incrementing its own version of the variable. This would cause potentially disastrous bugs.

Programming around these bugs is extremely hard, so a variety of techniques have arisen to do so. We need to control execution trajectories through time, we can visualize this through a progress graph, as shown below:

![](https://i.imgur.com/sKVOwtO.png)

The 'unsafe' region is places in code where one variable has been changed in one scope, but not updated in the place where the other thread(s) will pull that variables value from.

#### Semaphores

Edsger Dijkstra proposed a solution to the problem of synchronizing different execution threads based on a special type of variable called a *semaphore*. A semaphore, *s*, is a global variable with a nonnegative integere value that can only be manipulating by two special operations called *P* and *V*:

 - *P(s)*: If *s* is nonzero, then *P* decrements *s* and returns immediately. If *s* is zero, then suspend the thread until *s* becomes nonzero and the thread is restarted by a *V* operation. AFter restarting the *P* operation decrements *s* and returns control to the caller.

 - *V(s)*: The *V* operation increments *s* by 1. If there are any threads blocked at a *P* oepration waiting for *s* to become nonzero, then the *V* operation restarts exactly one of these threads which then completes its *P* operation by decrementing *s*.

 *The names P and V come from the Dutch words proberen (to test) and verhogen (to increment).*

The test and decrement operations in *P* occur indivisbly in the sense that once the semaphore *s* becomes nonzero, the decrement of *s* occurs without interruption.

The increment operation in *V* occurs indivisbly in that it loads increments and stores the semaphore without interruption.

The definition of *P* and *V* ensure that a running program can never enter a state where a properly intialized semaphore has a negative value. This propert, known as the *semaphore invariant* provides a powerful tool for controlling the trajectories of concurrent programs.

The Posix standard defines a variety of cuntions for manipulating semaphores:

```c
#include <semaphore.h>

int sem_init(sem_t* sem, unsigned int value);

int sem_wait(sem_t* s); /* P(s) */

int sem_post(sem_t* s); /* V(s) */
```

The `sem_init` function intializes semaphore `sem` to `value`. Each semaphore must be initalized before it can be used. Each semaphore must be intialized before it can be used.

#### Using Semaphores for Mutual Exclusion

Semaphores provide a convenient way to ensure mutually exclusive access to shared variables.

The basic idea is to associate a semaphore *s*, intially 1, with each shared variable (or related set of shared variables) and surrond the corresponding critical section with *P(s)* and *V(s)* operations.

A semaphore that is used in this way to protect shared variables is called a *binary semaphore* because its value is alawys 0 or 1. Binary semaphores whose pupose is to provide mutual exclusion are often called *mutexes*. Performing a *P* operation on a mutex is called *locking* a mutex. Performing a *V* operation is called *unlocking* a mutex.

#### Using Semaphores to Schedule Shared Resources

An importnat use of semaphores besides providing mutual exclusion is to schedule accesses to shared resources. Two classical and useful examples are the *producer-consumer* and *readers-writers* problems:

##### The Producer-Consumer Problem

![](https://i.imgur.com/ZcznluE.png)

Shown above is the problem. The producer generates items and inserts them into a bounded buffer. The consumer removes items from the buffer and then consumes them.
The producer and consumer share a *bounded buffer* with *n slots*. The produces thread repeatdly produces new *items* and inserts them in the buffer. The consumer thread repreatedly removes items and consumes(uses) them.

Since inserting and removing items involves updating shared variable we must guarantee mutually exclusive access to the buffer. But guaranteeing access is not enough, we need to schedule access to the buffer, If the buffer is full, then the producer must wait until a slot becomes available, and vice versa for the consumer.

#### Using Threads for Parallelism

Thus far, we have studied concurrent threads on a single-core processor. However, modern systems have multiple cores, enabling execution of concurrent threads in parallel.

The most simle approach for assigning work to different threads is to partition the seuquence into *t* disjoint regions and then assign each of *t* different threads to work on its own. 

Let's assume that *n* is a multiple of *t* such that each region has *n / t* elements. 

How might multiple threads work on their assigned regions in parallel?

The most simple way to do this is to have threads sum into a shared global variable that is protected by a mutex.

Upon testing run times across adjusted no. of threads, we can see that as we increase by orders of magnitude. 


This highlights an important lession about paralell programming: *Synchronization overhead is expensive and should be avoided if possible. If it cannot be avoided, the overhead should be amortized by as much useful computation as possible.

#### Characterizing the Performance of Parallel Programs

In the ideal case, we would expect the running time to decrease linearly with the number of cores. That is, we would expect running time to drop by half each time we double the number of threads. This is indeed the case until we reach the point (*t* > 4) where each of the four cores is busy running at least one thread. Running time actually increases a bit as we increase the number of threads because of the overhead of context switching multiple threads on the same core.

For this reason, parallel programs are often written so that each core runs exactly one thread. Although absolute running time is the ultimate measure of any program's performance there are some useful relative measures that can provide insight into how well a parallel program is exploiting potential parallelism. 

The *speedup* of a parallel program is typically defined as *S<sub>p</sub> = T<sub>1</sub> / T<sub>p</sub>* where *p* is the number of processor cores and *T<sub>k</sub>* is the running time on *k* cores. This formulation is sometimes referred to as *strong scaling*. 

When *T<sub>1</sub>* is the execution time of a sequential version of the program then *S<sub>p</sub>* is called the *absolute speedup*. When *T<sub>1</sub>* is the exeuction time of the parallel version of the program running on one core then *S<sub>p</sub>* is called the *relative speedup*. 

Absolute speedup is a tuer measure of the benefits of parallelism than relative speedup. Parallel programs often suffer from synchornization overheads even when they run on one processor, and these overheads can artifically inflate the relative speedup numbers because the increase the size of the numerator. 

On the other hand, absolute speedup is more difficult to measure than relative speedup because measuring absolute speedup requires two different versions of the program.

A related measure, known as effciency is defined as *E<sub>p</sub> = S<sub>p</sub> / p = T<sub>1</sub> / pT<sub>p</sub>* and is typically reported as a percentage in the range 0-100.


Efficiency is a measure of the overhead due to parallelization. Programs with high effciency are spending more time doing useful work and less time synchronizing and communicating than programs with low efficiency.

There is another view of speedup, known as *weak scaling* which increases the problem size along with the number of processers such that the amount of work performed on each processor is held constant as the number of processors increases. Weak scaling is often a truer measure than strong scaling because it more accurately reflects our desire to use bigger machines to do more work. 

### Other Concurrency Issues

#### Thread Safety

When programming with threads, we must be careful to write functions that have a property known as thread safety. A function is said to be *thread safe* if and only if it will always produce correct results when called repeatdly from multiple concurrent threads. We can identify four classes of thread-unsafe functions:

- Class 1: *Functions that do not protect shared variables*. We need to protect the shared variables with synchornization operations such as *P* and *V*. This will slow down the function mind.

- Class 2: *Functions that keep state across multiple invocations*. A pseduorandom number generator (PRNG) is a simple example of this class. The result of a `rand` function depends on an intermediate result from the previous iteration(increments post `rand` call to seed variable). If multiple threads are calling `rand` then we will see usage of the same seed and similar numbers.

- Class 3: *Functions that return  a pointer to a static variable*. Some functions compute a result in a `static` variable and then return a pointer to that variable. If we call a function of this class in concurrent threads, then disaster is likely as results being used by one thread are silently overwritten by another thread.

- Class 4: *Functions that call thread-unsafe functions*. We can still work with the unsafe function and mitigate its potential errors to make it safe.

#### Reentrancy

There is a class of thread-safe functions known as *reentrant functions* that are characterized by the property that they do not reference **any** shared data when they are called by multiple threads.

The terms *thread-safe* and *reentrant* are sometimes used as synonymns there is a clear technical distinction.

Reentrant functions are typically more efficent than non-reentrant functions as they require no synchronization operations.

TODO research more on this

#### Races

A *race* occurs when the correctness of a program depends on one thread reaching point *x* in its control flow before another thread reaches point *y*. Races usually occur assume that threads will take some particular trajectory through the execution state space.

#### Deadlocks 

Semaphores introduce the potential for a run-time error called *deadlock* where a collection of threads is blocked, mutually waiting upon each other to meet some condition(s) that will never be true. As such, the program is *deadlocked* and will never continue.





### Boolean Algebra

Operators:

	`~` - NOT
	`&` - AND
	`|` - OR
	`^` - XOR

`~` correpsonds to the math symbol `¬` so when we see `¬P` it means NOT P. BIT FLIPPERR

`^` corresponds to the math symbol `⊕` for XOR.

Bit vectors are strings of 0's and 1's of a fixed length, usually encoded in notation as `w`.



Invented by George Boole(mid 1850's).



### Shift Operations

Shift operations associate from left to right, so `x << j << k` is the same as `(x << j) << k`.
Arithmetic operations take precedence over shifts, so `1 << 2 + 3 << 4` will become `1 << (2+3) << 4`

Left Shift fills the new space on the right with zeros.


However, There are two forms of right shifts:

Logical: A logical right shift fills the left end with `k` zeros. Giving a result `[0,...,0,xw-1...]`

Arithmetic: An arithmetic right shift fills the left end with `k` repititions of the most significant bit. giving a result `[xw-1,...,xw-1,xw-1,xw-2,...,xk]`(cant do subscript in vim, imagine the `k` and `w` are :D)

In C, right shift operations on signed numbers must be arithmetic.
Right shift operations on unsigned numbers must be logical.

Unlike C, Java has explicit operations for logical(`>>>`) and arithmetic operations(`>>`).

## Numbers

### Representation of numbers

Conversion/Type Casting unsigned to signed, or binary etc. can cause many a bug, especially if the conversion is done implicitly; such as when you compare a signed to an unsigned number, the signed number will be implicly converted to compare.

For this reason, many languages do not allow you to use unsigned integers, like Python or Java.


Unsigned encodings are based on traditional binary notation, representing numbers >= 0.

Two's complement encodings are the most common way to represent signed integers.

Floating point encodings are a base-2 version of scientific notation for representing real numbers.

Number  operations overflow when results are too large to representing in the x bits the system can handle.


Signed numbers can use 1 less positive number than negative(e.g int 8 = -128 - 127) because 0 counts as a nonnegative number, meaning you still have 128 of negative and nonnegative numbers.

One's complement is like two's complement but the bits are all inverted? Bit weird ngl.

Sign Magnitude is a sign bit and 7 normal bits, with a 0 and a -0 value. An 8 bit number ranges from -127 to 127 as such.

In C, adding a `u` or `U` to the end of declaring a constant, makes it unsigned.

Interestingly, a signed w-bit sum has the exact same bit-level representation as the unsigned sum. Most computers use the same machine code instruction to perform both.

#### Binary

###### Binary To Unsigned (B2U) equation

p.99 for proper equation.

Iterate through `x` number `w` times, and do `x[i]*2^i`. Then add all up, this converts to an unsigned integer.

Get the greatest value via (p.99) UMax<sub>w</sub>=2<sup>w</sup>-1.

The function B2U is a *bijection*, meaning the function goes two ways.

Still dont really understand this.

To prove this: y = f(x) and x = f<sup>-1</sup>(y)

So in this example, we would do U2B(Unsigned to Binary), mapping each number from 0-2<sup>w</sup> - 1 to a unique pattern of `w` bits.


###### Binary To Two's Complement(B2T)

aka signed.

The most significant bit is defined as having negative weight.

This is actually the same equation as B2U, but as the signed bit starting from -x<sub>w</sub>-1 2<sup>w</sup>-1 + `∑w-2, i=0 x[i]*2^i`.

Its 'weight' is the negation of its weight in an signed representation.


So in a 4 bit number, if the sign bit is 1, you do -8 + the rest of the nibble.

So number `1011` = -8 + 0 + 2 + 1 = -5
And number `0101` = 0 + 4 + 0 + 1 =  5

###### Two's Complement to Unsigned (T2U)
If your number is negative, do x + 2<sup>w</sup> to get the unsigned version.

For a 4 bit number:
	If `x < 0`: `f(x) = x + 16`

	So -5 becomes +11, -6 becomes +12 etc.

	If `x >= 0`: `f(x) = x`.

	So 5 becomes 5, 6 becomes 6 etc.


###### Unsigned to Two's Complement (U2T)

if x > TMax<sub>w</sub> then do x - 2<sup>w</sup>.

### Truncating Numbers

Truncating can be done by type casting an int to a short:

```
int x = 53191;
short sx = (short) x;
```
`sx` is -12345, the truncation halved the bits.

When casting `sx` back to `int` sign extension will set the high-order 16(half) to ones, making the 32-bit two's complement representation of -12,345.

```
int y = sx; // sx = -12345
```
When truncating a `w`-bit number to a `k`-bit number, we drop the high order `w` - `k` bits.

Truncation of unsigned numbers is done by the property that 2<sup>i</sup> mod 2<sup>k</sup> = 0 for any i >= k.

Truncation of a signed(two's complement) number is similar and it keeps the most significant bit as a sign bit.

Equation - `x` = U2T<sub>k</sub>(`x` mod 2<sup>k</sup>)

Applying U2T to the `x` mod 2<sup>k</sup> will transform the most signicant bit from having weight 2<sup>k - 1</sup> to having weight -2<sup>k - 1</sup>.

We can see the difference in our example from above, as the sign bit when a short type cast is applied becomes a 1, at 2<sup>16</sup> so the number becomes negative,.

The number is then converted back, to 32-bit int and the negative sign bit is converted back to, making the same -12345 number.

###### Truncating Signed(Two's Complement

Say we want to truncate `-4` from 4 bits to 3 bits.

We apply the equation: (x mod 2<sup>k</sup>)-(-2<sup>k</sup>)
Ex:
```
(-4 % -8) - (-2 ** 3) = 4
```
Our truncated result is 4.

Now lets try on a non-negative `x`:

	(-5 % -8) - (-2 ** 3) = 5

Our truncated result is 5.

(ref p.118)



######  Unsigned Addition Equation

Modular additon.

For 0 <= x, y <= 2<sup>w</sup>

Normal Case: x+y,	x + y < 2<sup>w</sup>

Overflow Case: x+y - 2<sup>w</sup>,	2<sup>w</sup> <= x + y < 2<sup>w+1</sup>


For example, say `x` = 9 and `y` = 12.

	x = 1001
	y = 1100
	--------
	   10101

As you can see the 2<sup>4</sup> column overflowed, so now we have a 5 bit integer, or a w+1 bits integer.

Per the equation, we do x + y - 2<sup>w</sup>. This will truncate our value to a 4 bit, cutting off the most significant bit, the 16 column in this example(2ᵂ)

In practice truncating the w+1 bit in a non-overflowed addition will not affect the outcome as it will be a 0.


detect if an overflow has occurred:

	sum = x+y;
	return sum <= x;

######  Unsigned Negation Equation

To get the inverse of an unsigned int, provided x != 0, as !0 = 0.

So, where x > 0: 2<sup>w</sup> - x

This is effectively flipping a sign bit, but obviously there isnt a sign bit here, so probably a bad example.

######  Signed Negation Equation

There are various techniques for doing this.
First there is the equation, which gives you an idea of the maths behind it:
	if x = TMin return TMin
	if x > TMin return -x
	if x < TMin return 2ᵂ - x

The bit patterns are the same for unsigned and signed negation.

Now for a bit-level technique, get the complement of the binary number, so `~x` in C. Then increment it once.
That's it!

Another way is to get the position of the right-most 1, call it `k`.
Then complement all bits to the left of `k`.

Maths ppl are fucking smart wtf

###### Signed(Two's Complement) Addition equation

We must be able to account for both positive and negative overflows, and deal with them appropriately.

We have a potential for a w + 1 bit size result. So we have to truncate according to the overflow rules as per below.

So for integers in the range -2<sup>w-1</sup> >= x, y <= 2<sup>w-1</sup> - 1(-2ᵂ will be half of the set, the negatives; 2ᵂ will be positives, take one for the zero):

For Positive overflow; where x + y >= 2<sup>w-1</sup>; do x + y - 2<sup>w</sup>.

For normal numbers; where  -2<sup>w-1</sup> >= x, y <= 2<sup>w-1</sup> - 1; do x + y.

For negative overflow; where x + y > -2<sup>w-1</sup>; do x + y + 2<sup>w</sup>.


###### Unsigned Multiplcation equation


Possible product between 0 and (2<sup>w</sup> - 1)<sup>2<sup>.
This could require up to 2`w` bits to represent.

To get around possible inefficiencies here, C uses the w-bit value given by the low-order w bits of the 2w-bit integer product.
This, in easier terms is truncation, doing (x * y) mod 2<sup>w</sup>.

##### Signed(Two's Complement') Multiplcation

This is done by applying the U2T equation onto the Unsigned Multiplication Equation result.

The reason this can be done is because of the nature of the truncation.




#### Hexadecimal

Hex uses digits 0-9 and characters A-F to represent base 16.

In C, numeric constants starting with a `0x` or `0X` are interpereted as hex.
Letters:
	A - 10
	B - 11
	C - 12
	D - 13
	E - 14
	F - 15

Often hex numbers start with a string of `f`'s as negative numbers the empty spaces need to be filled by 1's.


##### CONVERSION IN YOUR HEAD

###### Decimal:

For each position to the left, multiply by 16, starting from the second position.

So - `0x55` would be 5 x 16 = 80 and then the smallest position is just in ones, so 5.
80+5 = 85

Automated Version:
(there is python code in pen/tools folder for this)

To convert decimal number `x` to hexadecimal, we repeatedly divide `x` by 16, giving a quotient `q` and a remainder `r` such that `x = q*16 + r`. We then use the hexadecimal digit representing `r` as a digit in our conversion, and then repeat the process on `q`.

Example:

314,156 = 19,634 * 16 + 12 = C <- This is our first hex digit in the conversion!
19,634  = 1,227  * 16 + 2  = 2
1,227   = 76     * 16 + 11 = B
76      = 4	 * 16 + 12 = C
4	= 0	 * 16 + 4  = 4

###### Binary:

Each hex digit is 4 binary digits so simply convert each digit to a nibble(4 bits) and keep going, from leftmost to right. Simple!

When converting to hex from binary, if the binary number contains a non-multiple of 4; make the *left most* group the one with leading zeros.

When a value x is a power of 2, that is `x = 2^n` and n is a non-negative integer;  `n = i+4j`, you get j from the closest mutiple of 4 to x, and then i is the remainder between the x and `4*j`.



### Integer Arithmetic

Say we have two non-negative integers, such that 0 <= x, y <= 2<sup>w</sup>
Each of these values can be represented by a `w`-bit unsigned number.

If we compute the sum of these integers, we have a possible range of 0 <= x + y <= 2<sup>w+1</sup> - 2

Representing this sum could require `w` + 1 bits.


### Multiplying by Constants

The integer multiply instruction on many machines was slow, requriing 10 or more clock cycles on modern machines it can be around 3.
So, compilers attempt to replace multiplications by constant factors with combinations of shift and addition operations.

For example, multiplying by factors of two can be done as: 2<sup>k</sup>
Take 11, or `1011` in binary; shift this left twice, and you get 44 in w+k bits.
So to multiply by 2, simply apply 2<sup>1</sup>, multiply by 4: 2<sup>2</sup> etc...

When using this on a fixed word size, the high order `k` bits are discarded.

So instead of doing `x*14` as multiplcation of 14, which would take many clock cycles;
We see that 14 = 2<sup>3</sup> + 2<sup>2</sup> + 2<sup>1</sup>.
So we can adjust this(say we are writing a compiler) to (x<<3) + (x<<2) + (x<<1).
Very fucking cool.

We can also use this for odd multiplication such as: `x*3 = (x<<1) + x`

This principle is also applied to division, which can take even more cycles than multiplcation, up to 30!
Oviously we use right shifts, instead.
Integer division always rounds to zero.

Rounds up: x/2<sup>k</sup>

### Floating Point

A floating-point representation encodes rational numbers of the form V = x * 2<sup>y</sup>

Devised under the IEEE 754 standard.

Digits are the point, are weighted as their distance from the point, and then that by negative powers of the base.
The same applies for the positive digits (prefixing the dot) but obviously positive powers of the base.

So for decimal `1.2`, the 2 would be weighted as b<sub>-1<sub> x 10.

For binary, the same, each digit weighed by 2, instead of 10.

Under the IEE floating-point standard, numbers are represented in the form V = (-1)<sup>s</sup> x M x 2<sup>E</sup>

The sign `s` dertermines whether the number is negative (s = 1).

The significand `M` is a fractional binary that ranges from 2 - ℯ pr betweem 0 and 1 - ℯ.

The exponent E weights the value by a power of 2.

How are these encoded in bits?

For single point precison, or `float` in C;
There is 1 bit for the sign bit `s`. 31th bit.
8 bits encode the exponent `E`. 30th-23rd bit.
Then 23 bits encode the significand `M`.22nd-0th bit.


Double point precision, `double` in C, is 64 bits.
Again 1 bit for the sign, and 8 for the exponent.
However, there are two parts of the fraction field encoding `M`, from the 51st - 32nd bit; and 31st-0th bit.
The two sections of this make the 'double' namesake.





### Addressing

A multi-byte object is stored as a contiguous sequence of bytes, with the address given(using `&` for example) being the smallest memory address used.

Say variable `x` of type `int` is stored at `0x100`, then the 4 bytes of `x` would be stored in `0x100`, `0x101`,`0x102`,`0x103`.


### Logical and Bitwise Operators

LOGICAL OPERATORS RETURN A BOOL

BITWISE OPERATORS RETURN THE PROPER VALUE ITERATED OVER THE BINARY DATA.

`~` FLIPS BITS

### Byte Ordering

Bytes being stored from least significant to most is called 'little endian'

The opposite, bytes stored from most significant to least significant is called 'big endian'.

There is still not a defined one that the computing world has decided on, most intel-compatible machines exclusively use little endian.

However machines from IBM and Oracle use big endian.

Some machines can operate in 'bi-endian', meaning can use both, such as ARM processors. However Android and iOS OSes use little endian only.

### 'Words'

Ever computer has a word size, indicating the nominal size of pointer data.

For a machine with `w` word size, the virtual addresses can range from 0 to 2ᵂ-1 , giving the program access to at most 2ᵂ bytes.

A 32-bit word size limits the machine to an address range of 4GB. Hence the 4GB limit on 32-bit Operating Systems.

A 64-bit word size has a virtual address space of 16 exabytes.

Most 64-bit machines can run 32 bit programs, you can do this with gcc via `gcc -m32` or `-m64`

The standard data types in C vary from 32-bit and 64 bit machines, hence the need to specify differences.

In C99, the 1999 C standard, was introduced a class of fixed size types such as `int32_t` and `int64_t` having exactly 4 and 8 bytes respectively.

Data types, usually are signed unless an `unsigned` is prefixed to the type.


This usually isn't a problem, however it is very important to be defined and ahered to in network protocols.

Both machines must adhere to the networking standard, and then convert back to little or big endian if they wish to, after receiving the bytes.

Often, when reading machine code, bytes will be ordered from smallest on the left; as this is the natural way to write numbers for most people.

However, ??? TODO i guess...
### Files

A file is a sequence of bytes, every I/O device, including disks keyboards, displays etc. are treated as a file. This way each input and output in the system is performed by reading and writinf files, using the Unix I/o system calls.


A network can also be looked at as just another I/O device, as, at the core and lowest abstraction it is simply the input and output of bytes to and from the system.

### Character Encoding

ASCII is a 7-bit char code, as such has 127 characters. Use `man ascii` to find a table of them.

Unicode uses a 32-bit representation of characters.

UTF-8 encodes each character as a sequence of bytes.



### Laws & Equations

##### Amdahl's Law

Gene Amdahl made an observation that when we speed of a part of a system, the effect depends on how significant the part was to the overall speed of the system.

It is the calcuation of the difference in overall time taken from the old version to the new, with the fraction of the time taken of the component being the change.

Check Wikipedia or page 58. Quite simple.


### Type Casting/Conversion

In C, you convert by having your to-cast-to type in brackets followed by your original variable.
So:

	short int v = -12345;
	unsigned short uv = (unsigned short) v;
This is explicit casting.
This particular example will fuck up the number due to the sign bit, but you get the point.
Casting keeps bit values identical, but changes how they are interpereted.

You can implicitly cast when an expression of one type is assigned to a variable of another:

	int x;
	unsigned y;

	x = y
x is now a unsigned integer.


When performing operations on unsigned and signed integer values, C will convert the signed int to unsigned and perform the operation.

However, this has some fucking rules for relational operations such as `<` and `>`.

Look them up, 'C promotion rules'.

###### Type cast vulnerabilities

This is done by overflows as specfieied in the conversion equations in the Representation of numbers section.

To check/prevent these vulns

###### Converting Smaller data types to larger

When converting an unsigned number to a larger data type, simply add leading zeros!

### gcc flags

Add a `-std` to specify the version of C you want to compile to.
For example: `gcc -std=c11` would compile to ISO C11 standard.

`-0g` will apply 0 levels of optimization in the compilation process.n You can add -01g or -02g for optimization levels.

`-S` flag will print the assembly code generated by the compilation.


### Architectures

The format and behaviour of a machine-level program is dfined by the the Instruction Set Architectures(ISA).
Most ISAs, including x86_64 describe behavior of a program as if each instruction is executed in seqence.
Memory address used by a machine-level program are virtual addresses.

The assembly code closely represents the machine code.

The machine code for x86_64 differs greatly from the original C code.
Many things are abstracted from you:

- The program counter indicating the address of which the next instruction is in. This rules what your program does.

- The register file contains 16 named locations storing 64-bit values. These registers can hold addresses(pointers) or integer data. These are sometimes used to keep track of some parts of the program, or to hold temporary data.

- The condition code registers hhold status info about the most recently executed arithmetic or logical instruction. These are used to implement conditional changes in the control or data flow, such as `if` or `while statements.`

- Vector registers holding one or more integer or float value.

Whereas C provides models of which different data types can be declared and allocedl machine code views the memory as a huge byte-addressable array.
Not even for signed vs unsigned integers.


##### Virtual Addresses

At any given time(at runtime) only a certain subrange of virtual addresses are considered valid.
For example, in x86_64 machines, virtual addresses are represented by 64-bit words.
The first upper 16 bits must be set to zero; and so an address can potentially speficy a byte over a range of 2<sup>48</sup> or 2 terabytes.(as such this is the RAM capacity.)
Most programs will have access to only megabytes of the total address space, some more hefty programs, like games could have more.



## Assembly

There is an assembly notes file -  `~/notes/dev/asm.md`.

Dissambley is the process of converting machine code into assembly code.
Most programs will have access to only megabytes of the total address space, some more hefty programs, like games could have more.



## Assembly

There is an assembly notes file -  `~/notes/dev/asm.md`.

Dissambley is the process of converting machine code into assembly code.
This can be done via IDA, Ghidra or the linux tool `objdump` with the `-d` flag for disassemble.


## Storage on the Stack

There are often times where we need more local storage than what is available in the registers. Each one can only hold 8 bytes, and we often need more than that to perform a task.
We may also need an address for something when the address operator `&` is used.

Typically, a procedure allocates space on the stack frame by decrementing the stack pointer.
This is the portion of the stack frame called 'local variables'.


## History

#### x86

Devised from the i386 process, first processor to use 32 bits.
This is known as IA32.
This was later expanded to 64 bits; creating x86_64.
This expansion was done by amd; which is why it is sometimes called amd64.



### Footnotes

###### Definitions

Vector - One dimensional array

High-Order Functions - Functions that take one or more functions as arguments.

Arity - The number of arguments that a function takes.

Let Expression - Associates a function definition with a restricted scope.

Modular Arithmetic - A system of arithmetic where numbers 'wrap around' after reaching a certain value.

###### Abbreviations

ISA - Instruction Set Architecture

ANSI - American National Standards Institute

RFC - Request for comments

ISO - International Standards Organization

DRAM - Dynamic Random Access Memory

SIMD - Single Instruction, Multiple Data

gcc - GNU Compiler Collection
