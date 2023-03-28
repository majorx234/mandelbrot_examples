from math import log
import numpy as np


def mandelbrot_point(x,y ,threshold,iterations):
    z_re = 0.0
    z_img = 0.0
    for i in range(0, iterations):
        z_re_old = z_re
        z_re = (z_re * z_re) - (z_img * z_img) + x
        z_img = 2 * (z_re_old * z_img) + y
        if (z_re * z_re) + (z_img * z_img) > (threshold * threshold):
            return i
    return 0


def mandelbrot_calc(
    buffer,
    x_min,
    x_max,
    y_min,
    y_max,
    resolution_x,
    resolution_y
    ):
    
    for y_coord in range(0, (resolution_y-1)):
        for x_coord in range(0, (resolution_x-1)):
            x = (x_coord/resolution_x) * (x_max - x_min) + x_min
            y = (y_coord/resolution_y) * (y_max - y_min) + y_min
            value = mandelbrot_point(x,y,2.0, 255)
            buffer[y_coord][x_coord] = value 


    
def main():
    mandelbrot_matrix = np.zeros((1000, 1000))
    mandelbrot_calc(
        mandelbrot_matrix,
        -1.5,
        0.5,
        -1.0,
        1.0,
        1000,
        1000)

    
if __name__ == "__main__":
    main()