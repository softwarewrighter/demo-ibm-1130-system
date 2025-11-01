# IBM 1130 Demo Content

This directory contains placeholder demo content for the IBM 1130 emulator web UI.

## Directory Structure

```
data/
├── demos/                  # TOML files describing demo programs
│   ├── apl_matrix_operations.toml
│   ├── forth_hello_world.toml
│   └── ...
├── README.md              # This file
└── LICENSE_REVIEW.md      # License considerations for third-party software
```

## About the Demo Content

The TOML files in `data/demos/` are **placeholder metadata** for educational demonstrations.
They describe:
- Demo program code and expected output
- Educational objectives and topics
- System requirements
- Attribution to original software authors

**Important:** The actual historical software (APL\1130, 1968-FORTH, etc.) is **NOT included**
in this repository due to unclear licensing.

## Obtaining Historical Software (For Local Use Only)

To run demos with actual historical IBM 1130 software on your development machine:

### Option 1: Manual Download

Download software to a local `tmp/` directory (gitignored):

**APL\1130:**
```bash
mkdir -p tmp/downloads/apl/source tmp/downloads/apl/binary
cd tmp/downloads/apl/source
curl -k -O http://media.ibm1130.org/sim/apl_source.zip
cd ../binary
curl -k -O http://media.ibm1130.org/sim/aplsetup.zip
```

**1968-FORTH:**
```bash
mkdir -p tmp/downloads/forth
cd tmp/downloads/forth
git clone https://github.com/monsonite/1968-FORTH.git
```

### Option 2: Download Utility Scripts (Planned)

Future utility scripts will automate downloading and format conversion:

```bash
# Planned utilities (not yet implemented)
./scripts/download_apl1130.sh        # Downloads APL to tmp/
./scripts/download_forth1968.sh      # Downloads FORTH to tmp/
./scripts/convert_to_emulator.sh     # Converts to emulator format
```

## License and Fair Use

This project complies with copyright law through:

1. **No Redistribution:** Historical software is NOT included in this git repository
2. **Fair Use:** Local downloads are for educational purposes (analysis, commentary, teaching)
3. **Attribution:** All original authors and preservationists are credited in demo metadata
4. **Educational Content:** Our MIT-licensed commentary and analysis is separate from the historical software

### What IS included (MIT License):
- Demo metadata files (`.toml`)
- Educational descriptions and learning objectives
- Our emulator code (in `crates/`)
- Our web UI (in `crates/yew-ui/`)

### What is NOT included:
- APL\1130 source or binary code
- 1968-FORTH source code
- Any other third-party software with unclear licenses

## Creating Original Demo Content

You can create original demo programs without licensing concerns:

1. Write programs in APL/FORTH/FORTRAN syntax yourself
2. Create TOML metadata files in `data/demos/`
3. These original works are covered by this project's MIT license

Example:
```toml
# data/demos/my_custom_demo.toml
[demo]
id = "my_custom_demo"
title = "My Custom 1130 Program"
language = "FORTRAN"

[source]
code = """
      PROGRAM HELLO
      PRINT *, 'Hello from custom code!'
      END
"""

[attribution]
author = "Your Name"
license = "MIT"
original_work = true
```

## Educational Use and Monetization

This project's approach to historical software follows fair use principles:

**Permitted:**
- Downloading software locally for study and analysis
- Running software in an emulator for educational demonstration
- Creating videos or blog posts analyzing/explaining the software
- Monetizing educational content (ads on videos/blog posts)
- Commentary on historical computing and programming languages

**Not Permitted:**
- Redistributing third-party software binaries in this repo
- Claiming ownership of historical software
- Commercial redistribution of APL\1130, 1968-FORTH, etc.

The monetization model is **indirect**: educational content (videos, blog posts) about
the emulator and historical software, not selling the software itself.

## References and Attribution

### APL\1130
- **Original Author:** IBM (1969)
- **Preservation:** IBM1130.org (Brian Knittel, Norm Aleks)
- **Source Contribution:** Robert Marinelli
- **Binary Contribution:** John Slazenger
- **Download:** http://ibm1130.org/sim/downloads/

### 1968-FORTH
- **Original Author:** Charles Moore (1968)
- **Preservation:** Carl Claunch (documentation and restoration, 2018)
- **Repository:** Ken Boak (monsonite) - https://github.com/monsonite/1968-FORTH
- **Historical Significance:** First implementation of FORTH

## Questions?

See `LICENSE_REVIEW.md` for detailed legal analysis and recommendations.
