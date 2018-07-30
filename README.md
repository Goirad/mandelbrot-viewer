# Rust Mandelbrot
---
This crate allows you to explore and print the mandelbrot set.
To run, clone this repo and run 
    
    cargo run --release
    


#### Commands

Esc :          Quit  
Num +/- : Zoom In/Out  
Left Click : Recenter window
N/M : Half/Double number of iteration before escape  
P : saves current window to fractal.png in crate directory  
I/O : Half/Double size of image to print
 
### Samples

![alt text](https://github.com/Goirad/mandelbrot-viewer/tree/master/samples/sample1.jpg "Sample 1")

![alt text](https://github.com/Goirad/mandelbrot-viewer/tree/master/samples/sample2.jpg "Sample 2")

Note, the program currently does not implement anti-aliasing, so the above images were created by generating very large images and then downsampling them in GIMP.