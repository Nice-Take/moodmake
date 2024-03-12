# Dynamic Mood Board Creator

## Project Overview

The Dynamic Mood Board Creator is a Rust-based application designed to automate the process of arranging and resizing a collection of images onto a virtual artboard. It intelligently positions images in a visually appealing manner, taking into consideration the dimensions of each image to ensure a harmonious and dynamic composition. This tool is perfect for creating digital collages, mood boards, or any project requiring the automatic arrangement of multiple images.
[Sample.](https://nicetake.com/images/artboard_1710282529.jpg), [Sample.](https://nicetake.com/images/artboard_1710282624.jpg)

### Features

- **Dynamic Image Arrangement**: Automatically places images on an artboard, adjusting their position to optimize visual balance and composition.
- **Image Resizing**: Randomly resizes images within specified limits to add variety and ensure that images fit well together on the artboard.
- **Customizable Artboard Size**: Allows users to define the desired width and height of the artboard, providing flexibility for different project requirements.
- **Intelligent Sorting**: Sorts images by size to ensure larger images serve as the background, enhancing the depth and aesthetic appeal of the final composition.
- **Border Management**: Automatically adds a fixed border around images, preventing overlap and maintaining clear separation between individual images.
- **Unique Output Filenames**: Generates unique filenames for each created artboard based on the current UNIX timestamp, preventing accidental overwrites.

### How It Works

The application prompts the user for the desired dimensions of the artboard. It then reads all image files from a specified directory, filters out unsupported formats, and randomly resizes the images to fit the artboard dimensions. Using a custom sorting algorithm, it arranges the images from largest to smallest, ensuring an aesthetically pleasing layering effect. The program then dynamically places each image onto the artboard, adjusting the cursor's position after each placement to achieve an optimal layout. The final artboard is saved as a `.jpg` file, with a unique filename to avoid overwriting existing files.

### Future Enhancements

- **CLI Tooling**: Transition from interactive prompts to command-line interface (CLI) flags for input parameters, enhancing usability and scriptability.
- **Sorting Algorithm Optimization**: Replace the current bubble sort algorithm with a more efficient sorting method to improve performance, especially with a large number of images.
- **Extended Image Format Support**: Increase the variety of supported image formats, accommodating a wider range of input files.

## Documentation

[Please see detailed documentation here.](DOCUMENTATION.md)

This detailed documentation provides a comprehensive overview of the program's structure and functionality, including descriptions of structs, functions, and modules, as well as a guide to the program's main workflow.

---
