import mibllib
import time
import threading

array = [128, 127, 126]
mibl_rs = mibllib.MiBlRustProcess()

mibl_rs.set_tx(10, bytes(array))
print("Testing Rust Struct content : ", list(mibl_rs.get_tx_data()))

server_thread = threading.Thread(
    target=mibl_rs.mi_start_server()
)

while True:
    rx_data = mibl_rs.get_rx_data()
    print(f"Received data in python: {list(rx_data)}")
    time.sleep(1)

server_thread.start()

time.sleep(30)
