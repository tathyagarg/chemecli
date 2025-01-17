# Chemecli
A CLI tool to help me with chemistry because I don't understand it.

## Commands
- Read - `read <target>` - reads and prints out the data of an element/compound.
  - Alias: `r`
  - Sub-functionality: `r <target> <fields>` reads out only the specified fields of the target.

- Add - `add <target>` - adds the specified target to the elements.json database
  - Alias: `w`
  - Sub-functionality: `a <target> <key> <value>` adds the specified key-value pair to the json database.

- Update - `update <target> <key> <value>` - updates data about target.
  - Alias: `u`

- Delete - `delete <target>` - deletes the target from the elements.json database
  - Alias: `d`
  - Sub-functionality: `d <target> <fields>` deletes data of only the specified fields

- Lookup - `lookup <target> <fields>` - looks up in-built data about the specified elements.
  - Alias: `l`
  - Sub-functionality: `lookup list` - lists all the lookup-able fields
    - Alias: `ll`

- Molar Mass - `molar <target>` - calculates the molar mass of the given target
  - Alias `mm`
  - Sub-functionality: `mm <targets>` - calculates the molar mass of the given targets


## Features
- [x] Periodic Table in your Terminal
- [x] Custom Grouping by Colors
- [x] Index for Custom Groups
- [x] Multiple Tables
- [x] Controls to naviagate between multiple tables
- [x] Viewing tabulated notes
- [x] Specific property notes lookup
- [x] Adding notes through CLI
- [x] Editing notes through CLI
- [x] Deleteing notes through CLI
- [x] More inbuilt data
    - [x] Atomic Numbers
    - [x] Atomic Masses
- [x] Command to lookup inbuilt data
    - [x] lookup | l <target> <property> - Lookup inbuilt property of target 
    - [x] lookup list | ll - List all inbuilt properties, along with their constraints
- [x] Compound molar mass calculator
- [ ] Notes about chemical equations 
- [ ] Automatic chemical equation balancer
- [ ] Quiz on notes
- [ ] Multiple files for notes
    - [ ] Divided section on element notes when element is present on more than one note file
- [ ] Command line arguments for notes and tables data
- [ ] Custom file format for notes
- [ ] Custom file format for tables
- [ ] In-built tables
- [ ] Use mouse events to display notes on selected element
- [ ] Refactor to get a good final product
- [ ] Config files
    - [ ] Notes files
    - [ ] Tables file(s)
    - [ ] Box drawing character config
    - [ ] Notes files for quiz config
    - [ ] Glob support for files
    - [ ] Selection of displayed in-built tables
    - [ ] Table element borders
- [ ] Empirical formula calculator

