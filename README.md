# The_Linux_Process_Manager

## To run the program, all together
1. Download the zip file and extract it.
2. Open the terminal and navigate to the folder where the files are extracted.
3. Simply, run `./test.sh` command

## To Launch GUI on it own
1. Navigate to the folder: *"Actix_Backend"*
2. Run the command `cargo run` for the backend of the GUI to render the data.

1. To run the GUI with the backend, open another terminal and navigate to the folder:*"GUI/lpm_gui"*
2. Run the command `npm install` to install the dependencies.
3. Run the command `npm start` to start the GUI.
4. Open the browser and go to *"localhost:3000"* to view the GUI.

## To Run the TUI on its own
1. Open the te rminal and navigate to the folder:*"CLI/tuipmgr"*
2. Run the command `cargo run`.

    To move around the TUI:
    1. Press on the 'd' button (case insensitive) to display the table of processes
    2. Press on the 'h' button (case insensitive) to navigate back to homepage
    3. When displaying the table you can press the following buttons:
        1. 's' : to sort
            1. use the arrow keys right and left to select the field you would like to sort on, press enter to see the results
            2. use 'x' to exit this option
        2. 'e' : to search
            1. Use the keyboard to type and press esc key to exit input entry, then press 'x' to exit this option
        3.  'f': to filter
            1. Use the keyboard to type and press esc key to exit input entry, then press 'x' to exit this option
        4. 't': to terminate
            1.  use 'x' to exit this option
        5. 'p' : to set priority 
        6. Press 'q' button to exit the program



