 # Kiosk
 
 A minimal fullscreen/borderless example intended to be a good starting point for **embedded/kiosk** deployments.
 
 ## Running
 
 ```sh
 cargo run --package kiosk
 ```
 
 ## Hotkeys
 
 - **Esc**: exit
 - **F11**: toggle fullscreen/windowed
 - **Shift+↑**: fullscreen
 - **Shift+↓**: windowed
 
 ## Wayland kiosk (Weston) notes
 
 This example runs on the standard `winit` windowing shell (Wayland/X11). A typical embedded setup is to use a compositor like Weston and run this example fullscreen.
 
 ### Weston config snippet (`weston.ini`)
 
 ```ini
 [core]
 idle-time=0
 
 [shell]
 panel-position=none
 locking=false
 animation=fade
 close-animation=none
 startup-animation=none
 ```
 
 ### systemd unit template (`iced-kiosk.service`)
 
 ```ini
 [Unit]
 Description=Iced kiosk example
 After=weston.service
 Requires=weston.service
 
 [Service]
 Type=simple
 Environment=RUST_LOG=info
 ExecStart=/usr/bin/kiosk
 Restart=always
 RestartSec=1
 
 [Install]
 WantedBy=multi-user.target
 ```
 
