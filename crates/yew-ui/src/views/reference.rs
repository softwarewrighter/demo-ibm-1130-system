use yew::prelude::*;

#[function_component(Reference)]
pub fn reference() -> Html {
    html! {
        <div class="reference-content">
            <section class="reference-intro">
                <h2>{ "IBM 1130 Reference Documentation" }</h2>
                <p>
                    { "Comprehensive collection of manuals, guides, and reference materials for the IBM 1130 Computing System. \
                    All documents are preserved by " }
                    <a href="http://bitsavers.org/pdf/ibm/1130/" target="_blank" rel="noopener noreferrer">{ "Bitsavers.org" }</a>
                    { ", " }
                    <a href="http://ibm1130.org/" target="_blank" rel="noopener noreferrer">{ "IBM1130.org" }</a>
                    { ", and " }
                    <a href="https://ibm1130.net/" target="_blank" rel="noopener noreferrer">{ "IBM1130.net" }</a>
                    { "." }
                </p>
            </section>

            <section class="reference-section">
                <h3>{ "System Documentation" }</h3>
                <div class="reference-links">
                    <div class="reference-item">
                        <h4>
                            <a href="http://bitsavers.org/pdf/ibm/1130/functional_characteristics/A26-5881-2_1130_Functional_Characteristics_1966.pdf"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM 1130 Functional Characteristics" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "A26-5881-2 • 1966" }</p>
                        <p>{ "CPU architecture, instruction set, I/O programming, timing and performance data. Essential reference for system programmers." }</p>
                        <p class="reference-alt">
                            { "Also available: " }
                            <a href="https://ibm1130.net/functional/" target="_blank" rel="noopener noreferrer">{ "HTML version" }</a>
                            { " • " }
                            <a href="http://bitsavers.org/pdf/ibm/1130/functional_characteristics/GA26-5881-6_1130_Functional_Characteristics_Apr72.pdf"
                               target="_blank" rel="noopener noreferrer">{ "1972 edition (GA26-5881-6)" }</a>
                        </p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="http://bitsavers.org/pdf/ibm/1130/GA26-5917-9_1130_System_Summary_Dec71.pdf"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM 1130 System Summary" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "GA26-5917-9 • December 1971" }</p>
                        <p>{ "Complete system overview, configuration options, capacity and speed specifications." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="http://bitsavers.org/pdf/ibm/1130/GA26-5717-1_1130_Operating_Procedures_Aug71.pdf"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM 1130 Operating Procedures" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "GA26-5717-1 • August 1971" }</p>
                        <p>{ "Operator console procedures, device operation, error recovery, and preventive maintenance." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="https://archive.org/details/bitsavers_ibm1130X26ard1968_1436202"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM 1130 Programmer's Reference Card" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "X26-3566-4 • 1968" }</p>
                        <p>{ "Quick reference for assembly programming: instruction formats, opcodes, I/O commands, register usage." }</p>
                    </div>
                </div>
            </section>

            <section class="reference-section">
                <h3>{ "Programming Languages" }</h3>

                <h4 class="language-heading">{ "FORTRAN" }</h4>
                <div class="reference-links">
                    <div class="reference-item">
                        <h4>
                            <a href="http://bitsavers.org/pdf/ibm/1130/lang/GC26-3715-8_1130_1800_Basic_FORTRAN_IV_Language_Jan73.pdf"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM 1130/1800 Basic FORTRAN IV Language" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "GC26-3715-8 • January 1973" }</p>
                        <p>{ "Complete language reference for FORTRAN IV compiler. Includes statement syntax, built-in functions, and I/O operations." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="https://archive.org/details/bitsavers_ibm1130lannLanguage1965_2701742"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM 1130 Fortran Language (1965)" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "C26-5933-2 • 1965" }</p>
                        <p>{ "Earlier edition of the FORTRAN language manual." }</p>
                    </div>
                </div>

                <h4 class="language-heading">{ "APL\\1130" }</h4>
                <div class="reference-links">
                    <div class="reference-item">
                        <h4>
                            <a href="http://bitsavers.org/pdf/ibm/1130/lang/1130-03.3.001_APL_1130_May69.pdf"
                               target="_blank" rel="noopener noreferrer">
                                { "APL\\1130 Manual" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "1130-03.3.001 • May 1969" }</p>
                        <p>{ "User manual for APL\\1130, the first publicly available APL system. Includes language syntax, operators, and system commands." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="http://bitsavers.informatik.uni-stuttgart.de/pdf/ibm/apl/C20-1697-0_APL_1130_Primer_1968.pdf"
                               target="_blank" rel="noopener noreferrer">
                                { "APL\\1130 Primer" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "C20-1697-0 • 1968" }</p>
                        <p>{ "Introductory tutorial for APL\\1130 by Paul Berry. Excellent starting point for learning APL on the 1130." }</p>
                    </div>
                </div>

                <h4 class="language-heading">{ "Assembly Language" }</h4>
                <div class="reference-links">
                    <div class="reference-item">
                        <h4>
                            <a href="http://ibm1130.org/lib/manuals/"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM 1130 Assembler Language Manual" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "GC26-5927 • IBM1130.org Library" }</p>
                        <p>{ "Macro assembler reference. Search IBM1130.org manuals library for available PDF." }</p>
                    </div>
                </div>
            </section>

            <section class="reference-section">
                <h3>{ "Disk Monitor System (DMS)" }</h3>
                <div class="reference-links">
                    <div class="reference-item">
                        <h4>
                            <a href="http://bitsavers.org/pdf/ibm/1130/monitor/C26-3717-9_1130_1130_Disk_Monitor_System_Version_2_Programming_and_Operators_Guide_May72.pdf"
                               target="_blank" rel="noopener noreferrer">
                                { "DMS Version 2 Programming and Operator's Guide" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "C26-3717-9 • May 1972" }</p>
                        <p>{ "Complete guide to DMS commands, job control, file system operations, batch processing, and system generation." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="http://bitsavers.org/pdf/ibm/1130/monitor/C26-3750-0_1130_Disk_Monitor_System_Reference_Manual_1966.pdf"
                               target="_blank" rel="noopener noreferrer">
                                { "DMS Reference Manual" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "C26-3750-0 • 1966" }</p>
                        <p>{ "Technical reference for DMS internals and system programming." }</p>
                    </div>
                </div>
            </section>

            <section class="reference-section">
                <h3>{ "Subroutine Libraries" }</h3>
                <div class="reference-links">
                    <div class="reference-item">
                        <h4>
                            <a href="https://archive.org/details/bitsavers_ibm1130sub0ScientificSubroutinePackageApplicationD_1025310"
                               target="_blank" rel="noopener noreferrer">
                                { "Scientific Subroutine Package (SSP)" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "H20-0225-0 • 1966" }</p>
                        <p>{ "Matrix operations, differential equations, statistical functions, Fourier transforms, and numerical analysis routines." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="https://archive.org/details/bitsavers_ibm1130sub0CommercialSubroutinePackageVer2Applicat_427088"
                               target="_blank" rel="noopener noreferrer">
                                { "Commercial Subroutine Package Version 2" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "H20-0221-2 • 1967" }</p>
                        <p>{ "Business calculations, date/time functions, report formatting, and financial routines." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="https://archive.org/details/bitsavers_ibm1130subneticTapeSubroutinesForAssemblerandFortr_3135907"
                               target="_blank" rel="noopener noreferrer">
                                { "Magnetic Tape Subroutines" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "00.3.003" }</p>
                        <p>{ "Tape I/O routines for Assembler and FORTRAN programs." }</p>
                    </div>
                </div>
            </section>

            <section class="reference-section">
                <h3>{ "Program Catalogs" }</h3>
                <div class="reference-links">
                    <div class="reference-item">
                        <h4>
                            <a href="https://archive.org/details/bitsavers_ibm1130pro0ProgramCatalogDec66_1891857"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM 1130 Program Catalog" }
                            </a>
                        </h4>
                        <p class="reference-meta">{ "C20-1630-1 • December 1966" }</p>
                        <p>{ "Type-III Library programs and applications by category: business, engineering, scientific, educational, and utilities." }</p>
                    </div>
                </div>
            </section>

            <section class="reference-section">
                <h3>{ "Online Resources" }</h3>
                <div class="reference-links">
                    <div class="reference-item">
                        <h4>
                            <a href="http://ibm1130.org/"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM1130.org" }
                            </a>
                        </h4>
                        <p>{ "Primary preservation site with simulator, software downloads, and extensive manual library." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="https://ibm1130.net/functional/"
                               target="_blank" rel="noopener noreferrer">
                                { "IBM1130.net Functional Characteristics" }
                            </a>
                        </h4>
                        <p>{ "Interactive HTML version of the Functional Characteristics manual with searchable instruction set." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="http://bitsavers.org/pdf/ibm/1130/"
                               target="_blank" rel="noopener noreferrer">
                                { "Bitsavers IBM 1130 Archive" }
                            </a>
                        </h4>
                        <p>{ "Comprehensive collection of scanned IBM 1130 documentation and software." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="https://github.com/simh/simh/tree/master/Ibm1130"
                               target="_blank" rel="noopener noreferrer">
                                { "SIMH IBM 1130 Simulator" }
                            </a>
                        </h4>
                        <p>{ "Open-source simulator with sample programs and cross-development tools." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="https://aplwiki.com/wiki/APL%5C1130"
                               target="_blank" rel="noopener noreferrer">
                                { "APL Wiki - APL\\1130" }
                            </a>
                        </h4>
                        <p>{ "Historical information about APL\\1130, the first publicly available APL system." }</p>
                    </div>

                    <div class="reference-item">
                        <h4>
                            <a href="https://github.com/monsonite/1968-FORTH"
                               target="_blank" rel="noopener noreferrer">
                                { "1968 Forth for IBM 1130" }
                            </a>
                        </h4>
                        <p>{ "Original Forth implementation by Charles Moore, preserved and documented on GitHub." }</p>
                    </div>
                </div>
            </section>

            <section class="reference-footer">
                <p>
                    <strong>{ "Note:" }</strong>
                    { " Some links point to external archives and preservation sites. If a link is unavailable, \
                    try searching the " }
                    <a href="http://ibm1130.org/lib/manuals/" target="_blank" rel="noopener noreferrer">{ "IBM1130.org manuals library" }</a>
                    { " or " }
                    <a href="http://bitsavers.org/pdf/ibm/" target="_blank" rel="noopener noreferrer">{ "Bitsavers IBM archives" }</a>
                    { "." }
                </p>
            </section>
        </div>
    }
}
