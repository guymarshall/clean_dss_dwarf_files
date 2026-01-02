<h2>clean_dss_dwarf_files</h2>

Remove temporary files left behind after shooting with the Dwarf 3 Smart Telescope, and after stacking with Deep Sky Stacker (DSS).

<h2>What does it do?</h2>
It deletes the following files, directories, and patterns:
<h3>Files</h3>
<ul>
    <li>"img_reference.png"</li>
    <li>"img_stacked_all.tif"</li>
    <li>"img_stacked_counter.png"</li>
    <li>"shotsInfo.json"</li>
    <li>"*stacked*"</li>
    <li>"*.info.txt"</li>
    <li>"*.stackinfo.txt"</li>
    <li>"failed*"</li>
</ul>
<h3>Directories</h3>
<ul>
    <li>"Thumbnail"</li>
</ul>

<h2>How do I run it?</h2>
You need Rust installed to compile and run this program. Please follow the instructions to install it from: <a href="https://www.rust-lang.org/tools/install">https://www.rust-lang.org/tools/install</a>
<br>
Once installed, you simply download this project, extract it into a directory, open a terminal, navigate to the directory, and run <code>cargo build --release</code>. This will create an executable in <code>clean_dss_dwarf_files/target/release/</code> called <code>clean_dss_dwarf_files</code>. Just move that executable to a directory with your space projects in and run it to clean the temporary files!