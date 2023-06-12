#To run the program, 
    +1. Download the zip file and extract it.
    +2. Open the terminal and navigate to the folder where the files are extracted.

#To Launch GUI
    +1** Navigate to the folder:"Actix_Backend"
    +2**. Run the command "cargo run" for the backend of the GUI to render the data.

    +1. To run the GUI with the backend, open another terminal and navigate to the folder:"GUI/lpm_gui"
    +2. Run the command "npm install" to install the dependencies.
    +3. Run the command "npm start" to start the GUI.
    +4. Open the browser and go to "localhost:3000" to view the GUI.

#To Run the TUI
    +1. Open the te rminal and navigate to the folder:"CLI/tuipmgr"
    +2. Run the command "cargo run".

    To move around the TUI:
        1. Press on the 'd' button (case insensitive) to display the table of processes
        2. Press on the 'h' button (case insensitive) to navigate back to homepage
        3. When displaying the table you can press the following buttons:
            a. 's' : to sort
                i. use the arrow keys right and left to select the field you would like to sort on, press enter to see the results
                ii. use 'x' to exit this option
            b. 'e' : to search
                i. Use the keyboard to type and press esc key to exit input entry, then press 'x' to exit this option
            c. 'f': to filter
                i. Use the keyboard to type and press esc key to exit input entry, then press 'x' to exit this option
            d. 't': to terminate
                i. use 'x' to exit this option
            e. 'p' : to set priority 
        4. Press 'q' button to exit the program



