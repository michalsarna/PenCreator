🌐 **English** · [Polski](Plik-projektu)

# Project File

PenCreator saves **`.cre`**, the project file of baseCreator that LightCreator and MillCreator write too: one JSON file with the drawing, the description and **one section per program** (`laser`, `mill`, and `pen` later). Each program writes its own section and keeps the sections of the others untouched, so a project passes between programs without losing anything.

PenCreator has no section of its own yet. A project made in LightCreator or MillCreator opens here with its drawing and description; when you save it, their sections are written back as they were. A file with a newer container format than the program knows is refused with a message.
