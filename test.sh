cd Actix_Backend && cargo run &
cd GUI/lpm_gui && npm start &
gnome-terminal --tab -e "bash -c 'cd CLI/tuipmgr && cargo run'"
