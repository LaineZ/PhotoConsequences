# PhotoConsequences
![PhotoConsequences](/resources/icon.png)

This software allows you apply almost any VST™ 2.4 effect and bound audio VST™ buffers on image channels **(hue, saturation, value)**. Combination of this parameters can achieve interesting effects on image. **Feel free to experement**!

**This software currently in development. breakable changes and random bugs are inevitable.**

# Interface modes (since 2.1.0)

PhotoConsequences has 2 UI modes:

* **CLI** - For batch image processing e.g frame sequences from videos. To run PhotoConsequences in this mode, you need use command like this: ```photoconsequences [project path] [input directory] [output directory]```. Program will be started in CLI mode automatically
* **GUI** - Default UI mode. 

# Building

Build insturctions is typical for any Rust project

* Clone this repository
* Run ``cargo build --release`` command

# Screenshots and other media

![PhotoConsequences](/resources/screenshoot1.png)

![PhotoConsequences](/resources/screenshoot2.png)

# Supported platforms

**Windows**

**Linux** (also supports plugins via WINE using bridging software like yabridgectl)

**macOS** (untested; but should work without problems)

# License and trademarks
MIT License

Copyright (c) 2022 140bpmdubstep

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

VST™ is a trademark of Steinberg Media Technologies GmbH.