import subprocess
import time
from binascii import a2b_uu
from enum import Enum

class DeviceLockState(Enum):
    LOCKED_SCREEN_OFF = 0
    LOCKED_SCREEN_ON = 1
    UNLOCKED_SCREEN_OFF = 2
    UNLOCKED_SCREEN_ON = 3


class Device:
    def __init__(self, serial):
        self.serial = serial

    def adb(self, cmd):
        return subprocess.check_output(["adb", "-s", self.serial] + cmd)

    def getprop(self, prop):
        return self.adb(["shell", "getprop", prop]).strip()

    def touch(self, x, y):
        self.adb(["shell", "input", "tap", str(x), str(y)])

    def swipe(self, x1, y1, x2, y2, duration=300):
        self.adb(["shell", "input", "swipe", str(x1), str(y1), str(x2), str(y2), str(duration)])

    def device_lock_state(self):
        power_state = self.adb(["shell", "dumpsys", "power"]).split()
        lockscreen_state = self.adb(["shell", "dumpsys", "window"]).split()
        mWakefulness = [line for line in power_state if b"mWakefulness=" in line][0]
        mShowingLockscreen = [line for line in lockscreen_state if b"mShowingLockscreen=" in line][0]

        if b"mWakefulness=Asleep" in mWakefulness:
            if b"mShowingLockscreen=true" in mShowingLockscreen:
                return DeviceLockState.LOCKED_SCREEN_OFF
            else:
                return DeviceLockState.UNLOCKED_SCREEN_OFF
        elif b"mWakefulness=Awake" in mWakefulness:
            if b"mShowingLockscreen=true" in mShowingLockscreen:
                return DeviceLockState.LOCKED_SCREEN_ON
            else:
                return DeviceLockState.UNLOCKED_SCREEN_ON

    def wake(self):
        self.adb(["shell", "input", "keyevent", "26"])

    def is_activity_running(self, package, activity):
        activities = self.adb(["shell", "dumpsys", "activity", "activities"]).split()
        return bytes(f"realActivity={package}/{activity}", "ascii") in activities

    def start_activity(self, package, activity):
        self.adb(["shell", "am", "start", "-n", package + "/" + activity])

def logprint(*args, **kwargs):
    print("[+]", *args, **kwargs)


sony = Device("ZH80014NDW")

logprint("Device lock state:", sony.device_lock_state())
logprint("Device           :", sony.getprop("ro.product.model"))
logprint("Android          :", sony.getprop("ro.build.version.release"))
logprint("Kernel           :", sony.getprop("ro.build.version.incremental"))

# Set the screen timeout to 30 minutes
logprint("Setting screen timeout to 30 minutes")
sony.adb(["shell", "settings", "put", "system", "screen_off_timeout", "1800000"])


if sony.device_lock_state() == DeviceLockState.LOCKED_SCREEN_OFF or sony.device_lock_state() == DeviceLockState.UNLOCKED_SCREEN_OFF:
    logprint("Screen is off, turning on")
    sony.wake()
    time.sleep(0.5)

if sony.is_activity_running("com.mojang.minecraftpe", ".MainActivity"):
    logprint("Minecraft is running, closing")
    sony.adb(["shell", "am", "force-stop", "com.mojang.minecraftpe"])
    time.sleep(1.0)

sony.start_activity("com.mojang.minecraftpe", ".MainActivity")

logprint("Waiting for Minecraft to start")
while not sony.is_activity_running("com.mojang.minecraftpe", ".MainActivity"):
    time.sleep(0.5)

logprint("Waiting for Minecraft to load")
