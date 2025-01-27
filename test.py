import mibllib
import time
import threading

array = [128, 127, 126]
mibl_rs = mibllib.MiBlRustProcess()

mibl_rs.set_tx(10, bytes(array))
print("Testing Rust Struct content : ", list(mibl_rs.get_tx()))

# mibllib.mi_start_server(mibl_rs)

# Start the MIDI server in a separate thread
server_thread = threading.Thread(
    target=mibllib.mi_start_server,
    args=(mibl_rs,)
)

# Periodically call get_rx
while True:
    rx_data = mibl_rs.get_rx()
    print(f"Received data: {list(rx_data)}")
    time.sleep(1)  # Adjust the sleep time as needed

server_thread.start()

# time.sleep(30)
