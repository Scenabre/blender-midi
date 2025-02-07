import mibllib
import threading
import time


def main():
    mibl_rs = mibllib.MiBlRustProcess()

    test = threading.Thread(target=mibl_rs.mi_start_server_allow_thread)
    test.start()

    try:
        while True:
            print("Try get value from python :)")
            stamp = mibl_rs.get_rx_stamp()
            print("----:", stamp)
            time.sleep(.4)
    except KeyboardInterrupt:
        print("SIGINT RECIEVE")
        test.join()
        print("Exiting…")


if __name__ == "__main__":
    main()
