# Detailed Documentation for Dynamic Artboard Creator

## Introduction

This document provides an in-depth look at the Dynamic Artboard Creator, a Rust-based application designed to automate the arrangement and resizing of images on a virtual artboard. The application intelligently positions images to create visually appealing compositions automatically. Here, we detail the program's components, including structs, functions, and the overall workflow.

## Components Overview

### Structs

#### `Cursor`

- **Purpose**: Represents a cursor's position on the artboard, crucial for placing images.
- **Fields**:
  - `x: i64`: Horizontal position on the artboard.
  - `y: i64`: Vertical position on the artboard.

### Functions

#### `create_cursor`

- **Returns**: A `Cursor` instance with `x` and `y` initialized to `0`.
- **Purpose**: Initializes the cursor at the top-left corner of the artboard.

#### `lg_to_sm`

- **Parameters**: `images: &mut Vec<DynamicImage>`
- **Purpose**: Sorts the images from largest to smallest based on their area, using a bubble sort algorithm. It is marked for future optimization.
- **Process**: Iteratively compares and swaps adjacent images to ensure they are in descending order of size.

## Main Functionality

### Setting Up the Artboard

Prompts the user to input the desired dimensions for the artboard, establishing the space where images will be arranged.

### Image Processing

1. **Reading and Filtering Images**: Loads image files from a specified directory, filtering out those not matching supported formats (`.jpg`, `.png`).
2. **Resizing**: Randomly resizes images within a specified range to ensure variety and fit on the artboard.
3. **Sorting**: Applies the `lg_to_sm` function to sort images by size, preparing them for placement.

### Image Placement

Uses the `Cursor` struct to track placement positions, ensuring images are laid out in an aesthetically pleasing manner. Adjusts cursor positions dynamically as images are placed, with logic to prevent overflow beyond the artboard's edges.

### Output

Generates a unique filename for the artboard based on the current UNIX timestamp and saves the composed artboard as a `.jpg` file. Outputs a success message indicating the file name upon completion.

## Future Enhancements

- **CLI Integration**: Plans include transitioning from interactive prompts to CLI flags to enhance usability.
- **Optimization of Sorting Algorithm**: A more efficient sorting algorithm will replace the current bubble sort to improve performance.
- **Extended Support for Image Formats**: Increase the range of supported image file formats to accommodate more input types.
