# Rust Async



## Non-Preemptive (Cooperative) Multitasking 
[Reference](https://en.wikipedia.org/wiki/Cooperative_multitasking#:~:text=Cooperative%20multitasking%2C%20also%20known%20as,running%20process%20to%20another%20process.)
- OS offloads responsibility of performing task to process
- Process (programmer) has to explicitly yield control to OS.
- Hence, one bad behaving process can block the entire system.

## Preemptive Multitasking
- OS does the context switching
- OS can interrupt a process and switch to another process
- Context switching happens so fast and so frequently that we don't observe any difference

## Hyper-threading
- CPU nowadays has several logical units
- CPU simulates two logical core on the same physical core by using unused part of the CPU to drive progress on thread 1 and thread 2 simultaneously


## Concurrency vs Parallelism
- Concurrency: Multiple tasks are making progress at the same time
  - Concurrency is about being efficient, in terms of resource utilization. Concurrency can never make one single program task go faster.
  - Concurrency is about working smarter
- Parallelism: Multiple tasks are running at the same time
    - Parallelism is increasing the resource we use to solve the task.


## 




