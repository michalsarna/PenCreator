# Security policy

## Supported version

Only the newest released version receives security fixes.

| Version | Supported |
|---------|-----------|
| newest release | yes |
| older | no, please update |

## Reporting a vulnerability

Please do not open a public issue for a security problem. Use GitHub's private vulnerability reporting:
<https://github.com/michalsarna/PenCreator/security/advisories/new>

Include the version, your operating system, what you did and what happened. Files that trigger the problem
(an SVG, PDF / Illustrator or image) are very helpful. You can expect an answer within a few days; a fix is released
as a new version.

## What counts

PenCreator opens files from other people (SVG, PDF / Illustrator) and will write files for plotters.
Reports about crashes, hangs or memory use when opening a crafted file, and about anything that could send unintended
commands to a machine, are in scope. Problems in the shared code are handled in
[baseCreator](https://github.com/michalsarna/baseCreator/security/advisories/new).

## Safety reminder

A plotter is a machine with moving parts. PenCreator is an early skeleton without plotting features yet; when they arrive,
test new jobs with the pen lifted first and never leave a running machine unattended.
