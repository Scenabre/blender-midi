import mibllib
import threading
import time

array = [128, 127, 126]
mibl_rs = mibllib.MiBlRustProcess()

mibl_rs.set_tx(10, bytes(array))
print("Testing Rust Struct content:", list(mibl_rs.get_tx_data()))


def start_server(mibl_rs):
    mibl_rs.mi_start_server()


server_thread = threading.Thread(target=start_server, args=(mibl_rs,))
server_thread.start()

try:
    while True:
        rx_data = mibl_rs.get_rx_data()
        print(f"Received data in Python: {list(rx_data)}")
        time.sleep(1)
except KeyboardInterrupt:
    print("Interrupted by user, stopping server...")
    mibl_rs.toggle_close_thread()
    server_thread.join()
