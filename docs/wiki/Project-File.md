🌐 **English** · [Polski](Plik-projektu)

# Project File

PenCreator saves **`.cre`**, the project file of baseCreator that LightCreator and MillCreator write too: one JSON file with the drawing, the description and **one section per program** (`laser`, `mill`, `pen`). Each program writes its own section and keeps the sections of the others untouched, so a project passes between programs without losing anything.

The `pen` section holds the 64 layers (name, colour, shown and drawn, the drawing, the pen of the library, pen width, speed, fill gap and angle, crossed lines, how many times to draw, the picture method and dithering, the place in the order of the job), **your own pens that the layers use** (so that a project opened on another computer brings them along; they join **My pens** there) and a note of the plotter the project was made for (its name and sheet size); when you open the project for a device with another sheet size, the status line says so. A layer with a number out of range is replaced by the default layer.

The description (the **Description** tab) is part of the project; PenCreator also writes it into the G-code as `(Note: …)` lines. A project that has a file is saved to it by itself about 20 seconds after a change (see [Settings and Files](Settings-and-Files)). A project made in LightCreator or MillCreator opens here with its drawing and description and the default pens; when you save it, their sections are written back as they were. A file with a newer container format (or a newer `pen` section) than the program knows is refused with a message.
