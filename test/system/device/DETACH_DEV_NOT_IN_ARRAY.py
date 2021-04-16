#!/usr/bin/env python3
import subprocess
import os
import sys
sys.path.append("../lib/")
sys.path.append("../volume/")
sys.path.append("../array/")

import json_parser
import pos
import pos_util
import cli
import test_result
import json
import time
import MOUNT_VOL_NO_SPARE
import CREATE_ARRAY_NO_SPARE
DETACH_TARGET_DEV = CREATE_ARRAY_NO_SPARE.REMAINING_DEV


def check_result():
    out = cli.get_pos_info()
    data = json.loads(out)
    if data['Response']['info']['state'] == "NORMAL":
        list = cli.list_device()
        data = json.loads(list)
        for item in data['Response']['result']['data']['devicelist']:
            if item['name'] == DETACH_TARGET_DEV :
                return "fail", list
        return "pass", out
    return "fail", out

def set_result():
    result, out = check_result()

    with open(__file__ + ".result", "w") as result_file:
        result_file.write(result + "\n" + out)

def execute():
    MOUNT_VOL_NO_SPARE.execute()
    pos_util.pci_detach(DETACH_TARGET_DEV)
    time.sleep(0.1)

if __name__ == "__main__":
    test_result.clear_result(__file__)
    execute()
    set_result()
    pos.kill_pos()
    pos_util.pci_rescan()