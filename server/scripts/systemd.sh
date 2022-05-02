sudo tee /etc/systemd/system/mengs.service >/dev/null << EOF
[Unit] 
Description=Mengs
After=network.target 
After=network-online.target 
After=wg-quick@wg0.service
[Service] 
ExecStart=sudo RUST_LOG=info /home/ubuntu/mengs/mengs /home/ubuntu/mengs/www
TimeoutSec=30 
Restart=always
RestartSec=30
StartLimitInterval=350 
StartLimitBurst=10 
[Install] 
WantedBy=multi-user.target
EOF
