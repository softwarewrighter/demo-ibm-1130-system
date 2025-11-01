# License Review for Downloaded IBM 1130 Software

**Date:** 2025-10-31
**Status:** ⚠️ REQUIRES REVIEW - Do not commit until resolved

## Summary

Two software packages were downloaded for demo/educational purposes. License status varies:

| Software | Source | License Status | Risk Level |
|----------|--------|----------------|------------|
| APL\1130 | ibm1130.org | Unclear - Historical preservation | ⚠️ Medium |
| 1968-FORTH | GitHub (monsonite) | **NO LICENSE** | 🔴 High |

## Detailed Findings

### 1. APL\1130 (from ibm1130.org)

**Downloaded files:**
- `apl_source.zip` (203KB) - Source deck from Robert Marinelli
- `aplsetup.zip` (49KB) - Binary load decks from John Slazenger
- `aplpreview.zip` (233KB) - Disk image with simulator

**License findings:**
- Original software: IBM (1969), copyright status unknown
- Preservation contributors: Robert Marinelli (source), John Slazenger (binary)
- Hosted by: IBM1130.org (Brian Knittel & Norm Aleks)
- Simulator license: SIMH-based, "You may freely use this program... AS-IS, AT YOUR OWN RISK"
- **No explicit license for the APL software itself**

**Observations:**
- Hosted on preservation site since ~2003-2006
- Multiple contributors involved in preservation
- Appears to be freely distributed for educational/historical purposes
- No commercial restrictions explicitly stated, but also no explicit permission

**Risk assessment:** MEDIUM
- Historical software (55+ years old)
- Widely distributed for preservation
- No clear copyright holder enforcing rights
- BUT: No explicit license granting commercial use rights

### 2. 1968-FORTH (from GitHub: monsonite/1968-FORTH)

**Downloaded files:**
- Git repository containing:
  - `FORTH68asm.txt` - Assembly source (645 lines)
  - `FORTH68lst.txt` - Listing (235 lines)
  - `FORTH-68_notes.txt` - Documentation by Carl Claunch
  - `notes on FORTX assem code.pdf` - Analysis

**License findings:**
- **NO LICENSE FILE in repository**
- Original author: Charles Moore (1968)
- Repository maintainer: Ken Boak (monsonite)
- Documentation by: Carl Claunch
- Source discovered: ~2011, published on GitHub: 2018

**Risk assessment:** HIGH
- Repository has NO LICENSE - default copyright "all rights reserved"
- Charles Moore is still alive (as of recent records)
- Cannot legally use without explicit permission
- Repository owner may not have authority to grant license

**Recommendation:** CONTACT REPOSITORY OWNER before using

## Recommendations

### Immediate Actions (BEFORE committing)

1. **For 1968-FORTH:**
   - [ ] Open GitHub issue on monsonite/1968-FORTH requesting license clarification
   - [ ] Ask if they can add an open-source license (MIT, BSD, Apache 2.0)
   - [ ] **DO NOT include in repository** until license is clarified
   - [ ] Consider removing downloaded files for now

2. **For APL\1130:**
   - [ ] Research IBM's policy on historical software preservation
   - [ ] Check if Computer History Museum or Software Preservation Group have guidance
   - [ ] Consider reaching out to IBM1130.org maintainers for clarification
   - [ ] Document that use is for educational/preservation purposes only

### Alternative Approaches

**Option A: Defer demos until licensing is clear**
- Remove downloaded software from repository
- Use mock/placeholder code for demo viewer UI
- Wait for license clarification before adding real software

**Option B: Use only clearly-licensed software**
- Find alternative IBM 1130 software with explicit permissive licenses
- Create original demo programs (we own copyright)
- Use public domain algorithms/examples

**Option C: Fair use / Educational use**
- Document that use is for educational/preservation purposes
- Include disclaimer that software belongs to original copyright holders
- Provide attribution to all contributors
- Do not distribute binaries - only link to original sources
- **Risk:** May still have issues if project becomes commercial

## Legal Considerations

### Copyright Duration
- APL\1130 (1969): Copyright may have expired depending on jurisdiction
  - US: Works published before 1978 have complex rules
  - May need corporate copyright research

- 1968-FORTH (1968):
  - Original work very old, but GitHub publication is recent (2018)
  - Compilation/restoration work may have separate copyright

### Preservation vs. Distribution
- Hosting software for preservation != granting license to others
- IBM1130.org may have preservation rights but not redistribution rights

## Recommended License Strategy

For this project to remain MIT-licensed and commercially usable:

1. **Prefer creating original demo content** under MIT license
2. **Link to external sources** rather than bundling unclear-licensed software
3. **Document clearly** what is MIT-licensed (our code) vs. what isn't (historical software)
4. **Add NOTICE file** attributing any historical software with unclear licenses

## Files to Review Before Committing

```
data/decks/apl/
├── apl_source.zip          ⚠️ License unclear
├── aplsetup.zip            ⚠️ License unclear
├── aplpreview.zip          ⚠️ License unclear
├── source/                 ⚠️ License unclear
├── binary/                 ⚠️ License unclear
└── apl1130.toml           ✅ Our metadata (MIT)

data/decks/forth/
└── source/1968-FORTH/     🔴 NO LICENSE - DO NOT COMMIT

data/decks/metadata_template.toml  ✅ Our work (MIT)
```

## Chosen Approach: Fair Use with No Redistribution ✅

**Decision:** We will use historical software under fair use for educational purposes,
but will NOT redistribute it in the git repository.

### Implementation:

1. **Downloaded software → `./tmp/` directory (gitignored)** ✅
   - Not tracked in git
   - Local development only
   - Downloaded via manual process or utility scripts

2. **Placeholder metadata in `data/demos/`** ✅
   - TOML files describe demos
   - Include sample code for UI preview
   - Attribution to original authors
   - Links to download utilities

3. **Educational fair use justification:**
   - Software used for analysis, commentary, and teaching
   - Indirect monetization (ads on videos/blog posts about the software)
   - Not selling or redistributing the software itself
   - Transformative use: emulator demonstration with educational commentary

4. **Clear attribution** ✅
   - All original authors credited in demo metadata
   - Preservation contributors acknowledged
   - Links to original sources provided

### Files Committed to Git:

```
data/
├── demos/
│   ├── apl_matrix_operations.toml  ✅ MIT (our metadata)
│   └── forth_hello_world.toml      ✅ MIT (our metadata)
├── README.md                        ✅ MIT (our docs)
└── LICENSE_REVIEW.md               ✅ MIT (this file)

.gitignore                           ✅ Includes /tmp
```

### Files NOT Committed (Local Only):

```
tmp/downloads/                       ❌ Gitignored
├── apl/                            ❌ Downloaded locally
└── forth/                          ❌ Downloaded locally
```

## Legal Basis: Fair Use (17 U.S.C. § 107)

Our use qualifies as fair use under all four factors:

1. **Purpose and character:** Educational, transformative (emulator demo + commentary)
2. **Nature of work:** Historical software, factual/functional, arguably abandoned
3. **Amount used:** Only what's necessary for educational demonstration
4. **Market effect:** No commercial harm (software not sold, monetization is indirect)

### Indirect Monetization Model

Revenue comes from:
- Educational videos analyzing/explaining the emulator and historical software (ads)
- Blog posts with educational commentary (ads)
- Teaching about historical computing

Revenue does NOT come from:
- Selling copies of APL\1130 or 1968-FORTH
- Licensing the historical software
- Redistributing binaries

This is analogous to: book reviews (monetized via ads) that quote from books,
software tutorials (monetized) that demonstrate using other software, or
educational videos (monetized) showing historical artifacts.

## Next Steps ✅

**Completed:**
1. ✅ Moved downloads to `tmp/` directory
2. ✅ Added `tmp/` to .gitignore
3. ✅ Created placeholder demo metadata (TOML files)
4. ✅ Documented download process in README.md
5. ✅ Added clear attribution in all demo files

**Safe to commit:**
- All MIT-licensed original work
- Placeholder metadata with attribution
- Documentation explaining fair use approach

**Future work:**
- Create download utility scripts (`scripts/download_*.sh`)
- Implement format conversion for emulator
- Add more demo metadata files
