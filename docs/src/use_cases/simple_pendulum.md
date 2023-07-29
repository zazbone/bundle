# Base exemple, Simple pendulum

## Problem description

Simulation / numerical solution of the simple pendulum differential equation system.

Depend on some physical parameters:
- length: [L] float 
- mass: [M] float 
- init angle: [-pi, pi] float 
- init momentum: [T^-1] float 
- T: [T] float 
- time step: [T] float 
- numerical method: Euler | RK4 | ... 

**out:** Table[time, momentum, angle]

## Needs
- Be able to store parameters as "metadatas" associate to values (float/enum/str...)
- Be able to document thoses metadatas (units, specification, ...)
- The software `Bundle` must be able to associate to metadatas a unique dataset.

