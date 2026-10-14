🌐 **English** · [Polski](Szybki-start)

# Quick Start

1. **Choose the device.** The start screen lists your plotters. **Add device…** makes one (sheet size, how the pen goes up and down, cameras); **I have no device** makes a plain device called Dummy (an A4 sheet). If you click **New** or **Open…** without choosing, the Dummy is used.
2. **New** starts an empty design on the sheet of the device. **Open…** reads an SVG, PDF, Illustrator or `.cre` file; **File → Import → Import LightBurn project…** reads a LightBurn project. Dropping a file on the window opens it.
3. **Draw** with the tools on the left (select, node edit, rectangle, ellipse, triangle, star, polygon, heart, spiral, line, pen, text, pan, zoom), or place a picture from the **Clipart** tab.
4. **Pens.** In the **Layers** tab every layer is a pen: set its colour, pen width and speed, and whether the shapes are drawn as outlines, filled with lines, or both. Select shapes and click **Put the selected shapes on this layer**.
5. **Plotter → Plot of the job…** (Ctrl+Shift+P) shows what the pen will do; the dashed lines are the moves with the pen up. **Export G-code…** (Ctrl+Shift+G) writes the file for GRBL.
6. **Save** as `.cre` ([Project File](Project-File)). The **Description** tab holds a note about the project.

> The program writes G-code but cannot connect to a plotter: send the file with your usual sender. See [Safety and Limitations](Safety-and-Limitations).
