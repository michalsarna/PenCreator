🌐 **English** · [Polski](Plik-projektu)

# Project File

PenCreator saves **`.cre`**, the project file of baseCreator that LightCreator and MillCreator write too: one JSON file with the drawing, the description and **one section per program** (`laser`, `mill`, `pen`). Each program writes its own section and keeps the sections of the others untouched, so a project passes between programs without losing anything.

The `pen` section holds the 16 layers (colour, pen width, speed, outline / fill, fill gap and angle, crossed lines, how many times to draw) and a note of the plotter the project was made for (its name and sheet size); when you open the project for a device with another sheet size, the status bar says so. A project made in LightCreator or MillCreator opens here with its drawing and description and the default pens; when you save it, their sections are written back as they were. A file with a newer container format (or a newer `pen` section) than the program knows is refused with a message.
