import mibllib
import threading
import time
import sys

mibl_rs = None
mibl_thread = None


def update_loop():
    try:
        while True:
            print("Try get value from python :)")
            stamp = mibl_rs.get_rx_stamp()
            print("----:", stamp)
            time.sleep(.4)
    except KeyboardInterrupt:
        print("SIGINT RECIEVE")
        sys.exit()
        print("Exiting…")


def main():
    global mibl_rs
    global mibl_thread

    mibl_rs = mibllib.MiBlRustProcess()

    mibl_thread = threading.Thread(target=mibl_rs.mi_start_server_allow_thread)
    mibl_thread.daemon = True

    mibl_thread.start()

    update_loop()


if __name__ == "__main__":
    main()
