# ATracer

A toy software ray-tracing engine for me to tinker with. I will be using this project to learn things such as window & GUI interactions, multithreading, file I/O, and more.

![Window Picture](img/spheres.png)

The project is split into two core parts: [the window](a_tracing_window) and [a raytracing library](a_tracing_window).
As this project is in a fairly early stage the features are pretty limited.

## Features

### Window

#### Implemented
* Resizeable viewport
* Camera movement using **wasd** + **rf**
* Camera orientation change using **qe** for left/right and **zx** for up/down
* Button to start render

#### Planned
* Camera movement using mouse
* Scene editing using mouse
* Extended GUI for rendering settings such as number of samples, sampling method and tracing depth

### Library

#### Implemented
* 2 Rendering methods: 
    * Quick render that completes within a single frame for use during camera movement
    * Long render with full detail
* Spheres
* Basic diffuse material
* Regular multisampling

#### Planned
* Multithreaded rendering
* Triangle meshes with bounding volume hierarchy acceleration structure
* Advanced materials: mirrors/glass/smoke/...
* glTF file import

