import mibllib
import threading
import time
import sys

mibl_rs = None
mibl_thread = None


def update_loop(thread):
    count = 0
    try:
        while True:
            signal = mibl_rs.get_close_signal()

            if signal:
                print("Getting stop signal !")
                thread.join()
                sys.exit()

            # print("Try get value from python :)")
            # stamp = mibl_rs.get_rx_stamp()
            # print("----:", stamp)

            # if count == 100:
            #     mibl_rs.set_close_signal(True)

            time.sleep(.4)
            count += 1
    except KeyboardInterrupt:
        print("SIGINT RECEIVE")
        mibl_rs.set_close_signal(True)
        thread.join()
        print("Exiting…")


def main():
    global mibl_rs
    global mibl_thread

    mibl_rs = mibllib.MiBlRustProcess()

    mibl_thread = threading.Thread(target=mibl_rs.mi_start_server_allow_thread)
    # mibl_thread.daemon = True

    mibl_thread.start()

    update_loop(mibl_thread)


if __name__ == "__main__":
    main()
